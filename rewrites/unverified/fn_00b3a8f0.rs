// original: 0x00b3a8f0 task_lookup_or_default (proposed)

/// Look up `key` through the table callee: when the lookup reports missing
/// (-1) return -1, otherwise format the found index with the shared buffer
/// global and the fixed width through the formatter callee and return its
/// result. Original: 0x00b3a8f0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b3a8f0(key: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const FORMAT: u32 = 2;
        const BUFFER_GLOBAL: u32 = 0x012b4138;
        const WIDTH: u32 = 10;
        const MISSING: u32 = 0xffff_ffff;
        let found: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key);
        if found == MISSING {
            return found;
        }
        let buffer = lf_checker_rt::global::<u32>(BUFFER_GLOBAL).read();
        lf_checker_rt::callee_cdecl!(FORMAT, u32, found, buffer, WIDTH)
    }
});
