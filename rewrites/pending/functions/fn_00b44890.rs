// original: 0x00b44890 merge_minmax_boxes
/// Merge two bound boxes into a destination: per-slot signed minima of the
/// min slots and signed maxima of the max slots. Returns the first source's
/// third max word.
export!(thiscall, rw_b44890(dst: u32, a: u32, b: u32) -> u32 {
    unsafe {
        let r = |p: u32, off: usize| (p as *const i16).byte_add(off).read();
        let w = |off: usize, v: i16| (dst as *mut i16).byte_add(off).write(v);
        w(0, r(a, 0).min(r(b, 0)));
        w(4, r(a, 4).min(r(b, 4)));
        w(8, r(a, 8).min(r(b, 8)));
        w(2, r(a, 2).max(r(b, 2)));
        w(6, r(a, 6).max(r(b, 6)));
        w(10, r(a, 10).max(r(b, 10)));
        (a as *const u16).byte_add(10).read() as u32
    }
});
