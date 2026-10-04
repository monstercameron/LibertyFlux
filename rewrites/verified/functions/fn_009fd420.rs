// original: 0x009FD420 frag_member_or_null (proposed)

/// Return the member pointer of a frag object when its flag bit is set.
///
/// `obj` points to an object whose byte at `+0x0a` carries status flags and
/// whose dword at `+0x0c` is a member pointer. Returns null when `obj` is
/// null or when bit `0x40` of the flag byte is clear, otherwise the dword
/// at `+0x0c`.
///
/// Original: 0x009FD420 (cdecl, one stack word; reads no registers).
lf_checker_rt::export!(cdecl, rw_009FD420(obj: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x0A;
        const MEMBER_OFF: u32 = 0x0C;
        const PRESENT_BIT: u8 = 0x40;
        if obj == 0 {
            return 0;
        }
        let flags = ((obj + FLAG_OFF) as *const u8).read();
        if flags & PRESENT_BIT == 0 {
            return 0;
        }
        ((obj + MEMBER_OFF) as *const u32).read_unaligned()
    }
});
