//! Native DualShock 4 input and the launcher's controller profile.

use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock, mpsc};

use windows_sys::Win32::Foundation::{HANDLE, HWND};
use windows_sys::Win32::UI::Input::*;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, GIDC_ARRIVAL, GIDC_REMOVAL, GetMessageW,
    HWND_MESSAGE, MSG, WM_INPUT, WM_INPUT_DEVICE_CHANGE,
};

use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum Input {
    #[default]
    Auto,
    DualShock,
    XInput,
    Keyboard,
}

const BUTTONS: usize = 14;
const IDENTITY: [u8; BUTTONS] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
const UNBOUND: u8 = BUTTONS as u8;

pub(super) struct Profile {
    pub(super) source: Input,
    buttons: [u8; BUTTONS],
    deadzone: f32,
    invert_left_y: bool,
    invert_right_y: bool,
    swap_sticks: bool,
}

impl Default for Profile {
    fn default() -> Self {
        let path = std::env::var_os("VICTORY_CONTROLLER_CONFIG")
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                let root = std::env::var_os("LOCALAPPDATA")?;
                Some(PathBuf::from(root).join("VIctory-RDNA").join("controller.ini"))
            });
        Self::parse(
            &path.and_then(|path| std::fs::read_to_string(path).ok()).unwrap_or_default(),
        )
    }
}

impl Profile {
    fn parse(text: &str) -> Self {
        let mut profile = Self {
            source: Input::Auto,
            buttons: IDENTITY,
            deadzone: 0.0,
            invert_left_y: false,
            invert_right_y: false,
            swap_sticks: false,
        };
        for (key, value) in text.lines().filter_map(|line| line.split_once('=')) {
            let value = value.trim();
            match key.trim() {
                "source" => profile.source = match value {
                    "dualshock" => Input::DualShock,
                    "xinput" => Input::XInput,
                    "keyboard" => Input::Keyboard,
                    _ => Input::Auto,
                },
                "buttons" => {
                    let parsed: Result<Vec<u8>, _> = value.split(',').map(str::parse).collect();
                    if let Ok(parsed) = parsed
                        && let Ok(buttons) = <[u8; BUTTONS]>::try_from(parsed)
                        && buttons.iter().all(|&button| button <= UNBOUND)
                    {
                        profile.buttons = buttons;
                    }
                }
                "deadzone" => {
                    if let Ok(deadzone) = value.parse::<f32>()
                        && (0.0..=0.5).contains(&deadzone)
                    {
                        profile.deadzone = deadzone;
                    }
                }
                "invert_left_y" => profile.invert_left_y = value == "1",
                "invert_right_y" => profile.invert_right_y = value == "1",
                "swap_sticks" => profile.swap_sticks = value == "1",
                _ => {}
            }
        }
        profile
    }

    pub(super) fn apply(&self, mut pad: PadState) -> PadState {
        if self.buttons != IDENTITY {
            let pressed = pad.buttons;
            pad.buttons = self.buttons.iter().enumerate().fold(0, |mask, (target, &source)| {
                if source != UNBOUND && pressed & (1 << source) != 0 {
                    mask | (1 << target)
                } else {
                    mask
                }
            });
        }
        if self.swap_sticks {
            std::mem::swap(&mut pad.left_x, &mut pad.right_x);
            std::mem::swap(&mut pad.left_y, &mut pad.right_y);
        }
        if self.deadzone > 0.0 {
            let adjust = |x: &mut f32, y: &mut f32| {
                let length = x.hypot(*y);
                let scale = if length <= self.deadzone {
                    0.0
                } else {
                    (length - self.deadzone) / ((1.0 - self.deadzone) * length)
                };
                *x = (*x * scale).clamp(-1.0, 1.0);
                *y = (*y * scale).clamp(-1.0, 1.0);
            };
            adjust(&mut pad.left_x, &mut pad.left_y);
            adjust(&mut pad.right_x, &mut pad.right_y);
        }
        if self.invert_left_y {
            pad.left_y = -pad.left_y;
        }
        if self.invert_right_y {
            pad.right_y = -pad.right_y;
        }
        pad
    }
}

