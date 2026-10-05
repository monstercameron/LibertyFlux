// original: 0x00AD5AB0 audio_convert_position_words (proposed)

/// Convert the indexed position floats to integers and publish the words.
///
/// Truncates the two position floats at slot `index * 16` (converting like
/// x86 cvttss2si: NaN, infinities and out-of-range values become i32::MIN)
/// into their globals, then copies four words from the slot-table entry
/// (cdecl/0 helper) at offsets 0x10b4-0x10c0 into the following globals.
/// Takes no arguments (cdecl/0); returns the last copied word.
lf_checker_rt::export!(cdecl, rw_00ad5ab0() -> u32 {
    unsafe {
        const SLOTS: u32 = 1;
        const INDEX: u32 = 0x01174794;
        const POSITIONS: u32 = 0x0158DE00;
        const POS_A: u32 = 0x0158D5FC;
        const POS_B: u32 = 0x015932C0;
        const WORD_BASE: u32 = 0x015932C4;
        const ENTRY_OFF: u32 = 0x10B4;
        #[inline(always)]
        fn cvttss2si_bits(f: f32) -> i32 {
            if f.is_nan() || f < -2147483648.0 || f >= 2147483648.0 {
                i32::MIN
            } else {
                f as i32
            }
        }
        let row = lf_checker_rt::global::<u32>(INDEX).read().wrapping_mul(16);
        let fa = (lf_checker_rt::relocated(POSITIONS).wrapping_add(row) as *const f32).read();
        lf_checker_rt::global::<i32>(POS_A).write(cvttss2si_bits(fa));
        let fb = (lf_checker_rt::relocated(POSITIONS).wrapping_add(row).wrapping_add(4) as *const f32)
            .read();
        lf_checker_rt::global::<i32>(POS_B).write(cvttss2si_bits(fb));
        let entry = lf_checker_rt::callee_cdecl!(SLOTS, u32,);
        for i in 0..4u32 {
            let w = (entry.wrapping_add(ENTRY_OFF).wrapping_add(i.wrapping_mul(4)) as *const u32)
                .read();
            lf_checker_rt::global::<u32>(WORD_BASE).add(i as usize).write(w);
        }
        (entry.wrapping_add(ENTRY_OFF).wrapping_add(12) as *const u32).read()
    }
});
