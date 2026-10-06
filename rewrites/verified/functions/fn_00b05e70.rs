// original: 0x00b05e70 guarded_release_triple
/// Call the release helper on each non-null pointer of three.
///
/// stdcall `(a0, a1, a2)`: for each argument that is non-null, calls the
/// helper as thiscall `(ptr, 0)` (the helper reads its object through
/// `ecx`). No meaningful return: the original passes the incoming return
/// register through when no call fires, which a rewrite cannot observe,
/// so the contract compares no return channel.
export!(stdcall, rw_00b05e70(a0: u32, a1: u32, a2: u32) -> u32 {
    if a0 != 0 {
        let _: u32 = callee_thiscall!(1, u32, a0, 0);
    }
    if a1 != 0 {
        let _: u32 = callee_thiscall!(1, u32, a1, 0);
    }
    if a2 != 0 {
        let _: u32 = callee_thiscall!(1, u32, a2, 0);
    }
    0 // unchecked: passthrough return, see doc comment
});
