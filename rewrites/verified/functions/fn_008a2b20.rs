// original: 0x008a2b20 audio_sound_entry_resolver
//! Sound entry resolver: resolve an index, validate the entry id through the
//! audio table, then dispatch on a hook result through a four-way jump table.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global};

export!(thiscall, rw_008a2b20(obj: *mut u8, a0: u32, a1: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6F40;
        const ROW_BASE_OFF: u32 = 0x6F10;
        let b4 = *(obj.add(0xB4) as *const u16);
        let ebp: u32 = if b4 == 0xFFFF {
            callee_cdecl!(1, u32, a0)
        } else {
            (b4 as i16) as i32 as u32
        };
        if ebp == 0xFFFFFFFF {
            return 2;
        }
        let idb = *(obj.add(0x48) as *const u8) as u32;
        if idb == 0xFF {
            return 2;
        }
        let bank = *(obj.add(0x40) as *const u8) as u32;
        let stride = *global::<u32>(0x115D964);
        let tab = *global::<u32>(0x115D988);
        let row = *((bank
            .wrapping_mul(ROW_STRIDE)
            .wrapping_add(tab)
            .wrapping_add(ROW_BASE_OFF)) as *const u32);
        let ebx = stride.wrapping_mul(idb).wrapping_add(row);
        if ebx == 0 {
            return 2;
        }
        let r2: u32 = callee_cdecl!(2, u32, ebp);
        let b8 = *(obj.add(0xB8) as *const u16) as u32;
        let b0 = *(obj.add(0xB0) as *const u32);
        let r3: u32 = callee_thiscall!(3, u32, r2, b8, b0);
        if (a1 & 0xFF) != 0 {
            if r3 == 0 {
                return 1;
            } else if r3 == 2 {
                return 2;
            } else {
                return 0;
            }
        }
        *(obj.add(0xB6) as *mut u16) = (ebp & 0xFFFF) as u16;
        let _: u32 = callee_thiscall!(4, u32, ebx, b0, ebp);
        if r3 > 3 {
            return 2;
        }
        match r3 {
            0 => {
                if *(obj.add(0xB4) as *const u16) != 0xFFFF {
                    return 1;
                }
                let r5: u32 = callee_cdecl!(5, u32, a0, b0);
                if r5 == 0 {
                    return 2;
                }
                let r6: u32 = callee_thiscall!(6, u32, obj as u32, 0);
                let w1a = *((r5 + 0x1A) as *const u16);
                *((r6 + 0xE4) as *mut u16) = w1a;
                let w18 = *((r5 + 0x18) as *const u16) as u32;
                let w10 = *((r5 + 0x10) as *const u32);
                let r7: u32 = callee_cdecl!(7, u32, w10, w18);
                *((r6 + 0xE0) as *mut u32) = r7;
                let w14 = *((r5 + 0x14) as *const u32);
                let ee = (r6 + 0xEE) as *mut u8;
                if (w14 as i32) < 0 {
                    *ee &= 0xFB;
                } else {
                    *ee |= 4;
                }
                1
            }
            1 => 0,
            2 => 2,
            _ => {
                if (*(obj.add(0x39) as *const u8) & 0x20) == 0 {
                    return 2;
                }
                let _: u32 = callee_thiscall!(8, u32, r2, b8, b0);
                0
            }
        }
    }
});
