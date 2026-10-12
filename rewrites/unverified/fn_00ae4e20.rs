// original: 0x00AE4E20 T_CB_Generic_3Args<void(*)(int, const rage::Matrix34&, unsigned char), int, rage::Matrix34, unsigned char>::invoke

/// Invoke the stored cdecl callback with an integer, a pointer to the embedded matrix, and a zero-extended byte.
/// The callback pointer is stored at object offset `+0x08` and the
/// object is passed in ECX. The callback uses cdecl and its return
/// value is returned unchanged in EAX. The contract plants its scripted
/// stand-in at that callback slot; the runtime call names the same stand-in.
lf_checker_rt::export!(thiscall, rw_00ae4e20(this: u32) -> u32 {
    const CALLBACK_ID: u32 = 1;
    let first_value = unsafe { (this.wrapping_add(0xC) as *const u32).read_unaligned() };
    const EMBEDDED_MATRIX: u32 = 0x10;
    let matrix_pointer = unsafe { this.wrapping_add(EMBEDDED_MATRIX) };
    let third_value = unsafe { (this.wrapping_add(0x50) as *const u8).read() as u32 };
    lf_checker_rt::callee_cdecl!(CALLBACK_ID, u32,
        first_value,
        matrix_pointer,
        third_value,
    )
});
