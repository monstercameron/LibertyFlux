// original: 0x008A4FB0 aud_switch_slot_update (proposed)
/// Update one audio switch slot: prepare and commit the voice selected by the
/// slot's selector bytes, then finalize the slot object.
///
/// `this` points to the slot object. Byte `+0x40` is the table row, bytes
/// `+0x48`/`+0x49` are the primary/secondary voice selectors (`0xFF` means
/// absent), dword `+0x54` is an extra prepare argument, dword `+0xC0` is the
/// mode (0 runs the first block, 1 the second, anything else returns at once)
/// and byte `+0xC4` selects whether the refresh callee runs first. `arg0` is
/// forwarded to the commit callee. All comparisons are equality checks
/// (bytes against `0xFF`/0, the mode against 0/1, resolved addresses against
/// 0); there are no ordered or signed comparisons.
///
/// A selector resolves through the audio table: the row cell read at
/// `table + row*0x6F40 + 0x6F10` plus `stride * selector`, where `stride` is
/// the dword global and `table` the table global. A zero resolution aborts
/// the block. Each block prepares the resolved voice (callee 2), commits it
/// with `arg0` (callee 3), pokes the slot through callee 4 with flag 1 in the
/// first block and 0 in the second when the other selector is present and
/// resolves, and the first block finalizes through callee 5.
///
/// The return value is whatever sits in EAX at the exit: the finalizer's
/// answer on the first block's full path, the poke's answer on the second
/// block's full path, `0xFF` (the re-read absent selector, zero-extended)
/// when the second block's secondary selector is absent, 0 when a resolution
/// is zero, and otherwise the refresh callee's answer, or entry EAX (defined
/// by the contract as `this`) when the refresh is skipped.
///
/// Original: 0x008A4FB0 (thiscall, one stack word).
export!(thiscall, rw_008a4fb0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f10;
        const ABSENT: u8 = 0xFF;

        let t = this as *const u8;
        let rd8 = |off: u32| t.add(off as usize).read();
        let rd32 = |off: u32| (t.add(off as usize) as *const u32).read_unaligned();
        let resolve = |row: u8, stride: u32, table: u32, idx: u8| {
            let cell = ((table
                .wrapping_add((row as u32).wrapping_mul(ROW_STRIDE))
                .wrapping_add(TABLE_BIAS)) as *const u32)
                .read_unaligned();
            stride.wrapping_mul(idx as u32).wrapping_add(cell)
        };

        // Entry EAX: the contract sets it to `this`, so paths that never set
        // EAX return `this`, exactly like the original returns entry EAX.
        let mut eaxv = this;
        if rd8(0xC4) == 0 {
            eaxv = callee_thiscall!(1, u32, this);
        }
        let mode = rd32(0xC0);
        let table = global::<u32>(0x115d988).read();
        let stride = global::<u32>(0x115d964).read();
        let row = rd8(0x40);

        // First block (mode 0): primary selector +0x48, secondary +0x49.
        if mode == 0 {
            let sel = rd8(0x48);
            if sel != ABSENT {
                let obj = resolve(row, stride, table, sel);
                if obj == 0 {
                    return 0;
                }
                eaxv = callee_thiscall!(2, u32, obj, rd32(0x54), 0);
                let sel_again = rd8(0x48);
                debug_assert_eq!(sel_again, sel);
                let obj2 = resolve(row, stride, table, sel_again);
                eaxv = callee_thiscall!(3, u32, obj2, arg0);
                let other = rd8(0x49);
                if other != ABSENT {
                    let obj3 = resolve(row, stride, table, other);
                    if obj3 != 0 {
                        eaxv = callee_thiscall!(4, u32, this, 1);
                    }
                }
                eaxv = callee_thiscall!(5, u32, this);
                return eaxv;
            }
            // Absent primary, or zero resolution: the mode is 0, never 1.
            return eaxv;
        }

        // Second block (mode 1): primary selector +0x49, secondary +0x48.
        if mode == 1 {
            let sel = rd8(0x49);
            if sel == ABSENT {
                return eaxv;
            }
            let obj = resolve(row, stride, table, sel);
            if obj == 0 {
                return 0;
            }
            eaxv = callee_thiscall!(2, u32, obj, rd32(0x54), 0);
            let sel_again = rd8(0x49);
            debug_assert_eq!(sel_again, sel);
            let obj2 = resolve(row, stride, table, sel_again);
            eaxv = callee_thiscall!(3, u32, obj2, arg0);
            let other = rd8(0x48);
            if other == ABSENT {
                // The original re-reads the selector with movzx, so EAX is
                // exactly 0xFF here, not the commit answer.
                return ABSENT as u32;
            }
            let obj3 = resolve(row, stride, table, other);
            if obj3 == 0 {
                return 0;
            }
            eaxv = callee_thiscall!(4, u32, this, 0);
            return eaxv;
        }
        eaxv
    }
});
