// original: 0x00e41510 gated_state_advance
// gated state advance with tail dispatch.
// If the level byte at this+0x399 is clear, does nothing. Otherwise runs the
// gate check: a negative answer parks the state at 3, a positive answer runs
// the continue check, whose negative answer tail-dispatches to the shared
// routine with this+0x3d0, and whose positive answer builds a token through
// two helpers and parks the state at 4. Returns nothing meaningful.
export!(thiscall, rw_00e41510(this_obj: u32) -> u32 {
    unsafe {
        const LEVEL_OFF: u32 = 0x399;
        const KEY_OFF: u32 = 0x38c;
        const HANDLER_OFF: u32 = 0x3b4;
        const OUT_OFF: u32 = 0x3b8;
        const PARAM_OFF: u32 = 0x39c;
        const AUX_OFF: u32 = 0x3ce;
        const TAIL_OFF: u32 = 0x3d0;
        const STATE_OFF: u32 = 0x44d;
        if *((this_obj.wrapping_add(LEVEL_OFF)) as *const u8) == 0 {
            return 0;
        }
        let gate: u32 = callee_thiscall!(1, u32, this_obj);
        if (gate & 0xFF) == 0 {
            *((this_obj.wrapping_add(STATE_OFF)) as *mut u8) = 3;
            *((this_obj.wrapping_add(AUX_OFF)) as *mut u8) = (gate & 0xFF) as u8;
            return 0;
        }
        let level = *((this_obj.wrapping_add(LEVEL_OFF)) as *const u8) as u32;
        let cont: u32 = callee_thiscall!(2, u32, this_obj, level);
        if (cont & 0xFF) == 0 {
            return callee_thiscall!(3, u32, this_obj.wrapping_add(TAIL_OFF));
        }
        let key = *((this_obj.wrapping_add(KEY_OFF)) as *const u32);
        let handler = *((this_obj.wrapping_add(HANDLER_OFF)) as *const u32);
        let token: u32 = callee_thiscall!(4, u32, handler, key);
        let out = this_obj.wrapping_add(OUT_OFF);
        let param = *((this_obj.wrapping_add(PARAM_OFF)) as *const u32);
        callee_cdecl!(5, u32, token, param, out);
        *((this_obj.wrapping_add(STATE_OFF)) as *mut u8) = 4;
        0
    }
});
