// original: 0x00D25D00 CCustomShaderEffectVehicleFX::vf3
/// Updates a vehicle shader effect from shader data. When the masked mode equals 0x80 it
/// multiplies the primary raw float by the relocated scale constant and stores the bits; a flag
/// may instead set the unit sentinel and call the update helper. Other modes query the shader
/// data's virtual method and, when AL is nonzero, scale its secondary float. The thiscall method
/// takes one data pointer, uses named field offsets, and preserves the original multiplication
/// order.
lf_checker_rt::export!(thiscall, rw_00d25d00(this: u32, shader_data: u32) -> () {
    const MODE_FLAGS: u32 = 0x28;
    const MODE_MASK: u32 = 0x3c0;
    const MODE_TRIGGER: u32 = 0x80;
    const PRIMARY_FLOAT: u32 = 0x10c8;
    const SECONDARY_FLOAT: u32 = 0x2b4;
    const UPDATE_FLAG: u32 = 0x11a;
    const SCALE_VA: u32 = 0x00fe877c;
    const DEST_VALUE: u32 = 0x74;
    const DEST_ACTIVE: u32 = 0x78;
    unsafe fn read_u32(base: u32, offset: u32) -> u32 {
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    }
    unsafe fn write_u32(base: u32, offset: u32, value: u32) {
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    }
    unsafe fn write_u8(base: u32, offset: u32, value: u8) {
        unsafe { (base.wrapping_add(offset) as *mut u8).write(value) }
    }
    unsafe fn read_u8(base: u32, offset: u32) -> u8 {
        unsafe { (base.wrapping_add(offset) as *const u8).read() }
    }
    fn scaled_bits(value_bits: u32) -> u32 {
        let scale_bits = unsafe { lf_checker_rt::global::<u32>(SCALE_VA).read_unaligned() };
        let value = f32::from_bits(value_bits);
        let scale = f32::from_bits(scale_bits);
        (core::hint::black_box(value) * core::hint::black_box(scale)).to_bits()
    }
    unsafe {
        if read_u32(shader_data, MODE_FLAGS) & MODE_MASK == MODE_TRIGGER {
            let value_bits = read_u32(shader_data, PRIMARY_FLOAT);
            write_u32(this, DEST_VALUE, scaled_bits(value_bits));
            if read_u8(shader_data, UPDATE_FLAG) & 1 != 0 {
                write_u8(this, DEST_ACTIVE, 1);
                write_u32(this, DEST_VALUE, 0x3f800000);
                let _ = lf_checker_rt::callee_thiscall!(1, u32, this, shader_data, u32::MAX);
            } else {
                write_u8(this, DEST_ACTIVE, 0);
            }
        } else {
            let vtable = read_u32(shader_data, 0);
            let target = read_u32(vtable, 0x28);
            let predicate: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
            if predicate(shader_data) as u8 != 0 {
                let value_bits = read_u32(shader_data, SECONDARY_FLOAT);
                write_u32(this, DEST_VALUE, scaled_bits(value_bits));
            }
        }
    }
});
