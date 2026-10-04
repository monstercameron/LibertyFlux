// original: 0x00e16518 write_global_or_error
/// Store the shared value through `out`, or report a null argument.
///
/// When `out` is null, the function asks the error-slot routine (cdecl/0,
/// stubbed as id 1) for a slot, records the null-argument code there, runs
/// the error-report routine (cdecl/0, stubbed as id 2) and returns the code.
/// Otherwise it copies the shared module global into `*out` and returns 0.
export!(cdecl, rw_00e16518(out: *mut u32) -> u32 {
    unsafe {
        const VALUE_GLOBAL: u32 = 0x17AC444;
        const NULL_ARG_CODE: u32 = 0x16;
        if out.is_null() {
            let slot = callee_cdecl!(1, u32,) as *mut u32;
            *slot = NULL_ARG_CODE;
            let _: u32 = callee_cdecl!(2, u32,);
            return NULL_ARG_CODE;
        }
        *out = *global::<u32>(VALUE_GLOBAL);
        0
    }
});
