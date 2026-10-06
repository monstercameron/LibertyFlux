// original: 0x00892680 audsound_resolve_then_open
/// Resolves `key` to a handle, then opens it, passing negatives through.
///
/// Calls the resolver callee with (`this`, `key`); when its answer is
/// NEGATIVE (signed) returns all-ones without calling further, otherwise
/// calls the opener callee with (`this`, answer) and returns its answer.
/// Both callees are thiscall with one stack word.
/// Original: 0x00892680 (thiscall, one stack word).
export!(thiscall, rw_00892680(this: *mut u8, key: u32) -> u32 {
    unsafe {
        const RESOLVER: u32 = 1;
        const OPENER: u32 = 2;
        let h: u32 = callee_thiscall!(RESOLVER, u32, this as u32, key);
        if (h as i32) < 0 {
            return 0xffff_ffff;
        }
        callee_thiscall!(OPENER, u32, this as u32, h)
    }
});
