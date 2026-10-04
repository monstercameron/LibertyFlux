// original: 0x00DF79F0 UIFileViewer::vf110
/// File-viewer mode dispatcher: reads the viewer's mode word through an info
/// lookup, then refreshes the matching pane.
///
/// `this` is the viewer object. The info lookup returns a small record whose
/// first word selects the path: `0x10` re-checks the delete guard and either
/// returns or tail-forwards to one of the two pane reset routines depending
/// on the flag words of the shared viewer state; `1` resolves the active
/// entry and, when it reports a live page, selects that page and walks three
/// nested view links before refreshing; `2` resolves the active entry and,
/// when its page count is behind the reported count, selects the next page
/// and refreshes; any other mode does nothing. Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rb97_fn2(this: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_INFO: u32 = 1; // info lookup (thiscall/2: global, kind, gen)
    const CAL_GUARD: u32 = 2; // delete-guard check (thiscall/0, al result)
    const CAL_REFRESH: u32 = 3; // pane refresh (thiscall/1: global, gen)
    const CAL_RESOLVE: u32 = 4; // active-entry resolve (thiscall/0)
    const CAL_SELECT: u32 = 5; // page select (thiscall/2: entry, page, gen)
    const CAL_COUNT: u32 = 6; // reported page count, vtable +0x21c (thiscall/0)
    const CAL_LINK0: u32 = 7; // first nested view link, vtable +0x224 (thiscall/0)
    const CAL_LINK1: u32 = 8; // second nested view link, vtable +0x220 (thiscall/0)
    const CAL_LINK2: u32 = 9; // third nested view link, vtable +0x1ac (thiscall/0)
    const CAL_PAGES: u32 = 10; // entry page count, vtable +0x1d4 (thiscall/0)
    const CAL_RESET_A: u32 = 11; // pane reset path A, tail call (thiscall/0)
    const CAL_RESET_B: u32 = 12; // pane reset path B, tail call (thiscall/0)

    // Globals (file VAs; resolved through the worker's image base).
    const GCTX_VA: u32 = 0x019D2E08; // shared UI context, passed as `this` to CAL_INFO/CAL_REFRESH
    const GSTATE_VA: u32 = 0x018B6C8C; // shared viewer state pointer

    const INFO_KIND: u32 = 0x20300; // info record kind requested
    const GEN: u32 = 1; // generation tag forwarded to refresh/select
    const MODE_GUARD: u32 = 0x10; // re-check delete guard, maybe reset a pane
    const MODE_PAGE: u32 = 1; // select the reported live page
    const MODE_NEXT: u32 = 2; // select the next page when behind
    const ENTRY_CHILD: u32 = 0x1E0; // active entry -> child view object
    const STATE_FLAG_A: u32 = 0x204; // reset path A selector (nonzero selects A)
    const STATE_FLAG_B: u32 = 0x200; // reset path B selector (nonzero selects B)
    const VT_COUNT: u32 = 0x21C;
    const VT_LINK0: u32 = 0x224;
    const VT_LINK1: u32 = 0x220;
    const VT_LINK2: u32 = 0x1AC;
    const VT_PAGES: u32 = 0x1D4;

    #[inline(always)]
    unsafe fn load(base: u32, off: u32) -> u32 {
        *((base.wrapping_add(off)) as *const u32)
    }

    /// Call a planted vtable slot exactly like the original does: load the
    /// slot address and call through it. Both sides land on the same stub.
    #[inline(always)]
    unsafe fn vcall(object: u32, slot: u32) -> u32 {
        let vtable = *(object as *const u32);
        let target = *((vtable.wrapping_add(slot)) as *const u32);
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        f(object)
    }

    unsafe {
        let gctx = lf_checker_rt::relocated(GCTX_VA);
        let info = lf_checker_rt::callee_thiscall!(CAL_INFO, u32, gctx, INFO_KIND, GEN);
        let mode = load(info, 0);
        if mode == MODE_GUARD {
            let guard = lf_checker_rt::callee_thiscall!(CAL_GUARD, u32, this);
            lf_checker_rt::callee_thiscall!(CAL_REFRESH, u32, gctx, GEN);
            if (guard & 0xFF) != 0 {
                let state = *(lf_checker_rt::global::<u32>(GSTATE_VA));
                if load(state, STATE_FLAG_A) != 0 {
                    return lf_checker_rt::callee_thiscall!(CAL_RESET_A, u32, state);
                }
                if load(state, STATE_FLAG_B) != 0 {
                    return lf_checker_rt::callee_thiscall!(CAL_RESET_B, u32, state);
                }
            }
            0
        } else if mode == MODE_PAGE {
            let entry = lf_checker_rt::callee_thiscall!(CAL_RESOLVE, u32, this);
            if entry == 0 {
                return 0;
            }
            let child = load(entry, ENTRY_CHILD);
            let live = vcall(child, VT_COUNT);
            if live == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(CAL_SELECT, u32, entry, live.wrapping_sub(1), GEN);
            let child = load(entry, ENTRY_CHILD);
            let vtable = *(child as *const u32);
            let target = *((vtable.wrapping_add(VT_LINK0)) as *const u32);
            let f0: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let o1 = f0(child);
            let o2 = vcall(o1, VT_LINK1);
            vcall(o2, VT_LINK2);
            lf_checker_rt::callee_thiscall!(CAL_REFRESH, u32, gctx, GEN);
            0
        } else if mode == MODE_NEXT {
            let entry = lf_checker_rt::callee_thiscall!(CAL_RESOLVE, u32, this);
            if entry == 0 {
                return 0;
            }
            let child = load(entry, ENTRY_CHILD);
            let next = vcall(child, VT_COUNT).wrapping_add(1);
            let have = vcall(entry, VT_PAGES);
            if next >= have {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(CAL_SELECT, u32, entry, next, GEN);
            lf_checker_rt::callee_thiscall!(CAL_REFRESH, u32, gctx, GEN);
            0
        } else {
            0
        }
    }
});
