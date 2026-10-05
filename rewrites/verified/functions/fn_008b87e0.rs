// original: 0x008B87E0 fe_action_table_lookup

/// Look up a frontend action id from one of four mapping tables.
///
/// `index` selects the row (0..24). The table is picked by the input-mode
/// flag global (zero selects the keyboard pair, nonzero the pad pair) and by
/// a device tag dword the pad-state query leaves at a fixed offset of its
/// result: tag `0x40` selects the primary table of the pair, any other value
/// the alternate table. Returns the mapped id, or -1 for unmapped rows.
export!(cdecl, rw_008B87E0(index: u32) -> u32 {
    unsafe {
        const PRIMARY_KEYS: [u32; 24] = [
            0xFFFF_FFFF, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xFFFF_FFFF, 0xFFFF_FFFF, 11, 12,
            0xFFFF_FFFF, 14, 15, 16, 17, 18, 19, 20, 0xFFFF_FFFF,
        ];
        const PRIMARY_PAD: [u32; 24] = [
            0xFFFF_FFFF, 33, 34, 2, 3, 4, 5, 6, 7, 8, 9, 0xFFFF_FFFF, 0xFFFF_FFFF, 25, 26,
            0xFFFF_FFFF, 14, 27, 37, 31, 18, 19, 20, 0xFFFF_FFFF,
        ];
        const ALT_KEYS: [u32; 24] = [
            0xFFFF_FFFF, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xFFFF_FFFF, 0xFFFF_FFFF, 11, 12,
            0xFFFF_FFFF, 14, 35, 28, 32, 18, 19, 20, 0xFFFF_FFFF,
        ];
        const ALT_PAD: [u32; 24] = [
            0xFFFF_FFFF, 33, 34, 2, 3, 4, 5, 6, 7, 8, 9, 0xFFFF_FFFF, 0xFFFF_FFFF, 25, 26,
            0xFFFF_FFFF, 14, 36, 30, 32, 18, 19, 20, 0xFFFF_FFFF,
        ];
        const MODE_FLAG: u32 = 0x0116_0C74;
        const DEVICE_TAG_OFF: u32 = 0x3A64;
        const PRIMARY_TAG: u32 = 0x40;
        let pad_mode = *global::<u32>(MODE_FLAG) != 0;
        let query = callee_cdecl!(1, u32, 0);
        let tag = *((query.wrapping_add(DEVICE_TAG_OFF)) as *const u32);
        let table = match (pad_mode, tag == PRIMARY_TAG) {
            (false, true) => &PRIMARY_KEYS,
            (true, true) => &PRIMARY_PAD,
            (false, false) => &ALT_KEYS,
            (true, false) => &ALT_PAD,
        };
        // NOTE: the original ends each path with its stack-cookie check
        // (verified: (an instruction of the original); ret — preserves every register and
        // writes nothing on success). The checker's recorder stub would
        // answer that call with a scripted value in EAX, clobbering the
        // table result the real check preserves — so the four cookie sites
        // are deliberately left unpatched (the real cmp+ret runs natively,
        // observably a no-op) and the rewrite performs no cookie call. The
        // single semantic call (the pad-state query) is intercepted.
        table[index as usize]
    }
});
