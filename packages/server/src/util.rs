//! Utility functions

use std::{fmt, time::Duration};

/// Macros to time code and display a human-readable duration.
pub mod timeit {
    #[allow(unused_macros)]
    macro_rules! timeit_n {
        ($name:expr, $loops:expr, $code:expr) => {{
            use util::DisplayDuration;
            let start = std::time::Instant::now();
            let r = $code;
            for _ in 1..$loops {
                let _ = $code;
            }
            info!(
                "⏱  {}: {}",
                $name,
                DisplayDuration(start.elapsed() / $loops)
            );
            r
        }};
    }

    #[allow(unused_macros)]
    macro_rules! timeit {
        ($name:expr, $code:expr) => {{
            use crate::util::DisplayDuration;
            let start = std::time::Instant::now();
            let r = $code;
            info!("⏱  {}: {}", $name, DisplayDuration(start.elapsed()));
            r
        }};
    }

    #[allow(unused_imports)]
    pub(crate) use timeit;
    #[allow(unused_imports)]
    pub(crate) use timeit_n;
}

/// Utility struct for displaying human-readable duration of the form "10.5 ms",
/// "350 μs", or "27 ns".
pub struct DisplayDuration(pub Duration);

impl fmt::Display for DisplayDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let duration_ns = self.0.as_nanos();
        if duration_ns < 1_000_u128 {
            write!(f, "{duration_ns} ns")
        } else if duration_ns < 1_000_000_u128 {
            write!(f, "{} μs", (duration_ns + 500) / 1_000)
        } else {
            let duration_ms_times_10 = (duration_ns + 50_000) / (100_000);
            write!(f, "{} ms", (duration_ms_times_10 as f64) / 10.0)
        }
    }
}