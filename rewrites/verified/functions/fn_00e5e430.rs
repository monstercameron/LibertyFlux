// original: 0x00e5e430 net_pool_init_e430
/// Initialise a pool of 17 network objects, returning the last answer.
///
/// Calls the object initializer on each of the 18 consecutive 0x190-byte
/// objects starting at `0x019D3B60`, and returns the last call's answer.
export!(cdecl, rw_00e5e430() -> u32 {
    unsafe {
        /// First object in the pool (file VA).
        const POOL: u32 = 0x019D3B60;
        /// Object stride in bytes.
        const STRIDE: u32 = 0x190;
        /// Objects in the pool.
        const COUNT: usize = 18;
        let base = relocated(POOL);
        let mut answer: u32 = 0;
        let mut i = 0;
        while i < COUNT {
            answer = callee_thiscall!(1, u32, base.wrapping_add(STRIDE * i as u32));
            i += 1;
        }
        answer
    }
});
