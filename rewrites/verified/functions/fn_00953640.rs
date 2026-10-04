// original: 0x00953640 bucket_limit_or_zero
/// Map a bucket number to its entry limit, or 0 when out of range.
export!(cdecl, rw_00953640(bucket: u32) -> u32 {
    match bucket {
        1 => 0xF,
        2 => 0xA,
        3 => 5,
        4 => 3,
        5 => 0x32,
        6 => 0x42,
        7 => 0x4B,
        8 => 0x63,
        _ => 0,
    }
});
