// original: 0x0097b1e0 audio_bank_read_param
/// Read one parameter word out of an audio bank row.
///
/// Selects a lane from `mode` (0xe/0xf use lane 4; 0, 1, 0x10, 0x11 use
/// lane 3 when banked, otherwise the mode halved; anything else banked
/// reads as 0) and returns the word at the selector's offset in the row
/// (0x35, 0x4a, 0x20 or 0xb; selector 6 picks 0x5f when the row tag at
/// +0x5e is 5, else 0x35). Selectors above 15 read as 0.
export!(thiscall, rw_0097b1e0(this: *const u8, row: *const u8, mode: u32, sel: u32) -> u32 {
    unsafe {
        let mut lane = mode >> 1;
        if mode == 0xe || mode == 0xf {
            lane = 4;
        }
        let b = *((this.add(0x120)) as *const u32);
        if *((b as *const u32).add(0xb80 / 4)) == 4 {
            match mode {
                0 | 1 | 0x10 | 0x11 => lane = 3,
                _ => return 0,
            }
        } else if mode == 0x10 || mode == 0x11 {
            lane = 3;
        }
        if sel > 0xf {
            return 0;
        }
        let idx = match sel {
            2 | 15 => 1,
            3 => 2,
            4 => 3,
            6 => 4,
            _ => 0,
        };
        let base = (row as u32).wrapping_add(lane.wrapping_mul(4));
        let off = match idx {
            1 => 0x4a,
            2 => 0x20,
            3 => 0x0b,
            4 => {
                if *row.add(0x5e) == 5 {
                    0x5f
                } else {
                    0x35
                }
            }
            _ => 0x35,
        };
        core::ptr::read_unaligned((base.wrapping_add(off)) as *const u32)
    }
});
