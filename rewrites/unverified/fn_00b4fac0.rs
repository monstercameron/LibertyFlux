// original: 0x00b4fac0 ranged_splash (proposed)

/// Score a splash against the last waypoint and stamp the clock.
///
/// The lookup hook (callee 1, thiscall on `obj`) returns a strided table
/// whose first word is an unsigned count. A count of 0 or 1 returns 0 at
/// once. Otherwise the entry at `base + count * 16` is treated as a
/// three-float point, its squared distance to `point` is formed in the
/// original's lane order `((dx*dx + dy*dy) + dz*dz)`, square-rooted and
/// biased by the float at `DIST_K`. The report hook (callee 2, cdecl, ten
/// words: entry, point, a zeroed three-word frame buffer, biased distance,
/// `0xA0B6`, five zeros) runs next; its result is returned after the clock
/// word at `CLOCK` is copied to `SNAPSHOT`.
///
/// Original: 0x00b4fac0 (cdecl, two stack words; float bit-exact).
lf_checker_rt::export!(cdecl, rw_00b4fac0(obj: u32, point: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0;
        const STRIDE: u32 = 16;
        const DIST_K: u32 = 0x00fe8b38;
        const CLOCK: u32 = 0x011735b4;
        const SNAPSHOT: u32 = 0x0166838c;
        const REPORT_TAG: u32 = 0xa0b6;
        const FETCH: u32 = 1;
        const REPORT: u32 = 2;
        #[inline(always)]
        fn fsub(a: u32, b: u32) -> u32 {
            let x = f32::from_bits(core::hint::black_box(a));
            let y = f32::from_bits(core::hint::black_box(b));
            (x - y).to_bits()
        }
        #[inline(always)]
        fn fmul(a: u32, b: u32) -> u32 {
            let x = f32::from_bits(core::hint::black_box(a));
            let y = f32::from_bits(core::hint::black_box(b));
            (x * y).to_bits()
        }
        #[inline(always)]
        fn fadd(a: u32, b: u32) -> u32 {
            let x = f32::from_bits(core::hint::black_box(a));
            let y = f32::from_bits(core::hint::black_box(b));
            (x + y).to_bits()
        }
        let base = lf_checker_rt::callee_thiscall!(FETCH, u32, obj);
        let count = ((base + COUNT) as *const u32).read_unaligned();
        if count <= 1 {
            return 0;
        }
        let entry = base.wrapping_add(count.wrapping_mul(STRIDE));
        let dx = fsub(
            ((entry) as *const u32).read_unaligned(),
            ((point) as *const u32).read_unaligned(),
        );
        let dy = fsub(
            ((entry + 4) as *const u32).read_unaligned(),
            ((point + 4) as *const u32).read_unaligned(),
        );
        let dz = fsub(
            ((entry + 8) as *const u32).read_unaligned(),
            ((point + 8) as *const u32).read_unaligned(),
        );
        let d2 = fadd(fadd(fmul(dx, dx), fmul(dy, dy)), fmul(dz, dz));
        let dist = f32::from_bits(core::hint::black_box(d2)).sqrt().to_bits();
        let k = lf_checker_rt::global::<u32>(DIST_K).read_unaligned();
        let biased = fadd(dist, k);
        let mut buf = [0u32; 3];
        let r = lf_checker_rt::callee_cdecl!(
            REPORT, u32, entry, point, buf.as_mut_ptr() as u32, biased, REPORT_TAG, 0, 0, 0, 0, 0
        );
        let now = lf_checker_rt::global::<u32>(CLOCK).read_unaligned();
        lf_checker_rt::global::<u32>(SNAPSHOT).write_unaligned(now);
        r
    }
});
