// original: 0x00cb23e0 CTaskComplexGoToAttractor::vf19

/// Build the go-to-attractor subtask down one of two paths.
///
/// `this` is the complex task: helper object at `+0x14`, lane point at
/// `+0x20`, speed at `+0x30`, fallback order at `+0x3c`. `ped` is the ped.
/// Callee 5 is the helper's virtual slot 0, callee 7 the ped's virtual
/// slot 2, callee 6 orders the seven-word attractor probe, callees 8 and 9
/// set the two speeds, callee 10 builds the fast subtask, callee 11 opens
/// the steady handle, callee 12 links it, callee 13 orders the ten-word
/// steady variant, callee 14 attaches, callee 15 orders the three-word
/// steady variant, and callees 1-4 and 16 fetch the behaviour object for
/// the global at `0x167e2a0` at the five fetch sites (one scripted answer
/// per site so every null/non-null combination is exercised).
///
/// Behaviour: slot 0 of the helper decides the fallback order (3 for answer
/// 4, else the stored one). When the ped's flag byte at `+0x26c` has bit 6
/// set, the probe is ordered with (lane, 0, 0, 0, 100.0, 14, 0), slot 2 of
/// the ped runs with (lane, 0, 0), both speeds are set from `+0x30`, and a
/// fetched object builds the fast subtask with (0, 0, 0, 8.0), whose answer
/// is returned (a null fetch returns 0). Otherwise the steady handle is
/// opened (or null), the link object is fetched (null skips both link
/// calls with a zero handle), the steady variant is ordered unless its
/// fetch failed with (order, lane, 0.5, 3.0, -1, 1, 0, 0, 0, 1), the link
/// and attach calls run, and a final fetch either attaches zero or orders
/// the short variant with (lane, speed, 0.5) and attaches its answer. The
/// steady handle is returned.
///
/// Original: 0x00cb23e0 (thiscall, one stack word; callee 6 is cdecl with
/// seven words, everything else is thiscall: fetches and slot 0 with none,
/// slot 2 with three, speeds and attach with one, fast and link with four,
/// steady with ten, short with three).
lf_checker_rt::export!(thiscall, rw_00cb23e0(this: u32, ped: u32) -> u32 {
    unsafe {
        const HELPER: u32 = 0x14;
        const LANE: u32 = 0x20;
        const SPEED: u32 = 0x30;
        const FALLBACK: u32 = 0x3c;
        const PED_FLAG: u32 = 0x26c;
        const GLOBAL_STATE: u32 = 0x167e2a0;
        const PROBE_DIST: f32 = f32::from_bits(0x42c8_0000); // 100.0
        const FAST_W: f32 = f32::from_bits(0x4100_0000); // 8.0
        const HALF: f32 = 0.5;
        const THREE: f32 = 3.0;
        const CALLEE_FHIGH: u32 = 1;
        const CALLEE_FE0: u32 = 2;
        const CALLEE_FEBX: u32 = 3;
        const CALLEE_FD10: u32 = 4;
        const CALLEE_V0: u32 = 5;
        const CALLEE_PROBE: u32 = 6;
        const CALLEE_V8: u32 = 7;
        const CALLEE_SPD1: u32 = 8;
        const CALLEE_SPD2: u32 = 9;
        const CALLEE_FAST: u32 = 10;
        const CALLEE_OPEN: u32 = 11;
        const CALLEE_LINK: u32 = 12;
        const CALLEE_STEADY: u32 = 13;
        const CALLEE_ATTACH: u32 = 14;
        const CALLEE_SHORT: u32 = 15;
        const CALLEE_FFIN: u32 = 16;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn glob() -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(GLOBAL_STATE) as *const u32).read_unaligned() }
        }

        let helper = rd32(this + HELPER);
        let slot0: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(helper)) as usize);
        let order = if slot0(helper) == 4 { 3 } else { rd32(this + FALLBACK) };
        let lane = this.wrapping_add(LANE);
        if rd8(ped + PED_FLAG) & 0x40 != 0 {
            lf_checker_rt::callee_cdecl!(
                CALLEE_PROBE, u32, lane, 0, 0, 0, PROBE_DIST.to_bits(), 0x0e, 0
            );
            let slot2: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(ped) + 8) as usize);
            slot2(ped, lane, 0, 0);
            let speed = rd32(this + SPEED);
            lf_checker_rt::callee_thiscall!(CALLEE_SPD1, u32, ped, speed);
            lf_checker_rt::callee_thiscall!(CALLEE_SPD2, u32, ped, speed);
            let f = lf_checker_rt::callee_thiscall!(CALLEE_FHIGH, u32, glob());
            if f == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(CALLEE_FAST, u32, f, 0, 0, 0, FAST_W.to_bits());
        }
        let f = lf_checker_rt::callee_thiscall!(CALLEE_FE0, u32, glob());
        let handle = if f == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(CALLEE_OPEN, u32, f)
        };
        let link = lf_checker_rt::callee_thiscall!(CALLEE_FEBX, u32, glob());
        let attached = if link == 0 {
            lf_checker_rt::callee_thiscall!(CALLEE_ATTACH, u32, handle, 0)
        } else {
            let g = lf_checker_rt::callee_thiscall!(CALLEE_FD10, u32, glob());
            let d = if g == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    CALLEE_STEADY, u32, g, order, lane,
                    HALF.to_bits(), THREE.to_bits(),
                    0xffff_ffff, 1, 0, 0, 0, 1
                )
            };
            let e = lf_checker_rt::callee_thiscall!(CALLEE_LINK, u32, link, d, 0, 0, 0);
            lf_checker_rt::callee_thiscall!(CALLEE_ATTACH, u32, handle, e)
        };
        let _ = attached;
        let fin = lf_checker_rt::callee_thiscall!(CALLEE_FFIN, u32, glob());
        if fin == 0 {
            lf_checker_rt::callee_thiscall!(CALLEE_ATTACH, u32, handle, 0);
        } else {
            let s = lf_checker_rt::callee_thiscall!(
                CALLEE_SHORT, u32, fin, lane, rd32(this + SPEED), HALF.to_bits()
            );
            lf_checker_rt::callee_thiscall!(CALLEE_ATTACH, u32, handle, s);
        }
        handle
    }
});
