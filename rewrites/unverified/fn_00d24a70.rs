// original: 0x00D24A70 CCustomShaderEffectVehicleFX::CCustomShaderEffectVehicleFX
/// Constructs a vehicle shader effect by delegating base setup, installing the derived virtual
/// table, clearing its cached command and resource fields, and setting the resource handle to the
/// invalid sentinel. The thiscall method accepts one stack parameter and returns the object
/// pointer.
lf_checker_rt::export!(thiscall, rw_00d24a70(this: u32, base_parameter: u32) -> u32 {
    const VTABLE_VA: u32 = 0x00EE1374;
    const RESET_HANDLE: u32 = 0xFFFFFFFF;
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, base_parameter);
        write_u32(this, 0, lf_checker_rt::relocated(VTABLE_VA));
        write_u32(this, 0xf4, 0);
        write_u32(this, 0xd8, RESET_HANDLE);
        write_u32(this, 0x108, 0);
        write_u32(this, 0x100, 0);
        write_u32(this, 0x104, 0);
        this
    }
});
