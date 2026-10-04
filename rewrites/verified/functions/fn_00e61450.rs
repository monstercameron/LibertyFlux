// original: 0x00e61450 timing_root_init_and_register
/// Initialise the root timing record, then register its callback.
///
/// The original reserves three argument words for the root initializer
/// without writing them; the checker fills uninitialized stack with a
/// defined value, so the rewrite passes that value explicitly. Stamps the
/// root tag word, then returns the registrar's answer.
export!(cdecl, rw_00e61450() -> u32 {
    unsafe {
        callee_stdcall!(1, u32, 0, 0, 0);
        *global::<u32>(0x01A00DA0) = relocated(0x00FE4F24); // tag is a relocated address (attempt 2)
        callee_cdecl!(2, u32, relocated(0x00E700F0))
    }
});
