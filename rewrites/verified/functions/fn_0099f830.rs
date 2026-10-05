// original: 0x0099F830 audio_release_slots (proposed)

/// Release up to four audio slots held by an object and flag their table entries.
///
/// `this` points to an object holding four nullable slot pointers at `+0x94`,
/// `+0x98`, `+0xb0` and `+0xb4`. A slot entry carries a small index byte at
/// `+0x04`, a kind word at `+0x06` and a row byte at `+0x40`.
///
/// When the mode global holds 2, the first two slots whose kind word is not 2
/// are released through the release callee (thiscall, one zero stack word);
/// every other live slot is retired in place: its row selects one pointer
/// from a row table (stride `ROW_STRIDE`, entries at `ROW_TABLE_BIAS` past the
/// table base), the index scales a stride, and bit `FLAG_BIT` is set on the
/// byte at `ENTRY_BIAS` past the resulting address. A slot index of
/// `NO_ENTRY` addresses entry zero. Each live slot pointer is cleared.
///
/// Every live slot is cleared even when its flag write faults; a null slot is
/// left alone. The release callee's answer is ignored.
///
/// Original: thiscall, no stack arguments, no meaningful return value.
lf_checker_rt::export!(thiscall, rw_0099F830(this: u32) -> u32 {
    unsafe {
        const SLOT_A: u32 = 0x94;
        const SLOT_B: u32 = 0x98;
        const SLOT_C: u32 = 0xb0;
        const SLOT_D: u32 = 0xb4;
        const ENTRY_INDEX: u32 = 0x04;
        const ENTRY_KIND: u32 = 0x06;
        const ENTRY_ROW: u32 = 0x40;
        const NO_ENTRY: u8 = 0xff;
        const RELEASE_MODE: u32 = 2;
        const KEEP_KIND: u16 = 2;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_TABLE_BIAS: u32 = 0x6f14;
        const ENTRY_BIAS: u32 = 0xe8;
        const FLAG_BIT: u8 = 0x10;
        const RELEASE_CALLEE: u32 = 1;
        const MODE_GLOBAL: u32 = 0x11d6fd4;
        const STRIDE_GLOBAL: u32 = 0x115d968;
        const ROW_TABLE_GLOBAL: u32 = 0x115d988;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn flag_entry(entry: u32) {
            unsafe {
                let index = rd8(entry.wrapping_add(ENTRY_INDEX));
                let mut target = 0u32;
                if index != NO_ENTRY {
                    let row = rd8(entry.wrapping_add(ENTRY_ROW));
                    let stride = rd32(lf_checker_rt::relocated(STRIDE_GLOBAL));
                    let table = rd32(lf_checker_rt::relocated(ROW_TABLE_GLOBAL));
                    let row_ptr =
                        rd32(table.wrapping_add((row as u32).wrapping_mul(ROW_STRIDE)).wrapping_add(ROW_TABLE_BIAS));
                    target = stride.wrapping_mul(index as u32).wrapping_add(row_ptr);
                }
                let at = target.wrapping_add(ENTRY_BIAS);
                let old = (at as *const u8).read();
                (at as *mut u8).write(old | FLAG_BIT);
            }
        }

        let release_mode = rd32(lf_checker_rt::relocated(MODE_GLOBAL)) == RELEASE_MODE;
        let a = rd32(this.wrapping_add(SLOT_A));
        if a != 0 {
            if release_mode && rd16(a.wrapping_add(ENTRY_KIND)) != KEEP_KIND {
                lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, a, 0u32);
            } else {
                flag_entry(a);
            }
            wr32(this.wrapping_add(SLOT_A), 0);
        }
        let b = rd32(this.wrapping_add(SLOT_B));
        if b != 0 {
            if release_mode && rd16(b.wrapping_add(ENTRY_KIND)) != KEEP_KIND {
                lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, b, 0u32);
            } else {
                flag_entry(b);
            }
            wr32(this.wrapping_add(SLOT_B), 0);
        }
        let c = rd32(this.wrapping_add(SLOT_C));
        if c != 0 {
            flag_entry(c);
            wr32(this.wrapping_add(SLOT_C), 0);
        }
        let d = rd32(this.wrapping_add(SLOT_D));
        if d != 0 {
            flag_entry(d);
            wr32(this.wrapping_add(SLOT_D), 0);
        }
        0
    }
});
