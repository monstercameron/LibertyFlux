// original: 0x00b57060 init_frame_slots
// rs05f5: initialise the 50 frame-buffer slots and clear the occupancy mask.
//
// Calls the slot initialiser (thiscall/0) for each slot at `this+4+i*0x14`,
// then zeroes the two mask words at `this+0x3EC`.
export!(thiscall, rw_b57060(this: *mut u8) -> () {
    unsafe {
        let init: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        for i in 0..50u32 {
            init(this.add(4 + (i as usize) * 0x14) as u32);
        }
        *(this.add(0x3EC) as *mut u32) = 0;
        *(this.add(0x3F0) as *mut u32) = 0;
    }
});
