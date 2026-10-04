// original: 0x005d6110 bytes_all_nonzero_sampled
/// Check that the sampled bytes of the string are all non-zero: the first
/// byte plus one probe per size bound the (signed) bound reaches. Only the
/// low result byte is significant. Returns 1 when the bound is not
/// positive or every reached probe is non-zero, else 0.
export!(fastcall, rw_005d6110(s: u32, bound: i32) -> u32 {
    /// (minimum bound that enables the probe, probed offset)
    const PROBES: [(i32, u32); 10] = [
        (4, 1),
        (8, 5),
        (0x0C, 9),
        (0x14, 0x11),
        (0x1A, 0x17),
        (0x22, 0x1F),
        (0x2E, 0x2B),
        (0x3A, 0x37),
        (0x40, 0x3D),
        (0x4C, 0x49),
    ];
    if bound <= 0 {
        return 1;
    }
    if unsafe { (s as *const u8).read() } == 0 {
        return 0;
    }
    for (min, off) in PROBES {
        if bound >= min && unsafe { (s.wrapping_add(off) as *const u8).read() } == 0 {
            return 0;
        }
    }
    1
});
