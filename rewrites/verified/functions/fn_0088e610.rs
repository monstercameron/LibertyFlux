// original: 0x0088E610 rage::audVoiceSoft::vf2

/// Tear down this software voice's child and buffer, then tail into the
/// base teardown.
///
/// `this` points to the voice. The child at `+0x130`, when non-null, is
/// shut down (callee 1, called with the child and a zero word) and the
/// slot is cleared. When bit 4 of the flag byte at `+0x8c` is set, the
/// buffer handle at `+0x138` is released (callee 2) and its slot cleared.
/// Control then passes to the base teardown entry with the voice as `this`;
/// its answer is the answer of this function.
///
/// Original: 0x0088E610 (thiscall, no stack arguments, tail call).
lf_checker_rt::export!(thiscall, rw_0088E610(this: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 0x130;
        const FLAGS: u32 = 0x8c;
        const HAS_BUFFER: u8 = 0x10;
        const BUFFER: u32 = 0x138;
        const SHUT_CHILD: u32 = 1;
        const FREE_BUFFER: u32 = 2;
        const BASE_TEARDOWN: u32 = 3;

        let child = ((this + CHILD) as *const u32).read_unaligned();
        if child != 0 {
            lf_checker_rt::callee_thiscall!(SHUT_CHILD, u32, child, 0);
            ((this + CHILD) as *mut u32).write_unaligned(0);
        }
        if ((this + FLAGS) as *const u8).read() & HAS_BUFFER != 0 {
            let buf = ((this + BUFFER) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(FREE_BUFFER, u32, buf);
            ((this + BUFFER) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(BASE_TEARDOWN, u32, this)
    }
});
