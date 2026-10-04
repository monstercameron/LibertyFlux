// original: 0x009529e0 kind_to_flag_bit
/// Map a small kind number to its single-bit flag. Values with no assigned
/// bit map to zero.
export!(cdecl, rw_009529e0(kind: u32) -> u32 {
    if (2..=6).contains(&kind) {
        1
    } else if (9..=18).contains(&kind) {
        2
    } else if (0x15..=0x18).contains(&kind) {
        4
    } else if (0x1b..=0x1d).contains(&kind) {
        8
    } else if (0x20..=0x21).contains(&kind) {
        0x10
    } else if (0x24..=0x25).contains(&kind) {
        0x20
    } else if (0x28..=0x67).contains(&kind) {
        0x40
    } else if (0x6a..=0x6f).contains(&kind) {
        0x80
    } else if (0x72..=0x91).contains(&kind) || kind == 0x98 {
        0x100
    } else if kind == 0x94 || kind == 0x95 {
        0x200
    } else if kind == 0x9c {
        0x400
    } else {
        0
    }
});
