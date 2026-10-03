use core::hint::cold_path;

#[inline(always)]
pub const fn unlikely(cond: bool) -> bool {
    if cond {
        cold_path();
    }
    cond
}
