// original: 0x00beac20 maybe_release_two
/// Conditionally release two owned words through a helper call.
///
/// When the half-word at `[this+0xe]` is nonzero, passes `[this+8]` to the
/// helper (cdecl, one stack arg; intercepted, answer ignored); when the
/// half-word at `[this+6]` is nonzero, passes `[this+0]`. The two tests are
/// independent, so 0-2 calls happen in site order. No stores, no return
/// value (EAX is the last helper answer, or the caller's value when no call
/// fires, so no return channel is compared). Thiscall, no stack arguments.
export!(thiscall, rw_00beac20(this: u32) -> u32 {
    unsafe {
        const HELPER: u32 = 1;
        if ((this + 0x0e) as *const u16).read_unaligned() != 0 {
            let w = ((this + 8) as *const u32).read_unaligned();
            let _: u32 = callee_cdecl!(HELPER, u32, w);
        }
        if ((this + 6) as *const u16).read_unaligned() != 0 {
            let w = ((this + 0) as *const u32).read_unaligned();
            let _: u32 = callee_cdecl!(HELPER, u32, w);
        }
        0
    }
});
