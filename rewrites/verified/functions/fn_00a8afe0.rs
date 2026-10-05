// original: 0x00a8afe0 pool_init_empty_list (proposed)

/// Initialise this object as an empty intrusive list under a lock.
///
/// `this` is the pool object. Takes the shared lock through the first
/// callee, points the head at the inline anchor (`this+8`, the empty-list
/// mark), then releases through the second callee. Returns whatever the
/// release callee returns, as the original leaves it in eax. The lock
/// slot addresses (frame pointers in the original) are unobserved by the
/// proof: only the pushed lock id and the head store are compared.
///
/// Original: 0x00A8AFE0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a8afe0(this: u32) -> u32 {
    unsafe {
        const CALLEE_LOCK: u32 = 1;
        const CALLEE_UNLOCK: u32 = 2;
        const LOCK_ID: u32 = 0x12fb1dc;
        const ANCHOR: u32 = 8;
        let mut slot: u32 = 0;
        let slot_addr = core::ptr::addr_of_mut!(slot) as u32;
        lf_checker_rt::callee_thiscall!(
            CALLEE_LOCK,
            u32,
            slot_addr,
            lf_checker_rt::relocated(LOCK_ID)
        );
        ((this) as *mut u32).write_unaligned(this.wrapping_add(ANCHOR));
        lf_checker_rt::callee_thiscall!(CALLEE_UNLOCK, u32, slot_addr)
    }
});
