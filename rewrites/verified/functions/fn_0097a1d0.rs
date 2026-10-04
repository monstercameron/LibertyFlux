// original: 0x0097a1d0 audio_dispatch_voice_update
/// Dispatches a record's voice update.
///
/// Validates the record's group and both slots through helpers, reconciles
/// an attached voice object against the helper answers (or allocates one),
/// runs the record through a setup call, then advances the voice state
/// machine (cases 1-3, each invoking the matching sibling updater) or
/// initializes it. Returns nothing.
export!(thiscall, rw_0097a1d0(this_ptr: u32, rec: u32) -> () {
    const CLOCK: u32 = 0x011735B4;

    unsafe {
        let grp = *(rec as *const u32).add(0xA0 / 4);
        let x = *((grp + 0x34) as *const u32);
        if x == 0 {
            return;
        }
        let a: u32 = callee_cdecl!(1, u32, x);
        if a == 0 {
            return;
        }
        let y = *((grp + 0x38) as *const u32);
        if y == 0 {
            return;
        }
        let b: u32 = callee_cdecl!(1, u32, y);
        if b == 0 {
            return;
        }
        // Allocates a fresh voice through the helper and links it.
        let alloc = || -> u32 {
            let s: u32 = callee_thiscall!(3, u32, this_ptr);
            if s == 0 {
                return 0;
            }
            *((grp + 0x4C) as *mut u32) = s;
            s
        };
        let mut esi = *((grp + 0x4C) as *const u32);
        if esi == 0 {
            esi = alloc();
            if esi == 0 {
                return;
            }
        } else {
            // Four match blocks; each calls the helper only when the first
            // comparison hits, and sets its flag only when both hit. All
            // four blocks always run; the combination below decides.
            let probe = |want: u32, slot: u32, arg: u32, check: u32| -> bool {
                if *((esi + slot) as *const u32) != want {
                    return false;
                }
                callee_thiscall!(2, u32, rec, arg) == *((esi + check) as *const u32)
            };
            let f1 = probe(a, 0x20, 0, 0x50);
            let f2 = probe(b, 0x24, 1, 0x54);
            let f3 = probe(b, 0x20, 1, 0x50);
            let f4 = probe(a, 0x24, 0, 0x54);
            if !(f1 && f2) && !(f3 && f4) {
                esi = alloc();
                if esi == 0 {
                    return;
                }
            }
        }
        let clock = *(global::<u32>(CLOCK));
        *((esi + 0x6C) as *mut u32) = clock;
        // Third argument re-pushes the caller's saved entry register; the
        // contract pins that register to zero and the value is compared.
        let r: u32 = callee_thiscall!(
            4,
            u32,
            this_ptr,
            esi,
            rec,
            0,
            &a as *const u32 as u32
        );
        if (r & 0xFF) == 0 {
            return;
        }
        // True when either link word decodes to the ready marker.
        let ready = |s: u32| -> bool {
            let p = *((s + 0x20) as *const u32);
            if p != 0 && (*((p + 0x28) as *const u32) & 0x3C0) == 0x80 {
                return true;
            }
            let q = *((s + 0x24) as *const u32);
            q != 0 && ((*((q + 0x28) as *const u32) & 0x3C0) == 0x80)
        };
        match *((esi + 0x60) as *const u32) {
            1 => {
                // The original passes whatever the previous call left in its
                // register here; the contract skips that register.
                callee_thiscall!(7, u32, this_ptr, esi);
                *((esi + 0x60) as *mut u32) = 2;
                let g2 = *(rec as *const u32).add(0xA0 / 4);
                if g2 == 0 {
                    return;
                }
                if *((g2 + 0x60) as *const u8) == 0 {
                    *((esi + 0x60) as *mut u32) = 3;
                }
            }
            2 => {
                callee_thiscall!(7, u32, this_ptr, esi);
                callee_thiscall!(9, u32, this_ptr, esi);
                *((esi + 0x64) as *mut u32) = clock.wrapping_add(0x64);
                if ready(esi) {
                    callee_thiscall!(6, u32, this_ptr, esi, &a as *const u32 as u32);
                }
                let g2 = *(rec as *const u32).add(0xA0 / 4);
                if g2 == 0 {
                    return;
                }
                if *((g2 + 0x60) as *const u8) != 0 {
                    return;
                }
                *((esi + 0x60) as *mut u32) = 3;
            }
            3 => {
                *((esi + 0x70) as *mut u8) = 1;
                callee_thiscall!(7, u32, this_ptr, esi);
                callee_thiscall!(8, u32, this_ptr, esi);
                *((esi + 0x68) as *mut u32) = clock.wrapping_add(0x64);
                if ready(esi) {
                    callee_thiscall!(6, u32, this_ptr, esi, &a as *const u32 as u32);
                }
                let g2 = *(rec as *const u32).add(0xA0 / 4);
                if g2 == 0 {
                    return;
                }
                if *((g2 + 0x60) as *const u8) == 0 {
                    return;
                }
                *((esi + 0x60) as *mut u32) = 2;
            }
            _ => {
                callee_thiscall!(5, u32, this_ptr, esi, 0);
                *((esi + 0x60) as *mut u32) = 1;
            }
        }
    }
});
