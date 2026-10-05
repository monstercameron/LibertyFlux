// original: 0x00B83CC0 block_addr
/// Address one of the fixed blocks or the indexed table past `this`.
///
/// Index -2 selects the block at `BLK_A`, -1 the block at `BLK_B`, and any
/// other value the `idx`-th entry of the table at `+4` with `STRIDE`-byte
/// entries. The multiplication wraps.
///
/// Original: 0x00B83CC0 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00B83CC0(this: u32, idx: u32) -> u32 {
    unsafe {
        const BLK_A: u32 = 0x1F724;
        const BLK_B: u32 = 0x1E52C;
        const STRIDE: u32 = 0x8FC;
        let signed = idx as i32;
        if signed == -2 {
            return this.wrapping_add(BLK_A);
        }
        if signed == -1 {
            return this.wrapping_add(BLK_B);
        }
        this.wrapping_add(4).wrapping_add(idx.wrapping_mul(STRIDE))
    }
});
