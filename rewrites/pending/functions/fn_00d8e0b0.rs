// original: 0x00d8e0b0 audDoorAudioEntity::vf1
/// Refresh the door audio entity, delegating tagged entities to the tail handler.
///
/// Runs the two refresh passes over `this`, then checks the tag at offset 4:
/// the exact tag 0xffff hands control to the shared handler (a tail call),
/// anything else ends the call with 0xffff still in the return slot.
lf_rs89_rt::export!(thiscall, rw_00d8e0b0(this: *mut u8) -> u32 {
    unsafe {
        let addr = this as u32;
        lf_rs89_rt::callee_thiscall!(1, u32, addr);
        lf_rs89_rt::callee_thiscall!(2, u32, addr);
        if *(this.wrapping_add(4) as *const u16) == 0xFFFF {
            lf_rs89_rt::callee_thiscall!(3, u32, addr)
        } else {
            0xFFFF
        }
    }
});
