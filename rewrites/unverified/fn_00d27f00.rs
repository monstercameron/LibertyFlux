// original: 0x00d27f00 targeting_ref_release (proposed)

/// Release the reference held at `this + 0x14`, if any.
///
/// When the word at `this + 0x14` is nonzero, passes its address to the
/// release helper (intercepted) and then clears the word. The original leaves
/// `eax` untouched, so no return channel is compared.
///
/// Original: 0x00D27F00 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d27f00(this: u32) -> u32 {
    unsafe {
        const REF_OFF: u32 = 0x14;
        let slot = this + REF_OFF;
        if unsafe { (slot as *const u32).read_unaligned() } != 0 {
            let _: u32 = lf_checker_rt::callee_stdcall!(1, u32, slot);
            unsafe { (slot as *mut u32).write_unaligned(0) };
        }
        0
    }
});
