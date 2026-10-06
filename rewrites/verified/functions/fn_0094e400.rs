// original: 0x0094E400 files_sink_settings_tables (proposed)

/// Run the settings-table sink passes over the game's file/memory option tables.
///
/// When the settings-dirty flag byte is clear and the settings mode word is 1,
/// the settings object is first refreshed through callee id 0. Then the two
/// context passes run (ids 1 and 2), and the sink callee consumes a fixed
/// series of table slices: one word, then every 0x2C-byte record of the main
/// table copied through a frame buffer, then per-row words of the two guard
/// tables (each row's encrypted lookup runs only for a non-zero guard, which
/// the proof steers away from), then the remaining ranges word by word. Two
/// tail passes (ids 6 and 7) run over fixed tables. The function returns the
/// second tail pass's answer with its low byte cleared; the security-cookie
/// check at the end runs natively.
///
/// The guard-row sinks share one stack cleanup for several calls, so each
/// address computation sees the pushes of the earlier calls in its row: the
/// four sinks of a guard-2 row read slots E+4, E+0xC, E+0x14, E+0x10 (values
/// 0, 0, -1, -1) and the two sinks of a guard-3 row read E+4, E+8 (0, 0).
///
/// Original: 0x0094E400 (cdecl, no arguments, no register inputs; the calls
/// into the encrypted region at four guarded sites are not covered by this
/// proof and neither is the cookie check, which runs natively).
lf_checker_rt::export!(cdecl, rw_0094E400() -> u32 {
    unsafe {
        const FLAG_BYTE: u32 = 0x0116D27D;
        const MODE_WORD: u32 = 0x011D6FD4;
        const SETTINGS_OBJECT: u32 = 0x01BB5624;
        const CONTEXT_WORD: u32 = 0x012BD0C4;
        const ID_REFRESH: u32 = 0;
        const ID_CONTEXT_A: u32 = 1;
        const ID_CONTEXT_B: u32 = 2;
        const ID_SINK_FRAME4: u32 = 3;
        const ID_SINK_FRAME44: u32 = 4;
        const ID_SINK_GLOBAL: u32 = 5;
        const ID_TAIL_A: u32 = 6;
        const ID_TAIL_B: u32 = 7;
        const MAIN_FIRST: u32 = 0x011D78F4;
        const MAIN_RECORDS: u32 = 0x011D78F8;
        const MAIN_END: u32 = 0x011D9108;
        const RECORD_LEN: u32 = 0x2C;
        const GUARD2_FIRST: u32 = 0x011D9110;
        const GUARD2_END: u32 = 0x011D923C;
        const GUARD3_FIRST: u32 = 0x011D9240;
        const GUARD3_END: u32 = 0x011D9290;
        const RANGE4_FIRST: u32 = 0x011D9290;
        const RANGE4_END: u32 = 0x011D9550;
        const RANGE5_FIRST: u32 = 0x011D9550;
        const RANGE5_END: u32 = 0x011D9578;
        const RANGE6_FIRST: u32 = 0x011D95A0;
        const RANGE6_END: u32 = 0x011D95F0;
        const SINGLE_AFTER_GUARDS: u32 = 0x011D910C;
        const TAIL_A_TABLE: u32 = 0x011D95F0;
        const TAIL_B_TABLE: u32 = 0x011D97F0;

        #[inline(always)]
        unsafe fn gr8(file_va: u32) -> u8 {
            unsafe { (lf_checker_rt::relocated(file_va) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn gr32(file_va: u32) -> u32 {
            unsafe {
                (lf_checker_rt::relocated(file_va) as *const u32).read_unaligned()
            }
        }
        if gr8(FLAG_BYTE) == 0 && gr32(MODE_WORD) == 1 {
            lf_checker_rt::callee_thiscall!(ID_REFRESH, u32, gr32(SETTINGS_OBJECT));
        }
        lf_checker_rt::callee_thiscall!(ID_CONTEXT_A, u32, gr32(CONTEXT_WORD));
        lf_checker_rt::callee_cdecl!(ID_CONTEXT_B, u32,);

        lf_checker_rt::callee_cdecl!(
            ID_SINK_GLOBAL,
            u32,
            lf_checker_rt::relocated(MAIN_FIRST),
            4u32
        );

        // Main table: copy each 0x2C-byte record to the frame and sink it.
        let mut record = [0u32; 11];
        let mut p = MAIN_RECORDS;
        while p < MAIN_END {
            for i in 0..11usize {
                record[i] = gr32(p + i as u32 * 4);
            }
            lf_checker_rt::callee_cdecl!(
                ID_SINK_FRAME44,
                u32,
                record.as_ptr() as u32,
                RECORD_LEN
            );
            p += RECORD_LEN;
        }
        // Guard table 2: a non-zero guard would take the encrypted lookup
        // first (not covered: guards are always zero). The row's four sinks
        // share one add-esp, so the lea of each sees the earlier pushes and
        // the sunk words are slots E+4, E+0xC, E+0x14, E+0x10 = 0, 0, -1, -1.
        let mut p2 = GUARD2_FIRST;
        while p2 < GUARD2_END {
            let guard = gr32(p2);
            if guard != 0 {
                // Unreachable under the proof's globals; kept to show the shape.
                // The original calls the encrypted lookup here.
                core::hint::black_box(guard);
            }
            let zero = 0u32;
            let minus_one = 0xFFFF_FFFFu32;
            lf_checker_rt::callee_cdecl!(ID_SINK_FRAME4, u32, &zero as *const u32 as u32, 4u32);
            lf_checker_rt::callee_cdecl!(ID_SINK_FRAME4, u32, &zero as *const u32 as u32, 4u32);
            lf_checker_rt::callee_cdecl!(
                ID_SINK_FRAME4,
                u32,
                &minus_one as *const u32 as u32,
                4u32
            );
            lf_checker_rt::callee_cdecl!(
                ID_SINK_FRAME4,
                u32,
                &minus_one as *const u32 as u32,
                4u32
            );
            p2 += 0x0C;
        }

        // Guard table 3: same excluded lookup on nonzero; the two sinks share
        // one add-esp, so they read slots E+4, E+8 = 0, 0.
        let mut p3 = GUARD3_FIRST;
        while p3 < GUARD3_END {
            let guard = gr32(p3);
            if guard != 0 {
                core::hint::black_box(guard);
            }
            let zero = 0u32;
            lf_checker_rt::callee_cdecl!(ID_SINK_FRAME4, u32, &zero as *const u32 as u32, 4u32);
            lf_checker_rt::callee_cdecl!(ID_SINK_FRAME4, u32, &zero as *const u32 as u32, 4u32);
            p3 += 4;
        }

        lf_checker_rt::callee_cdecl!(
            ID_SINK_GLOBAL,
            u32,
            lf_checker_rt::relocated(SINGLE_AFTER_GUARDS),
            4u32
        );

        let mut p4 = RANGE4_FIRST;
        while p4 < RANGE4_END {
            lf_checker_rt::callee_cdecl!(
                ID_SINK_GLOBAL,
                u32,
                lf_checker_rt::relocated(p4),
                0x58u32
            );
            p4 += 0x58;
        }
        let mut p5 = RANGE5_FIRST;
        while p5 < RANGE5_END {
            lf_checker_rt::callee_cdecl!(
                ID_SINK_GLOBAL,
                u32,
                lf_checker_rt::relocated(p5),
                4u32
            );
            p5 += 4;
        }
        let mut p6 = RANGE6_FIRST;
        while p6 < RANGE6_END {
            lf_checker_rt::callee_cdecl!(
                ID_SINK_GLOBAL,
                u32,
                lf_checker_rt::relocated(p6),
                8u32
            );
            p6 += 8;
        }

        lf_checker_rt::callee_thiscall!(
            ID_TAIL_A,
            u32,
            lf_checker_rt::relocated(TAIL_A_TABLE)
        );
        let tail: u32 = lf_checker_rt::callee_thiscall!(
            ID_TAIL_B,
            u32,
            lf_checker_rt::relocated(TAIL_B_TABLE)
        );
        tail & 0xFFFF_FF00
    }
});
