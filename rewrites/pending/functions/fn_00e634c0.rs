// original: 0x00e634c0 f32_ratio_store_1
/// Store the ratio of two f32 globals into a third: `C = A / B`.
///
/// All three operands live in writable game data; the pristine inputs are
/// 1024.0 and 768.0, so the pristine result is 4/3.
export!(cdecl, rw_00e634c0() -> () {
    unsafe {
        let a = *global::<f32>(0x1034664);
        let b = *global::<f32>(0x103668C);
        *global::<f32>(0x119BF10) = a / b;
    }
});
