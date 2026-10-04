// original: 0x009b7600 rw_009b7600
/// Element address within a stride-0x84 table: `index * 0x84 + table`.
/// Wraps on overflow like the original multiply-and-add.
export!(thiscall, rw_009b7600(table: u32, index: u32) -> u32 {
    index.wrapping_mul(0x84).wrapping_add(table)
});
