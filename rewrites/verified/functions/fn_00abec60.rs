// original: 0x00ABEC60 T_CB_Generic_2Args_CBox::invoke

/// Invoke a two argument callback with a receiver-owned value and a normalized flag.
///
/// The callback address is stored at receiver offset 8. Its first argument is
/// a pointer to the receiver's value beginning at offset 16. Its second
/// argument is one when the 32-bit flag at offset 48 is nonzero, and zero
/// otherwise. The callback uses the C calling convention; the receiver is
/// passed in ECX.
lf_checker_rt::export!(thiscall, rw_00abec60(this: u32) -> () {
    unsafe {
        const CALLBACK_SLOT: usize = 8;
        const INLINE_VALUE: usize = 0x10;
        const PRESENT_FLAG: usize = 0x30;
        let receiver = this as *const u8;
        let callback_address = receiver.add(CALLBACK_SLOT).cast::<u32>().read_unaligned();
        let flag = receiver.add(PRESENT_FLAG).cast::<u32>().read_unaligned();
        let callback: unsafe extern "C" fn(u32, u32) =
            core::mem::transmute(callback_address as usize);
        callback(this.wrapping_add(INLINE_VALUE as u32), u32::from(flag != 0));
    }
});
