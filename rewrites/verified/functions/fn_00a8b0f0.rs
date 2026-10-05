// original: 0x00A8B0F0 pool_record_append (proposed)

/// Append a fully-specified record to the pool's inline record array.
///
/// Like the clone sibling, but every field comes from the stack: the two
/// name strings (record `+0x34` from `name_a`, `+0x54` from `name_b`) are
/// copied byte by byte with the terminator, fourteen words and two bytes
/// are stored, and the count at `this+0x9C4` is incremented. The link word
/// of the following slot becomes the new slot's `+0x58` plus argument
/// `addend` (which is stored nowhere else). Returns the old count. No calls.
///
/// Original: thiscall, nineteen stack words, returns u32 in EAX.
lf_checker_rt::export!(thiscall, rw_00A8B0F0(
    this: u32,
    name_a: u32,
    name_b: u32,
    addend: u32,
    f04: u32,
    f00: u32,
    f2c: u32,
    f30: u32,
    f10: u32,
    f14: u32,
    f18: u32,
    f1c: u32,
    f20: u32,
    f24: u32,
    f28: u32,
    f08: u32,
    f0c: u32,
    b5c: u32,
    b5d: u32,
    f60: u32,
) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x9c4;
        const REC: u32 = 100;
        const NAME_A: u32 = 0x34;
        const NAME_B: u32 = 0x54;
        const LINK: u32 = 0x58;
        let count = ((this + COUNT_OFF) as *const u32).read_unaligned();
        let dst = this.wrapping_add(count.wrapping_mul(REC));
        let mut p = name_a;
        let mut q = dst.wrapping_add(NAME_A);
        loop {
            let b = (p as *const u8).read();
            (q as *mut u8).write(b);
            p = p.wrapping_add(1);
            q = q.wrapping_add(1);
            if b == 0 {
                break;
            }
        }
        let mut p = name_b;
        let mut q = dst.wrapping_add(NAME_B);
        loop {
            let b = (p as *const u8).read();
            (q as *mut u8).write(b);
            p = p.wrapping_add(1);
            q = q.wrapping_add(1);
            if b == 0 {
                break;
            }
        }
        ((dst + 4) as *mut u32).write_unaligned(f04);
        ((dst + 0) as *mut u32).write_unaligned(f00);
        ((dst + 0x2c) as *mut u32).write_unaligned(f2c);
        ((dst + 0x30) as *mut u32).write_unaligned(f30);
        ((dst + 0x10) as *mut u32).write_unaligned(f10);
        ((dst + 0x14) as *mut u32).write_unaligned(f14);
        ((dst + 0x18) as *mut u32).write_unaligned(f18);
        ((dst + 0x1c) as *mut u32).write_unaligned(f1c);
        ((dst + 0x20) as *mut u32).write_unaligned(f20);
        ((dst + 0x24) as *mut u32).write_unaligned(f24);
        ((dst + 0x28) as *mut u32).write_unaligned(f28);
        ((dst + 8) as *mut u32).write_unaligned(f08);
        ((dst + 0xc) as *mut u32).write_unaligned(f0c);
        ((dst + 0x5c) as *mut u8).write(b5c as u8);
        ((dst + 0x5d) as *mut u8).write(b5d as u8);
        ((dst + 0x60) as *mut u32).write_unaligned(f60);
        let new_count = count.wrapping_add(1);
        ((this + COUNT_OFF) as *mut u32).write_unaligned(new_count);
        let next = this.wrapping_add(new_count.wrapping_mul(REC));
        let chained = ((dst + LINK) as *const u32)
            .read_unaligned()
            .wrapping_add(addend);
        ((next + LINK) as *mut u32).write_unaligned(chained);
        count
    }
});
