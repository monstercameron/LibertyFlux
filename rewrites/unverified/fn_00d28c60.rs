// original: 0x00d28c60 target_slot_configure (proposed)

/// Configure a slot on first use, then run the two follow-up helpers.
///
/// Resets the float at `this + 0x224` to 4.0. When the byte at `this + 0x220`
/// is clear, sets it and forwards the three arguments to the setup helper
/// (intercepted), then runs the refresh helper (intercepted) with 0. Either
/// way finishes with the finish helper (intercepted) on `this`. The original
/// leaves `eax` untouched, so no return channel is compared.
///
/// Original: 0x00D28C60 (thiscall, three stack arguments).
lf_checker_rt::export!(thiscall, rw_00d28c60(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const FLOAT_OFF: u32 = 0x224;
        const FLAG_OFF: u32 = 0x220;
        const FOUR_BITS: u32 = 0x4080_0000; // 4.0f
        unsafe { ((this + FLOAT_OFF) as *mut u32).write_unaligned(FOUR_BITS) };
        if unsafe { ((this + FLAG_OFF) as *const u8).read() } == 0 {
            unsafe { ((this + FLAG_OFF) as *mut u8).write(1) };
            let _: u32 = lf_checker_rt::callee_stdcall!(1, u32, a0, a1, a2);
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, 0);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, this);
        0
    }
});
