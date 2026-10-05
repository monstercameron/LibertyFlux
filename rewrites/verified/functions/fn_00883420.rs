// original: 0x00883420 stream_req_configure (proposed)
/// Configure a streaming request from four parameters.
///
/// When the handle at `this+0x0c` is null, derives one from the third
/// parameter (`handle = alloc(third >> 1, 0x10)` through intercepted callee
/// 1, cdecl, two arguments) after recording `third >> 1` at `this+0x10`.
/// When the direct flag at `this+0x30` is set, stores the four parameters at
/// `this+0x14`..`this+0x20` in order and commits through the commit routine
/// (intercepted callee 2, cdecl, one argument: the word at `this+0x28`).
/// Otherwise binds through the binder (intercepted callee 3, thiscall: the
/// object in `ecx`, parameters in order as the stack arguments) and then
/// commits the fourth parameter.
///
/// Return value: the original ends with `(an instruction of the original)` after the intercepted
/// commit call, so the upper bytes are the commit stub's scripted answer:
/// the rewrite returns that answer with its low byte set to 1.
///
/// Original: thiscall, four stack arguments, callee cleans 16.
lf_checker_rt::export!(thiscall, rw_00883420(this: u32, p1: u32, p2: u32, p3: u32, p4: u32) -> u32 {
    unsafe {
        const HANDLE: u32 = 0x0c;
        const HALF: u32 = 0x10;
        const PARAMS: u32 = 0x14;
        const COMMIT_WORD: u32 = 0x28;
        const DIRECT: u32 = 0x30;
        const ALLOC_TAG: u32 = 0x10;
        const ALLOC_CALLEE: u32 = 1;
        const COMMIT_CALLEE: u32 = 2;
        const BIND_CALLEE: u32 = 3;
        if ((this + HANDLE) as *const u32).read_unaligned() == 0 {
            let half = p3 >> 1;
            ((this + HALF) as *mut u32).write_unaligned(half);
            let handle: u32 = lf_checker_rt::callee_cdecl!(ALLOC_CALLEE, u32, half, ALLOC_TAG);
            ((this + HANDLE) as *mut u32).write_unaligned(handle);
        }
        let commit_answer: u32;
        if ((this + DIRECT) as *const u8).read() != 0 {
            ((this + PARAMS) as *mut u32).write_unaligned(p1);
            ((this + PARAMS + 4) as *mut u32).write_unaligned(p2);
            ((this + PARAMS + 8) as *mut u32).write_unaligned(p3);
            ((this + PARAMS + 12) as *mut u32).write_unaligned(p4);
            let word = ((this + COMMIT_WORD) as *const u32).read_unaligned();
            commit_answer = lf_checker_rt::callee_cdecl!(COMMIT_CALLEE, u32, word);
        } else {
            let _: u32 = lf_checker_rt::callee_thiscall!(BIND_CALLEE, u32, this, p1, p2, p3);
            commit_answer = lf_checker_rt::callee_cdecl!(COMMIT_CALLEE, u32, p4);
        }
        (commit_answer & 0xFFFF_FF00) | 1
    }
});
