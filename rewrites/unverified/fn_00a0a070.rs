// original: 0x00a0a070 mission_cleanup_find (proposed)
/// Find a mission-cleanup entry by type tag and handle.
///
/// Scans the 0x100 records of 0x2c bytes at `this + 4` for the first whose
/// dword at +4 equals `handle` and whose tag byte at +0 equals `tag`. The
/// tag comparison is a full 32-bit compare against the zero-extended byte,
/// so a `tag` above 0xff never matches. `extra` is unread. Returns the
/// record pointer, or 0. Thiscall with three stack words.
lf_checker_rt::export!(thiscall, rw_00a0a070(this: u32, tag: u32, handle: u32, _extra: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x100;
        const STRIDE: u32 = 0x2c;
        let mut rec = this + 4;
        let mut i = 0u32;
        while i < COUNT {
            let h = ((rec + 4) as *const u32).read_unaligned();
            if h == handle {
                let t = ((rec as *const u8).read()) as u32;
                if t == tag {
                    return rec;
                }
            }
            i += 1;
            rec += STRIDE;
        }
        0
    }
});
