// original: 0x009DC750 registry_find_object (proposed)

/// Find the object registered for a key, copying its handle out on success.
///
/// When the registry count is zero there is nothing to search and 0 is
/// returned. Otherwise a search call runs over the registry base with the
/// word comparator; its out-slot is a dead stack word whose pre-write value
/// (the key) is snapshotted and whose address is skipped. A miss returns 0.
/// On a hit the found record's word at `+4` is a table handle: it is stored
/// through the caller's out-pointer when non-null, and the table's object
/// for it is returned.
///
/// Original: 0x009DC750 (cdecl, two stack arguments, one outgoing call).
lf_checker_rt::export!(cdecl, rw_009DC750(key: u32, out: u32) -> u32 {
    unsafe {
        const COUNT_VA: u32 = 0x0103AE8C;
        const BASE_VA: u32 = 0x0103AE88;
        const COMPARE_VA: u32 = 0x009DDEE0;
        const TABLE_VA: u32 = 0x01295CD8;
        const FIND: u32 = 1;

        let mut slot = key;
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
