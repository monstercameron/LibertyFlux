// original: 0x00a49470 vehicle_info_field_3bc
/// The dword at offset 0x3BC of this model's vehicle-info row.
///
/// The row comes from the model table via the signed index at `[this+0x2E]`
/// (thiscall, no stack arguments).
export!(thiscall, rw_00a49470(this: u32) -> u32 {
    unsafe {
        const MODEL_INDEX_OFF: u32 = 0x2e;
        const MODEL_TABLE: u32 = 0x01295cd8;
        let model =
            ((this.wrapping_add(MODEL_INDEX_OFF)) as *const i16).read_unaligned() as i32 as u32;
        let row = (relocated(MODEL_TABLE).wrapping_add(model.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        (row.wrapping_add(0x3bc) as *const u32).read_unaligned()
    }
});
