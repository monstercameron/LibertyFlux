// original: 0x0091F0F0 btn_prompts_init (proposed)

/// Initialise the button-prompt UI tables and register every prompt slot.
///
/// Takes no arguments (cdecl, plain frame) and never returns normally: it
/// ends in a tail jump to the UI refresher. Under the checker the tail
/// target is a scripted stub whose answer becomes the return value.
///
/// What it does, in order:
/// * Flags: writes -1 to the state word and 0 to the ready byte, then calls
///   the reset helper (callee 1, no arguments).
/// * Mode dispatch: resolves the handle table through callee 2, then reads
///   the mode byte and the layout flag. Byte 0x72 selects the third table
///   scan, byte 0x6a the second, anything else the first unless the layout
///   flag is set (then the second). Each scan carries a tag (0x26, 0x33,
///   0x30) and a fallback string.
/// * Table scan: asks callee 3 for the current entry (passing a frame
///   buffer), then walks the shared table at the table pointer: the row
///   base at +0x0, the row count as a 16-bit word at +0x4, rows of 0x88
///   bytes whose key slot sits at +0x80. The entry's key at +0x80 is
///   matched against the rows from the top down starting from the entry's
///   index at +0x84. Comparisons on the index are all SIGNED: -1 skips the
///   scan, a negative index is clamped to 0 (`test/jns`), the clamp
///   against the count is signed (`cmp/cmovg`), and both loop ends test
///   the sign (`dec/js`, `dec/jns`). A found row becomes the entry and is
///   re-checked; otherwise the entry falls back to table+0x690. Callee 4
///   runs once per pass; its low byte decides whether the fallback string
///   is resolved through callee 4 again.
/// * Asset triples: resolves three string handles through callee 7 and
///   registers each through callee 8, storing the result plus five
///   constant words per triple. The third triple runs only when the mode
///   byte is 0x6a or the layout flag is set.
/// * Slot registration: resolves a second handle, resolves one string
///   through callee 4, releases both handles through callees 5 and 6, then
///   registers 46 prompt slots through callee 9 (slot address in ECX, name
///   string pushed), in a fixed order that is not sorted.
/// * Finish: calls callee 10 with -1, pulses the state word 0/1/-1 around
///   two calls of callee 11, resolves one more handle into its global,
///   zeroes the prompt buffer through callee 12, zeroes two words, and
///   tail-jumps.
///
/// Edge cases: a zero row count skips the row loop; an unmatched key falls
/// back to table+0x690; a zero answer byte from callee 4 takes the extra
/// resolve call. The frame buffer passed to callee 3 is never written
/// before the call; the rewrite passes zeroes, matching the checker's
/// zero stack fill.
lf_checker_rt::export!(cdecl, rw_0091F0F0() -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }

        const STATE_WORD: u32 = 0x010345D8;
        const STATE_PULSE: u32 = 0x010345DC;
        const HANDLE_SLOT: u32 = 0x01034660;
        const MODE_BYTE: u32 = 0x0116C250;
        const LAYOUT_FLAG: u32 = 0x0116C253;
        const READY_BYTE: u32 = 0x01195E9D;
        const ZERO_QW0: u32 = 0x01195EA0;
        const ZERO_QW1: u32 = 0x01195EA8;
        const MEMSET_DST: u32 = 0x01195F70;
        const TABLE_PTR: u32 = 0x01BB5628;
        const OWNER_PTR: u32 = 0x01BB5554;
        const TRIPLE0: u32 = 0x011956B8;
        const TRIPLE1: u32 = 0x01195910;
        const TRIPLE2: u32 = 0x01195B68;

        const MODE_ALT2: u8 = 0x72;
        const MODE_ALT1: u8 = 0x6a;
        const TAG_DEFAULT: u32 = 0x26;
        const TAG_ALT1: u32 = 0x33;
        const TAG_ALT2: u32 = 0x30;

        const STR_TABLE: u32 = 0x00E85EDC;
        const STR_FAIL_DEFAULT: u32 = 0x00E85EE4;
        const STR_FAIL_ALT2: u32 = 0x00E85F00;
        const STR_FAIL_ALT1: u32 = 0x00E84D18;
        const STR_TRIPLE0: u32 = 0x00E84D34;
        const STR_TRIPLE1: u32 = 0x00E84D44;
        const STR_TRIPLE2: u32 = 0x00E84D4C;
        const STR_HANDLE2: u32 = 0x00E84D60;
        const STR_RESOLVE: u32 = 0x00E84D68;
        const STR_FINAL: u32 = 0x00E8507C;

        const ROW_STRIDE: u32 = 0x88;
        const ROW_KEY: u32 = 0x80;
        const ENTRY_KEY: u32 = 0x80;
        const ENTRY_INDEX: u32 = 0x84;
        const TABLE_ROWS: u32 = 0x0;
        const TABLE_COUNT: u32 = 0x4;
        const FALLBACK_OFF: u32 = 0x690;
        const MEMSET_LEN: u32 = 0x5FA0;

        const TRIPLE_WORDS: [u32; 5] = [0x44000000, 0x44000000, 0x42200000, 0x40800000, 0x421E0000];

        const C_RESET: u32 = 1;
        const C_HANDLE: u32 = 2;
        const C_ENTRY: u32 = 3;
        const C_RESOLVE: u32 = 4;
        const C_RELEASE_A: u32 = 5;
        const C_RELEASE_B: u32 = 6;
        const C_ASSET: u32 = 7;
        const C_REGISTER: u32 = 8;
        const C_SLOT: u32 = 9;
        const C_FINALIZE: u32 = 10;
        const C_PULSE: u32 = 11;
        const C_ZERO: u32 = 12;
        const C_TAIL: u32 = 13;

        const SLOTS: [(u32, u32); 46] = [
            (0x01195EB4, 0x00E84D88),
            (0x01195EB8, 0x00E84DA0),
            (0x01195EBC, 0x00E84DAC),
            (0x01195EC0, 0x00E84DB8),
            (0x01195EC4, 0x00E84DD0),
            (0x01195EC8, 0x00E84DD8),
            (0x01195ECC, 0x00E84DE4),
            (0x01195ED0, 0x00E84DF8),
            (0x01195ED4, 0x00E84E04),
            (0x01195ED8, 0x00E84E18),
            (0x01195EE0, 0x00E84E28),
            (0x01195EDC, 0x00E84E40),
            (0x01195EE4, 0x00E84E4C),
            (0x01195EE8, 0x00E84E58),
            (0x01195EEC, 0x00E84E70),
            (0x01195EF0, 0x00E84E7C),
            (0x01195EF4, 0x00E84E8C),
            (0x01195EF8, 0x00E84EA0),
            (0x01195F00, 0x00E84EAC),
            (0x01195EFC, 0x00E84EC0),
            (0x01195F04, 0x00E84EDC),
            (0x01195F08, 0x00E84EE8),
            (0x01195F0C, 0x00E84EF4),
            (0x01195F10, 0x00E84F0C),
            (0x01195F14, 0x00E84F1C),
            (0x01195F18, 0x00E84F28),
            (0x01195F20, 0x00E84F4C),
            (0x01195F1C, 0x00E84F60),
            (0x01195F24, 0x00E84F70),
            (0x01195F28, 0x00E84F84),
            (0x01195F30, 0x00E84F8C),
            (0x01195F2C, 0x00E84F94),
            (0x01195F34, 0x00E84FB0),
            (0x01195F38, 0x00E84FB8),
            (0x01195F3C, 0x00E84FC0),
            (0x01195F40, 0x00E84FD0),
            (0x01195F44, 0x00E84FD8),
            (0x01195F48, 0x00E84FE4),
            (0x01195F4C, 0x00E85000),
            (0x01195F50, 0x00E85008),
            (0x01195F58, 0x00E85010),
            (0x01195F5C, 0x00E85024),
            (0x01195F60, 0x00E85034),
            (0x01195F64, 0x00E8504C),
            (0x01195F68, 0x00E8505C),
            (0x01195F6C, 0x00E85068),
        ];

        unsafe fn scan(tag: u32, fail_str: u32, handle: u32) {
            unsafe {
                let table = lf_checker_rt::global::<u32>(TABLE_PTR).read();
                let _tag = tag;
                let _count = rd16(table.wrapping_add(TABLE_COUNT));
                let buf = [0u32; 4];
                let mut entry =
                    lf_checker_rt::callee_cdecl!(C_ENTRY, u32, buf.as_ptr() as u32);
                if rd32(entry.wrapping_add(ENTRY_INDEX)) == 0xFFFF_FFFFu32 {
                    lf_checker_rt::callee_cdecl!(
                        C_RESOLVE, u32, handle, lf_checker_rt::relocated(fail_str)
                    );
                    return;
                }
                'passes: loop {
                    let answered =
                        lf_checker_rt::callee_cdecl!(C_RESOLVE, u32, handle, entry);
                    let flag = (answered & 0xFF) as u8;
                    let tbl = lf_checker_rt::global::<u32>(TABLE_PTR).read();
                    let mut index = rd32(entry.wrapping_add(ENTRY_INDEX)) as i32;
                    let count = rd16(tbl.wrapping_add(TABLE_COUNT)) as i32;
                    // Signed clamp: negatives to 0, then signed min with count.
                    if index < 0 {
                        index = 0;
                    } else if index > count {
                        index = count;
                    }
                    index -= 1;
                    if index >= 0 {
                        let rows = rd32(tbl.wrapping_add(TABLE_ROWS));
                        let want = rd32(entry.wrapping_add(ENTRY_KEY));
                        loop {
                            let cell = rows
                                .wrapping_add((index as u32).wrapping_mul(ROW_STRIDE))
                                .wrapping_add(ROW_KEY);
                            if rd32(cell) == want {
                                entry = rows.wrapping_add(
                                    (index as u32).wrapping_mul(ROW_STRIDE),
                                );
                                break;
                            }
                            index -= 1;
                            if index < 0 {
                                entry = tbl.wrapping_add(FALLBACK_OFF);
                                break;
                            }
                        }
                    } else {
                        entry = tbl.wrapping_add(FALLBACK_OFF);
                    }
                    if rd32(entry.wrapping_add(ENTRY_INDEX)) != 0xFFFF_FFFFu32 {
                        continue 'passes;
                    }
                    if flag == 0 {
                        lf_checker_rt::callee_cdecl!(
                            C_RESOLVE, u32, handle, lf_checker_rt::relocated(fail_str)
                        );
                    }
                    break 'passes;
                }
            }
        }

        unsafe fn asset_triple(slot: u32, name: u32) {
            unsafe {
                let owner = lf_checker_rt::global::<u32>(OWNER_PTR).read();
                let resolved = lf_checker_rt::callee_cdecl!(
                    C_ASSET, u32, lf_checker_rt::relocated(name), 0u32
                );
                let registered =
                    lf_checker_rt::callee_thiscall!(C_REGISTER, u32, owner, resolved);
                let out = lf_checker_rt::global::<u32>(slot);
                out.write(registered);
                for (i, w) in TRIPLE_WORDS.iter().enumerate() {
                    out.add(i + 1).write(*w);
                }
            }
        }

        lf_checker_rt::global::<u32>(STATE_WORD).write(0xFFFF_FFFFu32);
        lf_checker_rt::global::<u8>(READY_BYTE).write(0);
        lf_checker_rt::callee_cdecl!(C_RESET, u32);
        let handle = lf_checker_rt::callee_cdecl!(
            C_HANDLE, u32, lf_checker_rt::relocated(STR_TABLE)
        );
        let mode = lf_checker_rt::global::<u8>(MODE_BYTE).read();
        if mode == MODE_ALT2 {
            scan(TAG_ALT2, STR_FAIL_ALT2, handle);
        } else if mode == MODE_ALT1 {
            scan(TAG_ALT1, STR_FAIL_ALT1, handle);
        } else if lf_checker_rt::global::<u8>(LAYOUT_FLAG).read() != 0 {
            scan(TAG_ALT1, STR_FAIL_ALT1, handle);
        } else {
            scan(TAG_DEFAULT, STR_FAIL_DEFAULT, handle);
        }

        lf_checker_rt::callee_cdecl!(C_RELEASE_A, u32, handle);
        lf_checker_rt::callee_cdecl!(C_RELEASE_B, u32, handle);
        asset_triple(TRIPLE0, STR_TRIPLE0);
        asset_triple(TRIPLE1, STR_TRIPLE1);
        if mode == MODE_ALT1 || lf_checker_rt::global::<u8>(LAYOUT_FLAG).read() != 0 {
            asset_triple(TRIPLE2, STR_TRIPLE2);
        }

        let handle2 = lf_checker_rt::callee_cdecl!(
            C_HANDLE, u32, lf_checker_rt::relocated(STR_HANDLE2)
        );
        lf_checker_rt::callee_cdecl!(
            C_RESOLVE, u32, handle2, lf_checker_rt::relocated(STR_RESOLVE)
        );
        lf_checker_rt::callee_cdecl!(C_RELEASE_A, u32, handle2);
        lf_checker_rt::callee_cdecl!(C_RELEASE_B, u32, handle2);
        for (slot, name) in SLOTS {
            lf_checker_rt::callee_thiscall!(
                C_SLOT, u32, lf_checker_rt::relocated(slot), lf_checker_rt::relocated(name)
            );
        }

        lf_checker_rt::callee_cdecl!(C_FINALIZE, u32, 0xFFFF_FFFFu32);
        lf_checker_rt::global::<u32>(STATE_PULSE).write(0);
        lf_checker_rt::callee_cdecl!(C_PULSE, u32);
        lf_checker_rt::global::<u32>(STATE_PULSE).write(1);
        lf_checker_rt::callee_cdecl!(C_PULSE, u32);
        lf_checker_rt::global::<u32>(STATE_PULSE).write(0xFFFF_FFFFu32);
        let final_handle = lf_checker_rt::callee_cdecl!(
            C_HANDLE, u32, lf_checker_rt::relocated(STR_FINAL)
        );
        lf_checker_rt::global::<u32>(HANDLE_SLOT).write(final_handle);
        lf_checker_rt::callee_cdecl!(
            C_ZERO, u32, lf_checker_rt::relocated(MEMSET_DST), 0u32, MEMSET_LEN
        );
        lf_checker_rt::global::<u64>(ZERO_QW0).write(0);
        lf_checker_rt::global::<u64>(ZERO_QW1).write(0);
        // Tail call: the stub returns straight to the trampoline, so this
        // must stay the last operation of the function.
        lf_checker_rt::callee_cdecl!(C_TAIL, u32)
    }
});
