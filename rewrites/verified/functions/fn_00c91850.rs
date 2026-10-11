// original: 0x00c91850 fn_00c91850_construct_unit_transform

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_global_f32(file_va: u32) -> f32 {
    let bits = unsafe { read_u32(lf_checker_rt::relocated(file_va)) };
    f32::from_bits(bits)
}

/// Initialize a transform-like object from fixed executable data: install its
/// vtable, clear the status and tail fields, and copy three unit values. The
/// thiscall result is the object pointer.
const UNIT_TRANSFORM_VTABLE: u32 = 0x00ED6ECC;
const UNIT_X_SOURCE: u32 = 0x0110DB10;
const UNIT_Y_SOURCE: u32 = 0x0110DB14;
const UNIT_Z_SOURCE: u32 = 0x0110DB18;

unsafe fn initialize_unit_transform(object: u32, corrupt_y: bool) {
    unsafe {
        write_u32(object, lf_checker_rt::relocated(UNIT_TRANSFORM_VTABLE));
        write_u32(object + 0x10, 0);
        write_u32(object + 0x20, read_u32(lf_checker_rt::relocated(UNIT_X_SOURCE)));
        write_u32(object + 0x24, if corrupt_y { 0 } else { read_u32(lf_checker_rt::relocated(UNIT_Y_SOURCE)) });
        write_u32(object + 0x28, read_u32(lf_checker_rt::relocated(UNIT_Z_SOURCE)));
        write_u32(object + 0x30, 0);
    }
}

lf_checker_rt::export!(thiscall, rw_00c91850_construct_unit_transform(object: u32) -> u32 {
    unsafe { initialize_unit_transform(object, false); }
    object
});
