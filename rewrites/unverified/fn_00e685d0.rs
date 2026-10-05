// original: 0x00e685d0 veh_pool_init_85d0

/// Initialise a 256-entry vehicle pool plus two singletons and 64 aux slots.
///
/// For each of 256 pool entries spaced `POOL_STRIDE` bytes apart from
/// `POOL_BASE`, calls initializer A with the entry and initializer B with
/// the entry plus `SUB_OFF`; then calls initializer C with each of the two
/// singletons at `SINGLE_A`/`SINGLE_B`; then calls initializer B with each
/// of 64 aux slots spaced `AUX_STRIDE` apart from `AUX_BASE`. Returns the
/// last call's answer (what the original leaves in EAX). All three callees
/// are intercepted and answered by the checker.
///
/// Original: 0x00E685D0 (cdecl/0, 578 direct calls).
lf_checker_rt::export!(cdecl, rw_00e685d0() -> u32 {
    unsafe {
        /// First pool entry (file VA).
        const POOL_BASE: u32 = 0x0139C280;
        /// Pool entries.
        const POOL_COUNT: u32 = 256;
        /// Bytes per pool entry.
        const POOL_STRIDE: u32 = 0x110;
        /// Offset of initializer B's object within an entry.
        const SUB_OFF: u32 = 0x20;
        /// First singleton (file VA).
        const SINGLE_A: u32 = 0x013AD280;
        /// Second singleton (file VA).
        const SINGLE_B: u32 = 0x013AD28C;
        /// First aux slot (file VA).
        const AUX_BASE: u32 = 0x013AD2A0;
        /// Aux slots.
        const AUX_COUNT: u32 = 64;
        /// Bytes per aux slot.
        const AUX_STRIDE: u32 = 0xF0;
        let mut ans = 0u32;
        let mut obj = lf_checker_rt::relocated(POOL_BASE);
        let mut i = 0u32;
        while i < POOL_COUNT {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, obj);
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, obj.wrapping_add(SUB_OFF));
            obj = obj.wrapping_add(POOL_STRIDE);
            i += 1;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(SINGLE_A));
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(SINGLE_B));
        let mut aux = lf_checker_rt::relocated(AUX_BASE);
        let mut j = 0u32;
        while j < AUX_COUNT {
            ans = lf_checker_rt::callee_thiscall!(2, u32, aux);
            aux = aux.wrapping_add(AUX_STRIDE);
            j += 1;
        }
        ans
    }
});
