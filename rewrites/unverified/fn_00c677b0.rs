// original: 0x00C677B0 cutscene_maybe_refresh (proposed)
//
// thiscall (ecx = this, no stack arguments). When either flag byte at
// +0x2a9 or +0x2aa is set, runs hook R (direct) and tail-jumps to the
// refresh routine (direct E9), returning its result; otherwise returns
// immediately.
//
// Narrowed proof: when both flags are zero the original returns with entry
// eax untouched, which a Rust rewrite cannot observe (no entry-eax
// transport exists), so the contract pins +0x2aa to nonzero and that path
// is never taken. The rewrite keeps the branch and returns 0 there.
lf_checker_rt::export!(thiscall, rw_00C677B0(this: u32) -> u32 {
    unsafe {
        const FLAG_A: u32 = 0x2a9;
        const FLAG_B: u32 = 0x2aa;
        const HOOK: u32 = 1;
        const TAIL: u32 = 2;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        if rd8(this + FLAG_A) != 0 || rd8(this + FLAG_B) != 0 {
            lf_checker_rt::callee_thiscall!(HOOK, u32, this);
            return lf_checker_rt::callee_thiscall!(TAIL, u32, this);
        }
        0
    }
});
