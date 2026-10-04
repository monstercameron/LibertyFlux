// original: 0x0089C3E0 audio_voice_attach
//! Voice attach: resolves a route entry and a voice slot from the audio
//! tables, validates the entry tag, polls the entry, then either returns the
//! poll verdict directly or dispatches on it: allocate and configure a new
//! voice, report success, report failure, or run a flagged re-poll.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global};

const ROUTE_STRIDE: u32 = 0x6F40;
const NO_VOICE: u8 = 0xFF;
const BAD_TAG: u32 = 0xFFFF;

export!(thiscall, rw_0089C3E0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let t = this as *const u8;
        let stride1 = *global::<u32>(0x115D964);
        let stride2 = *global::<u32>(0x115D968);
        let table = *global::<u32>(0x115D988);

        let probe: u32 = callee_cdecl!(1, u32, arg0);
        let row = table.wrapping_add((*t.add(0x40) as u32).wrapping_mul(ROUTE_STRIDE));
        let entry = stride2
            .wrapping_mul(*t.add(0xB4) as u32)
            .wrapping_add(*((row + 0x6F14) as *const u32));
        if entry == 0 {
            return 2;
        }
        if probe == 0xFFFF_FFFF {
            return 2;
        }
        let slot = *t.add(0x48);
        let voice = if slot == NO_VOICE {
            0
        } else {
            stride1
                .wrapping_mul(slot as u32)
                .wrapping_add(*((row + 0x6F10) as *const u32))
        };
        let head = *(entry as *const u32);
        if *((entry + 4) as *const u32) == BAD_TAG {
            return 2;
        }
        if voice == 0 {
            return 2;
        }
        let tag = *((entry + 4) as *const u16) as u32;
        let poll: u32 = callee_thiscall!(2, u32, arg0, tag, head);
        if arg1 & 0xFF != 0 {
            if poll == 0 {
                return 1;
            }
            if poll == 2 {
                return 2;
            }
            return 0;
        }
        *((this as *mut u8).add(0xB0) as *mut u32) = probe;
        callee_thiscall!(3, u32, voice, head, probe);
        match poll {
            0 => {
                let inst: u32 = callee_cdecl!(4, u32, arg0, head);
                if inst == 0 {
                    return 2;
                }
                let cfg: u32 = callee_thiscall!(5, u32, this, 0);
                *((cfg + 0xE4) as *mut u16) = *((inst + 0x1A) as *const u16);
                let lo = *((inst + 0x18) as *const u16) as u32;
                let pick: u32 = callee_cdecl!(6, u32, *((inst + 0x10) as *const u32), lo);
                *((cfg + 0xE0) as *mut u32) = pick;
                let flag = (cfg + 0xEE) as *mut u8;
                if *((inst + 0x14) as *const i32) >= 0 {
                    *flag |= 4;
                } else {
                    *flag &= !4;
                }
                1
            }
            1 => 0,
            2 => 2,
            3 => {
                if *t.add(0x39) & 0x20 == 0 {
                    return 2;
                }
                let tag = *((entry + 4) as *const u16) as u32;
                callee_thiscall!(7, u32, arg0, tag, head);
                0
            }
            _ => 2,
        }
    }
});
