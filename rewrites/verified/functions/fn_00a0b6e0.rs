// original: 0x00a0b6e0 cleanup_entry_is_valid (proposed)
/// Test whether a cleanup entry still names a live object.
///
/// With the strict flag clear, an entry with a zero tag is already invalid.
/// With it set, tags 0 and 0x17 are invalid without further checks. Any
/// other tag loads the handle at +8: a zero handle is invalid, otherwise
/// the handle is resolved and the target asked whether it is alive.
/// Returns 1 for alive, 0 otherwise (low byte only). Stdcall, two words.
lf_checker_rt::export!(stdcall, rw_00a0b6e0(entry: u32, strict: u32) -> u32 {
    unsafe {
        const SKIP_TAG: u8 = 0x17;
        const RESOLVE: u32 = 0;
        const IS_ALIVE: u32 = 1;
        let tag = (entry as *const u8).read();
        if (strict & 0xff) == 0 {
            if tag == 0 {
                return 0;
            }
        } else {
            if tag == 0 || tag == SKIP_TAG {
                return 0;
            }
        }
        let handle = ((entry + 8) as *const u32).read_unaligned();
        if handle == 0 {
            return 0;
        }
        let target = lf_checker_rt::callee_cdecl!(RESOLVE, u32, handle);
        let alive: u32 = lf_checker_rt::callee_thiscall!(IS_ALIVE, u32, target);
        ((alive & 0xff) != 0) as u32
    }
});
