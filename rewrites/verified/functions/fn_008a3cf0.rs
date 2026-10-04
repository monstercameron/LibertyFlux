// original: 0x008a3cf0 audio_indexed_sound_trigger
//! Indexed sound trigger: derive a slot index from a scaled float, then invoke
//! the slot hook for small tables or the bank hook for large ones. The
//! original's SSE rounding lane computes floor(x) (a 2^23 round-to-nearest
//! step plus a not-less-or-equal fixup; checked over signs, zeros, boundary
//! magnitudes and NaN/infinity) and converts with x87 chop, which yields low
//! dword 0 whenever the value is NaN or out of i64 range.

use lf_k2_rt::{callee_thiscall, export, global, relocated};

export!(thiscall, rw_008a3cf0(obj: *mut u8, _arg: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6F40;
        const ROW_BASE_OFF: u32 = 0x6F10;
        let flag = *(obj.add(0xF3) as *const u8);
        let fptr = *(obj.add(0xC8) as *const u32);
        let count = *(obj.add(0xCC) as *const u32);
        let x: f32 = if flag != 0 {
            let raw = f32::from_bits(*(fptr as *const u32));
            let clamped = if raw < 0.0 {
                0.0
            } else if raw > 1.0 {
                1.0
            } else {
                raw
            };
            (count as f32) * clamped
        } else {
            f32::from_bits(*(fptr as *const u32))
        };
        let t = x.floor();
        let mut edx: u32 = if t.is_nan() || t >= 9223372036854775808.0 || t <= -9223372036854775808.0
        {
            0
        } else {
            (t.trunc() as i64) as u32
        };
        if flag != 0 && edx >= count {
            edx = count.wrapping_sub(1);
        }
        if edx >= count {
            return count;
        }
        if count <= 8 {
            let slot = *(obj.add(0xD0).add(edx as usize * 4) as *const u32);
            let dest = obj.add(0xB0) as u32;
            let r: u32 = callee_thiscall!(1, u32, obj as u32, slot, 0, dest);
            *(obj.add(0xF2) as *mut u8) = 1;
            r
        } else {
            let bank = *(obj.add(0x40) as *const u8) as u32;
            let tab = *global::<u32>(0x115D988);
            let row = *((bank
                .wrapping_mul(ROW_STRIDE)
                .wrapping_add(tab)
                .wrapping_add(ROW_BASE_OFF)) as *const u32);
            let f0 = *(obj.add(0xF0) as *const u8) as u32;
            let stride = *global::<u32>(0x115D964);
            let esi = row.wrapping_add(stride.wrapping_mul(f0));
            let slot = *((esi.wrapping_add(edx.wrapping_mul(4))) as *const u32);
            let dest = obj.add(0xB0) as u32;
            let _: u32 = callee_thiscall!(1, u32, obj as u32, slot, 0, dest);
            let r2: u32 =
                callee_thiscall!(2, u32, relocated(0x115D8A0), esi, bank);
            *(obj.add(0xF0) as *mut u8) = 0xFF;
            *(obj.add(0xF2) as *mut u8) = 1;
            r2
        }
    }
});
