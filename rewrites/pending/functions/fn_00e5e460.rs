// original: 0x00e5e460 net_queue_shift_e460
/// Shift a network queue buffer forward, clear its tail, register the handler.
///
/// Copies 62 dwords from `0x0110E778` to `0x0110E780`, zeroes the dword at
/// `0x0110E878`, then registers the handler at `0x00E6EED0` and returns the
/// registrar's answer.
export!(cdecl, rw_00e5e460() -> u32 {
    unsafe {
        /// Copy source (file VA).
        const SRC: u32 = 0x0110E778;
        /// Copy destination, two words past the source (file VA).
        const DST: u32 = 0x0110E780;
        /// Tail word cleared after the shift (file VA).
        const TAIL: u32 = 0x0110E878;
        /// Handler registered (file VA).
        const HANDLER: u32 = 0x00E6EED0;
        /// Words copied.
        const WORDS: usize = 62;
        let src = global::<u32>(SRC);
        let dst = global::<u32>(DST);
        // The original is a forward block copy whose destination overlaps the
        // source, so later words read values written earlier in the same copy.
        // A volatile word loop pins that order; a slice copy or memmove would
        // read the pristine source instead.
        let mut i = 0;
        while i < WORDS {
            let v = src.add(i).read_volatile();
            dst.add(i).write_volatile(v);
            i += 1;
        }
        global::<u32>(TAIL).write(0);
        callee_cdecl!(2, u32, relocated(HANDLER))
    }
});
