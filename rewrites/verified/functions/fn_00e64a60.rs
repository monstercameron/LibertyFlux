// original: 0x00e64a60 ratio_store_00e64a60
/// Refresh the stored ratio of the floats at 0x01038B40 / 0x01038B44.
///
/// Same shape as [`rw_00e647f0`], storing the quotient at 0x01283204.
export!(cdecl, rw_00e64a60() -> () {
    unsafe {
        const NUM: u32 = 0x01038B40;
        const DEN: u32 = 0x01038B44;
        const DST: u32 = 0x01283204;
        *global::<f32>(DST) = *global::<f32>(NUM) / *global::<f32>(DEN);
    }
});
