// original: 0x00B85C60 registry_release_lazy
/// Release `key` from the lazily created singleton registry.
///
/// Creates the registry on first use (callees 1-2, stored to `REG`);
/// allocation failure records a null registry. Callee 3 then releases
/// the key, and its answer is returned.
///
/// Original: 0x00B85C60 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00B85C60(key: u32) -> u32 {
    unsafe {
        const REG: u32 = 0x0167E3B4;
        const SIZE: u32 = 0x20020;
        let cur = (lf_checker_rt::global::<u32>(REG)).read_unaligned();
        if cur == 0 {
            let mem = lf_checker_rt::callee_cdecl!(1, u32, SIZE);
            if mem == 0 {
                (lf_checker_rt::global::<u32>(REG)).write_unaligned(0);
                return lf_checker_rt::callee_thiscall!(3, u32, 0, key);
            }
            let obj = lf_checker_rt::callee_thiscall!(2, u32, mem);
            (lf_checker_rt::global::<u32>(REG)).write_unaligned(obj);
            return lf_checker_rt::callee_thiscall!(3, u32, obj, key);
        }
        lf_checker_rt::callee_thiscall!(3, u32, cur, key)
    }
});
