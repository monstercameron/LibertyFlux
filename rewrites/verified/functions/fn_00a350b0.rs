// original: 0x00a350b0 vehicle_locked_lookup (proposed)

/// Search a 90-entry table under the table lock.
///
/// Acquires the lock (id 1, thiscall/1) on a two-word scratch slot,
/// scans dwords spaced `0x24` apart from `TABLE` to `END` for `needle`,
/// releases through the two unlock steps (ids 2 and 3, thiscall/0 each
/// on the same slot), and answers whether the value was found. Cdecl/1,
/// returns AL. The scratch addresses legitimately differ per side, so
/// only their contents are compared.
lf_checker_rt::export!(cdecl, rw_00a350b0(needle: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x012D_F0C8;
        const END: u32 = 0x012D_FD70;
        const STRIDE: u32 = 0x24;
        const LOCK_ARG: u32 = 0x012E_1EC0;
        const ACQUIRE: u32 = 1;
        const UNLOCK_A: u32 = 2;
        const UNLOCK_B: u32 = 3;
        let mut slot = [0u32; 2];
        let p = core::ptr::addr_of_mut!(slot) as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(ACQUIRE, u32, p, lf_checker_rt::relocated(LOCK_ARG));
        let mut q = TABLE;
        let mut found = false;
        while q < END {
            if core::ptr::read(lf_checker_rt::global::<u32>(q)) == needle {
                found = true;
                break;
            }
            q = q.wrapping_add(STRIDE);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(UNLOCK_A, u32, p);
        let _: u32 = lf_checker_rt::callee_thiscall!(UNLOCK_B, u32, p);
        found as u32
    }
});
