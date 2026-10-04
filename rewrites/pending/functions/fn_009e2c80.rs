// original: 0x009e2c80 audio_slot_add_and_dispatch
/// Add the argument into the big tracker slot, then tail-dispatch
/// to the slot-store routine. (thiscall/1, tail jump)
export!(thiscall, rw_009e2c80(this: *mut u8, a: u32) -> u32 {
    unsafe {
        let slot = *((this.add(0xB88)) as *const u32);
        callee_thiscall!(1, u32, this as u32, a.wrapping_add(slot))
    }
});
