// original: 0x00A0D3B0 building_swap_model
/// Calls the object's first virtual method unconditionally. If its status
/// word at +0x38 is nonzero, it calls a second virtual method and a direct
/// thiscall helper with a zero argument. It then copies the low input word
/// to +0x2e and returns that same 16-bit value.
lf_checker_rt::export!(thiscall, rw_00A0D3B0(building: u32, model_arg: u32) -> u16 {
    unsafe {
        const STATUS: u32 = 0x38;
        const MODEL_FIELD: u32 = 0x2E;

        let _ = lf_checker_rt::callee_thiscall!(1, u32, building);
        if (building.wrapping_add(STATUS) as *const u32).read_unaligned() != 0 {
            let _ = lf_checker_rt::callee_thiscall!(2, u32, building);
            let _ = lf_checker_rt::callee_thiscall!(3, u32, building, 0);
        }
        (building.wrapping_add(MODEL_FIELD) as *mut u16).write_unaligned(model_arg as u16);
        model_arg as u16
    }
});
