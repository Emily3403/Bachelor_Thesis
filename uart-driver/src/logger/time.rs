use crate::logger::TimeStamp;
use nix::time::ClockId;

#[inline(always)]
pub fn get_time() -> TimeStamp {
    let ts = nix::time::clock_gettime(ClockId::CLOCK_MONOTONIC_RAW).unwrap();

    // This can still store 584 years
    ts.tv_sec() as u64 * 1_000_000_000 + ts.tv_nsec() as u64
}
