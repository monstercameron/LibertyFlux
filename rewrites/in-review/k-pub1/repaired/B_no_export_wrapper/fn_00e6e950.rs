// original: 0x00E6E950 mainloop_timing_initialize_object_table

/// Initializes the first pointer-sized field in each timing object, walking back
/// through the fixed-stride global array from its origin.
lf_checker_rt::export!(cdecl, rw_00e6e950() -> u32 {
    let vtable = lf_checker_rt::relocated(OBJECT_VTABLE_VA);
    for element in 1..=OBJECT_COUNT {
        let address = OBJECT_ARRAY_ORIGIN_VA.wrapping_sub(element * OBJECT_STRIDE_BYTES);
        unsafe { lf_checker_rt::global::<u32>(address).write(vtable); }
    }
    lf_checker_rt::relocated(OBJECT_ARRAY_ORIGIN_VA)
        .wrapping_sub(OBJECT_COUNT * OBJECT_STRIDE_BYTES)
});

const OBJECT_ARRAY_ORIGIN_VA: u32 = 0x019d_2208;

const OBJECT_COUNT: u32 = 0x50;

const OBJECT_STRIDE_BYTES: u32 = 0x118;

const OBJECT_VTABLE_VA: u32 = 0x00fe_4c18;
