// original: 0x00bd6a80 Txd_FindLoaded

/// Use the primary dictionary name when nonnull, otherwise the fallback name; normalize the selected handle, hash the lookup name, and search the dictionary manager. If no dictionary is found, call slot 3 of its fabricated function table and return that result. Both direct helpers and the final indirect callback are intercepted by the checker.
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn rw_00bd6a80(
    primary_name: u32,
    fallback_name: u32,
) -> u32 {
    const TRY_PRIMARY: u32 = 1;
    const TRY_FALLBACK: u32 = 2;
    const NORMALIZE_NAME: u32 = 3;
    const HASH_NAME: u32 = 4;
    const FIND_DICTIONARY: u32 = 5;
    const INDIRECT_FALLBACK: u32 = 6;
    const PRIMARY_MANAGER: u32 = 0x011d_95f0;
    const TXD_MANAGER_GLOBAL: u32 = 0x017f_5630;

    unsafe {
        let selected_name = if primary_name != 0 {
            lf_checker_rt::callee_thiscall!(
                TRY_PRIMARY,
                u32,
                lf_checker_rt::relocated(PRIMARY_MANAGER),
                primary_name,
            )
        } else {
            lf_checker_rt::callee_cdecl!(TRY_FALLBACK, u32, fallback_name)
        };
        let normalized_name = lf_checker_rt::callee_cdecl!(NORMALIZE_NAME, u32, selected_name);
        let hash = lf_checker_rt::callee_cdecl!(HASH_NAME, u32, fallback_name);
        let found = lf_checker_rt::callee_thiscall!(FIND_DICTIONARY, u32, normalized_name, hash);
        if found != 0 {
            found
        } else {
            let manager = lf_checker_rt::global::<u32>(TXD_MANAGER_GLOBAL).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let slot_3 = (vtable as *const u32).add(3).read_unaligned();
            let callback: unsafe extern "C" fn() -> u32 = core::mem::transmute(slot_3);
            callback()
        }
    }
}
