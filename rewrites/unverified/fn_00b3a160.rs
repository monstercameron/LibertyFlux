// original: 0x00b3a160 task_nearby_commit (proposed)

/// Commit the nearby task entry when `point` (three floats) is within
/// `radius` of the active entry's anchor: look the entry up through the
/// indexed entry table (a null entry, or index zero, commits nothing and
/// returns 0), compare the squared distance against the squared radius with
/// the ordered greater test (NaN takes the commit path), and on commit call
/// the placer with the point, the `param` word and a zero word, returning
/// nonzero when the placer does. Out of range returns the point address with
/// its low byte cleared; the commit path keeps the placer's answer with its
/// low byte replaced by the placer-nonzero flag. The float operation order
/// is the original's. Original: 0x00b3a160 (cdecl, three stack words: point,
/// param, radius bits).
lf_checker_rt::export!(cdecl, rw_00b3a160(point: u32, param: u32, radius_bits: u32) -> u32 {
    unsafe {
        const PLACER: u32 = 1;
        const ENTRY_TABLE: u32 = 0x0118d818;
        const ANCHOR_OFF: u32 = 0x80;
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        let table = lf_checker_rt::relocated(ENTRY_TABLE);
        let index = (table as *const u32).read();
        let entry = ((table + index.wrapping_mul(4)) as *const u32).read();
        if entry == 0 {
            return 0;
        }
        let ax = f32::from_bits(
            ((entry + ANCHOR_OFF) as *const u32).read_unaligned(),
        );
        let ay = f32::from_bits(
            ((entry + ANCHOR_OFF + 4) as *const u32).read_unaligned(),
        );
        let az = f32::from_bits(
            ((entry + ANCHOR_OFF + 8) as *const u32).read_unaligned(),
        );
        let px =
            f32::from_bits((point as *const u32).read_unaligned());
        let py = f32::from_bits(((point + 4) as *const u32).read_unaligned());
        let pz = f32::from_bits(((point + 8) as *const u32).read_unaligned());
        let dx = sub(ax, px);
        let dy = sub(ay, py);
        let dz = sub(az, pz);
        let dist2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        let radius = f32::from_bits(radius_bits);
        let r2 = mul(radius, radius);
        if dist2 > r2 {
            return point & 0xffff_ff00;
        }
        let placed: u32 = lf_checker_rt::callee_cdecl!(
            PLACER, u32, px.to_bits(), py.to_bits(), pz.to_bits(), param, 0
        );
        (placed & 0xffff_ff00) | ((placed != 0) as u32)
    }
});
