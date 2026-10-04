// original: 0x008a9470 audeffect_set_slot_float
/// Store one float into the five-wide slot table at `this+0x34`.
///
/// The slot index is `5 * count + byte_arg`, where `count` is the dword at
/// `this+0x30` and only the low byte of the second argument is used.
/// Returns the masked index byte (matching exit EAX).
export!(thiscall, rw_008a9470(this: *mut u8, value: f32, index: u32) -> u32 {
    unsafe {
        let count = *(this.add(0x30) as *const u32);
        let i = index & 0xFF;
        let slot = count.wrapping_mul(5).wrapping_add(i);
        *((this.add(0x34) as *mut f32).add(slot as usize)) = value;
        i
    }
});
