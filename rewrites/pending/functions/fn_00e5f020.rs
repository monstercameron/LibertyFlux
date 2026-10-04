// original: 0x00e5f020 store_const_block_00e5f020
/// Store a four-word constant block at 0x0110ECE8; the second word echoes the incoming stack scratch, which the contract defines as zero.
export!(cdecl, rw_00e5f020() -> u32 {
    unsafe {
        let block = global::<u32>(0x0110ECE8);
        *block.add(0) = relocated(0x0043EA90);
        *block.add(1) = 0;
        *block.add(2) = 0;
        *block.add(3) = relocated(0x00409610);
        0
    }
});
