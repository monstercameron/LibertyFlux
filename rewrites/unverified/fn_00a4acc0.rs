// original: 0x00A4ACC0 vehicle_kind_in_234 (proposed)

/// True when the word at `this + KIND` is 2, 3 or 4.
///
/// Three chained equality tests against small constants; anything else,
/// including 0, 1 and 5 and up, returns 0. Pure view, no writes.
///
/// Original: 0x00A4ACC0 (thiscall, no stack words), leaf, no globals.
lf_checker_rt::export!(thiscall, rw_00A4ACC0(this: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x0C;
        let kind = ((this + KIND) as *const u32).read_unaligned();
        u32::from(kind == 2 || kind == 3 || kind == 4)
    }
});
