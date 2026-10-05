// original: 0x00B82C50 dist_check_3d
/// Test whether a point is inside a slot's radius (squared distances).
///
/// The entity's slot index (signed word at `+0x228`) selects one of the
/// `STRIDE`-byte slots past `this`; unless its kind byte is 1 the answer
/// is 0. Otherwise the squared distance from `pt` to the entity's anchor
/// (its override plus `+0x30`, or `+0x10` when null) is compared against
/// the slot's radius: 1 exactly when the radius is ordered-greater (a NaN
/// on either side answers 0). Float order matches the original.
///
/// Original: 0x00B82C50 (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_00B82C50(this: u32, ent: u32, pt: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> i32 {
            unsafe { (a as *const u16).read_unaligned() as i16 as i32 }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        const STRIDE: u32 = 0x2C;
        const KIND: u32 = 0x18;
        const RADIUS: u32 = 0x1C;
        let idx = rd16s(ent + 0x228);
        let slot = this.wrapping_add((idx.wrapping_mul(STRIDE as i32)) as u32);
        if rd8(slot + KIND) != 1 {
            return 0;
        }
        let anchor = rd32(ent + 0x20);
        let base = if anchor != 0 { anchor.wrapping_add(0x30) } else { ent.wrapping_add(0x10) };
        let dx = sub(rdf(pt), rdf(base));
        let dy = sub(rdf(pt + 4), rdf(base + 4));
        let dz = sub(rdf(pt + 8), rdf(base + 8));
        let d2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        let radius = rdf(slot + RADIUS);
        if radius > d2 { 1 } else { 0 }
    }
});
