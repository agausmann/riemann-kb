use heapless::Deque;

pub struct Debounce<T, const N: usize> {
    prev_raw: [T; N],
    prev_debounced: T,
}

impl<T: Copy + PartialEq, const N: usize> Debounce<T, N> {
    pub fn new(initial: T) -> Self {
        Self {
            prev_raw: [initial; N],
            prev_debounced: initial,
        }
    }

    /// Handle the new state `next`.
    ///
    ///
    pub fn update(&mut self, next: T) -> Option<(T, T)> {
        let changed =
            next != self.prev_debounced && self.prev_raw.iter().all(|prev_raw| *prev_raw == next);
        self.prev_raw.copy_within(1.., 0);
        self.prev_raw[N - 1] = next;
        if changed {
            let prev = self.prev_debounced;
            self.prev_debounced = next;
            Some((prev, next))
        } else {
            None
        }
    }
}

struct DeferredEvent<T> {
    event: T,
    time: u8,
}

/// A simple scheduler that defers events forward by a fixed amount of time.
pub struct Defer<T, const LEN: usize, const DELAY: u8> {
    events: Deque<DeferredEvent<T>, LEN>,
    ticker: u8,
}

impl<T, const LEN: usize, const DELAY: u8> Defer<T, LEN, DELAY> {
    /// Create a new empty scheduler.
    pub fn new() -> Self {
        Self {
            events: Deque::new(),
            ticker: 0,
        }
    }

    /// Enqueue an event to be handled `DELAY` ticks later.
    ///
    /// If the queue is full, the event will be returned as an `Err`.
    pub fn defer(&mut self, event: T) -> Result<(), T> {
        // Note that no sorting needs to take place, because the time is
        // monotonic and the delay is constant for all events.
        self.events
            .push_back(DeferredEvent {
                event,
                time: self.ticker.wrapping_add(DELAY),
            })
            .map_err(|deferred| deferred.event)
    }

    /// Advance the tick counter forward by one.
    pub fn tick(&mut self) {
        self.ticker = self.ticker.wrapping_add(1);
    }

    /// Check whether there are any events to be handled in this tick.
    ///
    /// This should be called repeatedly after calling `tick()`, until it
    /// returns `None`, indicating no more events this tick.
    pub fn poll(&mut self) -> Option<T> {
        let time = self.events.front()?.time;
        if time == self.ticker {
            Some(self.events.pop_front().unwrap().event)
        } else {
            None
        }
    }
}
