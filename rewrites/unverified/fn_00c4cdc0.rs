// original: 0x00C4CDC0 CCamFpsWeapon::vf7
/// Release the optional owned pointer stored at `this + 0x140`, then clear
/// that field and return true in AL. When non-null, the release helper receives
/// the pointed-to object in ECX and the address of the field as its sole stack
/// argument; the call is intercepted by the proof contract. The method takes
/// `this` in ECX and has no stack arguments.
lf_checker_rt::export!(thiscall, rw_c4cdc0(this: u32) -> u32 {
    unsafe {
        const OWNED_POINTER: u32 = 0x140;
        let field = this.wrapping_add(OWNED_POINTER);
        let object = (field as *const u32).read_unaligned();
        if object != 0 {
            let _ = lf_checker_rt::callee_thiscall!(1, u32, object, field);
        }
        (field as *mut u32).write_unaligned(0);
        1
    }
});
