// original: 0x00d695d0 reset_slot_state
// s16f01: reset this record's slot state (thiscall/1).
//
// Clears the status bit, zeroes four slot words and parks the mode byte.
// The stack argument is accepted but never read. Leaves EAX untouched, so
// there is no meaningful return value.
export!(thiscall, rw_s16f01(this: *mut u8, _arg: u32) -> () {
    unsafe {
        *this.add(0x29) &= !0x40;
        *((this.add(0x30)) as *mut u32) = 0;
        *((this.add(0x34)) as *mut u32) = 0;
        *((this.add(0x38)) as *mut u32) = 0;
        *((this.add(0x3C)) as *mut u32) = 0;
        *this.add(0x25) = 4;
    }
});
