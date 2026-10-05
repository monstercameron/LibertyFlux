// original: 0x00E5CAD0 timer_store_callback_pair
/// Install two mainloop-timing callback slots.
///
/// Writes two 8-byte slots: the first holds a fixed code address in its
/// low half and one word of whatever the stack held in its high half
/// (the original stages the store through two frame slots but writes only
/// the low one, so the high word is genuinely uninitialized stack); the
/// second holds zero and a second fixed code address. Takes no arguments
/// and returns nothing; entry registers are ignored.
export!(cdecl, rw_00e5cad0() -> u32 {
    unsafe {
        // A wide escaping frame forces a real `(an instruction of the original)` frame: without the
        // escape, LLVM push-materializes the slot and the read captures a
        // live register instead of the stack word the original reads.
        let frame: [core::mem::MaybeUninit<u32>; 8] =
            [core::mem::MaybeUninit::uninit(); 8];
        let base = core::hint::black_box(frame.as_ptr() as *const u32);
        let padding = core::ptr::read_volatile(base.add(1));
        *global::<u32>(0x0110E19C) = relocated(0x004732F0);
        *global::<u32>(0x0110E1A0) = padding;
        *global::<u32>(0x0110E1A4) = 0;
        *global::<u32>(0x0110E1A8) = relocated(0x00409610);
        0
    }
});
