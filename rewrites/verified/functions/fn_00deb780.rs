// original: 0x00DEB780 UIMontageEditor::vf110
//! Refresh one montage-editor slot.
//!
//! Queries the global registry for the slot's 12-byte descriptor and dispatches
//! on its kind word: kind `0x12` rebuilds the slot's live binding through the
//! text service, kind 6 re-resolves it through the clip source, and any other
//! kind leaves the slot cleared. Both live paths first resolve the editor's
//! inner handle through two object links and bail out cleanly when a link is
//! null or the handle reports no rows. The registry query, the text service
//! and the clip source are all intercepted callees; the two vtable steps on
//! the slot's child objects land on planted stubs.
//!
//! Returns whatever the original leaves in eax on each path (the last call's
//! result, the kind word, or the null/row test value), so the rewrite stays
//! exact whether callers treat this slot as void or value-returning.
//!
//! Verified by checker v3: 1000/1000 trials, 7 branch shapes, honesty mutant
//! (main kind off by one) fails on the return channel. Lane r-b140.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

export!(thiscall, rw_00deb780(this: u32) -> u32 {
    run_slot(this, 0x12)
});

/// Shared body; `main_kind` is the descriptor kind that selects the rebuild path.
fn run_slot(this: u32, main_kind: u32) -> u32 {
    const REGISTRY: u32 = 0x019D2E08;
    const TEXT_SVC: u32 = 0x01176888;
    const CLIP_SRC: u32 = 0x01981A4C;
    const SEL_OTHER: u32 = 0x00EFF38C;
    const SEL_CLIP: u32 = 0x00EFF3A0;
    const SEL_MENU: u32 = 0x00EFF3AC;
    const SHARED_WORD: u32 = 0x01173594;
    const KIND_ALT: u32 = 6;
    const QUERY_A: u32 = 0x40002;
    unsafe {
        let base = this as *mut u8;
        // Slot starts cleared; the alt path sets it live at the end.
        base.add(0x208).write(0);
        let desc = callee_thiscall!(1, u32, relocated(REGISTRY), QUERY_A, 1);
        let kind = (desc as *const u32).read();
        if kind == main_kind {
            let outer = (base.add(0x1E4) as *const u32).read();
            if outer == 0 {
                return 0;
            }
            let mid = inner_link(outer);
            let rows = callee_thiscall!(2, u32, mid);
            if (rows as i32) <= 0 {
                return rows;
            }
            let live = (base.add(0x224) as *const u32).read();
            if live != 0 {
                let extra = (base.add(0x228) as *const u32).read();
                callee_thiscall!(3, u32, mid, live, extra);
                callee_cdecl!(4, u32, live);
                (base.add(0x224) as *mut u32).write(0);
                (base.add(0x228) as *mut u32).write(0);
            }
            callee_thiscall!(5, u32, mid, 0);
            base.add(0x20B).write(0);
            callee_thiscall!(6, u32, relocated(REGISTRY), 1);
            let shared = global::<u32>(SHARED_WORD).read();
            (base.add(0x21C) as *mut u32).write(shared);
            let first = (base.add(0x1F4) as *const u32).read();
            vcall1(first, 0x120, 0);
            let second = (base.add(0x1F0) as *const u32).read();
            vcall1(second, 0x120, 1);
            callee_thiscall!(9, u32, relocated(TEXT_SVC), relocated(SEL_OTHER))
        } else if kind == KIND_ALT {
            let outer = (base.add(0x1E4) as *const u32).read();
            if outer == 0 {
                return 0;
            }
            let mid = inner_link(outer);
            let rows = callee_thiscall!(2, u32, mid);
            if (rows as i32) <= 0 {
                return rows;
            }
            let clip = callee_thiscall!(10, u32, relocated(CLIP_SRC), relocated(SEL_CLIP));
            let target = ((clip + 0x1F0) as *const u32).read();
            if target != 0 {
                let keep = vcall0(target, 0x124);
                if (keep & 0xFF) != 0 {
                    return keep;
                }
            }
            callee_thiscall!(9, u32, relocated(TEXT_SVC), relocated(SEL_MENU));
            base.add(0x208).write(1);
            callee_thiscall!(6, u32, relocated(REGISTRY), 1)
        } else {
            kind
        }
    }
}

/// The editor's inner handle: the second of two object links off the slot.
#[inline(always)]
unsafe fn inner_link(outer: u32) -> u32 {
    let mid = ((outer + 0x1E0) as *const u32).read();
    ((mid + 0x1F8) as *const u32).read()
}

/// One vtable call with no stack arguments, through the planted object.
#[inline(always)]
unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
    let vt = (obj as *const u32).read();
    let addr = ((vt + slot) as *const u32).read();
    let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(addr as usize);
    f(obj)
}

/// One vtable call with a single stack argument, through the planted object.
#[inline(always)]
unsafe fn vcall1(obj: u32, slot: u32, arg0: u32) -> u32 {
    let vt = (obj as *const u32).read();
    let addr = ((vt + slot) as *const u32).read();
    let f: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(addr as usize);
    f(obj, arg0)
}
