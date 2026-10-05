// original: 0x00a8b010 pool_append_entry_copy (proposed)

/// Append a pool entry copied from another entry plus new field values.
///
/// `this` is the pool (live count at +0x9C4, entries of 0x64 bytes from
/// +0). The new entry at index `count` takes its name from the entry at
/// `src_idx` (+0x34, NUL-terminated copy), its label from `label` (+0x54),
/// five header words from the source entry (+0, +0x1C, +0x20, +0x24,
/// +0x28), and the remaining words from the trailing arguments (+4, +0x2C,
/// +0x30, +0x10, +0x14, +0x18, +8, +0xC, bytes +0x5C/+0x5D, +0x60). Then
/// the count is incremented, the following entry's link word (+0x58) takes
/// the new entry's stale link, and the new entry's link takes the source
/// entry's. Returns the previous count. The proof pins the count to 0..3
/// and NUL-pins both strings.
///
/// Original: 0x00A8B010 (thiscall, thirteen stack words).
lf_checker_rt::export!(thiscall, rw_00a8b010(
    this: u32,
    src_idx: u32,
    label: u32,
    a3: u32,
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
) -> u32 {
    unsafe {
        const COUNT: u32 = 0x9c4;
        const ENTRY_SIZE: u32 = 0x64;
        const NAME: u32 = 0x34;
        const LABEL: u32 = 0x54;
        const LINK: u32 = 0x58;
        const HDR: [u32; 5] = [0, 0x1c, 0x20, 0x24, 0x28];
        let count = ((this + COUNT) as *const u32).read_unaligned();
        let dst = this.wrapping_add(count.wrapping_mul(ENTRY_SIZE));
        let src = this.wrapping_add(src_idx.wrapping_mul(ENTRY_SIZE));
        let mut i = 0u32;
        loop {
            let b = ((src + NAME + i) as *const u8).read();
            ((dst + NAME + i) as *mut u8).write(b);
            if b == 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
        let mut j = 0u32;
        loop {
            let b = ((label + j) as *const u8).read();
            ((dst + LABEL + j) as *mut u8).write(b);
            if b == 0 {
                break;
            }
            j = j.wrapping_add(1);
        }
        for k in 0..5 {
            let w = ((src + HDR[k]) as *const u32).read_unaligned();
            ((dst + HDR[k]) as *mut u32).write_unaligned(w);
        }
        ((dst + 4) as *mut u32).write_unaligned(a3);
        ((dst + 0x2c) as *mut u32).write_unaligned(a4);
        ((dst + 0x30) as *mut u32).write_unaligned(a5);
        ((dst + 0x10) as *mut u32).write_unaligned(a6);
        ((dst + 0x14) as *mut u32).write_unaligned(a7);
        ((dst + 0x18) as *mut u32).write_unaligned(a8);
        ((dst + 8) as *mut u32).write_unaligned(a9);
        ((dst + 0xc) as *mut u32).write_unaligned(a10);
        ((dst + 0x5c) as *mut u8).write(a11 as u8);
        ((dst + 0x5d) as *mut u8).write(a12 as u8);
        ((dst + 0x60) as *mut u32).write_unaligned(a13);
        let new_count = count.wrapping_add(1);
        ((this + COUNT) as *mut u32).write_unaligned(new_count);
        let next = this.wrapping_add(new_count.wrapping_mul(ENTRY_SIZE));
        let stale = ((dst + LINK) as *const u32).read_unaligned();
        ((next + LINK) as *mut u32).write_unaligned(stale);
        let src_link = ((src + LINK) as *const u32).read_unaligned();
        ((dst + LINK) as *mut u32).write_unaligned(src_link);
        count
    }
});
