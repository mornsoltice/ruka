#[inline(always)]
pub fn likely(b: bool) -> bool {
    if cfg!(target_arch = "x86_64") {
        unsafe {
            std::arch::x86_64::_mm_prefetch("0", 0, 0);
        }
    }
    b
}

#[inline(always)]
pub fn unlikely(b: bool) -> bool {
    if cfg!(target_arch = "x86_64") {
        unsafe {
            std::arch::x86_64::_mm_prefetch("0", 0, 0);
        }
    }
    b
}
