// original: 0x009DC6F0 registry_find_object_hashed (proposed)

/// Hash the key, then find the object exactly like its unhashed sibling.
///
/// The hashed key is discarded (it only overwrites a saved register slot);
/// the search itself matches `rw_009DC750`, except the search call's
/// out-slot is misaligned dead scratch straddling two pushed arguments, so
/// its pre-write value is skipped rather than snapshotted (the straddled
/// argument bytes are still compared as call arguments).
///
/// Original: 0x009DC6F0 (cdecl, two stack arguments, two outgoing calls).
lf_checker_rt::export!(cdecl, rw_009DC6F0(key: u32, out: u32) -> u32 {
    unsafe {
        const COUNT_VA: u32 = 0x0103AE8C;
        const BASE_VA: u32 = 0x0103AE88;
        const COMPARE_VA: u32 = 0x009DDEE0;
        const TABLE_VA: u32 = 0x01295CD8;
        const HASH: u32 = 1;
        const FIND: u32 = 2;

        let _: u32 = lf_checker_rt::callee_cdecl!(HASH, u32, key);
        let mut slot = 0u32;
        let total = (lf_checker_rt::global::<u16>(COUNT_VA) as *const u16).read_unaligned();
        if total == 0 {
            return 0;
        }
        let base = (lf_checker_rt::global::<u32>(BASE_VA) as *const u32).read_unaligned();
        let found: u32 = lf_checker_rt::callee_cdecl!(
            FIND, u32,
            &mut slot as *mut u32 as u32,
            base,
            total as u32,
            8u32,
            lf_checker_rt::relocated(COMPARE_VA)
        );
        if found == 0 {
            return 0;
        }
        let record = found.wrapping_add(4);
        if record == 0 {
            return 0;
        }
        let handle = (record as *const u32).read_unaligned();
        if out != 0 {
            (out as *mut u32).write_unaligned(handle);
        }
        (lf_checker_rt::global::<u32>(TABLE_VA).wrapping_add(handle as usize) as *const u32)
            .read_unaligned()
    }
});
