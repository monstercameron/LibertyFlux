// original: 0x00afdc40 select_float_by_flag_bit
/// Return one of two constant floats selected by bit 0x80 of the flag byte
/// at offset 6 of the object.
export!(thiscall, rw_00afdc40(this: u32) -> f32 {
    unsafe {
        let flag = *((this + 6) as *const u8);
        if flag & 0x80 != 0 {
            *global::<f32>(0xfe8ab8)
        } else {
            *global::<f32>(0xe833d4)
        }
    }
});
