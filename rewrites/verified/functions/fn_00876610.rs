// original: 0x00876610 rage::crmtRequestBlend::vf1

/// Release each non-null child through its vtable slot +0x08, then clear
/// both child pointers and the two scalar state words. The object fields are
/// at +0x28, +0x2c, +0x18, and +0x14. This is a thiscall method with no
/// stack arguments and no return value.
lf_checker_rt::export!(thiscall, rw_00876610(this: u32) -> () {
    const CHILD_A: u32 = 0x28;
    const CHILD_B: u32 = 0x2c;
    const START_WEIGHT: u32 = 0x18;
    const ELAPSED_TICKS: u32 = 0x14;
    const RELEASE_SLOT: u32 = 8;

    #[inline(always)]
    unsafe fn release_child(child: u32) -> u32 {
        unsafe {
            let vtable = (child as *const u32).read_unaligned();
            let target = (vtable.wrapping_add(RELEASE_SLOT) as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            release(child)
        }
    }

    unsafe {
        let first = ((this + CHILD_A) as *const u32).read_unaligned();
        if first != 0 { let _ = release_child(first); }
        let second = ((this + CHILD_B) as *const u32).read_unaligned();
        ((this + CHILD_A) as *mut u32).write_unaligned(0);
        if second != 0 { let _ = release_child(second); }
        ((this + CHILD_B) as *mut u32).write_unaligned(0);
        ((this + START_WEIGHT) as *mut u32).write_unaligned(0);
        ((this + ELAPSED_TICKS) as *mut u32).write_unaligned(0);
    }
});
