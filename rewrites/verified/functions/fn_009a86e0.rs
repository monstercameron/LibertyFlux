// original: 0x009a86e0 entity_notify_all
/// Notify each of the nine entity slots that `event` happened.
///
/// Walks the nine object pointers at `this+0x2df0`; each non-null entry
/// is notified (stubbed, thiscall/1) with ECX pointing 0x570 bytes into
/// the object and `event` as the stack argument. Null entries are
/// skipped. Thiscall, one stack word, no result.
export!(thiscall, rw_009A86E0(this: u32, event: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x2df0;
        const COUNT: u32 = 9;
        const NOTIFY_OFF: u32 = 0x570;
        let mut k = 0u32;
        while k < COUNT {
            let p = ((this + SLOTS + k * 4) as *const u32).read_unaligned();
            if p != 0 {
                let _: u32 = callee_thiscall!(1, u32, p.wrapping_add(NOTIFY_OFF), event);
            }
            k += 1;
        }
        0
    }
});
