// original: 0x00CDBD40 CTaskSimpleNMHighFall::vf21

/// Updates a high-fall task after its event key is checked.
///
/// The method compares the fixed eight-byte key with the text at the second
/// stack argument's `+0x0c`, using a scripted cdecl helper. It indexes a
/// 64-byte-stride mode table with the unsigned word at `this + 0xe0`. A
/// matching key and enabled table byte apply blend mode 3 and set the status
/// bit. A matching key also checks the unsigned game tick against the
/// wrapping sum of the start word at `+0x20` and duration at `+0x24`; a later
/// tick applies blend mode 4. Otherwise the arm/grab helper receives
/// `this + 0x60` and the two original stack arguments, and its return is
/// forwarded. This x86 thiscall method has two stack arguments and callee
/// cleanup of eight bytes.
lf_checker_rt::export!(thiscall, rw_00cdbd40(this: u32, arg_one: u32, params: u32) -> u32 {
    unsafe {
        const HIGH_FALL_KEY: u32 = 0x00ed_cdc8;
        const TABLE_BASE: u32 = 0x0171_d066;
        const GAME_TICK: u32 = 0x0117_35b4;
        const START_TICK: u32 = 0x20;
        const DURATION: u32 = 0x24;
        const STATUS: u32 = 0x14;
        const BLEND_WORD: u32 = 0x1c;
        const MODE_INDEX: u32 = 0xe0;
        const ARM_GRAB_CONTEXT: u32 = 0x60;
        const KEY_LENGTH: u32 = 8;
        const BLEND_MASK: u32 = 0xff08_45ff;
        const BLEND_BITS: u32 = 0x0008_4500;
        const COMPARE_HELPER: u32 = 1;
        const ARM_GRAB_HELPER: u32 = 2;

        let compare = lf_checker_rt::callee_cdecl!(
            COMPARE_HELPER,
            u32,
            lf_checker_rt::relocated(HIGH_FALL_KEY),
            params.wrapping_add(0x0c),
            KEY_LENGTH,
        );
        let key_matches = compare == 0;
        let table_index = (this.wrapping_add(MODE_INDEX) as *const u32).read_unaligned();
        let table_flag = (lf_checker_rt::relocated(TABLE_BASE) as *const u8)
            .wrapping_add(table_index.wrapping_shl(6) as usize)
            .read();

        if key_matches && table_flag != 0 {
            let status = this.wrapping_add(STATUS) as *mut u8;
            status.write(status.read() | 4);

            let blend_word = this.wrapping_add(BLEND_WORD) as *mut u32;
            let previous_blend = blend_word.read_unaligned();
            let mode_three_blend = (previous_blend & BLEND_MASK) | BLEND_BITS;
            blend_word.write_unaligned(mode_three_blend);
            (this.wrapping_add(BLEND_WORD) as *mut u8).write(3);
        }

        if key_matches {
            let start_tick = (this.wrapping_add(START_TICK) as *const u16).read_unaligned() as u32;
            let duration = (this.wrapping_add(DURATION) as *const u32).read_unaligned();
            let end_tick = start_tick.wrapping_add(duration);
            let current_tick = lf_checker_rt::global::<u32>(GAME_TICK).read();
            if current_tick > end_tick {
                let status = this.wrapping_add(STATUS) as *mut u8;
                status.write(status.read() | 4);

                let blend_word = this.wrapping_add(BLEND_WORD) as *mut u32;
                let previous_blend = blend_word.read_unaligned();
                let mode_four_blend = (previous_blend & BLEND_MASK) | BLEND_BITS;
                blend_word.write_unaligned(mode_four_blend);
                (this.wrapping_add(BLEND_WORD) as *mut u8).write(4);
                return mode_four_blend;
            }
            return end_tick;
        }

        lf_checker_rt::callee_thiscall!(
            ARM_GRAB_HELPER,
            u32,
            this.wrapping_add(ARM_GRAB_CONTEXT),
            arg_one,
            params,
        )
    }
});
