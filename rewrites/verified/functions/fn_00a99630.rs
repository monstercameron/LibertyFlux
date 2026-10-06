// original: 0x00a99630 filemem_entry_is_special

/// Report whether an entry is special: resolved, indexed, typed and flagged.
///
/// `arg` points to the entry. The resolve callee (vtable slot `+0xa0`)
/// names its subject, defaulting to the word at `+0x38` when it answers
/// null. The entry's index word at `+0x2e` must then read non-negative as
/// a SIGNED 16-bit value (the original compares against -1 with a signed
/// jump: -1 and below fail), the subject must be non-null, and its kind
/// callee (vtable slot `+0x4`, asked twice) must answer neither 6 nor 5
/// (equality checks). Finally the pointer tabled at `0x01295cd8` under the
/// index must carry 0x02 in the top byte of its word at `+0x40`. Returns 1
/// when every gate passes, else 0.
///
/// Original: 0x00A99630 (stdcall, one stack word; two indirect callees).
lf_checker_rt::export!(stdcall, rw_00a99630(arg: u32) -> u32 {
    unsafe {
        /// Resolve slot in the entry vtable; fallback subject, from entry.
        const VT_RESOLVE: u32 = 0xa0;
        const SUB_OFF: u32 = 0x38;
        /// Index word (signed), from the entry.
        const INDEX_OFF: u32 = 0x2e;
        /// Kind slot in the subject vtable.
        const VT_KIND: u32 = 0x4;
        /// Excluded kinds, first and second ask.
        const KIND_EXCL1: u32 = 6;
        const KIND_EXCL2: u32 = 5;
        /// Subject pointer table (file VA); flag word and mask.
        const TABLE: u32 = 0x01295cd8;
        const FLAG_OFF: u32 = 0x40;
        const FLAG_MASK: u32 = 0xff000000;
        const FLAG_WANT: u32 = 0x02000000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let slot = rd32(rd32(arg).wrapping_add(VT_RESOLVE));
        let resolve: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let mut sub = resolve(arg);
        if sub == 0 {
            sub = rd32(arg.wrapping_add(SUB_OFF));
        }
        let idx = ((arg.wrapping_add(INDEX_OFF)) as *const i16).read_unaligned();
        if idx < 0 {
            return 0;
        }
        if sub == 0 {
            return 0;
        }
        let kslot = rd32(rd32(sub).wrapping_add(VT_KIND));
        let kind: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(kslot as usize);
        if kind(sub) == KIND_EXCL1 {
            return 0;
        }
        if kind(sub) == KIND_EXCL2 {
            return 0;
        }
        let ent = rd32(lf_checker_rt::relocated(TABLE).wrapping_add((idx as u32).wrapping_mul(4)));
        if rd32(ent.wrapping_add(FLAG_OFF)) & FLAG_MASK != FLAG_WANT {
            return 0;
        }
        1
    }
});
