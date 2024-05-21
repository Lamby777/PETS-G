//!
//! Totally not stolen from [mdwn](https://github.com/insomnimus/nodi/issues/1)
//! (thanks m8)
//!
//! Temporary fix until that issue is solved.
//!

use std::ops::Add as _;
use std::time::Instant;

use nodi::Timer;

/// AccurateTimer is a timer for the nodi player that allows a more accurate clock. It uses the last
/// known instant to properly calculate the next intended sleep duration.
#[derive(Clone)]
pub struct AccurateTimer<T: Timer> {
    timer: T,
    last_instant: Option<Instant>,
}

impl<T: Timer> AccurateTimer<T> {
    pub fn new(timer: T) -> AccurateTimer<T> {
        AccurateTimer {
            timer,
            last_instant: None,
        }
    }
}

impl<T: Timer> Timer for AccurateTimer<T> {
    fn sleep_duration(&mut self, n_ticks: u32) -> std::time::Duration {
        let mut duration = self.timer.sleep_duration(n_ticks);

        // Modify the sleep duration if the last duration is populated, as we
        // know about when the next tick should be.
        match self.last_instant {
            Some(last_instant) => {
                self.last_instant = Some(last_instant.add(duration));

                // Subtract the duration unless it would be an overflow. If so, use the original duration.
                duration = match duration
                    .checked_sub(Instant::now().duration_since(last_instant))
                {
                    Some(duration) => duration,
                    None => duration,
                };
            }
            None => self.last_instant = Some(Instant::now()),
        };

        duration
    }

    fn change_tempo(&mut self, tempo: u32) {
        self.timer.change_tempo(tempo);
    }
}
