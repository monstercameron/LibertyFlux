// original: 0x00A8B010 pool_record_clone (proposed)

/// Clone a pool record into the next free slot, overriding its fields.
///
/// The pool holds 100-byte records inline from `this`; slot `count`
/// (`this+0x9C4`) is the destination and stack argument `src_idx` the source.
/// The two name strings (record `+0x34` from the source row, `+0x54` from the
/// `name` argument) are copied byte by byte with the terminator; five words
/// (`+0`, `+0x1C`, `+0x20`, `+0x24`, `+0x28`) are copied from the source row
/// and the remaining fields come from the stack arguments. The count is then
/// incremented: the link word at `+0x58` of the new slot is carried to the
/// following slot and the source row's link takes its place. Returns the old
/// count. No calls.
///
/// Original: thiscall, thirteen stack words, returns u32 in EAX.
lf_checker_rt::export!(thiscall, rw_00A8B010(
    this: u32,
    src_idx: u32,
    name: u32,
    f04: u32,
    f2c: u32,
    f30: u32,
    f10: u32,
    f14: u32,
    f18: u32,
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
        let src = this.wrapping_add(src_idx.wrapping_mul(REC));
        let mut p = src.wrapping_add(NAME_A);
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
        let mut p = name;
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
        for off in [0u32, 0x1c, 0x20, 0x24, 0x28] {
            let v = ((src + off) as *const u32).read_unaligned();
            ((dst + off) as *mut u32).write_unaligned(v);
        }
        ((dst + 4) as *mut u32).write_unaligned(f04);
        ((dst + 0x2c) as *mut u32).write_unaligned(f2c);
        ((dst + 0x30) as *mut u32).write_unaligned(f30);
        ((dst + 0x10) as *mut u32).write_unaligned(f10);
        ((dst + 0x14) as *mut u32).write_unaligned(f14);
        ((dst + 0x18) as *mut u32).write_unaligned(f18);
        ((dst + 8) as *mut u32).write_unaligned(f08);
        ((dst + 0xc) as *mut u32).write_unaligned(f0c);
        ((dst + 0x5c) as *mut u8).write(b5c as u8);
        ((dst + 0x5d) as *mut u8).write(b5d as u8);
        ((dst + 0x60) as *mut u32).write_unaligned(f60);
        let new_count = count.wrapping_add(1);
        ((this + COUNT_OFF) as *mut u32).write_unaligned(new_count);
        let next = this.wrapping_add(new_count.wrapping_mul(REC));
        let carried = ((dst + LINK) as *const u32).read_unaligned();
        ((next + LINK) as *mut u32).write_unaligned(carried);
        let from_src = ((src + LINK) as *const u32).read_unaligned();
        ((dst + LINK) as *mut u32).write_unaligned(from_src);
        count
    }
});
