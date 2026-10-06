// original: 0x008B5CD0 SG_TIT

/// Refresh a titled list: optionally re-highlight the menu, scan the current
/// title's entries, then fill the list rows.
///
/// The function takes no arguments and returns nothing (both callers ignore
/// `eax`). It reads the list object from `OBJ` and works in four stages:
///
/// 1. Gate: if the flag byte at `BRANCH_OFF` past the object is non-zero,
///    the object is re-registered (callee 1), the menu item is highlighted
///    (callee 2) and the list is reset (callee 3).
/// 2. Scan: the title index `n` at `IDX` selects entry `n` of the table at
///    `TABLE` (24 bytes per entry: a dword entry pointer at `+0`, a
///    zero-extended 16-bit entry count at `+4`). Leading entries whose first
///    byte is not `b'.'` are counted (entries are 0x16 bytes apart), stopping
///    at the first dotted entry or when the count is reached; the count and
///    the scan bound use signed comparison, but the count is zero-extended
///    so no negative value can occur. The row bound is the scanned number
///    minus one.
/// 3. Rows: a shared name buffer is fetched (callee 4, thiscall) and stored
///    (callee 5); then episode rows are appended (callee 6 answers the next
///    row index, `-1` ends): each row resolves its episode name through
///    callee 7 (thiscall) into a stack buffer, converts it with callee 8
///    and stores the row (callee 9).
/// 4. Fill: remaining rows up to the bound are stored empty (callee 10).
///
/// Two callees use the early-push idiom (words pushed before their call
/// belong to a later call): callee 4 takes only the name-table pointer (the
/// three words below it are callee 5's bottom arguments), and callee 7 takes
/// only the row index (the word below it is callee 8's destination buffer).
/// The stack buffer cannot be observed by address (frame layouts differ), so
/// the contract skips those pointer arguments and snapshots the pointed-to
/// words instead, with callee 8's writes scripted.
///
/// Original: 0x008B5CD0 (cdecl, no stack words, `eax` ignored by callers).
lf_checker_rt::export!(cdecl, rw_008B5CD0() -> u32 {
    unsafe {
        const OBJ: u32 = 0x0116_0C0C;
        const IDX: u32 = 0x0116_0C40;
        const TABLE: u32 = 0x019D_33A0;
        const CTX: u32 = 0x0116_BFF0;
        const EP_CTX: u32 = 0x01BB_5624;
        const NAME_TABLE: u32 = 0x00E7_E03C;
        const BRANCH_OFF: u32 = 0x0117_354C;
        const ENTRY_STRIDE: u32 = 24;
        const NAME_STRIDE: u32 = 0x16;
        const DOT: u8 = 0x2E;
        const NONE: u32 = 0xFFFF_FFFF;

        let obj = lf_checker_rt::global::<u32>(OBJ).read();
        if ((obj.wrapping_add(BRANCH_OFF)) as *const u8).read() != 0 {
            lf_checker_rt::callee_cdecl!(1, u32, obj, 0);
            lf_checker_rt::callee_cdecl!(2, u32, obj, 0, 1);
            lf_checker_rt::callee_cdecl!(3, u32, 0);
        }

        let n = lf_checker_rt::global::<u32>(IDX).read();
        let entry = lf_checker_rt::relocated(TABLE).wrapping_add(n.wrapping_mul(ENTRY_STRIDE));
        // Zero-extended count; the original compares it signed (jle/jl).
        let count = ((entry + 4) as *const u16).read() as i32;
        let mut shown: i32 = 0;
        if count > 0 {
            let mut p = (entry as *const u32).read();
            loop {
                if (p as *const u8).read() == DOT {
                    break;
                }
                shown += 1;
                p = p.wrapping_add(NAME_STRIDE);
                if !(shown < count) {
                    break;
                }
            }
        }

        let shared: u32 =
            lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(CTX), lf_checker_rt::relocated(NAME_TABLE));
        lf_checker_rt::callee_cdecl!(5, u32, obj, 0, 0, shared, 1, 0, 0);

        // One buffer reused across rows, as in the original's frame.
        let mut buf = [0u32; 16];
        let mut row: u32 = lf_checker_rt::callee_cdecl!(6, u32, 0);
        let mut seq: u32 = 1;
        if row != NONE {
            loop {
                let ep: u32 = lf_checker_rt::callee_thiscall!(
                    7,
                    u32,
                    lf_checker_rt::global::<u32>(EP_CTX).read(),
                    row
                );
                lf_checker_rt::callee_cdecl!(8, u32, ep, buf.as_mut_ptr() as u32);
                lf_checker_rt::callee_cdecl!(9, u32, obj, 0, seq, buf.as_ptr() as u32, 1, 0, 0);
                row = lf_checker_rt::callee_cdecl!(6, u32, row);
                seq = seq.wrapping_add(1);
                if row == NONE {
                    break;
                }
            }
        }

        // Signed bound, as the original's jge/jl.
        let bound = shown - 1;
        while (seq as i32) < bound {
            lf_checker_rt::callee_cdecl!(10, u32, obj, 0, seq, 0, 1, 0, 0);
            seq = seq.wrapping_add(1);
        }

        // The original's stack-cookie check; preserves all registers.
        lf_checker_rt::callee_cdecl!(11, u32,);
        0
    }
});
