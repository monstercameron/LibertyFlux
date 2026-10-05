// original: 0x00c740a0 ped_task_zone_test (proposed)

/// Test whether a ped's point falls in a hot zone, else run a fallback check.
///
/// `this` points to the ped task object. The routine first gates on a
/// counter: the 16-bit value at `+0x2C` plus a game global must be a multiple
/// of 16, else nothing happens. It then clears status bit 5 at `+0xF20` and
/// reads the point (three floats at `+0x30`, `+0x34`, `+0x38` of the object
/// at `+0x20`). A global zone count (signed; nothing is tested when it is
/// not positive) bounds a loop over 32-byte zone records starting at a fixed
/// game address, each holding minimum xyz at `-0x14`, `-0x10`, `-0x0C` and
/// maximum xyz at `-0x04`, `+0x00`, `+0x04` from the record cursor. Each
/// bound the point violates contributes a game-global miss marker; the six
/// contributions are or-ed per axis, and the zone matches when every axis
/// compares equal to zero (the same `ucomiss; lahf; (an instruction of the original); jp` idiom
/// as the spatial box finder: either signed zero passes, anything else
/// fails). On a match, status bit 5 is set and the routine returns. With no
/// match, a fallback runs: it returns unless `+0x1304` is zero, `+0xB0` is
/// nonzero and `+0xB8` is not negative (a null check on a lea-derived
/// address between them can never fire), then asks the intercepted thiscall
/// callee (no stack arguments) and sets status bit 5 when its low byte is
/// nonzero.
///
/// Original: 0x00C740A0 (thiscall, no stack arguments; no defined return).
lf_checker_rt::export!(thiscall, rw_00C740A0(this: u32) -> u32 {
    unsafe {
        const COUNT_GLOBAL: u32 = 0x1173604;
        const ZONE_COUNT: u32 = 0x16EBCA0;
        const ZONE_BASE: u32 = 0x16EBCC4;
        const MISS_GLOBAL: u32 = 0x17AD148;
        const STATUS: u32 = 0xF20;
        const STATUS_BIT: u8 = 0x20;
        const ZONE_STRIDE: u32 = 0x20;
        const FALLBACK: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
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
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read() }
        }
        /// Original `comiss a, b` followed by `jbe taken`: taken exactly
        /// when `a > b` is false, including unordered operands.
        #[inline(always)]
        fn below_or_equal(a: f32, b: f32) -> bool {
            !(core::hint::black_box(a) > core::hint::black_box(b))
        }

        let tick = rd16(this + 0x2C) as u32 + g32(COUNT_GLOBAL);
        if (tick as u8) & 0xF != 0 {
            return 0;
        }
        wr8(this + STATUS, rd8(this + STATUS) & !STATUS_BIT);
        let anchor = rd32(this + 0x20);
        let count = g32(ZONE_COUNT) as i32;
        let mut status = rd8(this + STATUS);
        let px = rdf(anchor + 0x30);
        let py = rdf(anchor + 0x34);
        let pz = rdf(anchor + 0x38);
        if count > 0 {
            let miss = f32::from_bits(g32(MISS_GLOBAL));
            let base0 = lf_checker_rt::relocated(ZONE_BASE);
            let mut k = 0i32;
            while k < count {
                let base = base0 + k as u32 * ZONE_STRIDE;
                let x0 = if below_or_equal(rdf(base - 0x14), px) { 0.0f32 } else { miss };
                let y0 = if below_or_equal(rdf(base - 0x10), py) { 0.0f32 } else { miss };
                let z0 = if below_or_equal(rdf(base - 0x0C), pz) { 0.0f32 } else { miss };
                let x1 = if below_or_equal(px, rdf(base - 0x04)) { 0.0f32 } else { miss };
                let y1 = if below_or_equal(py, rdf(base)) { 0.0f32 } else { miss };
                let z1 = if below_or_equal(pz, rdf(base + 0x04)) { 0.0f32 } else { miss };
                let ox = f32::from_bits(x0.to_bits() | x1.to_bits());
                let oy = f32::from_bits(y0.to_bits() | y1.to_bits());
                let oz = f32::from_bits(z0.to_bits() | z1.to_bits());
                if ox != 0.0 || oy != 0.0 {
                    k += 1;
                    continue;
                }
                if oz == 0.0 {
                    status |= STATUS_BIT;
                    wr8(this + STATUS, status);
                    return 0;
                }
                k += 1;
            }
        }
        if rd32(this + 0x1304) != 0 {
            return 0;
        }
        if this.wrapping_add(0x80) == 0 {
            return 0;
        }
        let arg = rd32(this + 0xB0);
        if arg == 0 {
            return 0;
        }
        if (rd32(this + 0xB8) as i32) < 0 {
            return 0;
        }
        let answer: u32 = lf_checker_rt::callee_thiscall!(FALLBACK, u32, arg);
        if answer as u8 == 0 {
            return 0;
        }
        wr8(this + STATUS, rd8(this + STATUS) | STATUS_BIT);
        0
    }
});
