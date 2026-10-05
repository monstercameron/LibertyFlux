// original: 0x0088F4A0 rage::audVoicePcAdpcm::vf2

/// Tear down this ADPCM voice's child and buffer, then tail into the base
/// teardown.
///
/// `this` points to the voice. The child at `+0x140`, when non-null, is
/// shut down (callee 1, called with the child and a zero word) and the
/// slot is cleared. The buffer handle at `+0x148` is always released
/// (callee 2) and its slot cleared. Control then passes to the base
/// teardown entry with the voice as `this`; its answer is the answer of
/// this function.
///
/// Original: 0x0088F4A0 (thiscall, no stack arguments, tail call).
lf_checker_rt::export!(thiscall, rw_0088F4A0(this: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 0x140;
        const BUFFER: u32 = 0x148;
        const SHUT_CHILD: u32 = 1;
        const FREE_BUFFER: u32 = 2;
        const BASE_TEARDOWN: u32 = 3;

        let child = ((this + CHILD) as *const u32).read_unaligned();
        if child != 0 {
            lf_checker_rt::callee_thiscall!(SHUT_CHILD, u32, child, 0);
            ((this + CHILD) as *mut u32).write_unaligned(0);
        }
        let buf = ((this + BUFFER) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(FREE_BUFFER, u32, buf);
        ((this + BUFFER) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(BASE_TEARDOWN, u32, this)
    }
});
