// original: 0x00B82CD0 dist_check_vt
/// Test whether a point is inside a slot's radius (virtual probe, 2d).
///
/// Resolves the entity's slot index through its virtual probe (callee
/// 1, table slot `0x3C`); a null probe or a negative index answers 0.
/// Otherwise the slot (kind 2 required) supplies the radius against the
/// squared 2d distance from `pt` to the entity's anchor: 1 exactly when
/// the radius is ordered-greater. Float order matches the original.
///
/// Original: 0x00B82CD0 (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_00B82CD0(this: u32, ent: u32, pt: u32) -> u32 {
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
        const VSLOT: u32 = 0x3C;
        const STRIDE: u32 = 0x2C;
        const KIND: u32 = 0x18;
        const RADIUS: u32 = 0x1C;
        let vt = rd32(ent);
        let target = rd32(vt + VSLOT);
        let probe: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        let ans = probe(ent);
        if ans == 0 {
            return 0;
        }
        let idx = rd16s(ans + 0x70);
        if idx < 0 {
            return 0;
        }
        let slot = this.wrapping_add((idx as u32).wrapping_mul(STRIDE));
        if rd8(slot + KIND) != 2 {
            return 0;
        }
        let dx = sub(rdf(pt), rdf(ent + 4));
        let dy = sub(rdf(pt + 4), rdf(ent + 8));
        let d2 = add(mul(dy, dy), mul(dx, dx));
        let radius = rdf(slot + RADIUS);
        if radius > d2 { 1 } else { 0 }
    }
});
