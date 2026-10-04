// original: 0x00bdf8e0 audio_dtor_retag_pair_tail
/// Re-tag the audio sub-object (vtable 0xEB8C3C, sub-tag 0xEB8C90 at +0x14)
/// and forward to the shared sub-object teardown (tail id 9).
/// Returns the tail answer.
export!(thiscall, rw_00bdf8e0(this: *mut u8) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xEB8C3C;
        const SUBTAG: u32 = 0xEB8C90;
        *(this as *mut u32) = relocated(VTABLE);
        *((this.add(0x14)) as *mut u32) = relocated(SUBTAG);
        callee_thiscall!(9, u32, this as u32)
    }
});
