// original: 0x00c07510 stream_teardown_and_free
/// Tear down a streaming object and release its memory, unless null.
///
/// A null argument does nothing. Otherwise calls the teardown helper
/// (thiscall/0) and then the release helper (cdecl/1) on the pointer, in that
/// order. Returns nothing. Cdecl: one stack word, caller cleans.
lf_checker_rt::export!(cdecl, rw_00c07510(obj: u32) -> u32 {
    unsafe {
        const TEARDOWN: u32 = 1;
        const RELEASE: u32 = 2;
        if obj != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(TEARDOWN, u32, obj);
            let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, obj);
        }
        0
    }
});
