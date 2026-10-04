// original: 0x00e64900 ratio_store_00e64900
/// Refresh the stored ratio of the floats at 0x01038A58 / 0x01038A5C.
///
/// Same shape as [`rw_00e647f0`], storing the quotient at 0x012388F0.
export!(cdecl, rw_00e64900() -> () {
    unsafe {
        const NUM: u32 = 0x01038A58;
        const DEN: u32 = 0x01038A5C;
        const DST: u32 = 0x012388F0;
        *global::<f32>(DST) = *global::<f32>(NUM) / *global::<f32>(DEN);
    }
});
