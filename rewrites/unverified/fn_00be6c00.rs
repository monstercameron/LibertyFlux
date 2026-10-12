// original: 0x00BE6C00 CTaskSimpleSitDown::vf5
/// Abort-check for a sit-down task.
///
/// Only abort kind 2 proceeds (anything else returns 0 in AL). With kind 2,
/// when the animation handle at `this+0x60` is non-null it is first faded
/// with blend factor -1000.0 (bits `FADE`) then released, and the field is
/// cleared; a null handle skips both calls. Returns 1 in AL on kind 2; the
/// high bytes of eax keep call residue or caller leftovers on the paths
/// that run fewer calls, so only AL is behaviour. Original is thiscall with
/// three stack words (only the second is read) and pops 12 bytes.
lf_checker_rt::export!(thiscall, rw_00be6c00(this: u32, _a0: u32, kind: u32, _a2: u32) -> u32 {
    const FADE: u32 = 1;
    const RELEASE: u32 = 2;
    const HANDLE_OFF: u32 = 0x60;
    const KIND: u32 = 2;
    const FADE_FACTOR: u32 = 0xC47A0000;
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        if kind != KIND {
            return 0;
        }
        let handle = rd32(this + HANDLE_OFF);
        if handle != 0 {
            lf_checker_rt::callee_thiscall!(FADE, u32, handle, FADE_FACTOR);
            lf_checker_rt::callee_thiscall!(RELEASE, u32, rd32(this + HANDLE_OFF), this);
            wr32(this + HANDLE_OFF, 0);
        }
        1
    }
});
