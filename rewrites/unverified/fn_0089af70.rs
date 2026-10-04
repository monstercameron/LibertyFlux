// original: 0x0089AF70 audio_envelope_tick
//! Envelope tick: samples five control words from a sampler callee, advances
//! a staged envelope (select/arm/ramp/hold/release by the stage byte and the
//! incoming level against stored thresholds), evaluates a curve callee for
//! the ramped stages, and reports whether the envelope went quiet.

use lf_k2_rt::{callee_thiscall, export, global};

const ROUTE_STRIDE: u32 = 0x6F40;

export!(thiscall, rw_0089AF70(this: u32, arg0: u32) -> u32 {
    unsafe {
        let t = this as *mut u8;
        let stride = *global::<u32>(0x115D968);
        let table = *global::<u32>(0x115D988);
        let tick = *global::<f32>(0xFE870C);
        let one = 1.0f32;
        let ri = |off: usize| -> i32 { *((t as *const u8).add(off) as *const i32) };
        let wf = |off: usize, v: f32| { *((t as *mut u8).add(off) as *mut f32) = v };

        // Five sample slots; the sampler takes frame pointers in reverse
        // slot order (arg0->slot4 ... arg4->slot1).
        let mut s = [0u32; 5];
        let sb = s.as_mut_ptr() as u32;
        callee_thiscall!(1, u32, this, sb + 16, sb + 12, sb + 8, sb, sb + 4);
        let gate = s[0] as i32;
        let step = s[1] as i32;
        let span = s[2];
        let level = arg0 as i32;

        if gate >= 0 && *t.add(0xF6) != 4 {
            let start = ri(0xC8);
            if level > start {
                *(t.add(0xCC) as *mut i32) = start;
                *(t.add(0xD0) as *mut i32) = start.wrapping_add(step);
                *t.add(0xF6) = 4;
                wf(0xB0, one);
                wf(0xB8, one);
                wf(0xB4, span as f32 * tick);
            }
        }

        let row = table.wrapping_add((*t.add(0x40) as u32).wrapping_mul(ROUTE_STRIDE));
        let entry = (*t.add(0xF7) as u32)
            .wrapping_mul(stride)
            .wrapping_add(*((row + 0x6F14) as *const u32));
        match *t.add(0xF6) {
            4 => {
                if step < 0 {
                    return 1;
                }
                let end = ri(0xD0);
                if level > end {
                    *t.add(0xF8) |= 4;
                    return 0;
                }
                let r: f32 = callee_thiscall!(
                    2, f32, entry.wrapping_add(0x50),
                    0, one.to_bits(),
                    (ri(0xCC) as f32).to_bits(),
                    (end as f32).to_bits(),
                    (level as f32).to_bits()
                );
                wf(0xB8, r);
                1
            }
            5 => {
                wf(0xB0, one);
                wf(0xB4, one);
                *((t as *mut u8).add(0xB8) as *mut u32) = 0;
                1
            }
            _ => {
                let top = ri(0xC4);
                if level >= top {
                    *t.add(0xF6) = 3;
                    wf(0xB0, one);
                    wf(0xB4, span as f32 * tick);
                } else {
                    let base = ri(0xBC);
                    if level < base {
                        *t.add(0xF6) = 0;
                        *((t as *mut u8).add(0xB0) as *mut u32) = 0;
                        wf(0xB4, one);
                    } else {
                        let knee = ri(0xC0);
                        if level < knee {
                            *t.add(0xF6) = 1;
                            let r: f32 = callee_thiscall!(
                                2, f32, entry,
                                0, one.to_bits(),
                                (base as f32).to_bits(),
                                (knee as f32).to_bits(),
                                (level as f32).to_bits()
                            );
                            wf(0xB4, one);
                            wf(0xB0, r);
                        } else {
                            *t.add(0xF6) = 2;
                            wf(0xB0, one);
                            let r: f32 = callee_thiscall!(
                                2, f32, entry.wrapping_add(0x28),
                                (span as f32 * tick).to_bits(), one.to_bits(),
                                (knee as f32).to_bits(),
                                (top as f32).to_bits(),
                                (level as f32).to_bits()
                            );
                            wf(0xB4, r);
                        }
                    }
                }
                wf(0xB8, one);
                1
            }
        }
    }
});
