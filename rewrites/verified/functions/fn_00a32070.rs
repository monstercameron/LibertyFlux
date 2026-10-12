// original: 0x00a32070 NativeImpl_GET_CAR_MASS
/// Physics-object fetch behind `GET_CAR_MASS`.
///
/// Returns the live object behind the entity's rigid-body record when the
/// record and its object link are both present, and otherwise falls back to
/// the model-info word at offset 4 for the entity's model index.
export!(thiscall, rw_00a32070(ent: *const u8) -> u32 {
    unsafe {
        let body = *(ent.add(0x38) as *const *const u8);
        if !body.is_null() {
            let obj = *(body.add(4) as *const u32);
            if obj != 0 {
                return obj;
            }
        }
        let model = *(ent.add(0x2E) as *const i16) as isize;
        let info = *(global::<u32>(MODEL_TABLE).offset(model)) as *const u8;
        *(info.add(4) as *const u32)
    }
});

/// File VA of the runtime-filled model-info pointer table, indexed by the
/// entity's signed 16-bit model index (`model_index` at entity+0x2E).
/// (Carried over verbatim from lane r-s65's crate root; used by 4 rewrites.)
const MODEL_TABLE: u32 = 0x1295CD8;
