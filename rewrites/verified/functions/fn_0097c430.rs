// original: 0x0097C430 audio_event_tick
// 0x0097C430 audio_event_tick (proposed): thiscall/1, no result.
//
// Three global gates, then four verses of roll/describe/emit: two dice rolls,
// a handle-table lookup, a frame-object probe, and per verse a float draw, two
// id draws, an emit call through the object plus a follow-up that takes two
// frame pointers (or a release call when the emit declines). Verse 3 also
// runs only when a scratch word above the frame still holds the stack fill,
// and only after a four-deep handle chase. The incoming stack argument is
// only compared against a global at the verse-3 gate. Verse 3 then
// overwrites the arg slot with its float (like fn2's clobber), which is why
// the stack channel is off for this contract. Returns nothing (the original's
// exit EAX is incidental).
export!(thiscall, rw_97c430(this: u32, arg: u32) -> u32 {
    unsafe {
        // Gates: the body runs unless G1 holds 1, unless the two words
        // differ, and unless G3 holds 0x12 (all three jumps go to the shared
        // epilogue).
        if *global::<u32>(0x11F7060) == 1 {
            return 0;
        }
        if *global::<u32>(0x12088B4) != *global::<u32>(0xF1C040) {
            return 0;
        }
        if *global::<u32>(0x1037720) == 0x12 {
            return 0;
        }
        let sel = *((this as *const u32).add(0x178 / 4));
        let base = if sel == 3 {
            f32::from_bits(*global::<u32>(0x10389EC))
        } else if sel == 4 {
            0.0
        } else {
            f32::from_bits(*global::<u32>(0x10389F0))
        };
        let g1774 = *global::<u32>(0x1231774);
        let g89f4 = *global::<u32>(0x10389F4);
        let _roll1: u32 = callee_cdecl!(1, u32, g1774, g89f4);
        let _roll2: u32 = callee_cdecl!(1, u32, g1774, g89f4);
        let idx = *((this as *const u32).add(0x80 / 4));
        let ent = *global::<u32>(0x1231320 + idx.wrapping_mul(4));
        let mut first: u32 = 0;
        if ent != 0 {
            first = core::ptr::read_unaligned((ent as *const u8).add(0xa) as *const u32);
        }
        // Frame slots passed by address (the second always holds 0: the only
        // stores to it write 0 and the fill is 0).
        let mut slot44: u32 = 0;
        let slot50: u32 = 0;
        let slot44p = &mut slot44 as *mut u32 as u32;
        let slot50p = &slot50 as *const u32 as u32;
        let _: u32 = callee_thiscall!(2, u32, slot44p);
        let cfg = *((this as *const u32).add(0x120 / 4));
        // Verse 1.
        let f1: f32 = callee_cdecl!(3, f32, 0xBF80_0000, 0x3F80_0000);
        slot44 = (f1 + (base - 4.0)).to_bits();
        let h1: u32 = callee_cdecl!(6, u32,);
        let k1: u32 = callee_cdecl!(10, u32, h1);
        let a1: u32 = callee_thiscall!(14, u32, this, first, slot44p, h1, k1, 0);
        if (a1 as u8) != 0 {
            let _: u32 = callee_cdecl!(18, u32, first, 0, 0, 1, slot44p, slot50p, cfg, h1);
        } else {
            let _: u32 = callee_cdecl!(19, u32, h1);
        }
        // Verse 2.
        let g1474 = *global::<u32>(0x1231474);
        let f2: f32 = callee_cdecl!(4, f32, 0xBF80_0000, 0x3F80_0000);
        slot44 = (f2 + base).to_bits();
        let h2: u32 = callee_cdecl!(7, u32,);
        let k2: u32 = callee_cdecl!(11, u32, h2);
        let a2: u32 = callee_thiscall!(15, u32, this, g1474, slot44p, h2, k2, 0);
        if (a2 as u8) != 0 {
            let _: u32 = callee_cdecl!(18, u32, g1474, 0, 0, 1, slot44p, slot50p, cfg, h2);
        } else {
            let _: u32 = callee_cdecl!(19, u32, h2);
        }
        // Verse-3 gate: the original compares its incoming stack argument
        // against the global; equal takes the shared epilogue.
        if arg != *global::<u32>(0x1231544) {
            let f3: f32 = callee_cdecl!(5, f32, 0xBF80_0000, 0x3F80_0000);
            slot44 = (f3 + base).to_bits();
            let c0 = cfg;
            if c0 != 0 {
                let c1 = *((c0 as *const u32).add(0x2c4 / 4));
                if c1 != 0 {
                    let c2 = *((c1 as *const u32).add(0x25c / 4));
                    if c2 != 0 {
                        let c3 = *((c2 as *const u32).add(0x10 / 4));
                        if c3 != 0 {
                            let w26 = core::ptr::read_unaligned(
                                (c3 as *const u8).add(0x26) as *const u32,
                            );
                            let w2a = core::ptr::read_unaligned(
                                (c3 as *const u8).add(0x2a) as *const u32,
                            );
                            let h3: u32 = callee_cdecl!(8, u32,);
                            let k3: u32 = callee_cdecl!(12, u32, h3);
                            let a3: u32 =
                                callee_thiscall!(16, u32, this, w26, slot44p, h3, k3, 0);
                            if (a3 as u8) != 0 {
                                let _: u32 =
                                    callee_cdecl!(18, u32, w26, 0, 0, 1, slot44p, slot50p, c0, h3);
                            } else {
                                let _: u32 = callee_cdecl!(19, u32, h3);
                            }
                            let h4: u32 = callee_cdecl!(9, u32,);
                            let k4: u32 = callee_cdecl!(13, u32, h4);
                            let a4: u32 =
                                callee_thiscall!(17, u32, this, w2a, slot44p, h4, k4, 0);
                            if (a4 as u8) != 0 {
                                let _: u32 =
                                    callee_cdecl!(18, u32, w2a, 0, 0, 1, slot44p, slot50p, c0, h4);
                            } else {
                                let _: u32 = callee_cdecl!(19, u32, h4);
                            }
                        }
                    }
                }
            }
        }
        0
    }
});
