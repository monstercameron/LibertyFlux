// original: 0x0089C3E0 aud_slot_action_dispatch (proposed)

/// Resolve the slot entity for a sound and dispatch an action on it.
///
/// `this` is an audio sound object, `arg1` an opaque handle passed through
/// to three helpers, `arg2` a flag word whose low byte selects the dispatch
/// shape. The start helper (callee 0, cdecl) runs first; its answer is kept.
/// Two table rows are resolved like the sibling updater: `edi_row` from the
/// byte at `+0xB4` (stride `G_STRIDE_B`, row table at `+TABLE_EDI`) must be
/// non-null, and `ebx_row` from the byte at `+0x48` (stride `G_STRIDE_A`,
/// row table at `+TABLE_EBX`, null when the byte is `0xFF`) must be
/// non-null; a `-1` start answer, a `0xFFFF` marker word at `edi_row+4`,
/// or a null row returns 2 (the marker test reads a full dword: only 0x0000FFFF misses). Otherwise the classify helper (callee 1,
/// `thiscall` on `arg1` with `edi_row[0]` and the marker word) answers
/// `code`. When the low byte of `arg2` is non-zero the code maps directly
/// (0 to 1, 2 to 2, anything else to 0). When it is zero, the start answer
/// is stored at `+0xB0` and handed with `edi_row[0]` to the apply helper
/// (callee 2, `thiscall` on `ebx_row`), and the code dispatches UNSIGNED
/// (`ja` past 3 returns 2): 0 resolves a target through the lookup helper
/// (callee 3, cdecl; a null target returns 2), builds a record through the
/// record helper (callee 4, `thiscall` on `this` with 0), fills its words
/// at `+0xE4`/`+0xE0` (the latter from the combine helper, callee 5,
/// cdecl) and sets or clears bit 2 of its byte at `+0xEE` by the SIGNED
/// sign of the dword at `target+0x14` (`jl`: negative clears), returning 1
/// either way; 1 returns 0; 2 returns 2; 3 returns 2 unless bit 5 of the
/// byte at `+0x39` is set, in which case the notify helper (callee 6,
/// `thiscall` on `arg1`) runs and 0 returns.
///
/// Original: 0x0089C3E0 (thiscall, two stack words, returns full `eax`).
lf_checker_rt::export!(thiscall, rw_0089C3E0(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const CAT_INDEX: u32 = 0x40;
        const FLAG_BYTE: u32 = 0x39;
        const SLOT_B: u32 = 0xB4;
        const SLOT_A: u32 = 0x48;
        const SAVED_OUT: u32 = 0xB0;
        const CAT_STRIDE: u32 = 0x6F40;
        const TABLE_EBX: u32 = 0x6F10;
        const TABLE_EDI: u32 = 0x6F14;
        const NO_SLOT: u8 = 0xFF;
        const NO_ENTITY: u16 = 0xFFFF;
        const G_STRIDE_A: u32 = 0x115D964;
        const G_STRIDE_B: u32 = 0x115D968;
        const G_TABLE_BASE: u32 = 0x115D988;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let start: u32 = lf_checker_rt::callee_cdecl!(0, u32, arg1);
        let base = (lf_checker_rt::global::<u32>(G_TABLE_BASE)).read_unaligned();
        let cat = rd8(this + CAT_INDEX) as u32;
        let cat_base = base.wrapping_add(cat.wrapping_mul(CAT_STRIDE));
        let stride_b = (lf_checker_rt::global::<u32>(G_STRIDE_B)).read_unaligned();
        let edi_row = stride_b
            .wrapping_mul(rd8(this + SLOT_B) as u32)
            .wrapping_add(rd32(cat_base.wrapping_add(TABLE_EDI)));
        if edi_row == 0 || start == 0xFFFF_FFFF {
            return 2;
        }
        let ebx_row = {
            let slot = rd8(this + SLOT_A);
            if slot == NO_SLOT {
                0
            } else {
                let stride_a = (lf_checker_rt::global::<u32>(G_STRIDE_A)).read_unaligned();
                stride_a
                    .wrapping_mul(slot as u32)
                    .wrapping_add(rd32(cat_base.wrapping_add(TABLE_EBX)))
            }
        };
        // Volatile: the original reads this dword before testing the row for
        // null, so the read must happen (and fault) on the same trials; a
        // plain read lets the compiler sink the load below the early exit.
        let marker_dword =
            unsafe { core::ptr::read_volatile(edi_row.wrapping_add(4) as *const u32) };
        if marker_dword == NO_ENTITY as u32 || ebx_row == 0 {
            return 2;
        }
        let marker = marker_dword as u16;
        let code: u32 =
            lf_checker_rt::callee_thiscall!(1, u32, arg1, marker as u32, rd32(edi_row));
        if (arg2 & 0xFF) != 0 {
            if code == 0 {
                return 1;
            }
            if code == 2 {
                return 2;
            }
            return 0;
        }
        wr32(this + SAVED_OUT, start);
        lf_checker_rt::callee_thiscall!(2, u32, ebx_row, rd32(edi_row), start);
        // Unsigned dispatch: values above 3 (including all negatives) miss.
        if code > 3 {
            return 2;
        }
        match code {
            0 => {
                let target: u32 = lf_checker_rt::callee_cdecl!(3, u32, arg1, rd32(edi_row));
                if target == 0 {
                    return 2;
                }
                let rec: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, 0);
                wr16(rec.wrapping_add(0xE4), rd16(target.wrapping_add(0x1A)));
                let combined: u32 = lf_checker_rt::callee_cdecl!(
                    5,
                    u32,
                    rd32(target.wrapping_add(0x10)),
                    rd16(target.wrapping_add(0x18)) as u32
                );
                wr32(rec.wrapping_add(0xE0), combined);
                let flag_byte = rec.wrapping_add(0xEE) as *mut u8;
                if (rd32(target.wrapping_add(0x14)) as i32) >= 0 {
                    flag_byte.write(flag_byte.read() | 4);
                } else {
                    flag_byte.write(flag_byte.read() & 0xFB);
                }
                1
            }
            1 => 0,
            2 => 2,
            _ => {
                if rd8(this + FLAG_BYTE) & 0x20 == 0 {
                    return 2;
                }
                lf_checker_rt::callee_thiscall!(6, u32, arg1, marker as u32, rd32(edi_row));
                0
            }
        }
    }
});
