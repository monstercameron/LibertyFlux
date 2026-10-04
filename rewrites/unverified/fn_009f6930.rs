// original: 0x009F6930 stat_system_reset (proposed)

/// Reset the statistic subsystem to its initial state, then tail-jump to a
/// shared shutdown routine.
///
/// Takes no arguments. It runs a setup callee, releases the old record block
/// through a free-like callee, writes the six table-size words, zeroes the
/// float table (0xfd words), the int table (0x18c words), the name-pointer
/// table (each entry pointed at its own 0x80-byte slot whose first byte is
/// also cleared), two small tables (0x17 and 0x64 words) and ten flag words,
/// runs a second setup callee, and clears the remaining scalar state. When
/// two generation counters differ it then walks up to eight lanes, xoring
/// each lane's record words with table bytes masked to three bits; when they
/// match that walk is skipped. Control leaves through a tail jump, which the
/// rewrite performs as a call through the checker's callee table.
///
/// The batch lists this entry as 596 bytes, but the function ends at the tail
/// jump 470 bytes in; the bytes after belong to the callback registered by
/// the neighbouring initialiser.
///
/// Original: cdecl, no arguments, no meaningful return value.
lf_checker_rt::export!(cdecl, rw_009F6930() -> u32 {
    unsafe {
        const COUNT_FLOAT: u32 = 0x12b79a4;
        const COUNT_INT: u32 = 0x12b7ff8;
        const COUNT_NAMES: u32 = 0x12b75a8;
        const COUNT_PTRS: u32 = 0x12b651c;
        const COUNT_SMALL_A: u32 = 0x12b62fc;
        const COUNT_SMALL_B: u32 = 0x12b6490;
        const FLOAT_TABLE: u32 = 0x12b75b0;
        const INT_TABLE: u32 = 0x12b79c8;
        const NAME_PTRS: u32 = 0x12b6498;
        const NAME_SLOTS: u32 = 0x12b6528;
        const NAME_SLOT_STRIDE: u32 = 0x80;
        const SMALL_A: u32 = 0x12b62a0;
        const SMALL_B: u32 = 0x12b6300;
        const FLAG_WORDS: u32 = 0x12b79a8;
        const COUNT_FLAGS: u32 = 0x12b79bc;
        const FREED_PTR: u32 = 0x12b79c0;
        const AUX_PTR: u32 = 0x12b79c4;
        const FIELD_627C: u32 = 0x12b627c;
        const FIELD_628C: u32 = 0x12b628c;
        const GEN_NEW: u32 = 0x11d6fd4;
        const GEN_CUR: u32 = 0x11d6fd0;
        const FIELD_6268: u32 = 0x12b6268;
        const FIELD_6262: u32 = 0x12b6262;
        const FIELD_6270: u32 = 0x12b6270;
        const FIELD_6274: u32 = 0x12b6274;
        const FIELD_6278: u32 = 0x12b6278;
        const LANE_BUF: u32 = 0x12b6520;
        const LANE_COUNT: u32 = 0x12b6524;
        const SETUP_CALLEE: u32 = 0;
        const FREE_CALLEE: u32 = 1;
        const SETUP2_CALLEE: u32 = 2;
        const LANE_CALLEE: u32 = 3;
        const POLL_CALLEE: u32 = 4;
        const TAIL_CALLEE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        lf_checker_rt::callee_cdecl!(SETUP_CALLEE, u32);
        let freed = rd32(lf_checker_rt::relocated(FREED_PTR));
        wr32(lf_checker_rt::relocated(COUNT_FLOAT), 0xfd);
        wr32(lf_checker_rt::relocated(COUNT_INT), 0x18c);
        wr32(lf_checker_rt::relocated(COUNT_NAMES), 0x21);
        wr32(lf_checker_rt::relocated(COUNT_PTRS), 0x21);
        wr32(lf_checker_rt::relocated(COUNT_SMALL_A), 0x17);
        wr32(lf_checker_rt::relocated(COUNT_SMALL_B), 0x64);
        lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, freed);
        wr32(lf_checker_rt::relocated(FREED_PTR), 0);
        wr32(lf_checker_rt::relocated(AUX_PTR), 0);
        wr32(lf_checker_rt::relocated(COUNT_FLAGS), 0xa);

        // Each zeroing loop re-reads its count every iteration, as the
        // original does; the counts stay fixed, so this clears each table.
        let mut i = 0u32;
        loop {
            let n = rd32(lf_checker_rt::relocated(COUNT_FLOAT));
            if (n as i32) <= (i as i32) {
                break;
            }
            wr32(lf_checker_rt::relocated(FLOAT_TABLE) + i * 4, 0);
            i += 1;
        }
        let mut i = 0u32;
        loop {
            let n = rd32(lf_checker_rt::relocated(COUNT_INT));
            if (n as i32) <= (i as i32) {
                break;
            }
            wr32(lf_checker_rt::relocated(INT_TABLE) + i * 4, 0);
            i += 1;
        }
        let mut i = 0u32;
        loop {
            let n = rd32(lf_checker_rt::relocated(COUNT_PTRS));
            if (n as i32) <= (i as i32) {
                break;
            }
            let slot = lf_checker_rt::relocated(NAME_SLOTS) + i * NAME_SLOT_STRIDE;
            wr32(lf_checker_rt::relocated(NAME_PTRS) + i * 4, slot);
            (slot as *mut u8).write(0);
            i += 1;
        }
        let mut i = 0u32;
        loop {
            let n = rd32(lf_checker_rt::relocated(COUNT_SMALL_A));
            if (n as i32) <= (i as i32) {
                break;
            }
            wr32(lf_checker_rt::relocated(SMALL_A) + i * 4, 0);
            i += 1;
        }
        let mut i = 0u32;
        loop {
            let n = rd32(lf_checker_rt::relocated(COUNT_SMALL_B));
            if (n as i32) <= (i as i32) {
                break;
            }
            wr32(lf_checker_rt::relocated(SMALL_B) + i * 4, 0);
            i += 1;
        }
        let mut i = 0u32;
        loop {
            let n = rd32(lf_checker_rt::relocated(COUNT_FLAGS));
            if (n as i32) <= (i as i32) {
                break;
            }
            ((lf_checker_rt::relocated(FLAG_WORDS) + i * 2) as *mut u16).write_unaligned(0);
            i += 1;
        }

        lf_checker_rt::callee_cdecl!(SETUP2_CALLEE, u32);
        ((lf_checker_rt::relocated(FIELD_627C)) as *mut u16).write_unaligned(0);
        wr32(lf_checker_rt::relocated(FIELD_628C), 0);
        let gen = rd32(lf_checker_rt::relocated(GEN_NEW));
        ((lf_checker_rt::relocated(FIELD_6268)) as *mut u64).write_unaligned(0);
        ((lf_checker_rt::relocated(FIELD_6262)) as *mut u8).write(0);
        wr32(lf_checker_rt::relocated(FIELD_6270), 0);
        wr32(lf_checker_rt::relocated(FIELD_6274), 0);
        wr32(lf_checker_rt::relocated(FIELD_6278), 0);

        if gen != rd32(lf_checker_rt::relocated(GEN_CUR)) {
            let mut pos = 0u32;
            let mut lane = 0u32;
            loop {
                let n = ((lf_checker_rt::relocated(LANE_COUNT)) as *const u16)
                    .read_unaligned() as u32;
                if (pos as i32) >= (n as i32) {
                    break;
                }
                let mut rec = lf_checker_rt::callee_cdecl!(LANE_CALLEE, u32, lane);
                let polls = lf_checker_rt::callee_cdecl!(POLL_CALLEE, u32, lane) as i32;
                if polls > 0 {
                    rec = rec.wrapping_add(4);
                    let mut done = 0u32;
                    loop {
                        let buf = rd32(lf_checker_rt::relocated(LANE_BUF));
                        let cell = rd32(buf.wrapping_add(pos * 8 + 4));
                        let cur = rd32(rec);
                        let tweak = (cell ^ cur) & 7;
                        pos += 1;
                        wr32(rec, cur ^ tweak);
                        done += 1;
                        rec = rec.wrapping_add(8);
                        let more = lf_checker_rt::callee_cdecl!(POLL_CALLEE, u32, lane) as i32;
                        if !((done as i32) < more) {
                            break;
                        }
                    }
                }
                lane += 1;
                if !((lane as i32) < 8) {
                    break;
                }
            }
        }
        lf_checker_rt::callee_cdecl!(TAIL_CALLEE, u32);
        0
    }
});
