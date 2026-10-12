// original: 0x00CDC6B0 select_shot_response_by_weapon

/// Selects the weapon response code from scripted weapon information.
///
/// The method reads a signed model index from the stack argument's word at
/// `+0x2e`, uses that index to load a model record from the image table, and
/// reads the record's response byte at `+0xef`. It asks the intercepted
/// weapon-info helper twice with the value at `this + 0x5c`. If the global
/// response flag is clear it returns zero. Otherwise the second result's
/// mode field selects either a packed bit from `+0x20` or one of the response
/// offsets 2, 4 or 6; the final comparison is signed. This x86 thiscall method
/// takes one stack argument and pops four bytes.
lf_checker_rt::export!(thiscall, rw_00cdc6b0(this: u32, weapon: u32) -> u32 {
    unsafe {
        const MODEL_INFO_TABLE: u32 = 0x0129_5cd8;
        const RESPONSE_ENABLED: u32 = 0x0105_21a0;
        const RESPONSE_THRESHOLD: u32 = 0x0105_2250;
        const WEAPON_INFO: u32 = 1;
        const INFO_ARGUMENT: u32 = 0x5c;
        const MODEL_INDEX: u32 = 0x2e;
        const RESPONSE_BYTE: u32 = 0xef;

        let model_index = (weapon.wrapping_add(MODEL_INDEX) as *const i16).read_unaligned() as i32;
        let info_slot = MODEL_INFO_TABLE.wrapping_add((model_index as u32).wrapping_mul(4));
        let model_info = lf_checker_rt::global::<u32>(info_slot).read_unaligned();
        let response_id = (model_info.wrapping_add(RESPONSE_BYTE) as *const u8).read() as u32;

        let weapon_info_argument = (this.wrapping_add(INFO_ARGUMENT) as *const u32).read_unaligned();
        let _first_info = lf_checker_rt::callee_cdecl!(WEAPON_INFO, u32, weapon_info_argument);
        let second_info = lf_checker_rt::callee_cdecl!(WEAPON_INFO, u32, weapon_info_argument);

        if lf_checker_rt::global::<u8>(RESPONSE_ENABLED).read() == 0 {
            return 0;
        }

        let response_mode = (second_info.wrapping_add(0x0c) as *const u32).read_unaligned();
        if response_mode == 1 {
            let packed = (second_info.wrapping_add(0x20) as *const u32).read_unaligned();
            return (((packed >> 9) as u8 as u32) & response_mode) | 8;
        }

        let response_type = (second_info.wrapping_add(4) as *const u32).read_unaligned();
        let response_offset = match response_type {
            5 => 2,
            6 => 4,
            3 => 6,
            _ => 0,
        };
        let threshold = lf_checker_rt::global::<u32>(RESPONSE_THRESHOLD).read();
        let below_threshold = (response_id as i32) < (threshold as i32);
        response_offset + u32::from(below_threshold)
    }
});
