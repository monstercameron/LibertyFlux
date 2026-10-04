// original: 0x00e65980 ratio_store_00e65980
/// Refresh the stored ratio of the floats at 0x01038E14 / 0x01038E18.
///
/// Same shape as [`rw_00e647f0`], storing the quotient at 0x01284574.
export!(cdecl, rw_00e65980() -> () {
    unsafe {
        const NUM: u32 = 0x01038E14;
        const DEN: u32 = 0x01038E18;
        const DST: u32 = 0x01284574;
        *global::<f32>(DST) = *global::<f32>(NUM) / *global::<f32>(DEN);
    }
});
