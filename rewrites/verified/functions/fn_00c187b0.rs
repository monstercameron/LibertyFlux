// original: 0x00c187b0 CCamReplay::vf7

/// Release the owned child object (if any), then run the base method.
///
/// When the pointer at `+0x2a0` is non-null, calls its virtual slot 0 with
/// argument 1 (thiscall) and clears the slot, then calls the base helper
/// (thiscall, no stack arguments) with `this`. Always returns 1.
///
/// Original: 0x00C187B0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c187b0(this: u32) -> u32 {
    unsafe {
        const CHILD_RELEASE: u32 = 1;
        const BASE_METHOD: u32 = 2;
        const CHILD_OFF: u32 = 0x2a0;
        let _ = CHILD_RELEASE;
        let child = ((this + CHILD_OFF) as *const u32).read_unaligned();
        if child != 0 {
            let vtable = (child as *const u32).read_unaligned();
            let slot0 = (vtable as *const u32).read_unaligned() as usize;
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot0);
            release(child, 1);
            ((this + CHILD_OFF) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(BASE_METHOD, u32, this);
        1
    }
});
