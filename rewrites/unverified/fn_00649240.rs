// original: 0x00649240 ptx_shader_name_lookup (proposed)

/// Look up a shader template by the name registry: resolve the handle for
/// `this` through the first callee, then, when the registry count word is
/// non-zero, ask the registry search callee (slot pointer passed by address,
/// table, count, key size 8, fixed key) for the entry and return the word
/// past its header. Returns null when the registry is empty, the search
/// fails, the entry address wraps to zero, or the payload word is zero.
/// Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00649240(this: u32) -> u32 {
    unsafe {
        const REGISTRY_TABLE: u32 = 0x0110_eef4;
        const REGISTRY_COUNT: u32 = 0x0110_eef8;
        const SEARCH_KEY: u32 = 0x0041_2950;
        const KEY_SIZE: u32 = 8;
        const RESOLVE_CALLEE: u32 = 1;
        const SEARCH_CALLEE: u32 = 2;

        let _handle =
            lf_checker_rt::callee_cdecl!(RESOLVE_CALLEE, u32, this, 0);
        let count = (lf_checker_rt::relocated(REGISTRY_COUNT) as *const u16)
            .read_unaligned();
        if count == 0 {
            return 0;
        }
        let table = (lf_checker_rt::relocated(REGISTRY_TABLE) as *const u32)
            .read_unaligned();
        let mut slot: u32 = 0;
        let found = lf_checker_rt::callee_cdecl!(
            SEARCH_CALLEE,
            u32,
            (&mut slot as *mut u32) as u32,
            table,
            count as u32,
            KEY_SIZE,
            SEARCH_KEY
        );
        if found == 0 {
            return 0;
        }
        let entry = found.wrapping_add(4);
        if entry == 0 {
            return 0;
        }
        let payload = (entry as *const u32).read_unaligned();
        if payload == 0 {
            0
        } else {
            payload
        }
    }
});
