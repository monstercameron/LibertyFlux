// original: 0x00974fa0 audio_row_select_and_publish (proposed)

/// Publish one table row into the voice object.
///
/// Row `index` of the table at `table` (rows of 0x33 bytes) supplies two
/// floats (at +0x10 and +0x14) and a tag byte (+0x18). The object records
/// the index (+0x118), the tag (+0x18), the floats (+0x10, +0x14), the table
/// pointer (+0x114) and a ready flag (+0x11C set to 1). Returns the tag byte,
/// zero-extended, in EAX.
/// Original: 0x00974FA0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00974fa0(this: u32, table: u32, index: u32) -> u32 {
    unsafe {
        const ROW: u32 = 0x33;
        const TAG_OFF: u32 = 0x18;
        const F0_OFF: u32 = 0x14;
        const F1_OFF: u32 = 0x10;
        const O_INDEX: u32 = 0x118;
        const O_TAG: u32 = 0x18;
        const O_F0: u32 = 0x10;
        const O_F1: u32 = 0x14;
        const O_READY: u32 = 0x11C;
        const O_TABLE: u32 = 0x114;
        let row = index.wrapping_mul(ROW);
        let f0 = ((table.wrapping_add(row).wrapping_add(F0_OFF)) as *const u32).read_unaligned();
        let f1 = ((table.wrapping_add(row).wrapping_add(F1_OFF)) as *const u32).read_unaligned();
        let tag = ((table.wrapping_add(row).wrapping_add(TAG_OFF)) as *const u8).read() as u32;
        ((this.wrapping_add(O_INDEX)) as *mut u32).write_unaligned(index);
        ((this.wrapping_add(O_TAG)) as *mut u32).write_unaligned(tag);
        ((this.wrapping_add(O_F0)) as *mut u32).write_unaligned(f0);
        ((this.wrapping_add(O_F1)) as *mut u32).write_unaligned(f1);
        ((this.wrapping_add(O_READY)) as *mut u8).write(1);
        ((this.wrapping_add(O_TABLE)) as *mut u32).write_unaligned(table);
        tag
    }
});
