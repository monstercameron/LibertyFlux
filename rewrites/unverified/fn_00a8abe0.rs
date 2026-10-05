// original: 0x00a8abe0 pool_construct_full (proposed)

/// Construct the pool object: capacity, two wired halves, joined config.
///
/// `this` is the pool. Zeroes the state word at +0x48, initialises the
/// +8 sub-object's capacity through the capacity callee, then wires two
/// descriptor halves through the wire callee (each taking a frame slot,
/// a zero tag, a handler address and two zero words; the wire callee
/// fills four words per slot). The eight slot words are then shuffled
/// into the config callee's eight arguments as [w4 w5 w6 w7 w0 w1 w2 w3]
/// with the two tag slots reading as the join address. Returns whatever
/// the config callee returns. The proof models the wire footprint as four
/// words per slot feeding the config arguments; frame addresses are
/// uncompared.
///
/// Original: 0x00A8ABE0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a8abe0(this: u32) -> u32 {
    unsafe {
        const CALLEE_CAPACITY: u32 = 1;
        const CALLEE_WIRE: u32 = 2;
        const CALLEE_CONFIG: u32 = 3;
        const STATE: u32 = 0x48;
        const SUB_OBJECT: u32 = 8;
        const HANDLER_A: u32 = 0xa8abc0;
        const HANDLER_B: u32 = 0x4016a0;
        const JOIN: u32 = 0x430260;
        ((this + STATE) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(
            CALLEE_CAPACITY,
            u32,
            this.wrapping_add(SUB_OBJECT)
        );
        let mut f = [0u32; 8];
        {
            let s1 = core::ptr::addr_of_mut!(f[1]) as u32;
            lf_checker_rt::callee_thiscall!(
                CALLEE_WIRE,
                u32,
                s1,
                0,
                lf_checker_rt::relocated(HANDLER_A),
                0,
                0
            );
        }
        {
            let s0 = core::ptr::addr_of_mut!(f[0]) as u32;
            lf_checker_rt::callee_thiscall!(
                CALLEE_WIRE,
                u32,
                s0,
                0,
                lf_checker_rt::relocated(HANDLER_B),
                0,
                0
            );
        }
        f[3] = JOIN;
        f[7] = JOIN;
        lf_checker_rt::callee_cdecl!(
            CALLEE_CONFIG,
            u32,
            f[4],
            f[5],
            f[6],
            f[7],
            f[0],
            f[1],
            f[2],
            f[3]
        )
    }
});
