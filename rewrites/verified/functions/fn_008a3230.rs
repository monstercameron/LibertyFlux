// original: 0x008a3230 audio_bank_entry_applier
//! Audio bank entry applier: walks the entry-id bytes at obj+0x48 while the
//! index is below the count at obj+0xB0, resolving each id through the audio
//! table globals and invoking two hooks per live entry.
//!
//! Returns 2 early when the second hook answers 2, else 1 when every hook
//! answer was nonzero (or no entry was live), else 0.

use lf_k2_rt::{callee_thiscall, export, global};

export!(thiscall, rw_008a3230(obj: *mut u8, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6F40;
        const ROW_BASE_OFF: u32 = 0x6F10;
        let count = *(obj.add(0xB0) as *const u32);
        if count == 0 {
            return 1;
        }
        let stride0 = *global::<u32>(0x115D964);
        let mut entry_ptr = stride0;
        let mut ok: u32 = 1;
        let mut ecx_reg: u32 = 1;
        let mut i: u32 = 0;
        while i < count {
            let id = *(obj.add(0x48 + i as usize) as *const u8) as u32;
            if id == 0xFF {
                i += 1;
                continue;
            }
            let bank = *(obj.add(0x40) as *const u8) as u32;
            let tab = *global::<u32>(0x115D988);
            let row = *((bank
                .wrapping_mul(ROW_STRIDE)
                .wrapping_add(tab)
                .wrapping_add(ROW_BASE_OFF)) as *const u32);
            let entry = entry_ptr.wrapping_mul(id).wrapping_add(row);
            if entry == 0 {
                ecx_reg = ok;
                i += 1;
                continue;
            }
            entry_ptr = entry;
            let param = *(obj.add(0x54) as *const u32);
            let _: u32 = callee_thiscall!(1, u32, entry_ptr, param, 0);
            let bit = ((*(obj.add(0x39) as *const u8) >> 5) & 1) as u32;
            let stride = *global::<u32>(0x115D964);
            let id2 = *(obj.add(0x48 + i as usize) as *const u8) as u32;
            let hook_this = if id2 == 0xFF {
                0
            } else {
                let bank2 = *(obj.add(0x40) as *const u8) as u32;
                let tab2 = *global::<u32>(0x115D988);
                let row2 = *((bank2
                    .wrapping_mul(ROW_STRIDE)
                    .wrapping_add(tab2)
                    .wrapping_add(ROW_BASE_OFF)) as *const u32);
                stride.wrapping_mul(id2).wrapping_add(row2)
            };
            let r: u32 = callee_thiscall!(2, u32, hook_this, arg0, bit, arg1);
            if r == 2 {
                return 2;
            }
            ecx_reg = if r == 0 { 0 } else { ok & 0xFF };
            ok = ecx_reg;
            entry_ptr = *global::<u32>(0x115D964);
            i += 1;
        }
        if (ecx_reg & 0xFF) != 0 {
            1
        } else {
            0
        }
    }
});
