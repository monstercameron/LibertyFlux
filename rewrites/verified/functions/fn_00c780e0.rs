// original: 0x00C780E0 rage::pgDictionary<rage::phBound>::vf0

/// Release the dictionary base, optionally release the object's stored data,
/// and return the same dictionary pointer. The method uses `this` in ECX and
/// removes one 32-bit stack argument on return. Its first helper call always
/// receives `this`; when bit zero of `release_data` is set, a second helper
/// receives that same pointer as its sole cdecl argument.
lf_checker_rt::export!(thiscall, rw_pg_dictionary_phbound_vf0(this: u32, release_data: u32) -> u32 {
    unsafe {
        const DESTROY_BASE: u32 = 1;
        const FREE_DATA: u32 = 2;

        let _base_result = lf_checker_rt::callee_thiscall!(DESTROY_BASE, u32, this);
        if release_data & 1 != 0 {
            let _ = lf_checker_rt::callee_cdecl!(FREE_DATA, u32, this);
        }
        this
    }
});
