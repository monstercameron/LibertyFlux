// original: 0x00B3A160 point_in_radius_or_vol

/// Test the point `p` against a radius around an engine-stored centre, then
/// against a volume check through callee 1.
///
/// Loads the centre record through the index table at `G_TABLE` (index word
/// first, record pointer at `table[index]`; a null record returns 0), forms
/// the squared distance in the original's operation order
/// (`dx*dx + dy*dy + dz*dz` with `dx = cx - px` etc.), and returns the point
/// word with its low byte cleared when the distance is strictly greater than
/// `r * r` (the original clears only `al`). Otherwise callee 1 (stdcall,
/// five words: the point copied by value plus `g` and a zero word) decides,
/// and the answer is its non-zeroness kept in the low byte with its own
/// upper 24 bits. NaN distances take the call path, as `comiss`/`jbe` does.
/// Cdecl, three stack words.
///
/// Original: 0x00B3A160.

lf_checker_rt::export!(cdecl, rw_00B3A160(p: u32, g: u32, r: u32) -> u32 {
    unsafe {
        const G_TABLE: u32 = 0x0118D818;
        const CENTRE_OFF: u32 = 0x80;
        const VOL: u32 = 1;
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        let tbl = lf_checker_rt::relocated(G_TABLE);
        let index = (tbl as *const u32).read_unaligned();
        let rec = (tbl.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned();
        if rec == 0 {
            return 0;
        }
        let cx = (rec.wrapping_add(CENTRE_OFF) as *const f32).read_unaligned();
        let cy = (rec.wrapping_add(CENTRE_OFF + 4) as *const f32).read_unaligned();
        let cz = (rec.wrapping_add(CENTRE_OFF + 8) as *const f32).read_unaligned();
        let px = (p as *const f32).read_unaligned();
        let py = (p.wrapping_add(4) as *const f32).read_unaligned();
        let pz = (p.wrapping_add(8) as *const f32).read_unaligned();
        let dx = sub(cx, px);
        let dy = sub(cy, py);
        let dz = sub(cz, pz);
        let d2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        let rr = mul(f32::from_bits(r), f32::from_bits(r));
        if d2 > rr {
            return p & 0xFFFF_FF00;
        }
        let v: u32 = lf_checker_rt::callee_stdcall!(
            VOL,
            u32,
            (p as *const u32).read_unaligned(),
            (p.wrapping_add(4) as *const u32).read_unaligned(),
            (p.wrapping_add(8) as *const u32).read_unaligned(),
            g,
            0
        );
        (v & 0xFFFF_FF00) | u32::from(v != 0)
    }
});
