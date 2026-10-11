// original: 0x00A95A60 CLoader::~CLoader
/// Replaces the loader object's vtable with its base-cleanup table. When the
/// deleting flag's low bit is set, it forwards the object to the cleanup
/// callee. The object pointer is `this`; the flag is one stack word, and the
/// function returns `this` after the optional call.
lf_checker_rt::export!(thiscall, rw_00A95A60(this: u32, deleting: u32) -> u32 {
    unsafe {
        const CLEANUP_VTABLE: u32 = 0x00EA374C;
        (this as *mut u32).write(lf_checker_rt::relocated(CLEANUP_VTABLE));
        if deleting & 1 != 0 {
            let _ = lf_checker_rt::callee_cdecl!(1, u32, this);
        }
        this
    }
});
