// original: 0x00BE6C60 CTaskSimpleStandUp::vf5
/// Abort-check for a stand-up task.
///
/// Abort kind 2 fades the animation handle at `this+0x14` with blend factor
/// -1000.0 (bits `FADE2`), releases it and clears the field (a null handle
/// skips the calls). Kind 1 sets flag 0x4000 on the handle's word at +4 and
/// then runs the same fade/release pair with blend factor -4.0 (bits
/// `FADE1`). Any other kind returns 0. Returns 1 in AL on kinds 1 and 2;
/// the high bytes of eax keep call residue or caller leftovers on the
/// paths that run fewer calls, so only AL is behaviour. Original is
/// thiscall with three stack words (only the second is read) and pops
/// 12 bytes.
lf_checker_rt::export!(thiscall, rw_00be6c60(this: u32, _a0: u32, kind: u32, _a2: u32) -> u32 {
    const FADE: u32 = 1;
    const RELEASE: u32 = 2;
    const HANDLE_OFF: u32 = 0x14;
    const KIND_FULL: u32 = 2;
    const KIND_FLAG: u32 = 1;
    const FADE2: u32 = 0xC47A0000;
    const FADE1: u32 = 0xC0800000;
    const FLAG_BIT: u32 = 0x4000;
    const FLAG_WORD: u32 = 4;
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        if kind == KIND_FULL {
            let handle = rd32(this + HANDLE_OFF);
            if handle != 0 {
                lf_checker_rt::callee_thiscall!(FADE, u32, handle, FADE2);
                lf_checker_rt::callee_thiscall!(RELEASE, u32, rd32(this + HANDLE_OFF), this);
                wr32(this + HANDLE_OFF, 0);
            }
            return 1;
        }
        if kind != KIND_FLAG {
            return 0;
        }
        let handle = rd32(this + HANDLE_OFF);
        if handle != 0 {
            wr32(handle + FLAG_WORD, rd32(handle + FLAG_WORD) | FLAG_BIT);
            lf_checker_rt::callee_thiscall!(FADE, u32, rd32(this + HANDLE_OFF), FADE1);
            lf_checker_rt::callee_thiscall!(RELEASE, u32, rd32(this + HANDLE_OFF), this);
            wr32(this + HANDLE_OFF, 0);
        }
        1
    }
});
