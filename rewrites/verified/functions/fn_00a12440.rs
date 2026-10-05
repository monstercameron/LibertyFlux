// original: 0x00a12440 target_slot_refresh (proposed)
/// Refresh the cached target object and report whether it is of kind 0x80.
///
/// `this + 0x170` holds the cached target. When empty, the current target
/// is fetched and, when non-null, retained. When occupied, the current
/// target is fetched and, when non-null and different, the old one is
/// released, the new one stored and retained, and a flag bit set. Returns
/// (low byte) whether the cached target's type bits (6-9 of the word at
/// `+0x28`) equal 0x80. Thiscall, no stack arguments.
export!(thiscall, rw_00a12440(this: u32) -> u32 {
    unsafe {
        const FETCH: u32 = 1;
        const RETAIN: u32 = 2;
        const RELEASE: u32 = 3;
        const SLOT_OFF: u32 = 0x170;
        const FLAG_OFF: u32 = 0x18c;
        const FLAG_BIT: u8 = 0x40;
        const TYPE_OFF: u32 = 0x28;
        const TYPE_MASK: u32 = 0x3c0;
        const WANT_TYPE: u32 = 0x80;
        let slot = this + SLOT_OFF;
        if ((slot) as *const u32).read_unaligned() == 0 {
            let cur = callee_cdecl!(FETCH, u32, 0);
            (slot as *mut u32).write_unaligned(cur);
            if cur == 0 {
                return 0;
            }
            callee_thiscall!(RETAIN, u32, cur, slot);
        } else {
            let cur = callee_cdecl!(FETCH, u32, 0);
            if cur != 0 && cur != (slot as *const u32).read_unaligned() {
                let old = (slot as *const u32).read_unaligned();
                if old != 0 {
                    callee_thiscall!(RELEASE, u32, old, slot);
                }
                (slot as *mut u32).write_unaligned(cur);
                callee_thiscall!(RETAIN, u32, cur, slot);
                let f = (this + FLAG_OFF) as *mut u8;
                f.write(f.read() | FLAG_BIT);
            }
        }
        let tgt = (slot as *const u32).read_unaligned();
        (((tgt + TYPE_OFF) as *const u32).read_unaligned() & TYPE_MASK == WANT_TYPE) as u32
    }
});
