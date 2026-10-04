// original: 0x0089EB20 aud_cdecl_thunk_89E2A0
// ---------------------------------------------------------------------------
// 0x0089EB20: tail-call thunk. Loads the single stack argument into ECX and
// jumps to the worker. The worker takes ECX only and cleans nothing (its
// other exit path is a bare `ret`), so this thunk is cdecl: the caller keeps
// the argument on the stack. The rewrite is a plain forwarding call.
// ---------------------------------------------------------------------------
export!(cdecl, rw_0089EB20(arg: u32) -> u32 {
    callee_thiscall!(1, u32, arg)
});
