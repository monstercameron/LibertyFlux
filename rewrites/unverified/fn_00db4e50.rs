// original: 0x00DB4E50 UIMouseCursor::vf82

/// Slot 82 of the mouse-cursor virtual table: refresh the active child.
///
/// `this` is the cursor object. A fetch helper (scripted) answers a
/// pointer whose target is stored into the current child object at
/// `CHILD_VALUE`. When the flag at `FORCE` is set the mode at `MODE` is
/// forced to 2. When the mode differs from the committed mode at
/// `COMMITTED`, the child is notified through slot `+0x120` of its virtual
/// table with 0, the new child is picked by mode (0 takes the word at
/// `CHILD_A`, 1 or 2 the word at `CHILD_B, any other mode keeps the
/// notification's answer), stored as the current child, the mode is
/// committed, a global helper runs (scripted), and the new child is
/// notified again with 1. Finally a tail helper and the mode setter run
/// (both scripted, the latter with 0). The helpers after the second
/// notification run on every path, including when the mode already
/// matched. Nothing observable is returned.
///
/// The fetch helper takes a pointer into this function's own frame; the
/// contract skips that call argument (the helper's output is never read
/// back from the frame).
///
/// Original: 0x00DB4E50 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00db4e50(this: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 0x1e8;
        const CHILD_VALUE: u32 = 0x1e0;
        const CHILD_A: u32 = 0x1e0;
        const CHILD_B: u32 = 0x1e4;
        const COMMITTED: u32 = 0x1f4;
        const MODE: u32 = 0x1f8;
        const FORCE: u32 = 0x200;
        const NOTIFY_SLOT: u32 = 0x120;
        const GLOBAL_HELPER_OBJ: u32 = 0x011737d0;
        const FETCH: u32 = 1;
        const NOTIFY: u32 = 2;
        const GLOBAL_HELPER: u32 = 3;
        const TAIL_HELPER: u32 = 4;
        const SET_MODE: u32 = 5;

        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let frame_slot = 0u32;
        let fetched: u32 = lf_checker_rt::callee_cdecl!(
            FETCH,
            u32,
            &frame_slot as *const u32 as u32,
            1
        );
        wr(rd(this + CHILD) + CHILD_VALUE, rd(fetched));
        if rd(this + FORCE) != 0 {
            wr(this + MODE, 2);
        }
        if rd(this + MODE) != rd(this + COMMITTED) {
            let notify = |obj: u32, arg: u32| -> u32 {
                let vtable = rd(obj);
                let hook: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                    ((vtable + NOTIFY_SLOT) as *const u32).read_unaligned() as usize,
                );
                hook(obj, arg)
            };
            let first = notify(rd(this + CHILD), 0);
            let mode = rd(this + MODE);
            let picked = if mode == 0 {
                rd(this + CHILD_A)
            } else if mode == 1 || mode == 2 {
                rd(this + CHILD_B)
            } else {
                first
            };
            wr(this + CHILD, picked);
            wr(this + COMMITTED, mode);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                GLOBAL_HELPER,
                u32,
                lf_checker_rt::relocated(GLOBAL_HELPER_OBJ),
                0
            );
            notify(rd(this + CHILD), 1);
            let _ = NOTIFY;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(TAIL_HELPER, u32, this);
        let _: u32 = lf_checker_rt::callee_thiscall!(SET_MODE, u32, this, 0);
        0
    }
});
