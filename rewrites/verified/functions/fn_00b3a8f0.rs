// original: 0x00B3A8F0 id_lookup_or_log

/// Look up `id` through callee 1; on a hit (`!= -1`) log it via callee 2.
///
/// Callee 1 maps the id to a handle, returning -1 for unknown ids. Known
/// handles are reported to callee 2 as `(handle, G_TAG, 10)`. Returns the
/// lookup result on a miss, else callee 2's answer. Cdecl, one stack word.
///
/// Original: 0x00B3A8F0.

lf_checker_rt::export!(cdecl, rw_00B3A8F0(id: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const LOG: u32 = 2;
        const G_TAG: u32 = 0x012B4138;
        const MISS: u32 = 0xFFFF_FFFF;
        const KIND: u32 = 10;
        let h: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, id);
        if h == MISS {
            return h;
        }
        let tag = (lf_checker_rt::global::<u32>(G_TAG) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(LOG, u32, h, tag, KIND)
    }
});
