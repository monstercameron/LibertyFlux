// original: 0x0093f1e0 stream_selected_sub seventy (proposed)

/// Return the selected object's auxiliary block, or null.
///
/// Calls the selected-slot getter; when it yields a live object, follows
/// the pointer at offset `OBJ_AUX`. A null at either step returns null.
/// Otherwise returns the auxiliary pointer plus `AUX_BIAS` (no further
/// dereference).
///
/// Original: 0x0093f1e0 (cdecl, no arguments; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093f1e0() -> u32 {
    const SELECTED_GETTER: u32 = 1;
    const OBJ_AUX: u32 = 0x228;
    const AUX_BIAS: u32 = 0x70;
    unsafe {
        let obj: u32 = lf_checker_rt::callee_cdecl!(SELECTED_GETTER, u32,);
        if obj == 0 {
            return 0;
        }
        let aux = ((obj + OBJ_AUX) as *const u32).read_unaligned();
        if aux == 0 {
            return 0;
        }
        aux.wrapping_add(AUX_BIAS)
    }
});
