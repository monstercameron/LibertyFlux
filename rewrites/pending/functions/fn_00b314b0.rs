// original: 0x00b314b0 task_event_forward (proposed)

/// Qualify a task event and forward it to the event sink.
///
/// `a` and `b` point to task records. A key callee reads `a`, a gate callee
/// reads `b`: when the gate's low byte is non-zero the key answer goes
/// forward, defaulting to 3 when it is 0; otherwise the key answer goes
/// forward as is. The object's virtual slot at `+0xd0` is then invoked with
/// `a`, and the word at `+0x178` of its answer joins the forwarded key and
/// the head word of `b` as the three arguments of the sink callee, whose
/// answer is returned.
///
/// Original: 0x00b314b0 (cdecl, two stack words; three direct callees and
/// one virtual call through the object's table).
lf_checker_rt::export!(cdecl, rw_00b314b0(a: u32, b: u32) -> u32 {
    unsafe {
        const KEY: u32 = 1;
        const GATE: u32 = 2;
        const SINK: u32 = 4;
        const VTABLE_SLOT: u32 = 0xd0;
        const PAYLOAD_OFF: u32 = 0x178;
        let key: u32 = lf_checker_rt::callee_cdecl!(KEY, u32, a);
        let gate: u32 = lf_checker_rt::callee_cdecl!(GATE, u32, b);
        let mut forward = key;
        if gate & 0xFF != 0 && forward == 0 {
            forward = 3;
        }
        let vtable = (a as *const u32).read_unaligned();
        let slot = (vtable as *const u32).byte_add(VTABLE_SLOT as usize).read_unaligned();
        let hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let gotten = hook(a);
        let payload = (gotten as *const u32).byte_add(PAYLOAD_OFF as usize).read_unaligned();
        let head = (b as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(SINK, u32, forward, head, payload)
    }
});
