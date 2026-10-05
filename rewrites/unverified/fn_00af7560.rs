// original: 0x00AF7560 veh_handle_copy (proposed)

/// Copy a handle's payload onto another handle, merging the flag nibble.
///
/// When source and destination are the same object nothing happens. Otherwise
/// callee 1 is told about the source's head word, then the payload moves:
/// dwords at `+0x04`, `+0x08`, `+0x0C`, `+0x10`, `+0x18`, words at `+0x20`,
/// `+0x22`, `+0x24`, bytes at `+0x26` through `+0x2A`, and the low five bits
/// of the flag byte at `+0x2B` (the original sets them one bit at a time;
/// the top three bits keep the destination's value). Returns the destination.
///
/// Original: 0x00AF7560 (thiscall, one stack argument, destination in EAX).
lf_checker_rt::export!(thiscall, rw_00AF7560(this: u32, src: u32) -> u32 {
    unsafe {
        const CLAIM: u32 = 1;
        const FLAG: u32 = 0x2B;
        const COPIED_BITS: u8 = 0x1F;
        if this == src {
            return this;
        }
        let head = (src as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(CLAIM, u32, this, head);
        for off in [0x04u32, 0x08, 0x0C, 0x10, 0x18] {
            let v = ((src + off) as *const u32).read_unaligned();
            ((this + off) as *mut u32).write_unaligned(v);
        }
        for off in [0x20u32, 0x22, 0x24] {
            let v = ((src + off) as *const u16).read_unaligned();
            ((this + off) as *mut u16).write_unaligned(v);
        }
        for off in [0x26u32, 0x27, 0x28, 0x29, 0x2A] {
            let v = ((src + off) as *const u8).read();
            ((this + off) as *mut u8).write(v);
        }
        let dst_flag = ((this + FLAG) as *const u8).read();
        let src_flag = ((src + FLAG) as *const u8).read();
        ((this + FLAG) as *mut u8).write((dst_flag & !COPIED_BITS) | (src_flag & COPIED_BITS));
        this
    }
});
