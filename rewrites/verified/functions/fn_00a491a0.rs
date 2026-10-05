// original: 0x00a491a0 CVehicle::vf105
/// The float at row offset 0x28, scaled by the shared constant (-1.0).
///
/// The row comes from the model table via the signed index at `[this+0x2E]`
/// (thiscall, no stack arguments). One `mulss`; the result returns in ST0.
export!(thiscall, rw_00a491a0(this: u32) -> f32 {
    unsafe {
        const MODEL_INDEX_OFF: u32 = 0x2e;
        const MODEL_TABLE: u32 = 0x01295cd8;
        let model =
            ((this.wrapping_add(MODEL_INDEX_OFF)) as *const i16).read_unaligned() as i32 as u32;
        let row = (relocated(MODEL_TABLE).wrapping_add(model.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let a = f32::from_bits((row.wrapping_add(0x28) as *const u32).read_unaligned());
        let b = f32::from_bits((relocated(0x00fe8d94) as *const u32).read_unaligned());
        core::hint::black_box(a) * core::hint::black_box(b)
    }
});
