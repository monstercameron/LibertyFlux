// original: 0x009a8810 script_event_forward
/// Forward the script event for slot `idx` to its owner's handler.
///
/// A negative index returns at once. Otherwise the resolver (stubbed,
/// cdecl/0) supplies a table whose word at `+4` selects the slot's
/// owner, and the opener (stubbed, thiscall/1) is tried: a
/// non-negative answer forwards the event (below) and returns. A
/// negative answer runs the resetter (stubbed, thiscall/1 with 0)
/// and retries the opener; a non-negative retry forwards the same
/// way. When both attempts fail, the event source (stubbed, cdecl/0)
/// is polled: a null event returns, otherwise the event is formatted
/// into a stack buffer (stubbed, cdecl/3) and logged (stubbed,
/// cdecl/5). Forwarding reads the slot pair at `this + idx*16 +
/// 0x33d4` (object, generation): a null object or a generation
/// mismatching the opener's answer returns, otherwise the object's
/// handler at vtable slot 3 is invoked with the object in ECX and
/// (`arg1`, `arg2`) on the stack (the vtable is fabricated heap
/// holding the stub address). The CRT security cookie is checked on
/// exit (stubbed, preserving all registers). Thiscall, three stack
/// words, no compared result (EAX is cookie-derived garbage).
export!(thiscall, rw_009A8810(this: u32, idx: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x33d4;
        const STRIDE: u32 = 16;
        const LOG_TAG: u32 = 0xe91918;
        if (idx as i32) < 0 {
            let _: u32 = callee_cdecl!(9, u32,);
            return 0;
        }
        let p: u32 = callee_cdecl!(1, u32,);
        let owner = ((p + 4) as *const u32).read_unaligned();
        let r1: u32 = callee_thiscall!(2, u32, this, owner);
        if (r1 as i32) >= 0 {
            event_forward(this, idx, arg1, arg2, r1);
            let _: u32 = callee_cdecl!(9, u32,);
            return 0;
        }
        let _: u32 = callee_thiscall!(3, u32, this, 0);
        let r3: u32 = callee_thiscall!(4, u32, this, owner);
        if (r3 as i32) >= 0 {
            event_forward(this, idx, arg1, arg2, r3);
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

/// Forward one event through the slot object's vtable handler.
unsafe fn event_forward(this: u32, idx: u32, arg1: u32, arg2: u32, gen: u32) {
    unsafe {
        let e = this + idx * 16 + 0x33d4;
        let obj = (e as *const u32).read_unaligned();
        if obj == 0 {
            return;
        }
        if ((e.wrapping_add(4)) as *const u32).read_unaligned() != gen {
            return;
        }
        let vt = (obj as *const u32).read_unaligned();
        let _tgt = ((vt + 0x0c) as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(8, u32, obj, arg1, arg2);
    }
}
