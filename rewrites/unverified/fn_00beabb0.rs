// original: 0x00beabb0 mode_byte_from_lookup
/// Map an index through a lookup call to a small mode byte, or leave 0.
///
/// Writes 0 to `[out]`. When `idx` is negative, that is all. Otherwise calls
/// the resolver (cdecl, two stack args: `idx` and 1; intercepted) and, when
/// it returns non-null, decodes `([answer+0x28] >> 6) & 0xf`: values 2, 3
/// and 4 are stored to `[out]`, anything else leaves the 0. Returns `idx`
/// when negative, the decoded value (or 0 for a null answer) otherwise.
/// Cdecl, two stack arguments.
export!(cdecl, rw_00beabb0(idx: u32, out: u32) -> u32 {
    unsafe {
        const RESOLVER: u32 = 1;
        const MODE_OFF: u32 = 0x28;
        ((out) as *mut u8).write(0);
        if (idx as i32) < 0 {
            return idx;
        }
        let answer: u32 = callee_cdecl!(RESOLVER, u32, idx, 1);
        if answer == 0 {
            return 0;
        }
        let v = (((answer + MODE_OFF) as *const u32).read_unaligned() >> 6) & 0xf;
        if v == 2 || v == 3 || v == 4 {
            (out as *mut u8).write(v as u8);
        }
        v
    }
});
