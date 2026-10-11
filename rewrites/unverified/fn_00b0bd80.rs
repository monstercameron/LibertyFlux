// original: 0x00B0BD80 Pickup_ApplyStateTransition

/// Apply a pickup state transition using the pickup's signed 16-bit slot
/// value, a lookup result, and a caller-supplied player record. The lookup
/// state must be one of 1 through 6. For state 3 in network mode, the
/// transition continues only when the signed lookup value and the player's
/// marker agree on whether they equal 0x1e. A nonzero timer is clamped to
/// 0x61a7 only when it is signed-greater than 0x61a8. Otherwise, an empty
/// pickup may notify the player and network helpers. The final stored word is
/// read from a global setting. All helper results and their call order are
/// part of the behavior.
///
/// Calling convention: thiscall with one 32-bit player pointer on the stack.
lf_checker_rt::export!(thiscall, rw_00B0BD80(this: u32, player: u32) -> u32 {
    unsafe {
        const SLOT_VALUE_OFFSET: u32 = 0x40;
        const STATE_OFFSET: u32 = 0x44;
        const TIMER_OFFSET: u32 = 0x48;
        const PLAYER_MARKER_OFFSET: u32 = 0x2f8;
        const PLAYER_ARRAY_OFFSET: u32 = 0x2d4;
        const PLAYER_METHOD_OFFSET: u32 = 0x2b0;
        const RESULT_STATE_OFFSET: u32 = 4;
        const RESULT_MODEL_OFFSET: u32 = 0x80;
        const STATE_THREE: u32 = 3;
        const SPECIAL_MARKER: u32 = 0x1e;
        const TIMER_LIMIT: i32 = 0x61a8;
        const TIMER_CLAMP: u32 = 0x61a7;
        const MODE_VA: u32 = 0x11d6_fd4;
        const COPY_VALUE_VA: u32 = 0x1173_5b4;
        const NETWORK_OBJECT_VA: u32 = 0x1176_888;
        const SLOT_INDEX_SCALE: u32 = 12;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u32 {
            unsafe { (address as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_global(file_va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(file_va).read_unaligned() }
        }

        let signed_slot = unsafe { read_u16(this + SLOT_VALUE_OFFSET) as i16 as i32 };
        let slot_word = signed_slot as u32;
        let lookup_value = lf_checker_rt::callee_cdecl!(1, u32, slot_word);
        let lookup_result = lf_checker_rt::callee_cdecl!(2, u32, lookup_value, slot_word);
        let state = unsafe { read_u32(lookup_result + RESULT_STATE_OFFSET) };

        if unsafe { read_u8(this + STATE_OFFSET) } != 1 && (1..=6).contains(&state) {
            let network_mode = unsafe { read_global(MODE_VA) };
            if (network_mode as i32) >= 2
                && state == STATE_THREE
                && ((lookup_value == SPECIAL_MARKER)
                    != (unsafe { read_u32(player + PLAYER_MARKER_OFFSET) } == SPECIAL_MARKER))
            {
                return 0;
            }

            let timer = unsafe { read_u32(this + TIMER_OFFSET) };
            if timer != 0 {
                let timer_arg = if (timer as i32) > TIMER_LIMIT { TIMER_CLAMP } else { timer };
                let _ = lf_checker_rt::callee_thiscall!(4, u32, this, timer_arg);
            } else {
                if unsafe { read_u32(this) } == 0 {
                    let model_result = lf_checker_rt::callee_cdecl!(3, u32, lookup_value);
                    let model_index = unsafe { read_u16(model_result + RESULT_MODEL_OFFSET) };
                    let player_method = player.wrapping_add(PLAYER_METHOD_OFFSET);
                    let model_entry = player
                        .wrapping_add(PLAYER_ARRAY_OFFSET)
                        .wrapping_add(model_index.wrapping_mul(SLOT_INDEX_SCALE));
                    let model_value = unsafe { read_u32(model_entry) };
                    let _ = lf_checker_rt::callee_thiscall!(5, u32, player_method, model_value);
                    let network_object = lf_checker_rt::relocated(NETWORK_OBJECT_VA);
                    let _ = lf_checker_rt::callee_thiscall!(6, u32, network_object, 0u32);
                }
                let _ = lf_checker_rt::callee_thiscall!(4, u32, this, 0u32);
            }

            let copy_value = unsafe { read_global(COPY_VALUE_VA) };
            unsafe { write_u32(this, copy_value) };
        }
    }
    0
});
