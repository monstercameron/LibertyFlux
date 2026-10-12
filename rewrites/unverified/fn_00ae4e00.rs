// original: 0x00AE4E00 T_CB_Generic_2Args<void(*)(bool, bool), bool, bool>::invoke

/// Invoke the stored cdecl callback with two byte values zero-extended to 32 bits.
/// The callback pointer is stored at object offset `+0x08` and the
/// object is passed in ECX. The callback uses cdecl and its return
/// value is returned unchanged in EAX. The contract plants its scripted
/// stand-in at that callback slot; the runtime call names the same stand-in.
lf_checker_rt::export!(thiscall, rw_00ae4e00(this: u32) -> u32 {
    const CALLBACK_ID: u32 = 1;
    let first_value = unsafe { (this.wrapping_add(0xC) as *const u8).read() as u32 };
    let second_value = unsafe { (this.wrapping_add(0xD) as *const u8).read() as u32 };
    lf_checker_rt::callee_cdecl!(CALLBACK_ID, u32,
        first_value,
        second_value,
    )
});
