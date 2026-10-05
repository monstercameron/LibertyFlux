// original: 0x0088E660 rage::audVoiceSoft::vf5

/// Stop this software voice: clear its active/looping flag bits, then tail
/// into the child voice's stop routine.
///
/// `this` points to the voice object. Bits 0 and 3 of the flag byte at
/// `+0x8c` are cleared, the child object at `+0x130` is loaded, and control
/// passes to the child's stop entry with the child as `this`; its answer is
/// the answer of this function.
///
/// Original: 0x0088E660 (thiscall, no stack arguments, tail call).
lf_checker_rt::export!(thiscall, rw_0088E660(this: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x8c;
        const CLEAR_BITS: u8 = 0x09;
        const CHILD: u32 = 0x130;
        let flags = (this + FLAGS) as *mut u8;
        flags.write(flags.read() & !CLEAR_BITS);
        let obj = ((this + CHILD) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(1, u32, obj)
    }
});
