// original: 0x008f8290 input_queue_key_event (proposed)

/// Probe the axis state, then stamp a key event into the device object.
///
/// `obj` is the input device object, `p1` a flag byte and `p2` an event
/// word. The function first calls the axis probe callee (thiscall, ECX =
/// `obj`, no stack words; its answer is ignored except as the EAX value on
/// the early-exit paths). It then reads the peer pointer at `+0x14` and the
/// ready word at `+0x18`: when either is zero it returns the probe answer
/// unchanged. Otherwise it stores `p2` at `+0x45C`, the global stamp word at
/// `+0x458`, sets bit 3 of the peer's flag byte at `+0x558` when `p1` is
/// nonzero, and stores the `p1` low byte at `+0x450`. The returned EAX is
/// the stamp with its low byte replaced by `p1`.
///
/// Thiscall: object in ECX, two stack words, callee cleans 8.
lf_checker_rt::export!(thiscall, rw_008f8290(obj: u32, p1: u32, p2: u32) -> u32 {
    unsafe {
        const C_PROBE: u32 = 1;
        const PEER_OFF: u32 = 0x14;
        const READY_OFF: u32 = 0x18;
        const EVENT_OFF: u32 = 0x45c;
        const STAMP_OFF: u32 = 0x458;
        const FLAG_OFF: u32 = 0x450;
        const PEER_FLAG: u32 = 0x558;
        const MARK_BIT: u8 = 8;
        const G_STAMP: u32 = 0x11735d4;
        let probe: u32 = lf_checker_rt::callee_thiscall!(C_PROBE, u32, obj);
        let peer = ((obj + PEER_OFF) as *const u32).read_unaligned();
        if peer == 0 {
            return probe;
        }
        if ((obj + READY_OFF) as *const u32).read_unaligned() == 0 {
            return probe;
        }
        ((obj + EVENT_OFF) as *mut u32).write_unaligned(p2);
        let stamp = (lf_checker_rt::global::<u32>(G_STAMP)).read_unaligned();
        ((obj + STAMP_OFF) as *mut u32).write_unaligned(stamp);
        let flag = (p1 & 0xff) as u8;
        if flag != 0 {
            let fp = (peer + PEER_FLAG) as *mut u8;
            fp.write(fp.read() | MARK_BIT);
        }
        ((obj + FLAG_OFF) as *mut u8).write(flag);
        (stamp & 0xffff_ff00) | (p1 & 0xff)
    }
});
