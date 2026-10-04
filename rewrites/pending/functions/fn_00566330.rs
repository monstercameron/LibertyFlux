// original: 0x00566330 race89_vf2_info_query

/// Leaderboard info query: publish this instance for a matching id.
///
/// Reads the object's virtual slot 1 result and compares it with `want`.
/// On a mismatch, or when `out` is null, returns 0 and writes nothing.
/// Otherwise stores the address of this leaderboard's info block
/// (`INFO_BLOCK`, relocated) through `out` and returns `out`.
///
/// Edge cases: null `out` with a matching id still returns 0; the stored
/// address is derived from the relocated image base, never hard-coded.
///
/// Original: thiscall, two stack words (`out`, `want`); ECX is the object.
lf_checker_rt::export!(thiscall, rw_00566330(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        const INFO_BLOCK: u32 = 0xFD54A4;
        const VTABLE_SLOT: u32 = 4;

        let vtable = (this as *const u32).read_unaligned();
        let query: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            (vtable.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned() as usize);
        if query(this) != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(INFO_BLOCK));
        out
    }
});
