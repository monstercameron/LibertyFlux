// original: 0x00B62FF0 veh_handle_float_18
/// Load a float through the object handle kept at `[this+0x18]`.
///
/// Calls the handle resolver (stubbed, cdecl/1) with `[this+0x18]` and returns
/// the single-precision value at offset 0x18 of the resolved object, loaded
/// with `fld` (returned in ST0, widened exactly to f64). Thiscall; the one
/// stack word is ignored (popped by the callee pops 4 bytes). Entry registers except ECX are
/// ignored.
export!(thiscall, rw_00b62ff0(this: u32, _a0: u32) -> f64 {
    unsafe {
        const HANDLE: u32 = 0x18;
        const FIELD: u32 = 0x18;
        let h: u32 = callee_cdecl!(1, u32, ((this + HANDLE) as *const u32).read_unaligned());
        f32::from_bits(((h + FIELD) as *const u32).read_unaligned()) as f64
    }
});
