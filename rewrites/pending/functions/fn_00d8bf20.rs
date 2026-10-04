// original: 0x00d8bf20 audio_state_histogram_tick
/// Account one audio tick for the entity: probe, histogram, events, counters.
///
/// Probes the entity through the shared predicate (a nonzero low byte ticks
/// the live counter down), files the entity's word field plus the caller's
/// index into the histogram table when the word is below 0x44c as a signed
/// 16-bit value (negative words index below the table base), fires the
/// state event matching the entity state, fires the mode event when the
/// global mode is 1 or 4 (variant picked by bit 0x20 of the flag byte), and
/// bumps the idle counter for index-zero entities that are parked and
/// flagged. Returns the mode event's answer when one fired, else the mode.
lf_rs89_rt::export!(cdecl, rw_00d8bf20(obj: *mut u8, idx: u32) -> u32 {
    unsafe {
        let probe: u32 = lf_rs89_rt::callee_cdecl!(1, u32, obj as u32);
        if probe & 0xFF != 0 {
            let live = lf_rs89_rt::global::<u32>(0x179BFB0);
            *live = (*live).wrapping_sub(1);
        }
        let w = *(obj.wrapping_add(0x2E) as *const u16);
        if (w as i16) < 0x44C {
            let i = ((w as i16) as i32).wrapping_add(idx as i32);
            let slot =
                (lf_rs89_rt::global::<u32>(0x179BFD0) as *mut u32).wrapping_add(i as usize);
            *slot = (*slot).wrapping_add(1);
        }
        let one = 1.0f32.to_bits();
        match *(obj.wrapping_add(0x1304) as *const u32) {
            0 => {
                lf_rs89_rt::callee_cdecl!(2, u32, 0x126, one);
            }
            1 => {
                lf_rs89_rt::callee_cdecl!(2, u32, 0x127, one);
            }
            2 => {
                lf_rs89_rt::callee_cdecl!(2, u32, 0x128, one);
            }
            4 => {
                lf_rs89_rt::callee_cdecl!(2, u32, 0x129, one);
            }
            5 => {
                lf_rs89_rt::callee_cdecl!(2, u32, 0x1D4, one);
            }
            _ => {}
        }
        let mode = *lf_rs89_rt::global::<u32>(0x179BF98);
        let mut out = mode;
        if mode == 1 || mode == 4 {
            let flags = *(obj.wrapping_add(0xF1F) as *const u8);
            let id = if flags & 0x20 != 0 { 0x1C9 } else { 0x1C8 };
            out = lf_rs89_rt::callee_cdecl!(2, u32, id, one);
        }
        if idx == 0
            && *(obj.wrapping_add(0x10B8) as *const u8) == 1
            && *(obj.wrapping_add(0xF1F) as *const u8) & 0x40 != 0
        {
            let idle = lf_rs89_rt::global::<u32>(0x179D104);
            *idle = (*idle).wrapping_add(1);
        }
        out
    }
});
