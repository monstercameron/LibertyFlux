// original: 0x00a319c0 CEntity::vf24
/// `CEntity` pointer virtual: returns the address of the first vector in
/// the entity's model-info record (offset 0x20).
export!(thiscall, rw_00a319c0(ent: *const u8) -> u32 {
    unsafe {
        let model = *(ent.add(0x2E) as *const i16) as isize;
        let info = *(global::<u32>(MODEL_TABLE).offset(model)) as *const u8;
        info.add(0x20) as u32
    }
});
