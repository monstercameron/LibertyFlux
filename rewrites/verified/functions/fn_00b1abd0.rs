// original: 0x00b1abd0 init_table_owner (proposed)

/// Resets the owner's fields and reinitialises the row tables.
///
/// Thiscall with no stack arguments. Keeps flag bit 1 at +0x41, sets bit
/// 0 there, zeroes +0x40, writes the default field block (+0x04 zeroed,
/// +0x08 one, +0x34/+0x38 zeroed, +0x3C zeroed as a halfword, +0x14/-1,
/// +0x10 zeroed, +0x18/+0x1C/+0x20 -1, +0x24 two, +0x28 -1, +0x2C 0x5E),
/// then asks the counter (thiscall, no stack arguments) how many rows
/// are live. For each live row it writes 0x5FF and -1 to the state word
/// pair, 0x500 to the companion slot, 1 to the row head and 0 to the
/// row's word at +0x6E. Writes 0 to +0 in both cases and returns the
/// live count.
lf_checker_rt::export!(thiscall, rw_00b1abd0(this: u32) -> u32 {
    unsafe {
        const STATE_TABLE: u32 = 0x016334c8;
        const COMPANION_TABLE: u32 = 0x01633640;
        const ROW_TABLE: u32 = 0x010401fa;
        const ROW_STRIDE: u32 = 0xe2;
        const STATE_FLAG: u32 = 0x5ff;
        const ROW_HEAD: u32 = 1;
        const COMPANION: u32 = 0x500;
        let flag = (this + 0x41) as *mut u8;
        flag.write(flag.read() & 2 | 1);
        for (off, val) in [
            (0x04u32, 0u32),
            (0x08, 0x3f800000),
            (0x34, 0),
            (0x38, 0),
            (0x14, 0xffff_ffff),
            (0x10, 0),
            (0x18, 0xffff_ffff),
            (0x1c, 0xffff_ffff),
            (0x20, 0xffff_ffff),
            (0x24, 2),
            (0x28, 0xffff_ffff),
            (0x2c, 0x5e),
        ] {
            ((this + off) as *mut u32).write_unaligned(val);
        }
        ((this + 0x3c) as *mut u16).write_unaligned(0);
        ((this + 0x40) as *mut u8).write(0);
        let count: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        if (count as i32) > 0 {
            let mut i = 0u32;
            while i < count {
                let state = STATE_TABLE.wrapping_add(i.wrapping_mul(4));
                (lf_checker_rt::relocated(state) as *mut u16).write_unaligned(STATE_FLAG as u16);
                (lf_checker_rt::relocated(state + 2) as *mut u16)
                    .write_unaligned(0xffffu16);
                (lf_checker_rt::relocated(COMPANION_TABLE.wrapping_add(i.wrapping_mul(4)))
                    as *mut u32)
                    .write_unaligned(COMPANION);
                let row = ROW_TABLE.wrapping_add(i.wrapping_mul(ROW_STRIDE));
                (lf_checker_rt::relocated(row - 2) as *mut u32).write_unaligned(ROW_HEAD);
                (lf_checker_rt::relocated(row + 0x6c) as *mut u16).write_unaligned(0);
                i += 1;
            }
        }
        (this as *mut u32).write_unaligned(0);
        count
    }
});
