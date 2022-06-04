use std::time::{SystemTime, UNIX_EPOCH};

use anchor_lang::prelude::{Clock, SolanaSysvar};

/// Get the current timestamp in seconds since Unix epoch
///
/// The function returns a [anchor_lang::prelude::Clock] value in the bpf arch,
/// and first checks if there is a [Clock] in other archs, returning the system
/// time if there is no clock (e.g. if not running in a simulator with its clock).
///
/// Note: This function was taken from [the Jet-v2 utilities module](https://github.com/jet-lab/jet-v2/blob/master/programs/margin/src/util.rs)
/// Credit where it's due :)
pub fn get_timestamp() -> u64 {
    #[cfg(target_arch = "bpf")]
    {
        Clock::get().unwrap().unix_timestamp as u64
    }
    #[cfg(not(target_arch = "bpf"))]
    {
        // Get the clock in case it's available in a simulation,
        // then fall back to the system clock
        if let Ok(clock) = Clock::get() {
            clock.unix_timestamp as u64
        } else {
            let time = SystemTime::now();
            time.duration_since(UNIX_EPOCH).unwrap().as_secs()
        }
    }
}
