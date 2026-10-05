// original: 0x00b77af0 handle_activate_or_create (proposed)

/// Activate the handle for `key`, creating and wiring it when missing.
///
/// Looks the handle up (cdecl, one stack word: `key`). Found means running
/// the start callee (thiscall on the handle, one stack word: `key`) and
/// returning its answer. Missing means resolving a fresh object (cdecl, no
/// words); a null resolution returns 0. An object whose first word is
/// already set is wired at once (thiscall, pushed 0 then `key`). Otherwise
/// it is initialised first (thiscall, one stack word 7): still zero means
/// returning the initialise answer, set means wiring it too.
///
/// Original: 0x00b77af0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b77af0(key: u32) -> u32 {
    unsafe {
        const LOOKUP: u32 = 1;
        const START: u32 = 2;
        const RESOLVE: u32 = 3;
        const INIT: u32 = 4;
        const WIRE: u32 = 5;
        const INIT_ARG: u32 = 7;
        let handle: u32 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, key);
        if handle != 0 {
            return lf_checker_rt::callee_thiscall!(START, u32, handle, key);
        }
        let obj: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32,);
        if obj == 0 {
            return 0;
        }
        if (obj as *const u32).read_unaligned() != 0 {
            return lf_checker_rt::callee_thiscall!(WIRE, u32, obj, key, 0);
        }
        let ans: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, obj, INIT_ARG);
        if (obj as *const u32).read_unaligned() == 0 {
            return ans;
        }
        // Reached only when INIT sets the object's first word; the stubbed
        // INIT never does, so this path never fires in the proof.
        lf_checker_rt::callee_thiscall!(WIRE, u32, obj, key, 0)
    }
});
