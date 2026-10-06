// original: 0x006A0050 parse_table_from_file
/// Load a count and per-entry words through a file reader, flagging entries.
///
/// Calls one file-reader callee (thiscall, `read(file, ptr, 4)`, scripted)
/// at six sites: two header reads into scratch, a count read into `this+0`,
/// then per entry `i` while `i < count` (both the entry test `jle` and the
/// loop test `jl` are signed; the count is re-read from `this+0` every
/// iteration): a read of a scratch slot preset to the entry word at
/// `this+8+4*i`, a read into `this+0x298+4*i`, and a read of a scratch slot
/// preset to 1 when byte `+4` of the descriptor at `this+0x528+4*i` is
/// non-zero, else 0. The second stack argument is never read. Returns 1 in
/// `al` (a bool-style return: the upper bytes keep whatever the last load
/// left, so the contract compares `al` only).
///
/// Two quirks shape the contract. The original passes pointers to its own
/// incoming argument slots as two of the scratch buffers (the second header
/// read takes the address of the second argument slot, still holding `a1`;
/// the per-entry read takes the address of the first slot after copying the
/// entry word into it); a Rust rewrite cannot address its incoming slots,
/// so those pointer arguments are skipped with 1-word call-time snapshots
/// observing the preset contents instead, and the stack check is off (the
/// original clobbers its first argument slot with the scripted write). The count comes
/// from the scripted write and spans 0, positives and -1. True size is 167
/// bytes (the batch list says 161, mid-epilogue). Original: thiscall, two
/// stack words, callee cleanup 8.
lf_checker_rt::export!(thiscall, rw_006a0050(this: u32, file: u32, a1: u32) -> u32 {
    unsafe {
        const MAGIC: u32 = 0x5041_4D43;
        const COUNT: u32 = 0;
        const WORDS: u32 = 8;
        const SLOTS: u32 = 0x298;
        const DESCS: u32 = 0x528;
        const FLAG_OFF: u32 = 4;
        let mut head = MAGIC;
        let _: u32 = lf_checker_rt::callee_thiscall!(0, u32, file, &mut head as *mut u32 as u32, 4);
        let mut head2 = a1;
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, file, &mut head2 as *mut u32 as u32, 4);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, file, this, 4);
        if ((this.wrapping_add(COUNT) as *const u32).read_unaligned() as i32) <= 0 {
            return 1;
        }
        let mut i = 0i32;
        while i < ((this.wrapping_add(COUNT) as *const u32).read_unaligned() as i32) {
            let u = i as u32;
            let entry = (this.wrapping_add(WORDS).wrapping_add(u.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            let mut slot = entry;
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, file, &mut slot as *mut u32 as u32, 4);
            let tgt = this.wrapping_add(SLOTS).wrapping_add(u.wrapping_mul(4));
            let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, file, tgt, 4);
            let desc = (this.wrapping_add(DESCS).wrapping_add(u.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            let flag = if (desc.wrapping_add(FLAG_OFF) as *const u8).read() != 0 {
                1u32
            } else {
                0
            };
            let mut out = flag;
            let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, file, &mut out as *mut u32 as u32, 4);
            i += 1;
        }
        1
    }
});