// original: 0x00a13280 null_guarded_query (proposed)
/// Query the table when the handle is non-null, else return 0.
///
/// Returns 0 for a null `handle`. Otherwise calls the query callee with
/// two zero arguments and returns its result. Stdcall, one argument.
export!(stdcall, rw_00a13280(handle: u32) -> u32 {
    unsafe {
        const QUERY: u32 = 1;
        if handle == 0 {
            0
        } else {
            callee_cdecl!(QUERY, u32, 0, 0)
        }
    }
});
