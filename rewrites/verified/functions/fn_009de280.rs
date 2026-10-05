// original: 0x009DE280 pool_obj_create_13880 (proposed)

/// Allocate a 0x1C-byte pool object, initialise it, and publish it globally.
///
/// On allocation failure the global slot is set to null and 0 is returned.
/// Otherwise the object is initialised with the constant parameters and the
/// resulting pointer is stored in the global slot and returned.
///
/// Original: 0x009DE280 (cdecl, no arguments, two outgoing calls).
lf_checker_rt::export!(cdecl, rw_009DE280() -> u32 {
    unsafe {
        const OBJ_SIZE: u32 = 0x1C;
        const PARAM_A: u32 = 0x13880;
        const VTABLE_VA: u32 = 0x00E97EA0;
        const PARAM_C: u32 = 0x8;
        const SLOT_VA: u32 = 0x012B4164;
        const ALLOC: u32 = 1;
        const INIT: u32 = 2;

        let buf: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, OBJ_SIZE);
        if buf == 0 {
            lf_checker_rt::global::<u32>(SLOT_VA).write_unaligned(0);
            return 0;
        }
        let obj: u32 = lf_checker_rt::callee_thiscall!(
            INIT, u32, buf, PARAM_A, lf_checker_rt::relocated(VTABLE_VA), PARAM_C
        );
        lf_checker_rt::global::<u32>(SLOT_VA).write_unaligned(obj);
        obj
    }
});
