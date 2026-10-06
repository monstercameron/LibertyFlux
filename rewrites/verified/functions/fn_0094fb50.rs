// original: 0x0094fb50 record_stream_interpreter
/// Run the record-stream interpreter over the shared record tables.
///
/// Scans the slot-mode table for the active stream, allocates the 0x818
/// scratch records and zeroes them, then walks the selected byte stream:
/// each record consumes bytes until a terminator, dispatching every byte
/// to one of twelve handlers. Link bytes attach the current stream pointer
/// to the addressed record (stamping its generation and tag); unlink bytes
/// fade the record's stored pointers by index times a third times the old
/// byte value, then release the record; byte zero advances to the next
/// stream; anything else just polls. One opening poll plus a per-byte poll
/// decide how far the stream advances. Afterwards every surviving record is
/// finalized (its stored pointers faded once more through tag-selected
/// lanes, a flag bit cleared for stream-table tags) and released, and the
/// scratch block is freed. Returns zero, or zero early when the allocation
/// fails.
export!(cdecl, rw_0094fb50() -> u32 {
    unsafe {
        const NSLOTS: u32 = 0x11F6FFB;
        const MODES: u32 = 0x11F6FF0;
        const STREAMS: u32 = 0x11F6F7C;
        const COUNT: u32 = 0x11F707C;
        const THIRD: u32 = 0xFE8808;
        const ALLOC_ID: u32 = 1;
        const POLL_ID: u32 = 2;
        const FREE_ID: u32 = 3;
        const NREC: u32 = 0x818;
        const RECSZ: u32 = 0x38;
        const REC_BYTES: u32 = 0x1C540;
        const GEN_OFF: u32 = 0x2C;
        const LINK_OFF: u32 = 0x2E;
        const COUNT0_OFF: u32 = 0x30;
        const ROT_OFF: u32 = 0x31;
        const COUNT1_OFF: u32 = 0x32;
        const TAG_OFF: u32 = 0x33;
        const BANK1_OFF: u32 = 0x0C;
        const BANK2_OFF: u32 = 0x18;

        /// Fade one stored byte: index scaled by a third, then by the byte.
        /// Evaluation order is pinned to the original's (first product fully
        /// formed before the second multiply, truncation toward zero).
        #[inline(always)]
        fn fade_step(index: i32, value: u8, third: f32) -> u8 {
            let scaled = core::hint::black_box(index as f32)
                * core::hint::black_box(third);
            let scaled = core::hint::black_box(scaled);
            let mixed = scaled * core::hint::black_box(value as f32);
            core::hint::black_box(mixed) as i32 as u8
        }

        #[inline(always)]
        unsafe fn clear_record(rec: u32) {
            *((rec.wrapping_add(GEN_OFF)) as *mut u16) = 0xFFFF;
            *((rec.wrapping_add(LINK_OFF)) as *mut u32) = 0;
            *((rec.wrapping_add(COUNT1_OFF)) as *mut u8) = 0;
            let mut k = 0u32;
            while k < 3 {
                *((rec.wrapping_add(k * 4)) as *mut u32) = 0;
                *((rec.wrapping_add(BANK1_OFF + k * 4)) as *mut u32) = 0;
                k += 1;
            }
            *((rec.wrapping_add(0x18)) as *mut u64) = 0;
            *((rec.wrapping_add(0x20)) as *mut u64) = 0;
            *((rec.wrapping_add(0x28)) as *mut u32) = 0;
        }

        // Entry slot scan: first mode-2 slot, step one forward (wrapping),
        // then cycle to the next mode-1-or-2 slot. A zero slot count would
        // divide by zero, which the contract excludes.
        let n = *(relocated(NSLOTS) as *const u8) as u32;
        let mut eax = 0u32;
        if n != 0 {
            loop {
                if *(relocated(MODES).wrapping_add(eax) as *const u8) == 2 {
                    break;
                }
                eax += 1;
                if eax >= n {
                    break;
                }
            }
        }
        let mut slot = (eax + 1) % n;
        loop {
            let al = *(relocated(MODES).wrapping_add(slot) as *const u8);
            if al == 2 {
                break;
            }
            if al == 1 {
                break;
            }
            slot = (slot + 1) % n;
        }
        let mut base: u32 =
            *((relocated(STREAMS).wrapping_add(slot * 4)) as *const u32);
        let buf: u32 = callee_cdecl!(ALLOC_ID, u32, REC_BYTES);
        if buf == 0 {
            return 0;
        }
        // Record init: every record starts unlinked with zeroed banks.
        let mut r = 0u32;
        while r < NREC {
            let rec = buf.wrapping_add(r * RECSZ);
            *((rec.wrapping_add(GEN_OFF)) as *mut i16) = -1;
            *((rec.wrapping_add(LINK_OFF)) as *mut u32) = 0;
            *((rec.wrapping_add(COUNT1_OFF)) as *mut u16) = 0;
            let mut k = 0u32;
            while k < 3 {
                *((rec.wrapping_add(k * 4)) as *mut u32) = 0;
                *((rec.wrapping_add(BANK1_OFF + k * 4)) as *mut u32) = 0;
                k += 1;
            }
            *((rec.wrapping_add(0x18)) as *mut u64) = 0;
            *((rec.wrapping_add(0x20)) as *mut u64) = 0;
            *((rec.wrapping_add(0x28)) as *mut u32) = 0;
            r += 1;
        }
        let c = *(relocated(COUNT) as *const i32);
        let third = *(relocated(THIRD) as *const f32);
        // Main loop: one stream run per record, bytes until the terminator.
        // The record poll runs once up front; every record end reuses its
        // answer as the stride and jumps back to the per-byte top.
        let mut off = 0u32;
        let mut rec_idx = 0i32;
        if c > 0 {
            let stride: u32 = callee_cdecl!(POLL_ID, u32, 5u32);
            loop {
                loop {
                    let ptr = base.wrapping_add(off);
                    let byte = *(ptr as *const u8);
                    if byte == 5 {
                        break;
                    }
                    match byte {
                        0x00 => {
                            let mode =
                                *(relocated(MODES).wrapping_add(slot) as *const u8);
                            if mode == 2 {
                                // The original re-reads this same byte forever;
                                // the contract never scripts that combination.
                                loop {
                                    core::hint::spin_loop();
                                }
                            }
                            slot = ((slot as i32 + 1) % (n as i32)) as u32;
                            off = 0;
                            base = *((relocated(STREAMS).wrapping_add(slot * 4))
                                as *const u32);
                        }
                        0x20 | 0x1B | 0x1C | 0x0A | 0x0B | 0x0C | 0x0D | 0x0E
                        | 0x15 | 0x17 => {
                            let field: u32 = match byte {
                                0x20 => 0x10,
                                0x15 => 0x08,
                                _ => 0x0C,
                            };
                            let tag: u8 = match byte {
                                0x20 => 0x20,
                                0x15 => 0x15,
                                0x17 => 0x17,
                                _ => byte,
                            };
                            let node = *((ptr.wrapping_add(field)) as *const u32);
                            if node != 0xFFFF_FFFF {
                                let idx = (node & 0xFFFF) as u16 as i16 as i32;
                                let gen = (node >> 16) as u16;
                                let rec = buf.wrapping_add(
                                    (idx.wrapping_mul(RECSZ as i32)) as u32,
                                );
                                let cur =
                                    *(rec.wrapping_add(GEN_OFF) as *const u16);
                                let mut attach = false;
                                if cur == 0xFFFF {
                                    *((rec.wrapping_add(LINK_OFF)) as *mut u16) =
                                        rec_idx as u16;
                                    attach = true;
                                } else if cur == gen {
                                    attach = true;
                                }
                                if attach {
                                    *((rec.wrapping_add(GEN_OFF)) as *mut u16) =
                                        gen;
                                    let count0 =
                                        *(rec.wrapping_add(COUNT0_OFF) as *const i8);
                                    *((rec.wrapping_add(TAG_OFF)) as *mut u8) = tag;
                                    if count0 < 3 {
                                        *((rec.wrapping_add((count0 as i32 * 4)
                                            as u32))
                                            as *mut u32) = ptr;
                                        *((rec.wrapping_add(COUNT0_OFF))
                                            as *mut i8) = count0.wrapping_add(1);
                                    }
                                    let rot =
                                        *(rec.wrapping_add(ROT_OFF) as *const i8);
                                    *((rec.wrapping_add(BANK1_OFF).wrapping_add(
                                        (rot as i32 * 4) as u32,
                                    )) as *mut u32) = ptr;
                                    let rot_next = (rot as i32 + 1) % 3;
                                    *((rec.wrapping_add(ROT_OFF)) as *mut u8) =
                                        rot_next as u8;
                                    if (0x0A..=0x0E).contains(&byte) {
                                        let flag = *((ptr.wrapping_add(0x40))
                                            as *const u8);
                                        if flag & 8 == 0 {
                                            if *(rec.wrapping_add(COUNT1_OFF)
                                                as *const u8)
                                                == 0
                                            {
                                                *((rec.wrapping_add(BANK2_OFF))
                                                    as *mut u32) = ptr;
                                                *((rec.wrapping_add(COUNT1_OFF))
                                                    as *mut u8) = 1;
                                            }
                                        } else {
                                            let s = *(rec.wrapping_add(COUNT1_OFF)
                                                as *const i8);
                                            if s != 0 && s < 5 {
                                                *((rec.wrapping_add(BANK2_OFF)
                                                    .wrapping_add(
                                                        (s as i32 * 4) as u32,
                                                    ))
                                                    as *mut u32) = ptr;
                                                *((rec.wrapping_add(COUNT1_OFF))
                                                    as *mut i8) =
                                                    s.wrapping_add(1);
                                            }
                                        }
                                    }
                                }
                            }
                            let again = *(ptr as *const u8);
                            let adv: u32 =
                                callee_cdecl!(POLL_ID, u32, again as u32);
                            off = off.wrapping_add(adv);
                        }
                        0x21 | 0x1D | 0x0F | 0x16 | 0x18 => {
                            let node = *((ptr.wrapping_add(4)) as *const u32);
                            if node != 0xFFFF_FFFF {
                                let idx = (node & 0xFFFF) as u16 as i16 as i32;
                                let gen = (node >> 16) as u16;
                                let rec = buf.wrapping_add(
                                    (idx.wrapping_mul(RECSZ as i32)) as u32,
                                );
                                if *(rec.wrapping_add(GEN_OFF) as *const u16) == gen
                                {
                                    let fade_off: u32 = match byte {
                                        0x21 => 9,
                                        0x1D | 0x0F => 0x0B,
                                        0x16 => 0x10,
                                        _ => 8,
                                    };
                                    let mut d = (*(rec.wrapping_add(ROT_OFF)
                                        as *const i8)
                                        as i32)
                                        .wrapping_sub(1);
                                    if d < 0 {
                                        d = 2;
                                    }
                                    let mut i = 0i32;
                                    while i < 3 {
                                        if (*(rec.wrapping_add(LINK_OFF)
                                            as *const u16)
                                            as i16 as i32)
                                            > 2
                                        {
                                            let np = *((rec.wrapping_add(
                                                (i * 4) as u32,
                                            ))
                                                as *const u32);
                                            if np != 0 {
                                                let mut run = true;
                                                if byte == 0x21
                                                    && *(np.wrapping_add(0x14)
                                                        as *const i32)
                                                        >= 0
                                                    && (*(np.wrapping_add(8)
                                                        as *const u8)
                                                        & 0x10)
                                                        == 0
                                                {
                                                    run = false;
                                                }
                                                if run {
                                                    let bv = *(np
                                                        .wrapping_add(fade_off)
                                                        as *const u8);
                                                    *(np.wrapping_add(fade_off)
                                                        as *mut u8) =
                                                        fade_step(i, bv, third);
                                                }
                                            }
                                        }
                                        if rec_idx > 3 {
                                            let np2 = *((rec
                                                .wrapping_add(BANK1_OFF)
                                                .wrapping_add((d * 4) as u32))
                                                as *const u32);
                                            if np2 != 0 {
                                                let bv = *(np2
                                                    .wrapping_add(fade_off)
                                                    as *const u8);
                                                *(np2.wrapping_add(fade_off)
                                                    as *mut u8) =
                                                    fade_step(i, bv, third);
                                            }
                                        }
                                        d -= 1;
                                        if d < 0 {
                                            d = 2;
                                        }
                                        i += 1;
                                    }
                                    clear_record(rec);
                                }
                            }
                            let again = *(ptr as *const u8);
                            let adv: u32 =
                                callee_cdecl!(POLL_ID, u32, again as u32);
                            off = off.wrapping_add(adv);
                        }
                        _ => {
                            let adv: u32 =
                                callee_cdecl!(POLL_ID, u32, byte as u32);
                            off = off.wrapping_add(adv);
                        }
                    }
            }
            off = off.wrapping_add(stride);
            rec_idx += 1;
            if rec_idx >= c {
                break;
            }
            }
        }
        // Finalize: fade every surviving record's bank through its tag's
        // lane, clear the stream-table flag bit, then release the record.
        let mut f = 0u32;
        while f < NREC {
            let rec = buf.wrapping_add(f * RECSZ);
            if *(rec.wrapping_add(GEN_OFF) as *const i16) != -1 {
                let mut k = 0u32;
                while k < 3 {
                    if (*(rec.wrapping_add(LINK_OFF) as *const u16) as i16
                        as i32)
                        > 2
                    {
                        let np = *((rec.wrapping_add(k * 4)) as *const u32);
                        if np != 0 {
                            let e = ((*(rec.wrapping_add(TAG_OFF)
                                as *const u8))
                                as u32)
                                .wrapping_sub(10);
                            if e <= 0x16 {
                                match e {
                                    0x16 => {
                                        let lane = *(np.wrapping_add(9)
                                            as *const u8);
                                        let gate = *(np.wrapping_add(0x14)
                                            as *const i32);
                                        let val = if gate < 0
                                            || (*(np.wrapping_add(8)
                                                as *const u8)
                                                & 0x10)
                                                != 0
                                        {
                                            fade_step(k as i32, lane, third)
                                        } else {
                                            lane
                                        };
                                        *(np.wrapping_add(9) as *mut u8) = val;
                                    }
                                    0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x11
                                    | 0x12 => {
                                        let lane = *(np.wrapping_add(0x0B)
                                            as *const u8);
                                        *(np.wrapping_add(0x0B) as *mut u8) =
                                            fade_step(k as i32, lane, third);
                                    }
                                    0x0B => {
                                        let lane = *(np.wrapping_add(0x10)
                                            as *const u8);
                                        *(np.wrapping_add(0x10) as *mut u8) =
                                            fade_step(k as i32, lane, third);
                                    }
                                    0x0D => {
                                        let lane = *(np.wrapping_add(8)
                                            as *const u8);
                                        *(np.wrapping_add(8) as *mut u8) =
                                            fade_step(k as i32, lane, third);
                                    }
                                    _ => {
                                        // Switch default lane: no tag a link
                                        // can stamp selects it, and nothing
                                        // is stored there.
                                    }
                                }
                            }
                        }
                    }
                    k += 1;
                }
                // The original funnels a flag-bit test through a conditional
                // that always picks zero here (its counter stays below five),
                // so the merged effect is clearing bit 3 for tags 0x0A-0x0E.
                let mut j = 0u32;
                while j < 5 {
                    let sp = *((rec.wrapping_add(BANK2_OFF + j * 4))
                        as *const u32);
                    if sp != 0
                        && (*(rec.wrapping_add(TAG_OFF) as *const u8))
                            .wrapping_sub(10)
                            <= 4
                    {
                        let m =
                            *(sp.wrapping_add(0x40) as *const u8);
                        *(sp.wrapping_add(0x40) as *mut u8) = m & !8;
                    }
                    j += 1;
                }
                clear_record(rec);
            }
            f += 1;
        }
        let _: u32 = callee_cdecl!(FREE_ID, u32, buf);
        0
    }
});
