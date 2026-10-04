// original: 0x00d8f840 audio_entry_scan_best_fit
//! Single-function extract of the verified rewrite; needs `lf_k2_rt`.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Scan indexed entries, score them, and keep the best fit.
///
/// The main path walks `count` entries of a 40-byte table selected through an
/// index array, drops candidates that fail flag-gated bit tests or six
/// signed 16-bit range checks against a reference table, announces survivors
/// through callee 1, then refines each along a global float ladder: callee 2
/// answers a blend factor, the rewrite interpolates, measures the squared
/// distance to a target point and keeps the closest entry and distance in the
/// two output slots. Returns the entry count.
///
/// When the selector is null the tail path instead scans four sibling slots
/// and recurses (callee 3) into each whose float bounds overlap the published
/// window, returning the last value seen.
export!(thiscall, rw_00d8f840(this_ptr: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> u32 {
    unsafe {
        const FLAG0: u32 = 0x0179F940;
        const FLAG1: u32 = 0x0179F941;
        const LADDER: u32 = 0x0179F970;
        const G_D0: u32 = 0x0179FAD0;
        const G_D4: u32 = 0x0179FAD4;
        const G_E0: u32 = 0x0179FAE0;
        const G_E4: u32 = 0x0179FAE4;

        #[inline(always)]
        unsafe fn i16_at(addr: u32) -> i16 {
            *(addr as *const i16)
        }

        let sel = *((arg0 + 0x2c) as *const u32);
        if sel == 0 {
            // Tail: scan four sibling slots.
            let g_d0 = *(global::<f32>(G_D0) as *const f32);
            let g_d4 = *(global::<f32>(G_D4) as *const f32);
            let g_e0 = *(global::<f32>(G_E0) as *const f32);
            let g_e4 = *(global::<f32>(G_E4) as *const f32);
            let mut eax: u32 = 0;
            let mut k = 0u32;
            while k < 4 {
                let slot = *((arg0 + 0x30 + k * 4) as *const u32);
                eax = slot;
                if *((slot) as *const f32) > g_e0 {
                    k += 1;
                    continue;
                }
                if *((slot + 4) as *const f32) > g_e4 {
                    k += 1;
                    continue;
                }
                if g_d0 > *((slot + 0x10) as *const f32) {
                    k += 1;
                    continue;
                }
                if g_d4 > *((slot + 0x14) as *const f32) {
                    k += 1;
                    continue;
                }
                eax = callee_thiscall!(3, u32, this_ptr, slot, arg1, arg2, arg3, arg4);
                k += 1;
            }
            return eax;
        }
        // Main: scan count entries.
        let count = *((sel + 0x0c) as *const u16) as u32;
        if count == 0 {
            return 0;
        }
        let table = *((sel + 4) as *const u32);
        let entries = *((this_ptr + 0x6c) as *const u32);
        let words = *((this_ptr + 0x60) as *const u32);
        let flag0 = *(global::<u8>(FLAG0) as *const u8);
        let flag1 = *(global::<u8>(FLAG1) as *const u8);
        let mut i = 0u32;
        while i < count {
            let idx = *((table + i * 2) as *const u16) as u32;
            let entry = entries.wrapping_add(idx.wrapping_mul(5).wrapping_mul(8));
            if flag0 != 0 {
                if (*((entry + 4) as *const u32) & 0xE0000000) != 0 {
                    i += 1;
                    continue;
                }
                if (((*((entry) as *const u32)) >> 2) & 1) == 0 {
                    i += 1;
                    continue;
                }
            }
            if flag1 != 0 {
                if (((*((entry + 0x1c) as *const u8)) >> 7) & 1) != 0 {
                    i += 1;
                    continue;
                }
            }
            if i16_at(entry + 0x10) > i16_at(arg2 + 2) {
                i += 1;
                continue;
            }
            if i16_at(entry + 0x14) > i16_at(arg2 + 6) {
                i += 1;
                continue;
            }
            if i16_at(entry + 0x18) > i16_at(arg2 + 0x0a) {
                i += 1;
                continue;
            }
            if i16_at(entry + 0x12) < i16_at(arg2) {
                i += 1;
                continue;
            }
            if i16_at(entry + 0x16) < i16_at(arg2 + 4) {
                i += 1;
                continue;
            }
            if i16_at(entry + 0x1a) < i16_at(arg2 + 8) {
                i += 1;
                continue;
            }
            let e0 = *((entry) as *const u32);
            if (e0 & 0x1E00000) != 0 {
                let count1 = (e0 >> 21) & 0xF;
                let mut img = relocated(LADDER);
                let mut j = 0u32;
                while j < count1 {
                    let w = *((words
                        .wrapping_add(((*((entry + 4) as *const u32) & 0x1FFFF) + j) * 2))
                        as *const u16) as u32;
                    let _: u32 = callee_thiscall!(1, u32, this_ptr, w, img);
                    img = img.wrapping_add(0x10);
                    j += 1;
                }
            }
            let count2 = (e0 >> 21) & 0xF;
            if count2 >= 2 {
                let base = relocated(LADDER).wrapping_add((count2 - 1).wrapping_mul(16));
                let mut img = relocated(LADDER).wrapping_add(8);
                let mut j = 0u32;
                while j < count2 {
                    let f0 = *((img - 8) as *const f32) - *((base) as *const f32);
                    let f1 = *((img - 4) as *const f32) - *((base + 4) as *const f32);
                    let f2 = *((img) as *const f32) - *((base + 8) as *const f32);
                    let buf = [f0, f1, f2];
                    let t: f32 = callee_cdecl!(2, f32, base, buf.as_ptr() as u32, arg1);
                    let x = *((base) as *const f32) + f0 * t - *((arg1) as *const f32);
                    let y = *((base + 4) as *const f32) + f1 * t - *((arg1 + 4) as *const f32);
                    let z = f2 * t + *((base + 8) as *const f32) - *((arg1 + 8) as *const f32);
                    let sq = y * y + x * x + z * z;
                    if *((arg3) as *const f32) > sq {
                        *((arg3) as *mut f32) = sq;
                        *((arg4) as *mut u32) = entry;
                    }
                    img = img.wrapping_add(0x10);
                    j += 1;
                }
            }
            i += 1;
        }
        count
    }
});
