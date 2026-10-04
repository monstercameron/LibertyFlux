// original: 0x00bdf7f0 audio_dtor_table_voice
/// Destroy the audio node with a table-indexed voice.
/// Stamps vtable 0xEB9294. When the +0x2a flag byte is set, looks up the
/// voice through the shared table using the +0x24 index and shuts it down
/// (id 1, thiscall/0), then clears the flag. When the +0x10 word is
/// non-null it marks the linked record (sets bit 22 of its second word).
/// Re-tunes the node with gain -8.0f (id 2, thiscall/1) and forwards to
/// the shared base destructor (tail id 9). Returns the tail answer.
export!(thiscall, rw_00bdf7f0(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB9294;
        const TABLE: u32 = 0x1295CD8;
        const RETUNE_GAIN_BITS: u32 = 0xC100_0000;
        *(this as *mut u32) = relocated(VTABLE);
        if *(this.add(0x2A)) != 0 {
            let idx = *((this.add(0x24)) as *const u32);
            let slot = relocated(TABLE).wrapping_add(idx.wrapping_mul(4));
            let voice = *(slot as *const u32);
            callee_thiscall!(1, u32, voice);
            *(this.add(0x2A)) = 0;
        }
        let linked = *((this.add(0x10)) as *const u32);
        if linked != 0 {
            let flags = (linked.wrapping_add(4)) as *mut u32;
            *flags |= 0x0040_0000;
        }
        callee_thiscall!(2, u32, this as u32, RETUNE_GAIN_BITS);
        callee_thiscall!(9, u32, this as u32)
    }
});
