// original: 0x009a8220 script_slot_release
/// Release script slot `arg0` and cascade the teardown.
///
/// Opens the slot (stubbed, thiscall/1); a negative answer returns
/// it at once. Otherwise the slot's state word at `this + slot*8 +
/// 0x3a14` is examined: a negative state or state 5 skips the tick,
/// any other state decrements the use word at `this + state*16 +
/// 0x337c`. When the state is exactly 5 and the selected word at
/// `this+0x33cc` equals the slot, that word is retired to -1. Then
/// all 100 entries at `this+0x33d4` (16 bytes each) are swept: an
/// entry whose generation word matches the slot has its object
/// released (stubbed, thiscall/1 with 0) unless null, and is zeroed
/// with generation -1. The slot's own words become 0 and -1, and
/// when the pending word at `this+0x3abc` is a non-negative match
/// for the slot, the closer (stubbed, thiscall/0) runs and the
/// pending word is retired to -1. Thiscall, one stack word, dword
/// result (the opener's answer on the early path, else trailing
/// values as the original leaves them).
export!(thiscall, rw_009A8220(this: u32, arg0: u32) -> u32 {
    unsafe {
        const STATE_BASE: u32 = 0x3a14;
        const USE_BASE: u32 = 0x337c;
        const SELECTED: u32 = 0x33cc;
        const ENTRIES: u32 = 0x33d4;
        const ENTRY_STRIDE: u32 = 16;
        const ENTRY_COUNT: u32 = 100;
        const PENDING: u32 = 0x3abc;
        let slot: u32 = callee_thiscall!(1, u32, this, arg0);
        if (slot as i32) < 0 {
            return slot;
        }
        let st = ((this + slot * 8 + STATE_BASE) as *const i32).read_unaligned();
        if st >= 0 && st != 5 {
            let u = ((this + (st as u32) * 16 + USE_BASE) as *const u16).read_unaligned();
            ((this + (st as u32) * 16 + USE_BASE) as *mut u16).write_unaligned(u.wrapping_sub(1));
        }
        if st == 5 && ((this + SELECTED) as *const u16).read_unaligned() == slot as u16 {
            ((this + SELECTED) as *mut u16).write_unaligned(0xFFFF);
        }
        let mut k = 0u32;
        while k < ENTRY_COUNT {
            let e = this + ENTRIES + k * ENTRY_STRIDE;
            if ((e.wrapping_add(4)) as *const u32).read_unaligned() == slot {
                let obj = (e as *const u32).read_unaligned();
                if obj != 0 {
                    let _: u32 = callee_thiscall!(2, u32, obj, 0);
                }
                ((e.wrapping_sub(4)) as *mut u32).write_unaligned(0);
                (e as *mut u32).write_unaligned(0);
                ((e.wrapping_add(4)) as *mut u32).write_unaligned(0xFFFFFFFF);
                ((e.wrapping_add(8)) as *mut u8).write(0);
            }
            k += 1;
        }
        ((this + slot * 8 + 0x3a10) as *mut u32).write_unaligned(0);
        ((this + slot * 8 + STATE_BASE) as *mut u32).write_unaligned(0xFFFFFFFF);
        let pend = ((this + PENDING) as *const i32).read_unaligned();
        if pend < 0 {
            return pend as u32;
        }
        if pend as u32 != slot {
            return pend as u32;
        }
        let ans: u32 = callee_thiscall!(3, u32, this);
        ((this + PENDING) as *mut u32).write_unaligned(0xFFFFFFFF);
        ans
    }
});
