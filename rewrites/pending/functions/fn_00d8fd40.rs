// original: 0x00d8fd40 audio_entry_walk_score_store
//! Single-function extract of the verified rewrite; needs `lf_k2_rt`.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global};

/// Walk entries, score candidates through a callback, and store the best.
///
/// For each of `count` 40-byte entries, drops candidates that fail six
/// signed 16-bit range checks, offers survivors to a caller-supplied callback
/// (callee 4, reached through the stub address passed as an argument),
/// announces them through callee 1, then asks callee 2 how many ladder rungs
/// to score: each rung measures a squared distance (optionally pre-scaling
/// one axis by a shared constant when the flag bit is set) and keeps the
/// closest point and distance in the output struct, consulting callee 3
/// before overwriting a locked slot. Returns `count * 40`.
///
/// The contract pins `count` to 1..3: with `count == 0` the original returns
/// its caller's entry EAX without touching anything else, which no rewrite
/// can reproduce and no check can verify.
export!(thiscall, rw_00d8fd40(this_ptr: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe {
        const ZSCALE_ADDR: u32 = 0x00FE8A24; // shared axis-scale constant

        #[inline(always)]
        unsafe fn i16_at(addr: u32) -> i16 {
            *(addr as *const i16)
        }

        let count = *((this_ptr + 0x7c) as *const u32);
        let entries = *((this_ptr + 0x6c) as *const u32);
        let words = *((this_ptr + 0x60) as *const u32);
        let t64 = *((this_ptr + 0x64) as *const u32);
        let mut i = 0u32;
        let mut off = 0u32;
        while i < count {
            let entry = entries.wrapping_add(off);
            if i16_at(entry + 0x10) > i16_at(arg0 + 2) {
                i += 1;
                off += 0x28;
                continue;
            }
            if i16_at(entry + 0x14) > i16_at(arg0 + 6) {
                i += 1;
                off += 0x28;
                continue;
            }
            if i16_at(entry + 0x18) > i16_at(arg0 + 0x0a) {
                i += 1;
                off += 0x28;
                continue;
            }
            if i16_at(entry + 0x12) < i16_at(arg0) {
                i += 1;
                off += 0x28;
                continue;
            }
            if i16_at(entry + 0x16) < i16_at(arg0 + 4) {
                i += 1;
                off += 0x28;
                continue;
            }
            if i16_at(entry + 0x1a) < i16_at(arg0 + 8) {
                i += 1;
                off += 0x28;
                continue;
            }
            if arg1 != 0 {
                // Indirect call through the caller's function pointer. Both
                // sides reach the same recorder stub: the contract passes its
                // address as this argument.
                let cb: extern "cdecl" fn(u32, u32) -> u32 =
                    core::mem::transmute(arg1 as usize);
                if (cb(this_ptr, entry) & 0xFF) == 0 {
                    i += 1;
                    off += 0x28;
                    continue;
                }
            }
            let e0 = *((entry) as *const u32);
            let e4 = *((entry + 4) as *const u32);
            if (e0 & 0x1E00000) != 0 {
                let count1 = (e0 >> 21) & 0xF;
                let mut img = arg3.wrapping_add(0x40);
                let mut j = 0u32;
                while j < count1 {
                    let w =
                        *((words.wrapping_add(((e4 & 0x1FFFF) + j) * 2)) as *const u16) as u32;
                    let _: u32 = callee_thiscall!(1, u32, this_ptr, w, img);
                    img = img.wrapping_add(0x10);
                    j += 1;
                }
            }
            let bit = (e0 >> 1) & 1;
            let idx2 = e4 & 0x1FFFF;
            let target = t64.wrapping_add(idx2.wrapping_mul(8));
            let n: u32 = callee_cdecl!(
                2,
                u32,
                entry,
                arg3.wrapping_add(0x40),
                target,
                0,
                bit,
                arg3.wrapping_add(0x140)
            );
            if n != 0 {
                let scale_z = (arg2 & 0x80) != 0;
                let zc = *(global::<f32>(ZSCALE_ADDR) as *const f32);
                let mut rung = arg3.wrapping_add(0x148);
                let mut c = n;
                while c != 0 {
                    let f1 = *((arg3 + 0x20) as *const f32) - *((rung - 8) as *const f32);
                    let f2 = *((arg3 + 0x24) as *const f32) - *((rung - 4) as *const f32);
                    let mut f0 = *((arg3 + 0x28) as *const f32) - *((rung) as *const f32);
                    if scale_z {
                        f0 = f0 * zc;
                    }
                    let sq = f1 * f1 + f2 * f2 + f0 * f0;
                    if *((arg3 + 0x554) as *const f32) > sq
                        && *((arg3 + 0x550) as *const f32) > sq
                    {
                        let mut store = true;
                        if *((arg3 + 0x560) as *const u32) != 0 {
                            let ok: u32 = callee_cdecl!(3, u32, rung - 8, arg3);
                            if (ok & 0xFF) == 0 {
                                store = false;
                            }
                        }
                        if store {
                            *((arg3 + 0x550) as *mut f32) = sq;
                            *((arg3 + 0x30) as *mut u32) = *((rung - 8) as *const u32);
                            *((arg3 + 0x34) as *mut f32) = *((rung - 4) as *const f32);
                            *((arg3 + 0x38) as *mut f32) = *((rung) as *const f32);
                            *((arg3 + 0x3c) as *mut u32) = *((rung + 4) as *const u32);
                            *((arg3 + 0x558) as *mut u32) = i;
                        }
                    }
                    rung = rung.wrapping_add(0x10);
                    c -= 1;
                }
            }
            i += 1;
            off += 0x28;
        }
        count.wrapping_mul(0x28)
    }
});
