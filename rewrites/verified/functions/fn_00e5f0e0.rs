// original: 0x00e5f0e0 store_const_block_00e5f0e0
/// Store a four-word constant block at 0x0110EE5C; the second word echoes the incoming stack scratch, which the contract defines as zero.
export!(cdecl, rw_00e5f0e0() -> u32 {
    unsafe {
        let block = global::<u32>(0x0110EE5C);
        *block.add(0) = relocated(0x00642150);
        *block.add(1) = 0;
        *block.add(2) = 0;
        *block.add(3) = relocated(0x00409610);
        0
    }
});
