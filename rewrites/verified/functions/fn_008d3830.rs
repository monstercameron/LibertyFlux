// original: 0x008D3830 IssueManagerConfigCalls
/// Emits the two manager calls driven by globals: a three-argument call
/// carrying the shared argument, then a single-argument call whose answer is
/// returned.
export!(cdecl, rw_008D3830() -> u32 {
    unsafe {
        let arg = *global::<u32>(0x11736d4);
        callee_thiscall!(0, u32, *global::<u32>(0x11736c8), 2, 0, arg);
        callee_thiscall!(1, u32, *global::<u32>(0x11736c8), 0)
    }
});
