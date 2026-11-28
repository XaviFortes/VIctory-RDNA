//! Coverage of a single host copy during submission preparation.

type Span = (usize, usize);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) enum RefreshRanges {
    #[default]
    Empty,
    One(Span),
    Several(Vec<Span>),
}

impl RefreshRanges {
    pub(super) fn add(&mut self, from: usize, to: usize, gap: usize) {
        if from >= to {
            return;
        }
        match self {
            Self::Empty => *self = Self::One((from, to)),
            Self::One((start, end)) if end.saturating_add(gap) >= from && to.saturating_add(gap) >= *start => {
                *start = (*start).min(from);
                *end = (*end).max(to);
            }
            Self::One(span) => {
                let other = (from, to);
                *self = Self::Several(if from < span.0 { vec![other, *span] } else { vec![*span, other] });
            }
            Self::Several(spans) => {
                let first = spans.partition_point(|span| span.1.saturating_add(gap) < from);
                let last = first + spans[first..].partition_point(|span| span.0 <= to.saturating_add(gap));
                if first == last {
                    spans.insert(first, (from, to));
                } else {
                    spans[first] = (from.min(spans[first].0), to.max(spans[last - 1].1));
                    spans.drain(first + 1..last);
                    if spans.len() == 1 {
                        *self = Self::One(spans[0]);
                    }
                }
            }
        }
    }

    pub(super) fn spans(&self) -> &[Span] {
        match self {
            Self::Empty => &[],
            Self::One(span) => std::slice::from_ref(span),
            Self::Several(spans) => spans,
        }
    }

    pub(super) fn covering(&self, from: usize, to: usize) -> Option<Span> {
        match self {
            Self::Empty => None,
            Self::One((start, end)) => (*start <= from && to <= *end).then_some((*start, *end)),
            Self::Several(spans) => {
                let i = spans.partition_point(|&(start, _)| start <= from).checked_sub(1)?;
                (to <= spans[i].1).then_some(spans[i])
            }
        }
    }

    pub(super) fn envelope(&self) -> usize {
        let Some(&(from, _)) = self.spans().first() else { return 0 };
        self.spans().last().unwrap().1 - from
    }

    pub(super) fn bytes(&self) -> usize {
        self.spans().iter().map(|&(from, to)| to - from).sum()
    }

    /// Limit per-copy refresh overhead once all lists' ranges are known.
    pub(super) fn bound_fragmentation(&mut self) {
        if let Self::Several(spans) = self
            && spans.len() > 8
        {
            *self = Self::One((spans[0].0, spans[spans.len() - 1].1));
        }
    }
}
