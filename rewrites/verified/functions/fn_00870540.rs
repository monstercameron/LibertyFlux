// original: 0x00870540 crmt_keyed_entry_add (proposed)

/// Build the entry key from `item` and forward to the node attacher.
///
/// Reads the descriptor at `item` + 0xC; when non-null its word at +0x72
/// forms the low half of the key, otherwise the low half is zero. The high
/// half is the low word of arg2. The float in arg1 is moved to vector
/// register 2 for the callee (a Rust rewrite cannot set a vector register
/// for a call except through the checker's transport, which would drop the
/// comparison of every stack argument including the key, so that move is
/// documented unobserved instead). The attacher (callee 1) receives the
/// item, the key and two frame slots carrying arg3..arg6. Returns the
/// attacher's result. The descriptor test is an exact null check.
///
/// Original: 0x00870540 (thiscall, seven stack words; the callee pops 28 bytes).
lf_checker_rt::export!(thiscall, rw_00870540(
    this: u32,
    item: u32,
    _weight_bits: u32,
    w0: u32,
    w1: u32,
    w2: u32,
    w3: u32,
    _w4: u32,
) -> u32 {
    const ATTACH: u32 = 1;
    unsafe {
        let desc = ((item + 0x0c) as *const u32).read_unaligned();
        let low: u32 = if desc != 0 {
            ((desc + 0x72) as *const u16).read_unaligned() as u32
        } else {
            0
        };
        let key = ((w0 & 0xffff) << 16) | (low & 0xffff);
        let mut frame = [w1, w2, w3, _w4];
        let first = frame.as_ptr() as u32;
        let second = frame.as_ptr().add(2) as u32;
        lf_checker_rt::callee_thiscall!(ATTACH, u32, this, item, key, first, second)
    }
});
