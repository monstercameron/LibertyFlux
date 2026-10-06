// original: 0x00D7C200 select_active_link_or_self (proposed)

/// Return the linked object when this one is active, else the object itself.
///
/// `obj` points to an object with a kind word at `+0x28`, a flag byte at
/// `+0x26c` and a link word at `+0xb30`. Returns the link when
/// `(kind & 0x3c0) == 0xc0`, the flag byte has bit 2 set, and the link is
/// non-null; otherwise returns `obj`. Cdecl, one stack word.
use lf_checker_rt::export;

export!(cdecl, rw_00d7c200(obj: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_WANT: u32 = 0xc0;
        const FLAG_OFF: u32 = 0x26c;
        const FLAG_BIT: u8 = 4;
        const LINK_OFF: u32 = 0xb30;
        if ((obj + KIND_OFF) as *const u32).read_unaligned() & KIND_MASK != KIND_WANT {
            return obj;
        }
        if ((obj + FLAG_OFF) as *const u8).read() & FLAG_BIT == 0 {
            return obj;
        }
        let link = ((obj + LINK_OFF) as *const u32).read_unaligned();
        if link == 0 { obj } else { link }
    }
});
