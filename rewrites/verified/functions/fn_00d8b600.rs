// original: 0x00D8B600 T_CB_Generic_2Args::invoke

/// Invokes the callback pointer stored at `this + 8`, forwarding the words at
/// `this + 0x0c` and `this + 0x10` in that order. The callback uses cdecl, so
/// the wrapper removes its two pushed words after the call and returns the
/// callback's EAX result. The proof plants a checker stand-in in the callback
/// field so this rewrite calls the same scripted endpoint as the original.
lf_checker_rt::export!(thiscall, rw_00d8b600(this: u32) -> u32 {
    const CALLBACK: u32 = 0x08;
    const FIRST_VALUE: u32 = 0x0c;
    const SECOND_VALUE: u32 = 0x10;

    unsafe {
        let callback_word = (this.wrapping_add(CALLBACK) as *const u32).read_unaligned();
        let first_value = (this.wrapping_add(FIRST_VALUE) as *const u32).read_unaligned();
        let second_value = (this.wrapping_add(SECOND_VALUE) as *const u32).read_unaligned();
        let callback: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callback_word as usize);
        callback(first_value, second_value)
    }
});
