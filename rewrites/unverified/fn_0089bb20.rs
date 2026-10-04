// original: 0x0089BB20 audio_twin_voice_mix
//! Twin voice mix: requires at least one of two voice slots live, folds a
//! level through a limiter callee, shapes two mix weights (directly, or via
//! a pair of curve callees fed through vector registers), runs each live
//! voice through configure/poll callees, and reports whether either poll
//! accepted.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global};

const ROUTE_STRIDE: u32 = 0x6F40;
const NO_VOICE: u8 = 0xFF;
const LIVE_TAG: u16 = 2;

export!(thiscall, rw_0089BB20(this: u32, arg0: u32) -> u32 {
    unsafe {
        let t = this as *const u8;
        let stride = *global::<u32>(0x115D964);
        let table = *global::<u32>(0x115D988);
        let half_pi = *global::<f32>(0xFE8978);
        let one = *global::<f32>(0xFE88E8);
        let w = |off: usize| -> u32 { *((t as *const u8).add(off) as *const u32) };

        let voice = |slot: u8| -> u32 {
            if slot == NO_VOICE { 0 } else {
            let row = table
                .wrapping_add((*t.add(0x40) as u32).wrapping_mul(ROUTE_STRIDE))
                .wrapping_add(0x6F10);
            stride
                .wrapping_mul(slot as u32)
                .wrapping_add(*(row as *const u32))
            }
        };
        // Tag reads land on even bytes (offsets 0/4/8 + 6), so u16 is aligned.
        let live = |v: u32| -> bool { v != 0 && *((v + 6) as *const u16) == LIVE_TAG };

        if !live(voice(*t.add(0x48))) && !live(voice(*t.add(0x49))) {
            return 0;
        }

        if arg0 > w(0xB0) {
            callee_thiscall!(1, u32, this, arg0);
        }
        let m: f32 = callee_thiscall!(
            2, f32, this.wrapping_add(0xDC),
            (t.add(0xD4) as *const f32).read_unaligned().to_bits(), arg0
        );

        // Curve pair, or a direct split; the two shaped weights each feed a
        // limiter callee whose answers go to the voice blocks below.
        let direct = w(0xD8) != 0;
        let (x1, x0) = if direct {
            (m, one - m)
        } else {
            let ra = f32::from_bits(callee_cdecl!(3, u32, (m * half_pi).to_bits()));
            let rb = f32::from_bits(callee_cdecl!(4, u32, ((one - m) * half_pi).to_bits()));
            (ra, rb)
        };
        let s14: f32 = callee_cdecl!(5, f32, x1.to_bits());
        let s18: f32 = callee_cdecl!(6, f32, x0.to_bits());

        let mut flag_a = 0u8;
        if live(voice(*t.add(0x48))) {
            let c1: u32 = callee_thiscall!(7, u32, this, 0);
            callee_thiscall!(8, u32, c1, s14.to_bits());
            let c2: u32 = callee_thiscall!(7, u32, this, 0);
            flag_a = callee_thiscall!(9, u32, c2, arg0) as u8;
        }
        let al_out = if live(voice(*t.add(0x49))) {
            let d1: u32 = callee_thiscall!(7, u32, this, 1);
            callee_thiscall!(8, u32, d1, s18.to_bits());
            let d2: u32 = callee_thiscall!(7, u32, this, 1);
            callee_thiscall!(10, u32, d2, arg0) as u8
        } else {
            0
        };
        if flag_a != 0 || al_out != 0 {
            1
        } else {
            0
        }
    }
});
