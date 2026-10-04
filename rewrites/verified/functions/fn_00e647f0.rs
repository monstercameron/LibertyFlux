// original: 0x00e647f0 ratio_store_00e647f0
/// Refresh the stored ratio of the floats at 0x01038A28 / 0x01038A2C.
///
/// Divides the first global float by the second and stores the quotient at
/// 0x01238838. Plain `f32` division, matching `divss` bit for bit.
export!(cdecl, rw_00e647f0() -> () {
    unsafe {
        const NUM: u32 = 0x01038A28;
        const DEN: u32 = 0x01038A2C;
        const DST: u32 = 0x01238838;
        *global::<f32>(DST) = *global::<f32>(NUM) / *global::<f32>(DEN);
    }
});
