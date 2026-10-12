// original: 0x009a8e90 script_stop_forward
/// Stop script slot `idx`, forwarding the live handle when it matches.
///
/// A negative index returns at once. Otherwise the resolver (stubbed,
/// cdecl/0) supplies a table whose word at `+4` selects the slot's
/// owner, and the opener (stubbed, thiscall/1) is tried: a
/// non-negative answer stops the slot (below) and returns. A negative
/// answer runs the resetter (stubbed, thiscall/1 with 0) and retries
/// the opener; a non-negative retry stops the same way. When both
/// attempts fail, the event source (stubbed, cdecl/0) is polled: a
/// null event returns, otherwise the event is formatted into a stack
/// buffer (stubbed, cdecl/3) and logged (stubbed, cdecl/5).
/// Stopping reads the slot pair at `this + idx*16 + 0x33d4` (handle,
/// generation): a null handle or a generation mismatching the
/// opener's answer returns, otherwise the handle is forwarded
/// (stubbed, thiscall/1 with 0). The CRT security cookie is checked
/// on exit (stubbed, preserving all registers). Thiscall, one stack
/// word, no compared result (EAX is cookie-derived garbage).
export!(thiscall, rw_009A8E90(this: u32, idx: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x33d4;
        const STRIDE: u32 = 16;
        const LOG_TAG: u32 = 0xe918d0;
        if (idx as i32) < 0 {
            let _: u32 = callee_cdecl!(9, u32,);
            return 0;
        }
        let p: u32 = callee_cdecl!(1, u32,);
        let owner = ((p + 4) as *const u32).read_unaligned();
        let r1: u32 = callee_thiscall!(2, u32, this, owner);
        if (r1 as i32) >= 0 {
            stop_forward(this, idx, r1);
            let _: u32 = callee_cdecl!(9, u32,);
            return 0;
        }
        let _: u32 = callee_thiscall!(3, u32, this, 0);
        let r3: u32 = callee_thiscall!(4, u32, this, owner);
        if (r3 as i32) >= 0 {
            stop_forward(this, idx, r3);
            let _: u32 = callee_cdecl!(9, u32,);
            return 0;
        }
        let ev: u32 = callee_cdecl!(5, u32,);
        let mut buf = [0u32; 32];
        let bufp = (&mut buf as *mut u32) as u32;
        let _: u32 = callee_cdecl!(6, u32, bufp, 0, 0x78);
        if ev != 0 {
            let _: u32 = callee_cdecl!(7, u32, bufp, 0x80, relocated(LOG_TAG), ev, r3);
        }
        let _: u32 = callee_cdecl!(9, u32,);
        0
    }
});

/// Stop one slot: forward the handle when the generation matches.
unsafe fn stop_forward(this: u32, idx: u32, gen: u32) {
    unsafe {
        let e = this + idx * 16 + 0x33d4;
        let handle = (e as *const u32).read_unaligned();
        if handle == 0 {
            return;
        }
        if ((e.wrapping_add(4)) as *const u32).read_unaligned() != gen {
            return;
        }
        let _: u32 = callee_thiscall!(8, u32, handle, 0);
    }
}
