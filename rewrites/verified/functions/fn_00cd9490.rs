// original: 0x00cd9490 CTaskComplexFollowPedFootsteps::vf21

/// Event dispatcher with a position-triple request (thiscall, two stack words).
///
/// `this` is the task: `+0x14` the target record (null returns 0 at once),
/// `+0x20` the position record (null likewise), `+0x24` a signed word passed
/// on. `ev` is the event code; the second stack word is never read. Paths
/// fetch a worker through the manager getter; a null worker returns 0,
/// except on the two staged paths where it only zeroes the intermediate.
///
/// Dispatch on `ev`: `0x1F4` runs the bare worker request; `0xCB` the
/// four-word request (8.0, 0, 0, 0x2710); `0x384` the triple path below;
/// `0x38B` the tuning path below; anything else, including `0x516`,
/// returns 0.
///
/// The triple path copies the position record's three words at `+0x10`,
/// `+0x14`, `+0x18` into a frame triple, runs the probe request on the
/// position record with (0), then runs the five-word request (sign-extended
/// `[this+0x24]`, triple address, 0x3C23D70A, 0, 0). The triple address
/// legitimately differs between the two sides' frames, so the proof skips
/// that argument and snapshots the three words it points at instead. The
/// tuning path picks a constant by bit 2 of the byte at
/// `[this+0x14]+0x26C` (4.0 when set, 1.25 otherwise, both kept in the
/// image) and runs the seven-word setup request (target, 50000, 1000, the
/// constant bits, 2.0, 2.0, 1). Both staged paths finish with the four-word
/// request (staged answer, 0, 0, 0) on a second worker, returning 0 when it
/// is null and the answer otherwise.
///
/// Original: 0x00cd9490 (thiscall, two stack words, second ignored).
lf_checker_rt::export!(thiscall, rw_00cd9490(this: u32, ev: u32, _p1: u32) -> u32 {
    unsafe {
        const TARGET_OFF: u32 = 0x14;
        const POSREC_OFF: u32 = 0x20;
        const SWORD_OFF: u32 = 0x24;
        const TRIPLE0: u32 = 0x10;
        const TRIPLE1: u32 = 0x14;
        const TRIPLE2: u32 = 0x18;
        const FLAG_BYTE: u32 = 0x26c;
        const FLAG_BIT: u8 = 4;
        const EV_BARE: u32 = 0x1f4;
        const EV_SMALL: u32 = 0xcb;
        const EV_TRIPLE: u32 = 0x384;
        const EV_TUNE: u32 = 0x38b;
        const MANAGER: u32 = 0x0167e2a0;
        const CONST_SET: u32 = 0x00fe8ab8;
        const CONST_CLEAR: u32 = 0x00fe8920;
        const EIGHT: u32 = 0x4100_0000;
        const TWO_BITS: u32 = 0x4000_0000;
        const TRIPLE_MAGIC: u32 = 0x3c23_d70a;
        const CALLEE_GET: u32 = 1;
        const CALLEE_BARE: u32 = 2;
        const CALLEE_SMALL: u32 = 3;
        const CALLEE_PROBE: u32 = 4;
        const CALLEE_TRIPLE: u32 = 5;
        const CALLEE_SETUP: u32 = 6;
        const CALLEE_GET2: u32 = 7;
        const CALLEE_FINISH: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn worker() -> u32 {
            unsafe {
                let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
                lf_checker_rt::callee_thiscall!(CALLEE_GET, u32, mgr)
            }
        }
        /// Shared tail of the two staged paths.
        #[inline(always)]
        unsafe fn finish(staged: u32) -> u32 {
            unsafe {
                let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
                let g2: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GET2, u32, mgr);
                if g2 == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(CALLEE_FINISH, u32, g2, staged, 0u32, 0u32, 0u32)
            }
        }

        let target = rd32(this + TARGET_OFF);
        if target == 0 {
            return 0;
        }
        let posrec = rd32(this + POSREC_OFF);
        if posrec == 0 {
            return 0;
        }
        if ev == EV_TRIPLE {
            let f0 = rd32(posrec + TRIPLE0);
            let f1 = rd32(posrec + TRIPLE1);
            let f2 = rd32(posrec + TRIPLE2);
            let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_PROBE, u32, posrec, 0u32);
            let g = worker();
            if g == 0 {
                return finish(0);
            }
            let triple = [f0, f1, f2];
            let sword = rd32(this + SWORD_OFF) as u16 as i16 as i32 as u32;
            let staged: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_TRIPLE, u32, g, sword, &triple as *const u32 as u32, TRIPLE_MAGIC, 0u32,
                0u32
            );
            // Keep the triple alive across the call.
            core::hint::black_box(&triple);
            return finish(staged);
        }
        if ev == EV_BARE {
            let g = worker();
            if g == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(CALLEE_BARE, u32, g);
        }
        if ev == EV_SMALL {
            let g = worker();
            if g == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(
                CALLEE_SMALL, u32, g, 0x2710u32, 0u32, 0u32, EIGHT
            );
        }
        if ev != EV_TUNE {
            return 0;
        }
        let flag = ((target + FLAG_BYTE) as *const u8).read();
        let addr = if flag & FLAG_BIT != 0 { CONST_SET } else { CONST_CLEAR };
        let tune = lf_checker_rt::global::<u32>(addr).read();
        let g = worker();
        if g == 0 {
            return finish(0);
        }
        let staged: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_SETUP, u32, g, target, 50000u32, 1000u32, tune, TWO_BITS, TWO_BITS, 1u32
        );
        finish(staged)
    }
});
