// original: 0x00B66680 veh_release_slot_14
/// Release the reference kept at `[this+0x14]` and clear the slot.
///
/// If the slot is null, returns at once. Otherwise calls the slot visitor
/// (stubbed, thiscall/1) with the slot value, re-reads the slot, calls the
/// release helper (stubbed, stdcall/1) with the slot address when still
/// non-null, and zeroes the slot. Thiscall, no stack words; entry registers
/// except ECX are ignored.
export!(thiscall, rw_00b66680(this: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x14;
        let slot = this + SLOT;
        if (slot as *const u32).read_unaligned() != 0 {
            let cur = (slot as *const u32).read_unaligned();
            let _: u32 = callee_thiscall!(1, u32, this, cur);
            if (slot as *const u32).read_unaligned() != 0 {
                let _: u32 = callee_stdcall!(2, u32, slot);
            }
            (slot as *mut u32).write_unaligned(0);
        }
        0
    }
});
