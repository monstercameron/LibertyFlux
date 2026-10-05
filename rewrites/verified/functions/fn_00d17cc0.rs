// original: 0x00D17CC0 cover_slide_pose_commit (proposed)

/// Store a cover-slide pose sample into the task's working state.
///
/// `this` is the task object. `src` points at a 60-byte sample block whose
/// words at offsets 0x00-0x08, 0x10-0x18, 0x20-0x28 and 0x30-0x38 (three
/// words per 16-byte row, skipping the fourth) are copied to the same
/// offsets under `this+0x13F0`, so the destination gaps at 0x13FC, 0x140C
/// and 0x141C are left untouched.
/// `vec` points at three floats copied to `this+0x1430..0x143C`, each then
/// ANDed in place with a per-lane mask word from the game's data. The
/// vector length `sqrt(y*y + x*x + z*z)` (adds in that order) is stored at
/// `this+0x1440`. The last three copied sample words are mirrored down to
/// `this+0x13D0..0x13D8`, and `this+0x1444` is set to the marker 4.
/// Straight-line code: no branches, no calls, no return value.
///
/// Original: 0x00D17CC0 (thiscall, two stack words, void).
lf_checker_rt::export!(thiscall, rw_00D17CC0(this: u32, src: u32, vec: u32) -> u32 {
    unsafe {
        const DEST_BASE: u32 = 0x13f0;
        const SRC_OFFS: [u32; 12] = [0x00, 0x04, 0x08, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];
        const VEC_DEST: u32 = 0x1430;
        const LEN_DEST: u32 = 0x1440;
        const MIRROR_SRC: u32 = 0x1420;
        const MIRROR_DEST: u32 = 0x13d0;
        const MARKER_DEST: u32 = 0x1444;
        const MARKER: u32 = 4;
        const MASK0: u32 = 0x18d2190;
        const MASK1: u32 = 0x18d2194;
        const MASK2: u32 = 0x18d2198;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let mut i = 0usize;
        while i < 12 {
            wr32(this + DEST_BASE + SRC_OFFS[i], rd32(src + SRC_OFFS[i]));
            i += 1;
        }
        wr32(this + VEC_DEST, rd32(vec) & rd32(lf_checker_rt::relocated(MASK0)));
        wr32(this + VEC_DEST + 4, rd32(vec + 4) & rd32(lf_checker_rt::relocated(MASK1)));
        wr32(this + VEC_DEST + 8, rd32(vec + 8) & rd32(lf_checker_rt::relocated(MASK2)));
        let x = f32::from_bits(rd32(vec));
        let y = f32::from_bits(rd32(vec + 4));
        let z = f32::from_bits(rd32(vec + 8));
        let len_sq = add(add(mul(y, y), mul(x, x)), mul(z, z));
        wr32(this + LEN_DEST, len_sq.sqrt().to_bits());
        wr32(this + MIRROR_DEST, rd32(this + MIRROR_SRC));
        wr32(this + MIRROR_DEST + 4, rd32(this + MIRROR_SRC + 4));
        wr32(this + MIRROR_DEST + 8, rd32(this + MIRROR_SRC + 8));
        wr32(this + MARKER_DEST, MARKER);
        0
    }
});
