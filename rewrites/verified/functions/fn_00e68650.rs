// original: 0x00e68650 veh_pool_init_8650

/// Initialise a 256-entry vehicle pool, one singleton, twelve small slots
/// and 32 aux slots.
///
/// Calls initializer A with each of 256 pool entries spaced `POOL_STRIDE`
/// apart from `POOL_BASE`; initializer B with the singleton at `SINGLE` and
/// with each of twelve slots spaced `SMALL_STRIDE` apart from `SMALL_BASE`;
/// initializer C with each of 32 aux slots spaced `AUX_STRIDE` apart from
/// `AUX_BASE`. Returns the last call's answer (what the original leaves in
/// EAX). All three callees are intercepted and answered by the checker.
///
/// Original: 0x00E68650 (cdecl/0, 301 direct calls).
lf_checker_rt::export!(cdecl, rw_00e68650() -> u32 {
    unsafe {
        /// First pool entry (file VA).
        const POOL_BASE: u32 = 0x013B0EF0;
        /// Pool entries.
        const POOL_COUNT: u32 = 256;
        /// Bytes per pool entry.
        const POOL_STRIDE: u32 = 0x50;
        /// Singleton (file VA).
        const SINGLE: u32 = 0x013B5EF0;
        /// First small slot (file VA).
        const SMALL_BASE: u32 = 0x013B5EFC;
        /// Small slots.
        const SMALL_COUNT: u32 = 12;
        /// Bytes per small slot.
        const SMALL_STRIDE: u32 = 0x0C;
        /// First aux slot (file VA).
        const AUX_BASE: u32 = 0x013B5F90;
        /// Aux slots.
        const AUX_COUNT: u32 = 32;
        /// Bytes per aux slot.
        const AUX_STRIDE: u32 = 0x40;
        let mut ans = 0u32;
        let mut obj = lf_checker_rt::relocated(POOL_BASE);
        let mut i = 0u32;
        while i < POOL_COUNT {
            let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, obj);
            obj = obj.wrapping_add(POOL_STRIDE);
            i += 1;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(SINGLE));
        let mut small = lf_checker_rt::relocated(SMALL_BASE);
        let mut j = 0u32;
        while j < SMALL_COUNT {
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, small);
            small = small.wrapping_add(SMALL_STRIDE);
            j += 1;
        }
        let mut aux = lf_checker_rt::relocated(AUX_BASE);
        let mut k = 0u32;
        while k < AUX_COUNT {
            ans = lf_checker_rt::callee_thiscall!(3, u32, aux);
            aux = aux.wrapping_add(AUX_STRIDE);
            k += 1;
        }
        ans
    }
});
