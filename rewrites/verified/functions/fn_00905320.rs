// original: 0x00905320 input_gate_check (proposed)
/// Decide whether input may flow, consulting gates and the prober.
///
/// Returns 0 when the kill word is set, the first state word is set, either
/// presence word is set, the prober callee reports an object that is busy
/// (byte `+0x210` set) or held (word `+0xa70` is 1), or the gauge callee
/// (a thiscall on the static gauge, run only when its word is set) answers
/// above `0x64` (unsigned low-byte `ja`). Otherwise runs the ready callee:
/// a non-zero low byte forces 0, else the static output byte is returned.
/// Cdecl with no arguments; only the low byte is set.
export!(cdecl, rw_00905320() -> u32 {
    unsafe {
        /// Kill word (file VA).
        const KILL: u32 = 0x018B6E8D;
        /// First state word (file VA).
        const STATE: u32 = 0x011609F6;
        /// Second state word (file VA).
        const STATE2: u32 = 0x011D6FA0;
        /// Presence words (file VAs).
        const PRES_A: u32 = 0x016154A0;
        const PRES_B: u32 = 0x016154A1;
        /// Static gauge object word (file VA).
        const GAUGE: u32 = 0x01161518;
        /// Static output byte (file VA).
        const OUT: u32 = 0x0118F4BC;
        /// Gauge limit, compared against the low byte unsigned.
        const GAUGE_MAX: u8 = 0x64;
        const PROBE_ID: u32 = 1;
        const GAUGE_ID: u32 = 2;
        const READY_ID: u32 = 3;
        if (global::<u8>(KILL)).read() != 0 {
            return 0;
        }
        if (global::<u8>(STATE)).read() != 0 {
            return 1;
        }
        if (global::<u32>(STATE2)).read_unaligned() != 0 {
            return 0;
        }
        if (global::<u8>(PRES_A)).read() != 0 {
            return 0;
        }
        if (global::<u8>(PRES_B)).read() != 0 {
            return 0;
        }
        let obj: u32 = callee_cdecl!(PROBE_ID, u32, 0u32);
        if obj != 0 {
            if ((obj.wrapping_add(0x210)) as *const u8).read() != 0 {
                return 0;
            }
            if ((obj.wrapping_add(0xA70)) as *const u32).read_unaligned() == 1 {
                return 0;
            }
        }
        if (global::<u8>(GAUGE)).read() != 0 {
            let g: u32 = callee_thiscall!(GAUGE_ID, u32, relocated(GAUGE));
            // Unsigned low-byte compare ((an instruction of the original); ja).
            if (g as u8) > GAUGE_MAX {
                return 0;
            }
        }
        let r: u32 = callee_cdecl!(READY_ID, u32,);
        if (r as u8) != 0 {
            0
        } else {
            (global::<u8>(OUT)).read() as u32
        }
    }
});
