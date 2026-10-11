// original: 0x005E2300 register_style_property_names
/// Registers the fixed style-property name table once. The global count at
/// `STYLE_NAME_COUNT_VA` is an unsigned 16-bit guard: any nonzero value skips
/// every registration. On zero, IDs 0 through 38 are sent in order with the
/// corresponding relocated string pointer in EDX. The helper is fastcall and
/// has no stack arguments; its return value is ignored. This cdecl routine has
/// no meaningful return value and does not mutate the count itself.
#[unsafe(no_mangle)]
pub extern "cdecl" fn rw_register_style_property_names() {
    use lf_checker_rt::{global, relocated};

    const STYLE_NAME_COUNT_VA: u32 = 0x019D3B5A;
    const STYLE_NAME_VAS: [u32; 39] = [
        0x00F921D4, 0x00F921DC, 0x00F921E4, 0x00F921EC, 0x00F92200,
        0x00F92214, 0x00F92228, 0x00F9223C, 0x00F92244, 0x00F92250,
        0x00F92260, 0x00F92270, 0x00F92278, 0x00F92284, 0x00F92290,
        0x00F9229C, 0x00F922AC, 0x00F92028, 0x00F9203C, 0x00F92050,
        0x00F92064, 0x00F92078, 0x00F92088, 0x00F9209C, 0x00F920B0,
        0x00F920C4, 0x00F920D8, 0x00F920E8, 0x00F920FC, 0x00F92110,
        0x00F92124, 0x00F92138, 0x00F92148, 0x00F92154, 0x00F92164,
        0x00F92170, 0x00F92180, 0x00F92190, 0x00F921A0,
    ];

    // The source compares the 16-bit field, not its containing dword.
    let registered = unsafe { core::ptr::read_unaligned(global::<u16>(STYLE_NAME_COUNT_VA)) };
    if registered != 0 {
        return;
    }

    for (style_id, name_va) in STYLE_NAME_VAS.iter().copied().enumerate() {
        lf_checker_rt::callee_fastcall!(1, u32, style_id as u32, relocated(name_va));
    }
}
