// original: 0x00cb2580 CTaskComplexGoToPointShooting::vf19

/// Build the go-to-point-shooting subtask from a fetched behaviour object.
///
/// `this` is the complex task: vtable at `+0`, mode word at `+0x14`,
/// sub-order at `+0x18`, anchor at `+0x20`, point at `+0x30`, speeds at
/// `+0x40`/`+0x44`, shooting flag byte at `+0x48`. The function's one stack
/// word is ignored. Callee 6 orders the ten-word shooting variant, callee 7
/// the six-word plain variant, callee 8 the eight-word aim variant, callee 9
/// is the object's own virtual slot 3, callee 10 finishes the task, and
/// callees 1-5 fetch the behaviour object for the global at `0x167e2a0` at
/// the five fetch sites (one scripted answer per site so every
/// null/non-null combination is exercised).
///
/// Behaviour: when the shooting flag is set, the object is fetched (site 1)
/// and, unless null, the shooting variant is ordered with (mode, anchor,
/// speeds, -1, 1, 0, 0, 0, 1); otherwise the object is fetched (site 2)
/// and the plain variant with (mode, anchor, speeds, 0, 0). Either answer,
/// or null when its fetch failed, is kept as the first handle. Virtual
/// slot 3 then selects the aim order, 5 for answer `0x385` and 2 otherwise;
/// the object is fetched again (sites 3/4) and, unless null, the aim
/// variant is ordered with (order, sub-order, anchor2, 608.0, 0, 1, 1,
/// 1.0), kept as the second handle (or null). A final fetch (site 5)
/// returning null returns 0; otherwise the finisher is ordered with (first,
/// second, 0, 0) and its answer is returned.
///
/// Original: 0x00cb2580 (thiscall, one ignored stack word; all callees are
/// thiscall: fetches with none, variants with ten, six, eight and four
/// words, the virtual slot with none).
lf_checker_rt::export!(thiscall, rw_00cb2580(this: u32, _arg: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x14;
        const SUB: u32 = 0x18;
        const ANCHOR2: u32 = 0x20;
        const ANCHOR: u32 = 0x30;
        const SPD0: u32 = 0x40;
        const SPD1: u32 = 0x44;
        const SHOOT: u32 = 0x48;
        const GLOBAL_STATE: u32 = 0x167e2a0;
        const AIM_ORDER_HIT: u32 = 5;
        const AIM_ORDER_MISS: u32 = 2;
        const AIM_MAGIC: f32 = f32::from_bits(0x4416_0000); // 608.0
        const ONE_F: f32 = 1.0;
        const CALLEE_F1: u32 = 1;
        const CALLEE_F2: u32 = 2;
        const CALLEE_F3: u32 = 3;
        const CALLEE_F4: u32 = 4;
        const CALLEE_F5: u32 = 5;
        const CALLEE_SHOOT: u32 = 6;
        const CALLEE_PLAIN: u32 = 7;
        const CALLEE_AIM: u32 = 8;
        const CALLEE_VT: u32 = 9;
        const CALLEE_FIN: u32 = 10;

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

        let first = if rd8(this + SHOOT) != 0 {
            let a = lf_checker_rt::callee_thiscall!(CALLEE_F1, u32, glob());
            if a == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    CALLEE_SHOOT, u32, a, rd32(this + MODE),
                    this.wrapping_add(ANCHOR), rd32(this + SPD0), rd32(this + SPD1),
                    0xffff_ffff, 1, 0, 0, 0, 1
                )
            }
        } else {
            let a = lf_checker_rt::callee_thiscall!(CALLEE_F2, u32, glob());
            if a == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    CALLEE_PLAIN, u32, a, rd32(this + MODE),
                    this.wrapping_add(ANCHOR), rd32(this + SPD0), rd32(this + SPD1),
                    0, 0
                )
            }
        };
        let slot: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(this) + 0x0c) as usize);
        let v = slot(this);
        let second = if v == 0x385 {
            let a = lf_checker_rt::callee_thiscall!(CALLEE_F3, u32, glob());
            if a == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    CALLEE_AIM, u32, a, AIM_ORDER_HIT, rd32(this + SUB),
                    this.wrapping_add(ANCHOR2), AIM_MAGIC.to_bits(), 0, 1, 1,
                    ONE_F.to_bits()
                )
            }
        } else {
            let a = lf_checker_rt::callee_thiscall!(CALLEE_F4, u32, glob());
            if a == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    CALLEE_AIM, u32, a, AIM_ORDER_MISS, rd32(this + SUB),
                    this.wrapping_add(ANCHOR2), AIM_MAGIC.to_bits(), 0, 1, 1,
                    ONE_F.to_bits()
                )
            }
        };
        let b = lf_checker_rt::callee_thiscall!(CALLEE_F5, u32, glob());
        if b == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CALLEE_FIN, u32, b, first, second, 0, 0)
    }
});
