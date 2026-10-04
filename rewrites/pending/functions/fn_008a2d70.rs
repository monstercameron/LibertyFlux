// original: 0x008a2d70 rage::audSimpleSound::vf7
//! Simple sound setup: gate on the first hook, resolve one entry id through
//! the shared audio hook and table, reconcile the index words through a chain
//! of small hooks, then build the voice through the tail hooks. The two sites
//! calling 0x888040 get distinct stub ids so consecutive answers may differ.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

export!(thiscall, rw_008a2d70(obj: *mut u8, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6F40;
        const ROW_BASE_OFF: u32 = 0x6F10;
        let gate: u32 = callee_thiscall!(1, u32, obj as u32, a0, a1, a2);
        if (gate & 0xFF) == 0 {
            return gate;
        }
        let arr = *(obj.add(0x94) as *const u32) as *mut u8;
        let r2: u32 = callee_thiscall!(2, u32, relocated(0x115DC18), obj as u32, a1, a2);
        let idb: u32 = if r2 == 0 {
            0xFF
        } else {
            let bank = *(obj.add(0x40) as *const u8) as u32;
            let tab = *global::<u32>(0x115D988);
            let row = *((bank
                .wrapping_mul(ROW_STRIDE)
                .wrapping_add(tab)
                .wrapping_add(ROW_BASE_OFF)) as *const u32);
            (r2.wrapping_sub(row) / *global::<u32>(0x115D964)) & 0xFF
        };
        *(obj.add(0x48) as *mut u8) = idb as u8;
        if idb == 0xFF {
            return 0;
        }
        let bank2 = *(obj.add(0x40) as *const u8) as u32;
        let stride = *global::<u32>(0x115D964);
        let tab2 = *global::<u32>(0x115D988);
        let row2 = *((bank2
            .wrapping_mul(ROW_STRIDE)
            .wrapping_add(tab2)
            .wrapping_add(ROW_BASE_OFF)) as *const u32);
        if row2.wrapping_add(stride.wrapping_mul(idb)) == 0 {
            return 0;
        }
        let r3: u32 = callee_thiscall!(3, u32, relocated(0x115DC18), *(arr.add(4) as *const u32));
        *(obj.add(0xB8) as *mut u16) = (r3 & 0xFFFF) as u16;
        *(obj.add(0xB0) as *mut u32) = *(arr.add(8) as *const u32);
        if *(arr as *const u32) != 0xFFFFFFFF {
            let q: u32 = callee_cdecl!(4, u32, *(arr as *const u32));
            *(obj.add(0xB4) as *mut u16) = (q & 0xFFFF) as u16;
            if (q & 0xFFFF) == 0xFFFF {
                let m: u32 = callee_cdecl!(5, u32, *(obj.add(0xB8) as *const u16) as u32);
                let q2: u32 = callee_cdecl!(10, u32, m);
                *(obj.add(0xB4) as *mut u16) = (q2 & 0xFFFF) as u16;
                if (q2 & 0xFFFF) == 0xFFFF {
                    *(arr as *mut u32) = 0xFFFFFFFF;
                } else {
                    let sx = ((q2 & 0xFFFF) as u16) as i16 as i32 as u32;
                    let g: u32 = callee_cdecl!(6, u32, sx);
                    *(arr as *mut u32) = g;
                }
            }
        }
        if *(obj.add(0xB4) as *const u16) == 0xFFFF {
            return 1;
        }
        let sv = *(obj.add(0xB0) as *const u32);
        let sx2 = (*(obj.add(0xB4) as *const u16) as i16) as i32 as u32;
        let g2: u32 = callee_cdecl!(6, u32, sx2);
        let r7: u32 = callee_cdecl!(7, u32, g2, sv);
        if r7 == 0 {
            *(obj.add(0x39) as *mut u8) |= 1;
            return 1;
        }
        let r8: u32 = callee_thiscall!(8, u32, obj as u32, 0);
        *((r8 + 0xE4) as *mut u16) = *((r7 + 0x1A) as *const u16);
        let w18 = *((r7 + 0x18) as *const u16) as u32;
        let r9: u32 = callee_cdecl!(9, u32, *((r7 + 0x10) as *const u32), w18);
        *((r8 + 0xE0) as *mut u32) = r9;
        let ee = (r8 + 0xEE) as *mut u8;
        if (*((r7 + 0x14) as *const u32) as i32) < 0 {
            *ee &= 0xFB;
        } else {
            *ee |= 4;
        }
        1
    }
});
