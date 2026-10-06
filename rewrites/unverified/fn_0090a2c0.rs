// original: 0x0090a2c0 font_range_extend_max (proposed)
/// Raise the range maximum to at least `v` and count one more sample.
///
/// `this` points to the range object (`+4` a sample count, `+8` the current
/// maximum as float). The new maximum is the larger of the old one and `v`
/// (an unordered `comiss`, i.e. either side NaN, keeps `v`, which a plain
/// `>` comparison reproduces exactly); the count is incremented and 1 is
/// returned. Only the fifth stack word is read; the first four are ignored.
/// Thiscall with five stack words, callee cleanup.
export!(thiscall, rw_0090a2c0(this: u32, _a: u32, _b: u32, _c: u32, _d: u32, v: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x04;
        const MAX_OFF: u32 = 0x08;
        let cur = f32::from_bits(((this.wrapping_add(MAX_OFF)) as *const u32).read_unaligned());
        let nv = f32::from_bits(v);
        let best = if cur > nv { cur } else { nv };
        let c = ((this.wrapping_add(COUNT_OFF)) as *const u32).read_unaligned();
        ((this.wrapping_add(COUNT_OFF)) as *mut u32).write_unaligned(c.wrapping_add(1));
        ((this.wrapping_add(MAX_OFF)) as *mut u32).write_unaligned(best.to_bits());
        1
    }
});
