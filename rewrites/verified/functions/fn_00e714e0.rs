// original: 0x00E714E0 drain_and_notify_streaming_entries
/// For each of 14,400 records, notify the associated slot at offset four and then release the record while walking backward by twelve bytes.
lf_checker_rt::export!(cdecl, rw_fn_00E714E0() -> () {
    unsafe {
        const FIRST_BLOCK: u32 = 0x011D4020;
        const BLOCK_STRIDE: u32 = 0x0C;
        const BLOCK_COUNT: u32 = 14400;

        let mut block = lf_checker_rt::relocated(FIRST_BLOCK);
        for _ in 0..BLOCK_COUNT {
            block = block.wrapping_sub(BLOCK_STRIDE);
            let associated = block.wrapping_add(4);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, associated);
            let _ = lf_checker_rt::callee_thiscall!(2, u32, block);
        }
    }
});
