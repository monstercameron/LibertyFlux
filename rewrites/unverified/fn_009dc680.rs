// original: 0x009DC680 registry_find_index_hashed (proposed)

/// Hash the key, then search the registry, returning the raw handle or -1.
///
/// The hash call's answer becomes the search's dead out-slot initial value.
/// The search matches the sibling lookups, but instead of mapping the found
/// record's handle through the object table it is returned directly, and
/// every miss path (empty registry, search miss, null record) returns -1.
/// The search code lives in the tail routine this function jumps to; the
/// rewrite inlines that routine's behaviour (same calls, same order).
///
/// Original: 0x009DC680 (cdecl, one stack argument, two outgoing calls).
lf_checker_rt::export!(cdecl, rw_009DC680(key: u32) -> u32 {
    unsafe {
        const COUNT_VA: u32 = 0x0103AE8C;
        const BASE_VA: u32 = 0x0103AE88;
        const COMPARE_VA: u32 = 0x009DDEE0;
        const HASH: u32 = 1;
        const FIND: u32 = 2;

        let hashed: u32 = lf_checker_rt::callee_cdecl!(HASH, u32, key);
        let mut slot = hashed;
        let total = (lf_checker_rt::global::<u16>(COUNT_VA) as *const u16).read_unaligned();
        if total == 0 {
            return 0xFFFF_FFFF;
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
            return 0xFFFF_FFFF;
        }
        let record = found.wrapping_add(4);
        if record == 0 {
            return 0xFFFF_FFFF;
        }
        (record as *const u32).read_unaligned()
    }
});
