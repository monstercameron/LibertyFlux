// original: 0x00a8b0f0 pool_append_entry_new (proposed)

/// Append a pool entry built from two names, an addend and field values.
///
/// `this` is the pool (live count at +0x9C4, entries of 0x64 bytes from
/// +0). The new entry at index `count` takes its name from `name1` (+0x34,
/// NUL-terminated copy), its label from `name2` (+0x54), and sixteen field
/// words from the trailing arguments (+4, +0, +0x2C, +0x30, +0x10, +0x14,
/// +0x18, +0x1C, +0x20, +0x24, +0x28, +8, +0xC, bytes +0x5C/+0x5D, +0x60).
/// Then the count is incremented and the following entry's link word
/// (+0x58) takes the new entry's stale link plus `addend`. Returns the
/// previous count. The proof pins the count to 0..3 and NUL-pins both
/// strings.
///
/// Original: 0x00A8B0F0 (thiscall, nineteen stack words).
lf_checker_rt::export!(thiscall, rw_00a8b0f0(
    this: u32,
    name1: u32,
    name2: u32,
    addend: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
    a9: u32,
    a10: u32,
    a11: u32,
    a12: u32,
    a13: u32,
    a14: u32,
    a15: u32,
    a16: u32,
    a17: u32,
    a18: u32,
    a19: u32,
) -> u32 {
    unsafe {
        const COUNT: u32 = 0x9c4;
        const ENTRY_SIZE: u32 = 0x64;
        const NAME: u32 = 0x34;
        const LABEL: u32 = 0x54;
        const LINK: u32 = 0x58;
        let count = ((this + COUNT) as *const u32).read_unaligned();
        let dst = this.wrapping_add(count.wrapping_mul(ENTRY_SIZE));
        let mut i = 0u32;
        loop {
            let b = ((name1 + i) as *const u8).read();
            ((dst + NAME + i) as *mut u8).write(b);
            if b == 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        let mut j = 0u32;
        loop {
            let b = ((name2 + j) as *const u8).read();
            ((dst + LABEL + j) as *mut u8).write(b);
            if b == 0 {
                break;
            }
            j = j.wrapping_add(1);
        }
        ((dst + 4) as *mut u32).write_unaligned(a4);
        ((dst) as *mut u32).write_unaligned(a5);
        ((dst + 0x2c) as *mut u32).write_unaligned(a6);
        ((dst + 0x30) as *mut u32).write_unaligned(a7);
        ((dst + 0x10) as *mut u32).write_unaligned(a8);
        ((dst + 0x14) as *mut u32).write_unaligned(a9);
        ((dst + 0x18) as *mut u32).write_unaligned(a10);
        ((dst + 0x1c) as *mut u32).write_unaligned(a11);
        ((dst + 0x20) as *mut u32).write_unaligned(a12);
        ((dst + 0x24) as *mut u32).write_unaligned(a13);
        ((dst + 0x28) as *mut u32).write_unaligned(a14);
        ((dst + 8) as *mut u32).write_unaligned(a15);
        ((dst + 0xc) as *mut u32).write_unaligned(a16);
        ((dst + 0x5c) as *mut u8).write(a17 as u8);
        ((dst + 0x5d) as *mut u8).write(a18 as u8);
        ((dst + 0x60) as *mut u32).write_unaligned(a19);
        let new_count = count.wrapping_add(1);
        ((this + COUNT) as *mut u32).write_unaligned(new_count);
        let next = this.wrapping_add(new_count.wrapping_mul(ENTRY_SIZE));
        let stale = ((dst + LINK) as *const u32).read_unaligned();
        ((next + LINK) as *mut u32)
            .write_unaligned(stale.wrapping_add(addend));
        count
    }
});
