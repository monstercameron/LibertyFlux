// original: 0x008ddfc0 CSmashMan_SetDrawBuckets::vf1

/// Forward this entry's draw bucket to the bucket install routine.
///
/// Same selection as the reset twin: the bucket object comes from the table
/// at `TABLE` indexed by `this + 8`, the primary link (`+8`) is preferred
/// through its `+0xb4` word, else the fallback link (`+0xc`) through its
/// head word, and a zero word on both sides means return without calling.
/// Forwards through the install routine (callee 1, called with the manager
/// singleton). Thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_008ddfc0(this: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0129_5cd8;
        const MANAGER: u32 = 0x0130_5d30;
        const INDEX_OFF: u32 = 8;
        const PRIMARY_OFF: u32 = 8;
        const FALLBACK_OFF: u32 = 0x0c;
        const BUCKET_OFF: u32 = 0xb4;
        const CALLEE_INSTALL: u32 = 1;
        let index = ((this + INDEX_OFF) as *const u32).read_unaligned();
        let table = lf_checker_rt::relocated(TABLE);
        let obj = ((table + index.wrapping_mul(4)) as *const u32).read_unaligned();
        let primary = ((obj + PRIMARY_OFF) as *const u32).read_unaligned();
        let picked = if primary != 0 {
            ((primary + BUCKET_OFF) as *const u32).read_unaligned()
        } else {
            let fallback = ((obj + FALLBACK_OFF) as *const u32).read_unaligned();
            if fallback == 0 {
                return 0;
            }
            (fallback as *const u32).read_unaligned()
        };
        if picked == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            CALLEE_INSTALL,
            u32,
            lf_checker_rt::relocated(MANAGER),
            picked
        );
        0
    }
});
