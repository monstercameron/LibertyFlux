// original: 0x00a315f0 CEntity::vf22
/// `CEntity` float virtual: loads one float from the entity's model-info
/// record (offset 0x1C) and returns it on the x87 stack.
export!(thiscall, rw_00a315f0(ent: *const u8) -> f32 {
    unsafe {
        let model = *(ent.add(0x2E) as *const i16) as isize;
        let info = *(global::<u32>(MODEL_TABLE).offset(model)) as *const u8;
        *(info.add(0x1C) as *const f32)
    }
});
