// original: 0x00ABEC40 T_CB_Generic_1Arg::invoke

/// Invoke the receiver's callback with its flag normalized to a boolean.
///
/// The receiver stores its callback at byte offset 8 and a 32-bit flag at
/// byte offset 12. A zero flag is passed as zero; every nonzero value is
/// passed as one. The callback uses the C calling convention and receives
/// exactly that single word. The receiver itself is passed in ECX.
lf_checker_rt::export!(thiscall, rw_00abec40(this: u32) -> () {
    unsafe {
        const CALLBACK_SLOT: usize = 8;
        const FLAG_WORD: usize = 12;
        let receiver = this as *const u8;
        let callback_address = receiver.add(CALLBACK_SLOT).cast::<u32>().read_unaligned();
        let flag = receiver.add(FLAG_WORD).cast::<u32>().read_unaligned();
        let callback: unsafe extern "C" fn(u32) =
            core::mem::transmute(callback_address as usize);
        callback(u32::from(flag != 0));
    }
});
