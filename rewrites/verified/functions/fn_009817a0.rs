// original: 0x009817a0 audio_toggle_slot_index
/// Toggle the two-slot selector at `this+0x5bf8` and clear the new slot.
///
/// Decrements the selector modulo 2 (wrapping, masked to one bit), stores it
/// back, zeroes the newly selected slot at `this+0x5bf0+sel*4`, and returns
/// the new selector value.
export!(thiscall, rw_009817a0(this: u32) -> u32 {
    unsafe {
        let slot = (this.wrapping_add(0x5bf8)) as *mut u32;
        let new = (*slot).wrapping_sub(1) & 1;
        *slot = new;
        *((this.wrapping_add(0x5bf0).wrapping_add(new * 4)) as *mut u32) = 0;
        new
    }
});
