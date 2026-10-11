// original: 0x009B0960 TimeShift_UpdateB

/// Clears all 32 report slots, resets three per-object mode words, then
/// optionally resolves an audio voice from the control object at `+0x58`.
/// The control object's key byte is at `+0x04`, its table row at `+0x40`,
/// and the voice table is selected through two one-word globals. A key of
/// `0xff` selects a null voice. The resolved voice receives the exact raw
/// bits for `-100.0f`, after which the control object is released. Every
/// engine call is routed through its contract stand-in; the function returns
/// the accumulator left by the final call, or the null control pointer when
/// the optional object is absent.
lf_checker_rt::export!(thiscall, rw_009b0960(this: u32) -> u32 {
    const REPORT_COUNT: u32 = 32;
    const SLOT_ENTRY_STRIDE: u32 = 0x1a0;
    const FIRST_REPORT_SLOT: u32 = 0x70;
    const SLOT_INDEX: u32 = 0x2c;
    const SLOT_BOUNDARY: u32 = 0x30;
    const CONTROL_POINTER: u32 = 0x58;
    const RESET_WORD_A: u32 = 0x4c;
    const RESET_WORD_B: u32 = 0x20;
    const RESET_WORD_C: u32 = 0x24;
    const CONTROL_KEY: u32 = 0x04;
    const CONTROL_ROW: u32 = 0x40;
    const VOICE_TABLE_SCALE: u32 = 0x0115D968;
    const VOICE_TABLE_BASE: u32 = 0x0115D988;
    const VOICE_ROW_STRIDE: u32 = 0x6f40;
    const VOICE_ROW_BASE: u32 = 0x6f14;
    const SILENT_GAIN_BITS: u32 = 0xc2c80000;

    unsafe {
        for report_index in 0..REPORT_COUNT {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, this, report_index);
        }

        ((this + RESET_WORD_A) as *mut u32).write_unaligned(0);
        ((this + RESET_WORD_B) as *mut u32).write_unaligned(0);
        ((this + RESET_WORD_C) as *mut u32).write_unaligned(0);

        let control = ((this + CONTROL_POINTER) as *const u32).read_unaligned();
        let mut result = control;
        if control != 0 {
            let key = ((control + CONTROL_KEY) as *const u8).read();
            let voice = if key == 0xff {
                0
            } else {
                let row = ((control + CONTROL_ROW) as *const u8).read() as u32;
                let scale = lf_checker_rt::global::<u32>(VOICE_TABLE_SCALE).read_unaligned();
                let table = lf_checker_rt::global::<u32>(VOICE_TABLE_BASE).read_unaligned();
                let entry = table
                    .wrapping_add(row.wrapping_mul(VOICE_ROW_STRIDE))
                    .wrapping_add(VOICE_ROW_BASE)
                    .wrapping_add(scale.wrapping_mul(key as u32));
                (entry as *const u32).read_unaligned()
            };

            result = lf_checker_rt::callee_thiscall!(2, u32, voice, SILENT_GAIN_BITS);
            result = lf_checker_rt::callee_thiscall!(3, u32, control, 0);
        }
        result
    }
});
