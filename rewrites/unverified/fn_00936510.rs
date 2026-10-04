// original: 0x00936510 net_blend_accum (proposed)

/// Accumulate blended values over two mode slots into running totals.
///
/// Takes one byte argument and reads two mode dwords from globals. An
/// availability probe decides two enable flags (both set when it answers
/// non-zero, else both clear: the probe writes nothing, so the cleared
/// stack slots it might have filled stay zero); the argument forces the
/// first flag on when non-zero, and a global enable forces the second off
/// when clear. When both modes are 2 or 3 and both flags are set, the four
/// running totals are preset from two read-only constants instead of zero.
/// Each mode slot is then visited once with its flag: modes 2 and 3 with
/// the flag set fetch a reference triple through one callee and invoke a
/// blend callee with five arguments (an uninitialized stack word the callee
/// never reads, the first total, constant 1, and two reference words);
/// mode 5 invokes a combine callee with the two live float registers. After
/// either call the totals fold pairwise (second plus first into first,
/// fourth plus zeroth into zeroth). The float registers carry across slots:
/// the combine arguments on the second slot are the first slot's folded
/// values (the preset path's copy of the incoming vector register is dead:
/// it always folds the first slot before any use).
/// Returns the last callee answer. cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_00936510(arg: u32) -> u32 {
    unsafe {
        const G_EN: u32 = 0x11A2EA2;
        const G_M0: u32 = 0x11A4024;
        const G_M1: u32 = 0x11A4DD4;
        const M_STEP: u32 = 0xDB0;
        const M_END: u32 = 0x11A5B84;
        const OBJ_OFF: u32 = 0xD64;
        const PRE0: u32 = 0xFE8D94;
        const PRE1: u32 = 0xFE8A24;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let g = lf_checker_rt::relocated;
        let probe = lf_checker_rt::callee_cdecl!(1, u32,);
        let mut last = probe;
        let (fa, fc) = if (probe as u8) != 0 { (1u8, 1u8) } else { (0u8, 0u8) };
        let fa = if (arg as u8) != 0 { 1u8 } else { fa };
        let fc = if rd8(g(G_EN)) == 0 { 0u8 } else { fc };
        let m0 = rd32(g(G_M0));
        let m1 = rd32(g(G_M1));
        let preset = (m0 == 3 || m0 == 2)
            && fa != 0
            && (m1 == 3 || m1 == 2)
            && fc != 0;
        let (mut t0, mut t1, mut t2, mut t3);
        let (mut x0, mut x1);
        if preset {
            let p = f32::from_bits(rd32(g(PRE0)));
            let q = f32::from_bits(rd32(g(PRE1)));
            t0 = p;
            t1 = p;
            t2 = q;
            t3 = q;
            x0 = p;
            // The original copies the incoming vector register here, but the
            // preset path always runs the first slot's blend and fold, which
            // overwrites this register before any use; the value is dead.
            x1 = 0.0f32;
        } else {
            t0 = 0.0f32;
            t1 = 0.0f32;
            t2 = 0.0f32;
            t3 = 0.0f32;
            x0 = 0.0f32;
            x1 = 0.0f32;
        }
        let mut slot = g(G_M0);
        while slot != g(M_END) {
            let mode = rd32(slot);
            let flag = if slot == g(G_M0) { fa } else { fc };
            if (mode == 3 || mode == 2) && flag != 0 {
                let mut refp = [0u32; 3];
                let a2 = lf_checker_rt::callee_cdecl!(2, u32, refp.as_mut_ptr() as u32);
                last = a2;
                let a3 = lf_checker_rt::callee_thiscall!(
                    3, u32, slot.wrapping_sub(OBJ_OFF),
                    0, t0.to_bits(), 1, refp[0], refp[1]
                );
                last = a3;
            } else if mode == 5 {
                let a4 = lf_checker_rt::callee_thiscall!(
                    4, u32, slot.wrapping_sub(OBJ_OFF),
                    x1.to_bits(), x0.to_bits()
                );
                last = a4;
            } else {
                slot = slot.wrapping_add(M_STEP);
                continue;
            }
            x1 = add(t2, t1);
            x0 = add(t3, t0);
            t1 = x1;
            t0 = x0;
            slot = slot.wrapping_add(M_STEP);
        }
        last
    }
});
