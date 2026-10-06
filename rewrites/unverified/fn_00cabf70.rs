// original: 0x00CABF70 follow_task_init (proposed)

/// Initialise a follow/escort movement task bound to a target object.
///
/// `this` is the task object. `target_opt` is null or points at the target
/// object (whose word at `+0x20` is the interesting one). `mode` is passed
/// untouched to callee 1. `pos` points at a three-float position copied to
/// `+0x20`. `param` (float bits) is stored at `+0x44`, `aux` at `+0x48`.
///
/// Behaviour: takes a float from callee 1 (cdecl of one word, x87 `ST0`
/// answer) and feeds it to the base initialiser (callee 2, thiscall, one
/// float argument); plants two virtual-table pointers (`+0x00` and `+0x14`);
/// copies `pos` to `+0x20`, stores `target_opt` at `+0x40`, zeroes `+0x4c`,
/// `+0x50` and the halfword at `+0x54`; then registers through callee 3
/// (thiscall on `target_opt`, one argument pointing at `this + 0x40`).
/// When the target is null the function returns `this` here.
/// Otherwise, when the target's word at `+0x20` is zero, callee 4 (thiscall,
/// no stack arguments, fills that word) runs first, then callee 5 (thiscall
/// on `target + 0x10`, one argument: the filled word). Finally the word at
/// target `+0x20` is read as a 3x4 matrix (row words at `+0x00/+0x04/+0x08`,
/// `+0x10/+0x14/+0x18`, `+0x20/+0x24/+0x28`, translation at
/// `+0x30/+0x34/+0x38`; the words at `+0x0c/+0x1c/+0x2c` are never read) and
/// the copied position is transformed by it into `+0x30/+0x34/+0x38`, each
/// output formed as `((m_row1*py + m_row0*px) + m_row2*pz) + m_t` in that
/// exact order.
///
/// One word is NOT computed from the inputs: `+0x3c` is loaded from an
/// aligned-frame scratch slot (`[esp+0x1c]`) that neither this function nor
/// its callees ever write (the float callee 1 returned is consumed by
/// callee 2 and its slot reused twice before this read), i.e. stale stack
/// contents. Under the checker's defined stack fill of `0` that word is
/// `0.0`, which is what this rewrite stores. The only comparisons are two
/// null/zero checks; nothing here is a signed/unsigned integer comparison.
///
/// Original: 0x00CABF70 (thiscall, five stack words, callee pops 20).
/// Returns `this`.
lf_checker_rt::export!(thiscall, rw_00CABF70(this: u32, target_opt: u32, mode: u32, pos: u32, param: u32, aux: u32) -> u32 {
    unsafe {
        const VTBL_MAIN: u32 = 0x00;
        const VTBL_SECOND: u32 = 0x14;
        const POS_COPY: u32 = 0x20;
        const OUT_X: u32 = 0x30;
        const OUT_Y: u32 = 0x34;
        const OUT_Z: u32 = 0x38;
        const STALE_OUT: u32 = 0x3c;
        const TARGET_SLOT: u32 = 0x40;
        const PARAM_SLOT: u32 = 0x44;
        const AUX_SLOT: u32 = 0x48;
        const VTBL_MAIN_FILE: u32 = 0x00ED840C;
        const VTBL_SECOND_FILE: u32 = 0x00ED8464;
        const MATRIX_WORD_OFF: u32 = 0x20;
        const HOOK_OFF: u32 = 0x10;
        const FACTOR_CALLEE: u32 = 1;
        const BASE_INIT_CALLEE: u32 = 2;
        const REGISTER_CALLEE: u32 = 3;
        const ENSURE_CALLEE: u32 = 4;
        const COMMIT_CALLEE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        // Factor from callee 1 (x87 ST0 answer), consumed by the base init.
        let factor: f32 = lf_checker_rt::callee_cdecl!(FACTOR_CALLEE, f32, mode);
        lf_checker_rt::callee_thiscall!(BASE_INIT_CALLEE, u32, this, factor.to_bits());

        wr32(this + VTBL_MAIN, lf_checker_rt::relocated(VTBL_MAIN_FILE));
        wr32(this + VTBL_SECOND, lf_checker_rt::relocated(VTBL_SECOND_FILE));

        wr32(this + POS_COPY, rd32(pos));
        wr32(this + POS_COPY + 4, rd32(pos + 4));
        wr32(this + POS_COPY + 8, rd32(pos + 8));
        wr32(this + PARAM_SLOT, param);
        wr32(this + TARGET_SLOT, target_opt);
        wr32(this + AUX_SLOT, aux);
        wr32(this + AUX_SLOT + 4, 0);
        wr32(this + AUX_SLOT + 8, 0);
        ((this + AUX_SLOT + 12) as *mut u16).write_unaligned(0);

        lf_checker_rt::callee_thiscall!(REGISTER_CALLEE, u32, target_opt, this + TARGET_SLOT);

        let target = rd32(this + TARGET_SLOT);
        if target != 0 {
            if rd32(target + MATRIX_WORD_OFF) == 0 {
                lf_checker_rt::callee_thiscall!(ENSURE_CALLEE, u32, target);
                let filled = rd32(target + MATRIX_WORD_OFF);
                lf_checker_rt::callee_thiscall!(COMMIT_CALLEE, u32, target + HOOK_OFF, filled);
            }
            let m = rd32(target + MATRIX_WORD_OFF);
            let px = rdf(this + POS_COPY);
            let py = rdf(this + POS_COPY + 4);
            let pz = rdf(this + POS_COPY + 8);
            let ox = add(
                add(add(mul(rdf(m + 0x10), py), mul(rdf(m), px)), mul(rdf(m + 0x20), pz)),
                rdf(m + 0x30),
            );
            let oy = add(
                add(add(mul(rdf(m + 0x14), py), mul(rdf(m + 4), px)), mul(rdf(m + 0x24), pz)),
                rdf(m + 0x34),
            );
            let oz = add(
                add(add(mul(rdf(m + 0x18), py), mul(rdf(m + 8), px)), mul(rdf(m + 0x28), pz)),
                rdf(m + 0x38),
            );
            wr32(this + OUT_X, ox.to_bits());
            wr32(this + OUT_Y, oy.to_bits());
            wr32(this + OUT_Z, oz.to_bits());
            // Stale stack slot under a zero fill (see doc comment).
            wr32(this + STALE_OUT, 0.0f32.to_bits());
        }

        this
    }
});
