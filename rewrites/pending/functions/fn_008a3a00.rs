// original: 0x008a3a00 rage::audRandomizedSound::vf7
//! Randomized sound setup: gate on the first hook, pick a slot through either
//! a virtual call (mode bit set) or a direct helper, then resolve one entry
//! id through the shared audio hook and table. Returns 1 for a live entry.

use lf_k2_rt::{callee_thiscall, export, global, relocated};

export!(thiscall, rw_008a3a00(obj: *mut u8, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6F40;
        const ROW_BASE_OFF: u32 = 0x6F10;
        let gate: u32 = callee_thiscall!(1, u32, obj as u32, a0, a1, a2);
        if (gate & 0xFF) == 0 {
            return gate & 0xFFFFFF00;
        }
        let edi = *(obj.add(0x94) as *const u32) as *mut u8;
        let e5 = *(edi.add(5) as *const u8);
        let dl = *(edi.add(6).add(e5 as usize) as *const u8);
        let key = edi.add(7).add(e5 as usize) as u32;
        let mode = *(obj.add(0x70) as *const u32) & 0xC0000;
        let bl_final: u8;
        if mode == 0x40000 {
            let inner = *(edi as *const u32);
            let mut neg: u8 = 0;
            if inner != 0 {
                let vt = *(obj as *const u32);
                let tgt = *((vt as *const u8).add(0x10) as *const u32);
                let pick: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                let r = pick(obj as u32, inner);
                if r != 0 {
                    let m = f32::from_bits(*(r as *const u32));
                    if m < 0.0 {
                        neg = 1;
                    }
                }
            }
            let e4 = *(edi.add(4) as *const u8) as u32;
            let dividend: u32 = if neg == 0 {
                e4.wrapping_add(1)
            } else {
                e4.wrapping_sub(1).wrapping_add(dl as u32)
            };
            let rem = (dividend as i32) % (dl as i32);
            *(edi.add(4) as *mut u8) = (rem & 0xFF) as u8;
            bl_final = e4 as u8;
        } else {
            let r3: u32 =
                callee_thiscall!(3, u32, obj as u32, edi as u32, dl as u32, key);
            let b = (r3 & 0xFF) as u8;
            let e5b = *(edi.add(5) as *const u8);
            if e5b != 0 {
                let e4b = *(edi.add(4) as *const u8);
                *(edi.add(6).add(e4b as usize) as *mut u8) = b;
                let dividend = (e4b as u32).wrapping_add(1);
                let rem = (dividend as i32) % (e5b as i32);
                *(edi.add(4) as *mut u8) = (rem & 0xFF) as u8;
            }
            bl_final = b;
        }
        let slot = *((key as *const u8).add((bl_final as usize) * 8) as *const u32);
        let r4: u32 = callee_thiscall!(
            4,
            u32,
            relocated(0x115DC18),
            slot,
            obj as u32,
            a1,
            a2
        );
        let idb: u32 = if r4 == 0 {
            0xFF
        } else {
            let bank = *(obj.add(0x40) as *const u8) as u32;
            let tab = *global::<u32>(0x115D988);
            let row = *((bank
                .wrapping_mul(ROW_STRIDE)
                .wrapping_add(tab)
                .wrapping_add(ROW_BASE_OFF)) as *const u32);
            r4.wrapping_sub(row) / *global::<u32>(0x115D964) & 0xFF
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
        let entry = row2.wrapping_add(stride.wrapping_mul(idb));
        if entry == 0 {
            0
        } else {
            1
        }
    }
});
