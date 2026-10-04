// original: 0x008a35f0 rage::audMultitrackSound::vf7
//! Multitrack sound setup: gate on the first hook, take the entry count from
//! the array at obj+0x94, then resolve one entry id per slot through the
//! second hook and the audio table. Returns the low byte of an ok flag.

use lf_k2_rt::{callee_thiscall, export, global, relocated};

export!(thiscall, rw_008a35f0(obj: *mut u8, a0: u32, a1: u32, blk: *mut u8) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6F40;
        const ROW_BASE_OFF: u32 = 0x6F10;
        let gate: u32 = callee_thiscall!(1, u32, obj as u32, a0, a1, blk as u32);
        if (gate & 0xFF) == 0 {
            return gate;
        }
        let arr = *(obj.add(0x94) as *const u32) as *mut u8;
        let count = *(arr as *const u8) as u32;
        *(obj.add(0xB0) as *mut u32) = count;
        let mode = *(obj.add(0x70) as *const u32) & 0xC0000;
        *(obj.add(0xB5) as *mut u8) = if mode == 0x40000 { 1 } else { 0 };
        let mut save = [0u32; 6];
        for k in 0..6 {
            save[k] = *((blk as *const u32).add(k));
        }
        if count > 8 {
            return 0;
        }
        if count == 0 {
            return 1;
        }
        let mut ok: u8 = 1;
        let mut dst = obj.add(0x9C) as *mut u32;
        let mut p = arr.add(5);
        let mut i: u32 = 0;
        while i < count {
            for k in 0..6 {
                *((blk as *mut u32).add(k)) = save[k];
            }
            let w = *(p.sub(4) as *const u32);
            let r: u32 = callee_thiscall!(
                2,
                u32,
                relocated(0x115DC18),
                w,
                obj as u32,
                a1,
                blk as u32
            );
            let idb: u8 = if r == 0 {
                0xFF
            } else {
                let bank = *(obj.add(0x40) as *const u8) as u32;
                let tab = *global::<u32>(0x115D988);
                let row = *((bank
                    .wrapping_mul(ROW_STRIDE)
                    .wrapping_add(tab)
                    .wrapping_add(ROW_BASE_OFF)) as *const u32);
                let q = r.wrapping_sub(row) / *global::<u32>(0x115D964);
                (q & 0xFF) as u8
            };
            *(obj.add(0x48 + i as usize) as *mut u8) = idb;
            if i < 2 {
                *dst = *(p as *const u32);
            }
            let chk = *(obj.add(0x48 + i as usize) as *const u8) as u32;
            if chk == 0xFF {
                ok = 0;
            } else {
                let bank2 = *(obj.add(0x40) as *const u8) as u32;
                let stride = *global::<u32>(0x115D964);
                let tab2 = *global::<u32>(0x115D988);
                let row2 = *((bank2
                    .wrapping_mul(ROW_STRIDE)
                    .wrapping_add(tab2)
                    .wrapping_add(ROW_BASE_OFF)) as *const u32);
                let entry = row2.wrapping_add(stride.wrapping_mul(chk));
                if entry == 0 {
                    ok = 0;
                }
            }
            dst = dst.add(1);
            i += 1;
            p = p.add(8);
        }
        ok as u32
    }
});
