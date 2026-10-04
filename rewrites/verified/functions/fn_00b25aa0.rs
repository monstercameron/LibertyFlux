// original: 0x00b25aa0 clear_ignored_collision
// s08_b25aa0: clear ignored-collision state. thiscall/0: the clear runs only
// when the state byte at +0x1F4 selects it (bit 0x20 without bit 1, or
// without bit 0x20, bit 0x40 without bit 2); then it releases the ignored
// entity at +0x1F8 through the shared helper when occupied and clears the
// byte and the slot. Either way the byte is masked to 0x7C. Returns nothing.
export!(thiscall, rw_b25aa0(this: *mut u8) -> () {
    unsafe {
        let st = *this.add(0x1F4);
        let clear = st & 0x20 != 0 && st & 1 == 0
            || st & 0x20 == 0 && st & 0x40 != 0 && st & 2 == 0;
        let keep = !clear;
        if !keep {
            let slot = this.add(0x1F8) as *mut u32;
            if *slot != 0 {
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(callee_addr(1) as usize);
                release(*slot, slot as u32);
            }
            *this.add(0x1F4) = 0;
            *slot = 0;
        }
        *this.add(0x1F4) &= 0x7C;
    }
});
