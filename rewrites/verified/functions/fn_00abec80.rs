// original: 0x00ABEC80 T_CB_Generic_2Args_Vec::invoke

/// Invoke the receiver's callback with its stored value and an inline vector reference.
///
/// The callback address is stored at receiver offset 8. The callback receives
/// the word at offset 12 first and a pointer to the receiver's inline value at
/// offset 16 second. It uses the C calling convention, while the receiver is
/// passed in ECX.
lf_checker_rt::export!(thiscall, rw_00abec80(this: u32) -> () {
    unsafe {
        const CALLBACK_SLOT: usize = 8;
        const VALUE_WORD: usize = 12;
        const INLINE_VALUE: usize = 0x10;
        let receiver = this as *const u8;
        let callback_address = receiver.add(CALLBACK_SLOT).cast::<u32>().read_unaligned();
        let first_value = receiver.add(VALUE_WORD).cast::<u32>().read_unaligned();
        let callback: unsafe extern "C" fn(u32, u32) =
            core::mem::transmute(callback_address as usize);
        callback(first_value, this.wrapping_add(INLINE_VALUE as u32));
    }
});
