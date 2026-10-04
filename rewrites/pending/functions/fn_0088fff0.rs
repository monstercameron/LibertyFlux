// original: 0x0088fff0 rage::audSound::vf8
/// Look up this sound's voice record, then advance its Doppler/position state.
///
/// The head reads two selector bytes from the sound object and fetches a
/// record pointer from the voice table: a row base (row stride 0x6f40, bias
/// 0x6f14) plus the column selector times a global stride. A 0xff column
/// selector means "no voice" and yields a null pointer, which faults the
/// same way on both sides. The shared tail
/// derives two small rotation counters from a global divisor table, ticks a
/// 5-bit lane counter, and either initialises the record's velocity slots and
/// timestamp (first call) or integrates a new velocity from the position
/// history scaled by the reciprocal of the elapsed stamp delta. It finishes
/// by shifting the history slots and returning the oldest tag word.
///
/// The tail's timer spin-wait slow path (taken only when a divisor-table
/// entry is 0, 1 or 2) is excluded by contract: every table entry is pinned
/// to 3 or more, so both sides always take the direct path.
export!(thiscall, rw_0088fff0(sound: u32, stamp: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f14;
        const NO_VOICE: u8 = 0xff;
        const STEP_SCALE: f32 = f32::from_bits(0x3d08850a);
        let col = *((sound + 4) as *const u8);
        let voice = if col == NO_VOICE {
            0u32
        } else {
            let row = *((sound + 0x40) as *const u8) as u32;
            let col_stride = *global::<u32>(0x115d968);
            let table = *global::<u32>(0x115d988);
            let slot = table
                .wrapping_add(row.wrapping_mul(ROW_STRIDE))
                .wrapping_add(TABLE_BIAS);
            (*(slot as *const u32))
                .wrapping_add((col as u32).wrapping_mul(col_stride))
        };
        let lane_sel = (*((voice + 0xe7) as *const u8) & 7) as u32;
        let count = *global::<u32>(0x115f808 + lane_sel * 8);
        let other = *global::<u32>(0x115f808 + lane_sel * 8 + 4);
        let rot = count.wrapping_add(1) % 3;
        let rot2 = count.wrapping_add(2) % 3;
        // Contract pins every divisor-table entry above 2, so the timer
        // spin-wait slow path (rot == other) is unreachable on both sides.
        let lane = (voice as *mut u8).add(count as usize * 32 + 0x5f);
        let tick = *lane;
        *lane = (tick & 0xe0) | (tick.wrapping_add(1) & 0x1f);
        let prev = *((voice + 0xd0) as *const u32);
        if prev == 0 {
            *((voice) as *mut u32) = 0;
            *((voice + 4) as *mut u32) = 0;
            *((voice + 8) as *mut u32) = 0;
            *((voice + 0xd0) as *mut u32) = stamp;
        } else {
            let cur = count.wrapping_add(1).wrapping_mul(2) as usize * 8;
            let ref_ = rot2.wrapping_add(1).wrapping_mul(2) as usize * 8;
            let dv0 = ((voice as usize + cur) as *const f32).read_unaligned()
                - ((voice as usize + ref_) as *const f32).read_unaligned();
            let dv1 = ((voice as usize + cur + 4) as *const f32).read_unaligned()
                - ((voice as usize + ref_ + 4) as *const f32).read_unaligned();
            let dv2 = ((voice as usize + cur + 8) as *const f32).read_unaligned()
                - ((voice as usize + ref_ + 8) as *const f32).read_unaligned();
            let dt = stamp.wrapping_sub(prev);
            *((voice + 0xd0) as *mut u32) = stamp;
            let tag = *lane;
            *lane = (tag & 0xe0) | (tag.wrapping_add(1) & 0x1f);
            if dt == 0 {
                *((voice) as *mut u32) = 0;
                *((voice + 4) as *mut u32) = 0;
                *((voice + 8) as *mut u32) = 0;
            } else {
                let wide = (dt as i32) as f64
                    + if dt >> 31 == 0 { 0.0 } else { 4294967296.0 };
                let rate = 1.0 / ((wide as f32) * STEP_SCALE);
                ((voice as *mut f32)).write_unaligned(dv0 * rate);
                (((voice + 4) as *mut f32)).write_unaligned(dv1 * rate);
                (((voice + 8) as *mut f32)).write_unaligned(dv2 * rate);
                // The original reloads a word of uninitialised below-ESP
                // scratch here; the contract defines it as zero.
                (((voice + 0xc) as *mut f32)).write_unaligned(0.0);
            }
        }
        let src_hist = (voice as usize) + (count as usize + 2) * 32;
        let dst_hist = (voice as usize) + (rot as usize + 2) * 32;
        let mut k = 0usize;
        while k < 32 {
            let w = ((src_hist + k) as *const u64).read_unaligned();
            ((dst_hist + k) as *mut u64).write_unaligned(w);
            k += 8;
        }
        let src_tag = (voice as usize) + (count as usize + 1) * 16;
        let dst_tag = (voice as usize) + (rot as usize + 1) * 16;
        ((dst_tag) as *mut u32).write_unaligned(((src_tag) as *const u32).read_unaligned());
        ((dst_tag + 4) as *mut f32)
            .write_unaligned(((src_tag + 4) as *const f32).read_unaligned());
        ((dst_tag + 8) as *mut f32)
            .write_unaligned(((src_tag + 8) as *const f32).read_unaligned());
        let tagw = ((src_tag + 0xc) as *const u32).read_unaligned();
        ((dst_tag + 0xc) as *mut u32).write_unaligned(tagw);
        let _ = other;
        tagw
    }
});
