// original: 0x009fa160 parse_chain_stage
/// Parse stage: run the base stage, then require a chain of field reads
/// (two 0x40 blob reads, four byte reads, one trailer read), each of which
/// must succeed. Returns 1 only if every stage succeeds.
export!(cdecl, rw_009fa160(a0: u32, a1: u32) -> u32 {
    unsafe {
        if callee_cdecl!(1, u32, a0, a1) as u8 == 0 {
            return 0;
        }
        if callee_thiscall!(2, u32, a0, a1.wrapping_add(0x38), 0x40) as u8 == 0 {
            return 0;
        }
        let w0 = (a1 as *const u32).byte_add(0x40).read();
        let w1 = (a1 as *const u32).byte_add(0x44).read();
        if (w0 | w1) != 0
            && callee_thiscall!(2, u32, a0, a1.wrapping_add(0x40), 0x40) as u8 == 0
        {
            return 0;
        }
        let byte_reads = [(0x49u32, 2u32), (0x48, 6), (0x4a, 6), (0x4b, 6)];
        let mut i = 0usize;
        while i < byte_reads.len() {
            let (off, n) = byte_reads[i];
            if callee_thiscall!(3, u32, a0, a1.wrapping_add(off), n) as u8 == 0 {
                return 0;
            }
            i += 1;
        }
        let ans = callee_thiscall!(4, u32, a0, a1.wrapping_add(0x4c));
        if ans as u8 == 0 {
            return 0;
        }
        (ans & 0xFFFF_FF00) | 1
    }
});
