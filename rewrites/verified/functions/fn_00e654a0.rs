// original: 0x00e654a0 ratio_store_00e654a0
/// Refresh the stored ratio of the floats at 0x01038C5C / 0x01038C64.
///
/// Same shape as [`rw_00e647f0`], storing the quotient at 0x01283F30.
export!(cdecl, rw_00e654a0() -> () {
    unsafe {
        const NUM: u32 = 0x01038C5C;
        const DEN: u32 = 0x01038C64;
        const DST: u32 = 0x01283F30;
        *global::<f32>(DST) = *global::<f32>(NUM) / *global::<f32>(DEN);
    }
});
