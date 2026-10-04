// original: 0x00e64a20 ratio_store_00e64a20
/// Refresh the stored ratio of the floats at 0x01038B10 / 0x01038B14.
///
/// Same shape as [`rw_00e647f0`], storing the quotient at 0x0128304C.
export!(cdecl, rw_00e64a20() -> () {
    unsafe {
        const NUM: u32 = 0x01038B10;
        const DEN: u32 = 0x01038B14;
        const DST: u32 = 0x0128304C;
        *global::<f32>(DST) = *global::<f32>(NUM) / *global::<f32>(DEN);
    }
});