/// USB and basic Bluetooth reports start immediately after the report ID.
/// Extended Bluetooth reports have two additional transport bytes.
fn decode(report: &[u8]) -> Option<PadState> {
    let offset = match report.first()? {
        0x01 => 1,
        0x11 => 3,
        _ => return None,
    };
    let [lx, ly, rx, ry, face, shoulders, _, lt, rt] =
        *report.get(offset..)?.first_chunk::<9>()?;
    let dpad = match face & 0x0f {
        0 => UP,
        1 => UP | RIGHT,
        2 => RIGHT,
        3 => DOWN | RIGHT,
        4 => DOWN,
        5 => DOWN | LEFT,
        6 => LEFT,
        7 => UP | LEFT,
        _ => 0,
    };
    let buttons = [
        (face & 0x10, X), (face & 0x20, A), (face & 0x40, B), (face & 0x80, Y),
        (shoulders & 0x01, LEFT_SHOULDER), (shoulders & 0x02, RIGHT_SHOULDER),
        (shoulders & 0x10, VIEW), (shoulders & 0x20, MENU),
        (shoulders & 0x40, LEFT_STICK), (shoulders & 0x80, RIGHT_STICK),
    ];
    let stick = |value: u8| {
        let value = f32::from(value) - 128.0;
        (value / 127.0).max(-1.0)
    };
    Some(PadState {
        buttons: buttons.iter().filter(|(pressed, _)| *pressed != 0)
            .fold(dpad, |mask, (_, button)| mask | button),
        left_x: stick(lx),
        left_y: -stick(ly),
        right_x: stick(rx),
        right_y: -stick(ry),
        left_trigger: f32::from(lt) / 255.0,
        right_trigger: f32::from(rt) / 255.0,
    })
}

static SONY: Mutex<BTreeMap<usize, PadState>> = Mutex::new(BTreeMap::new());

pub(super) fn dualshock_pads() -> [Option<PadState>; PADS] {
    static STARTED: OnceLock<()> = OnceLock::new();
    STARTED.get_or_init(|| {
        let (ready, started) = mpsc::sync_channel(1);
        match std::thread::Builder::new().name("victory-dualshock".into()).spawn(move || receive(ready)) {
            Ok(_) => {
                if let Err(error) = started.recv().unwrap_or_else(|_| Err(io::Error::other("input thread exited"))) {
                    tracing::warn!(%error, "DualShock input unavailable");
                }
            }
            Err(error) => tracing::warn!(%error, "could not start DualShock input"),
        }
    });
    let sony = lock(&SONY);
    let mut pads = sony.values().copied();
    std::array::from_fn(|_| pads.next())
}

fn add_device(handle: HANDLE) {
    let mut info = RID_DEVICE_INFO { cbSize: size_of::<RID_DEVICE_INFO>() as u32, ..Default::default() };
    let mut size = info.cbSize;
    // SAFETY: the API fills the supplied RID_DEVICE_INFO structure.
    if unsafe { GetRawInputDeviceInfoW(handle, RIDI_DEVICEINFO, (&mut info as *mut RID_DEVICE_INFO).cast(), &mut size) }
        == u32::MAX || info.dwType != RIM_TYPEHID
    {
        return;
    }
    // SAFETY: dwType selected the HID union member.
    let hid = unsafe { info.Anonymous.hid };
    if hid.dwVendorId == 0x054c && matches!(hid.dwProductId, 0x05c4 | 0x09cc)
        && hid.usUsagePage == 1 && matches!(hid.usUsage, 4 | 5)
    {
        lock(&SONY).entry(handle as usize).or_default();
    }
}

fn enumerate_devices() {
    let mut count = 0;
    let size = size_of::<RAWINPUTDEVICELIST>() as u32;
    // SAFETY: a null buffer queries the count.
    if unsafe { GetRawInputDeviceList(std::ptr::null_mut(), &mut count, size) } != 0 || count == 0 {
        return;
    }
    let mut devices = vec![RAWINPUTDEVICELIST::default(); count as usize];
    // SAFETY: devices has room for count entries.
    let found = unsafe { GetRawInputDeviceList(devices.as_mut_ptr(), &mut count, size) };
    if found != u32::MAX {
        for device in devices.into_iter().take(found as usize).filter(|d| d.dwType == RIM_TYPEHID) {
            add_device(device.hDevice);
        }
    }
}

fn read_input(raw: HRAWINPUT) {
    let header_size = size_of::<RAWINPUTHEADER>();
    let mut header = RAWINPUTHEADER::default();
    let mut size = header_size as u32;
    // SAFETY: header is writable and raw belongs to the current WM_INPUT.
    let read = unsafe { GetRawInputData(raw, RID_HEADER, (&mut header as *mut RAWINPUTHEADER).cast(), &mut size, header_size as u32) };
    if read != header_size as u32 || header.dwType != RIM_TYPEHID {
        return;
    }
    let mut sony = lock(&SONY);
    let Some(pad) = sony.get_mut(&(header.hDevice as usize)) else { return };
    let mut packet = [0usize; 256];
    let mut size = size_of_val(&packet) as u32;
    // SAFETY: packet has the supplied capacity and native word alignment.
    let read = unsafe { GetRawInputData(raw, RID_INPUT, packet.as_mut_ptr().cast(), &mut size, header_size as u32) };
    if read <= header_size as u32 + 8 || read as usize > size_of_val(&packet) {
        return;
    }
    // SAFETY: read checked against the initialized packet storage.
    let bytes = unsafe { std::slice::from_raw_parts(packet.as_ptr().cast::<u8>(), read as usize) };
    let hid = &bytes[header_size..];
    let report_size = u32::from_le_bytes(hid[..4].try_into().unwrap()) as usize;
    let reports = u32::from_le_bytes(hid[4..8].try_into().unwrap()) as usize;
    if report_size == 0 {
        return;
    }
    if let Some(state) = hid[8..].chunks_exact(report_size).take(reports).filter_map(decode).last() {
        *pad = state;
    }
}

