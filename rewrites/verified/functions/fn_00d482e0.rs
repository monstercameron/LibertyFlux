// original: 0x00d482e0 audio_gate_check
/// Gate check deciding whether an audio event may start (1) or not (0).
///
/// Returns 1 early when any veto applies, in order: the disabled flag on
/// this, a mode/class veto from the table-driven helper pair, a state veto,
/// a global mute byte, a 22-unit range check on the reported position
/// (squared length above 484.0), a floor check on a related value, or a
/// nonzero answer from any of the four follow-up helpers. Otherwise runs
/// the final two helpers: 0 when the first declines, else the second
/// helper's verdict. Only the low byte of the helpers' answers is
/// significant where the original tests AL.
export!(thiscall, rw_00d482e0(this: u32, arg1: u32) -> u8 {
    unsafe {
        const ENTRY_TABLE: u32 = 0x1295CD8;
        const MUTE_FLAG: u32 = 0x12FB3B3;
        const RANGE2: f32 = 484.0;
        const FLOOR: f32 = -1000.0;
        const CLASS_TAG: u32 = 0x59;
        const FINAL_ARG: u32 = 0x770;
        let ask_pair: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let ask_pair_again: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let ask_state: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(3) as usize);
        let report_pos: extern "cdecl" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(4) as usize);
        let follow0: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(5) as usize);
        let follow1: extern "cdecl" fn() -> u32 =
            core::mem::transmute(callee_addr(6) as usize);
        let follow2: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(7) as usize);
        let decide: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(8) as usize);
        let confirm: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(9) as usize);

        if *((this.wrapping_add(0x5D)) as *const u8) & 4 != 0 {
            return 1;
        }
        let mut veto = false;
        if *((arg1.wrapping_add(0x26C)) as *const u8) & 4 != 0 {
            let cab = *((arg1.wrapping_add(0xB30)) as *const u32);
            if cab != 0 {
                let idx = *((cab.wrapping_add(0x2E)) as *const i16) as i32;
                let entry = *((relocated(ENTRY_TABLE)
                    .wrapping_add((idx as u32).wrapping_mul(4)))
                    as *const u32);
                let flags = *((entry.wrapping_add(0x94)) as *const u32);
                if (flags >> 5) & 1 != 0 {
                    if ask_pair(cab, arg1) == 1 {
                        veto = true;
                    } else if ask_pair_again(cab, arg1) == 3 {
                        veto = true;
                    }
                }
                if *((entry.wrapping_add(0xC4)) as *const u32) == CLASS_TAG
                    && (ask_state(cab, arg1) as u8) == 0
                {
                    return 1;
                }
            }
        }
        if veto {
            return 1;
        }
        if ((*((arg1.wrapping_add(0x2A0)) as *const u32)) >> 0x11) & 1 != 0 {
            return 1;
        }
        if *((relocated(MUTE_FLAG)) as *const u8) != 0 {
            return 1;
        }
        let mut scratch = [0u32; 3];
        let pos = report_pos(scratch.as_mut_ptr() as u32);
        let x = *((pos as *const f32));
        let y = *((pos.wrapping_add(4)) as *const f32);
        let z = *((pos.wrapping_add(8)) as *const f32);
        if x * x + y * y + z * z > RANGE2 {
            return 1;
        }
        let base = *((arg1.wrapping_add(0x20)) as *const u32);
        if FLOOR > *((base.wrapping_add(0x38)) as *const f32) {
            return 1;
        }
        let aux = *((arg1.wrapping_add(0x224)) as *const u32);
        if follow0(aux.wrapping_add(0x44), 4) != 0 {
            return 1;
        }
        if (follow1() as u8) != 0 {
            return 1;
        }
        if (follow2(arg1) as u8) != 0 {
            return 1;
        }
        if (decide(arg1) as u8) == 0 {
            return 0;
        }
        if (confirm(aux.wrapping_add(0x2E0), FINAL_ARG, 0) as u8) != 0 {
            1
        } else {
            0
        }
    }
});
