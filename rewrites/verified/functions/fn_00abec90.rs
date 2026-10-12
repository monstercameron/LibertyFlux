// original: 0x00ABEC90 T_CB_Generic_6Args::invoke

/// Forward six stored arguments to the receiver's callback.
///
/// The callback address is at receiver offset 8. Arguments are forwarded in
/// call order: the word at offset 12, a pointer to inline data at offset 16,
/// words at offsets 80 and 84, the signed byte at offset 88 extended to a
/// signed 32-bit value, then the word at offset 92. The callback uses the C
/// calling convention and the receiver is passed in ECX.
lf_checker_rt::export!(thiscall, rw_00abec90(this: u32) -> () {
    unsafe {
        const CALLBACK_SLOT: usize = 8;
        const FIRST_WORD: usize = 12;
        const INLINE_VALUE: usize = 0x10;
        const THIRD_WORD: usize = 0x50;
        const FOURTH_WORD: usize = 0x54;
        const SIGNED_BYTE: usize = 0x58;
        const SIXTH_WORD: usize = 0x5c;
        let receiver = this as *const u8;
        let callback_address = receiver.add(CALLBACK_SLOT).cast::<u32>().read_unaligned();
        let arg0 = receiver.add(FIRST_WORD).cast::<u32>().read_unaligned();
        let arg2 = receiver.add(THIRD_WORD).cast::<u32>().read_unaligned();
        let arg3 = receiver.add(FOURTH_WORD).cast::<u32>().read_unaligned();
        let arg4 = receiver.add(SIGNED_BYTE).cast::<i8>().read() as i32;
        let arg5 = receiver.add(SIXTH_WORD).cast::<u32>().read_unaligned();
        let callback: unsafe extern "C" fn(u32, u32, u32, u32, i32, u32) =
            core::mem::transmute(callback_address as usize);
        callback(arg0, this.wrapping_add(INLINE_VALUE as u32), arg2, arg3, arg4, arg5);
    }
});