fn receive(ready: mpsc::SyncSender<io::Result<()>>) {
    // SAFETY: use a built-in window class with a message-only parent.
    let window = unsafe {
        CreateWindowExW(0, windows_sys::w!("STATIC"), std::ptr::null(), 0, 0, 0, 0, 0,
            HWND_MESSAGE, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null())
    };
    let devices = [4, 5].map(|usage| RAWINPUTDEVICE {
        usUsagePage: 1, usUsage: usage, dwFlags: RIDEV_INPUTSINK | RIDEV_DEVNOTIFY, hwndTarget: window,
    });
    // SAFETY: devices contains valid registrations targeting our window.
    if window.is_null() || unsafe { RegisterRawInputDevices(devices.as_ptr(), devices.len() as u32, size_of::<RAWINPUTDEVICE>() as u32) } == 0 {
        let _ = ready.send(Err(io::Error::last_os_error()));
    } else {
        enumerate_devices();
        let _ = ready.send(Ok(()));
        let mut message = MSG::default();
        // SAFETY: the queue and window belong to this thread.
        while unsafe { GetMessageW(&mut message, window, 0, 0) } > 0 {
            match message.message {
                WM_INPUT => read_input(message.lParam as HRAWINPUT),
                WM_INPUT_DEVICE_CHANGE => match message.wParam as u32 {
                    GIDC_ARRIVAL => add_device(message.lParam as HANDLE),
                    GIDC_REMOVAL => { lock(&SONY).remove(&(message.lParam as usize)); }
                    _ => {}
                },
                _ => {}
            }
            // SAFETY: let the default window procedure clean up the message.
            unsafe { DefWindowProcW(message.hwnd, message.message, message.wParam, message.lParam) };
        }
    }
    if !window.is_null() {
        // SAFETY: our message-only window.
        unsafe { DestroyWindow(window as HWND) };
    }
    lock(&SONY).clear();
}

/// Raw controller snapshots for the launcher's live input tester.
pub fn controller_status(watch: bool) -> io::Result<()> {
    let mut output = io::stdout().lock();
    loop {
        let mut sep = "";
        for (source, pads) in [("dualshock", dualshock_pads()), ("xinput", host::xinput_pads())] {
            for (slot, pad) in pads.into_iter().enumerate() {
                let Some(pad) = pad else { continue };
                write!(output, "{sep}{source},{slot},{},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3}",
                    pad.buttons, pad.left_x, pad.left_y, pad.right_x, pad.right_y,
                    pad.left_trigger, pad.right_trigger)?;
                sep = "|";
            }
        }
        writeln!(output)?;
        output.flush()?;
        if !watch {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usb_and_bluetooth_have_the_same_controls() {
        let controls = [0, 0, 128, 255, 0x23, 0x91, 0, 255, 64];
        let usb = [&[1][..], &controls].concat();
        let bluetooth = [&[0x11, 0xc0, 0][..], &controls].concat();
        let state = decode(&usb).unwrap();
        assert_eq!(decode(&bluetooth), Some(state));
        assert_eq!(state.buttons, A | DOWN | RIGHT | LEFT_SHOULDER | VIEW | RIGHT_STICK);
        assert_eq!((state.left_x, state.left_y, state.right_x, state.right_y), (-1.0, 1.0, 0.0, -1.0));
        assert_eq!(state.left_trigger, 1.0);
        assert_eq!(state.right_trigger, 64.0 / 255.0);
        assert!(decode(&[1, 2, 3]).is_none());
    }

    #[test]
    fn remapping_preserves_other_controls_and_disables_unbound_buttons() {
        let profile = Profile::parse("source=dualshock\nbuttons=0,1,3,14,4,5,6,7,8,9,10,11,12,13\ndeadzone=0.12\ninvert_right_y=1");
        assert_eq!(profile.source, Input::DualShock);
        let pad = profile.apply(PadState { buttons: B | Y, left_x: 0.04, right_y: 1.0, left_trigger: 0.5, ..Default::default() });
        assert_eq!(pad.buttons, A | Y);
        assert_eq!((pad.left_x, pad.right_y, pad.left_trigger), (0.0, -1.0, 0.5));
    }
}
