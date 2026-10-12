// original: 0x00a319b0 CEntity::vf25
/// `CEntity` pointer virtual: returns the address of the second vector in
/// the entity's model-info record (offset 0x30).
export!(thiscall, rw_00a319b0(ent: *const u8) -> u32 {
    unsafe {
        let model = *(ent.add(0x2E) as *const i16) as isize;
        let info = *(global::<u32>(MODEL_TABLE).offset(model)) as *const u8;
        info.add(0x30) as u32
    }
});

/// File VA of the runtime-filled model-info pointer table, indexed by the
/// entity's signed 16-bit model index (`model_index` at entity+0x2E).
/// (Carried over verbatim from lane r-s65's crate root; used by 4 rewrites.)
const MODEL_TABLE: u32 = 0x1295CD8;
