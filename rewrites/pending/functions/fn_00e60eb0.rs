// original: 0x00e60eb0 zero_block_then_submit
/// zero_block_then_submit: zero one globals block, then submit one stub.
///
/// Clears seven words, clears bit 0 of the flag byte, stores the
/// block's limit word (0x64), then hands this block's stub to the
/// shared submit routine and returns its answer. (The original
/// re-reads two of the zeros between the stores; the values are
/// dead, so the rewrite does not repeat the reads.)
lf_checker_rt::export!(cdecl, rw_00e60eb0() -> u32 {
    unsafe {
        const LIMIT: u32 = 0x64;
        let w = lf_checker_rt::global::<u32>(0x019F925C);
        w.add(0).write(0);
        w.add(1).write(0);
        let flag = lf_checker_rt::global::<u8>(0x019F927C);
        flag.write(flag.read() & 0xFE);
        w.add(2).write(0);
        w.add(3).write(0);
        w.add(4).write(0);
        w.add(5).write(0);
        w.add(6).write(0);
        w.add(7).write(LIMIT);
        lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00E6FD50))
    }
});
