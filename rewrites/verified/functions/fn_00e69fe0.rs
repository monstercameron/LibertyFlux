// original: 0x00e69fe0 veh_asset_lookup_block (proposed)

/// Resolves eight vehicle asset identifiers and stores each globally.
///
/// Calls the name lookup (a two-word cdecl callee) eight times, once
/// per entry of `NAMES` with a zero flags word, storing each returned
/// id at the matching entry of `OUTS`. Takes no arguments, returns
/// nothing (cdecl/0). The eight calls run in table order; every id is
/// stored, including the last.
///
/// Original: 0x00e69fe0 (cdecl, no arguments, eight calls).
lf_checker_rt::export!(cdecl, rw_00e69fe0() -> () {
    unsafe {
        /// Asset name pointers (file VAs), in call order.
        const NAMES: [u32; 8] = [0x00EB99F4, 0x00EB9A10, 0x00EB9A30, 0x00EB9A4C, 0x00EB9A6C, 0x00EB9A88, 0x00EB9AA8, 0x00EB9AC0];
        /// Resolved-id destination globals (file VAs), in call order.
        const OUTS: [u32; 8] = [0x01682F3C, 0x01682F40, 0x01682F44, 0x01682F48, 0x01682F4C, 0x01682F50, 0x01682F54, 0x01682F58];
        /// Name-lookup callee id.
        const LOOKUP: u32 = 1;
        for (name, out) in NAMES.iter().zip(OUTS.iter()) {
            let id: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, lf_checker_rt::relocated(*name), 0);
            lf_checker_rt::global::<u32>(*out).write_unaligned(id);
        }
    }
});
