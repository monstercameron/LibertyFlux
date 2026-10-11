// original: 0x00E6E950 mainloop_timing_initialize_object_table

/// Writes the relocated vtable pointer into the first word of each of eighty
/// fixed-stride objects, walking backward from the array origin, and returns
/// the address of the first object initialized.
pub extern "cdecl" fn rw_00e6e950() -> u32 {
    const ARRAY_ORIGIN: u32 = 0x019d_2208;
    const OBJECT_STRIDE: u32 = 0x118;
    const OBJECT_COUNT: u32 = 0x50;
    const OBJECT_VTABLE: u32 = 0x00fe_4c18;

    let vtable = lf_checker_rt::relocated(OBJECT_VTABLE);
    for index in 1..=OBJECT_COUNT {
        let address = ARRAY_ORIGIN.wrapping_sub(index * OBJECT_STRIDE);
        unsafe { lf_checker_rt::global::<u32>(address).write(vtable); }
    }
    lf_checker_rt::relocated(ARRAY_ORIGIN)
        .wrapping_sub(OBJECT_COUNT * OBJECT_STRIDE)
}
