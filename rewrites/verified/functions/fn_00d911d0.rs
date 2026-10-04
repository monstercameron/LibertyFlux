// original: 0x00d911d0 audio_box_query_recursive
/// Find the leaf box containing a point, searching a four-way box tree.
///
/// The point's x and y (single precision, read through `pt`) must lie in
/// the half-open range of the box at `bx`; an out-of-range point, including
/// any NaN coordinate, matches nothing. A box flagged at offset 0x2c
/// matches itself, otherwise each nonzero child at offsets 0x30..0x40 is
/// searched in order and the first match wins. Returns the matching box
/// pointer, or null.
lf_rs89_rt::export!(thiscall, rw_00d911d0(this: u32, bx: u32, pt: u32) -> u32 {
    unsafe {
        let px = *(pt as *const f32);
        if px < *(bx as *const f32) {
            return 0;
        }
        if !(*(bx.wrapping_add(0x10) as *const f32) > px) {
            return 0;
        }
        let py = *(pt.wrapping_add(4) as *const f32);
        if py < *(bx.wrapping_add(4) as *const f32) {
            return 0;
        }
        if !(*(bx.wrapping_add(0x14) as *const f32) > py) {
            return 0;
        }
        if *(bx.wrapping_add(0x2C) as *const u32) != 0 {
            return bx;
        }
        let mut child_at = bx.wrapping_add(0x30);
        let end = bx.wrapping_add(0x40);
        while child_at < end {
            let child = *(child_at as *const u32);
            if child != 0 {
                let hit: u32 = lf_rs89_rt::callee_thiscall!(1, u32, this, child, pt);
                if hit != 0 {
                    return hit;
                }
            }
            child_at += 4;
        }
        0
    }
});
