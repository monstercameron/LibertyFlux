// original: 0x009FD190 frag_tune_singleton_create (proposed)

/// Initialise the frag-tune singleton object and optionally delete it.
///
/// Merged symbol: `atSingleton<rage::fragTuneStruct>::vf0`. Writes the
/// singleton's function table into `obj`, runs the base initialiser callee
/// on it, and when bit 0 of `flags` is set passes `obj` to the shared free
/// callee. Returns `obj`.
///
/// Original: 0x009FD190 (thiscall, `obj` in `ecx`, `flags` one stack word).
lf_checker_rt::export!(thiscall, rw_009FD190(obj: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E99B04;
        (obj as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(1, u32, obj);
        if flags & 1 != 0 {
            lf_checker_rt::callee_cdecl!(2, u32, obj);
        }
        obj
    }
});
