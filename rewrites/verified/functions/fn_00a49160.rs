// original: 0x00a49160 vehicle_model_row_addr
/// Address of a per-model, per-index record: `row + 12*index + 0x350`.
///
/// `row` is the vehicle-info row for the signed model index at
/// `[this+0x2E]` (thiscall, one stack argument). Pure address arithmetic;
/// nothing is dereferenced past the row load.
export!(thiscall, rw_00a49160(this: u32, index: u32) -> u32 {
    unsafe {
        const MODEL_INDEX_OFF: u32 = 0x2e;
        const MODEL_TABLE: u32 = 0x01295cd8;
        let model =
            ((this.wrapping_add(MODEL_INDEX_OFF)) as *const i16).read_unaligned() as i32 as u32;
        let row = (relocated(MODEL_TABLE).wrapping_add(model.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        row.wrapping_add(index.wrapping_mul(3).wrapping_mul(4)).wrapping_add(0x350)
    }
});
