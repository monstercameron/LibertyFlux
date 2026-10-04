// original: 0x00b24e70 clear_movement_state
// s08_b24e70: clear movement state. thiscall/1 (flags: u32): zeroes the two
// movement dwords at +0x170/+0x174, releases the movement record at +0x16C
// through the shared helper when occupied, and unless the low flag byte is
// set, clears the touching-list count at +0x150. Returns nothing.
export!(thiscall, rw_b24e70(this: *mut u8, flags: u32) -> () {
    unsafe {
        *(this.add(0x170) as *mut u32) = 0;
        *(this.add(0x174) as *mut u32) = 0;
        let slot = this.add(0x16C) as *mut u32;
        if *slot != 0 {
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(callee_addr(1) as usize);
            release(*slot, slot as u32);
            *slot = 0;
        }
        if flags & 0xFF == 0 {
            *this.add(0x150) = 0;
        }
    }
});
