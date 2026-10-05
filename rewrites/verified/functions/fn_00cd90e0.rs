// original: 0x00cd90e0 task_event_dispatch (proposed)

/// Event dispatcher for a ped task (thiscall, two stack words).
///
/// `this` is the task object (`+0x14` and `+0x18` hold words passed to some
/// requests), `ev` the event code, `p1` a record read only on the `0x2D4`
/// path (word at `+0xB30`). Every path first fetches a worker through the
/// manager getter; a null worker returns 0, except on the `0x39A` path (see
/// below). The return value is otherwise the request's answer.
///
/// Dispatch on `ev` (first comparison signed): `0x2D4` runs the fifteen-word
/// setup request (1, 0x14, 0x1E, -1.0, 0x14, 4, 0, 0, 0, 0, 0x28, 2, 7,
/// `[this+0x18]`, `[p1+0xB30]`); `0xCB` the four-word request (8.0, 0, 0,
/// 0x7D0); `0xCA` the one-word request (1); `0x391` the three-word request
/// (-1.0, `[this+0x18]`, `[this+0x14]`); `0x39A` the object-filling request;
/// anything else returns 0 (the `0x516` code lands here too, through a block
/// shared with the null-worker exits).
///
/// The `0x39A` path runs the three-word request (8.0, the image word,
/// `[this+0x18]`) and stamps the answered object: zero at `+0x40`, `+0x44`
/// and `+0x48`, a frame word at `+0x4C`, 4 at `+0x50`. That frame word is
/// never written by the function: it is whatever the stack held, pinned to
/// zero by the proof's stack fill, so the rewrite writes zero. A null worker
/// — or a null request answer — faults writing through it on both sides,
/// exactly like the original.
///
/// Original: 0x00cd90e0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00cd90e0(this: u32, ev: u32, p1: u32) -> u32 {
    unsafe {
        const WORD14_OFF: u32 = 0x14;
        const WORD18_OFF: u32 = 0x18;
        const REC_B30: u32 = 0xb30;
        const EV_SETUP: u32 = 0x2d4;
        const EV_SMALL: u32 = 0xcb;
        const EV_ONE: u32 = 0xca;
        const EV_TRIPLE: u32 = 0x391;
        const EV_FILL: u32 = 0x39a;
        const MANAGER: u32 = 0x0167e2a0;
        const IMAGE_WORD: u32 = 0x01051ae8;
        const NEG_ONE: u32 = 0xbf80_0000;
        const EIGHT: u32 = 0x4100_0000;
        const CALLEE_GET: u32 = 1;
        const CALLEE_SETUP: u32 = 2;
        const CALLEE_SMALL: u32 = 3;
        const CALLEE_ONE: u32 = 4;
        const CALLEE_TRIPLE: u32 = 5;
        const CALLEE_FILL: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if ev == EV_TRIPLE {
            let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
            let g: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GET, u32, mgr);
            if g == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(
                CALLEE_TRIPLE, u32, g, rd32(this + WORD14_OFF), rd32(this + WORD18_OFF), NEG_ONE
            );
        }
        if (ev as i32) > EV_TRIPLE as i32 {
            if ev != EV_FILL {
                return 0;
            }
            let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
            let g: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GET, u32, mgr);
            // No null check here: a null worker faults below, like the original.
            let obj: u32 = if g == 0 {
                0
            } else {
                lf_checker_rt::callee_thiscall!(
                    CALLEE_FILL, u32, g, rd32(this + WORD18_OFF),
                    lf_checker_rt::global::<u32>(IMAGE_WORD).read(), EIGHT
                )
            };
            // The original forwards an uninitialized frame word at +0x4C; the
            // proof pins the stack fill to zero, so zero is written.
            wr32(obj.wrapping_add(0x40), 0);
            wr32(obj.wrapping_add(0x44), 0);
            wr32(obj.wrapping_add(0x48), 0);
            wr32(obj.wrapping_add(0x4c), 0);
            wr32(obj.wrapping_add(0x50), 4);
            return obj;
        }
        if ev == EV_ONE {
            let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
            let g: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GET, u32, mgr);
            if g == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(CALLEE_ONE, u32, g, 1u32);
        }
        if ev == EV_SMALL {
            let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
            let g: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GET, u32, mgr);
            if g == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(
                CALLEE_SMALL, u32, g, 0x7d0u32, 0u32, 0u32, EIGHT
            );
        }
        if ev != EV_SETUP {
            return 0;
        }
        let mgr = lf_checker_rt::global::<u32>(MANAGER).read();
        let g: u32 = lf_checker_rt::callee_thiscall!(CALLEE_GET, u32, mgr);
        if g == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            CALLEE_SETUP, u32, g, rd32(p1 + REC_B30), rd32(this + WORD18_OFF), 7u32, 2u32,
            0x28u32, 0u32, 0u32, 0u32, 0u32, 4u32, 0x14u32, NEG_ONE, 0x1eu32, 0x14u32, 1u32
        )
    }
});
