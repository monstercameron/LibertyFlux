// original: 0x00bed260 forward_slot_then_probe
/// Forward `[this+4]` with a scratch buffer, then probe another callee.
///
/// Passes `[this+4]` and a scratch buffer address to the first helper
/// (cdecl, intercepted, answer ignored), then calls the second helper
/// (thiscall with ECX = `arg`, intercepted) with the same scratch address.
/// Both buffers are the function's own uninitialized frame, so the contract
/// skips those arguments without snapshots; what is compared is the call
/// order, `[this+4]`, ECX and the returned answer. Returns the second
/// helper's answer. Thiscall, one stack argument.
export!(thiscall, rw_00bed260(this: u32, arg: u32) -> u32 {
    unsafe {
        const FIRST: u32 = 1;
        const SECOND: u32 = 2;
        let mut buf = [0u32; 4];
        let v = ((this + 4) as *const u32).read_unaligned();
        let _: u32 = callee_cdecl!(FIRST, u32, v, buf.as_mut_ptr() as u32);
        let r: u32 = callee_thiscall!(SECOND, u32, arg, buf.as_mut_ptr() as u32);
        r
    }
});
