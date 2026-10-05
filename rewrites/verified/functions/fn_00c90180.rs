// original: 0x00c90180 task_pool_free
/// Free a pool slot by index when its bit is set, else return the bit base.
///
/// `index` selects a slot. The bit for the slot lives in the bit array at
/// the shared base: word `(index >> 5)` (arithmetic shift), bit
/// `(1 << (index & 31))`. When the bit is clear the function returns the
/// bit-array base unchanged. When the bit is set it tail-jumps to the shared
/// pool-free body with the slot address `index * 36 + pool_base` as the
/// argument and returns that body's answer.
///
/// The contract covers indices 0..64 with bit patterns cycling so every
/// index sees its bit both set and clear.
///
/// Original: 0x00c90180 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00c90180(index: u32) -> u32 {
    unsafe {
        const BIT_BASE: u32 = 0x01713aa4;
        const POOL_BASE: u32 = 0x016fc990;
        const TAIL_CALLEE: u32 = 1;
        const SLOT_STRIDE: u32 = 36;
        let base = lf_checker_rt::global::<u32>(BIT_BASE).read();
        let bit: u32 = 1u32.wrapping_shl(index & 31);
        let word_index: u32 = ((index as i32 >> 5) as u32).wrapping_mul(4);
        let word: u32 =
            (base.wrapping_add(word_index) as *const u32).read_unaligned();
        if word & bit == 0 {
            return base;
        }
        let slot: u32 = index
            .wrapping_mul(SLOT_STRIDE)
            .wrapping_add(lf_checker_rt::relocated(POOL_BASE));
        lf_checker_rt::callee_cdecl!(TAIL_CALLEE, u32, slot)
    }
});
