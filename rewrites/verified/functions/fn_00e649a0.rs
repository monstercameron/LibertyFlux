// original: 0x00e649a0 ratio_store_00e649a0
/// Refresh the stored ratio of the floats at 0x01038A9C / 0x01038AA0.
///
/// Same shape as [`rw_00e647f0`], storing the quotient at 0x01282F40.
export!(cdecl, rw_00e649a0() -> () {
    unsafe {
        const NUM: u32 = 0x01038A9C;
        const DEN: u32 = 0x01038AA0;
        const DST: u32 = 0x01282F40;
        *global::<f32>(DST) = *global::<f32>(NUM) / *global::<f32>(DEN);
    }
});
