// original: 0x00a8abd0 pool_init_capacity_10000 (proposed)

/// Initialise the sub-object at +8 with capacity 10000.
///
/// `this` is the pool object. Forwards 10000 to the capacity callee with
/// `this+8` as the object. Returns whatever the callee returns, as the
/// original leaves it in eax.
///
/// Original: 0x00A8ABD0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a8abd0(this: u32) -> u32 {
    unsafe {
        const CALLEE_SET_CAPACITY: u32 = 1;
        const SUB_OBJECT: u32 = 8;
        const CAPACITY: u32 = 10000;
        lf_checker_rt::callee_thiscall!(
            CALLEE_SET_CAPACITY,
            u32,
            this.wrapping_add(SUB_OBJECT),
            CAPACITY
        )
    }
});
