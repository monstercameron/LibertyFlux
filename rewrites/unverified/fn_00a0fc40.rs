// original: 0x00a0fc40 port_table_init_tail (proposed)
/// Initialise the port table, then tail-jump to the starter.
///
/// Marks the table slot missing, resolves the first port (which fills the
/// slot), and when the slot is still missing afterwards, resolves the second
/// port, primes the resolver, resolves the third port, then tags the result
/// (sets bit 3 at `+0x40`) and stores 40.0 (0x42200000) at `+0x2c`. Either
/// way control passes to the starter with no arguments (a tail jump,
/// expressed here as a returning call). Cdecl, no arguments.
export!(cdecl, rw_00a0fc40() -> u32 {
    unsafe {
        const RESOLVE: u32 = 1;
        const SECOND: u32 = 2;
        const PRIME: u32 = 3;
        const STARTER: u32 = 4;
        const SLOT: u32 = 0x012bd0f4;
        const PORT_A: u32 = 0x00e9a770;
        const PORT_B: u32 = 0x00e9a77c;
        const PORT_C: u32 = 0x00e9a788;
        const TAG_OFF: u32 = 0x40;
        const TAG_BIT: u32 = 8;
        const RATE_OFF: u32 = 0x2c;
        const RATE_BITS: u32 = 0x42200000;
        *global::<u32>(SLOT) = 0xffff_ffff;
        callee_cdecl!(RESOLVE, u32, relocated(PORT_A), relocated(SLOT));
        if *global::<u32>(SLOT) != 0xffff_ffff {
            return callee_cdecl!(STARTER, u32,);
        }
        callee_stdcall!(SECOND, u32, relocated(PORT_B));
        callee_cdecl!(PRIME, u32,);
        let r = callee_cdecl!(RESOLVE, u32, relocated(PORT_C), relocated(SLOT));
        let t = (r + TAG_OFF) as *mut u32;
        t.write_unaligned(t.read_unaligned() | TAG_BIT);
        ((r + RATE_OFF) as *mut u32).write_unaligned(RATE_BITS);
        callee_cdecl!(STARTER, u32,)
    }
});
