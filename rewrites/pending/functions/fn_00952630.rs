// original: 0x00952630 size_class_picker
/// Pick a size class from a byte count, comparing as a signed 32-bit
/// value: at least 0x780 gives 0x3000, at least 0x500 gives 0x1fa0,
/// anything smaller (including every negative value) gives 0x1000.
export!(cdecl, rw_00952630(count: u32) -> u32 {
    let s = count as i32;
    if s >= 0x780 {
        0x3000
    } else if s >= 0x500 {
        0x1fa0
    } else {
        0x1000
    }
});
