// original: 0x00a32090 entity_subobject_by_flag
/// Sub-object selector: reads bits 6..10 of the entity flag word
/// (`m_nEntityFlags2` at +0x28) and returns the address of the matching
/// embedded sub-object (selector 3, 2, 4), or null for any other value.
export!(thiscall, rw_00a32090(ent: *const u8) -> u32 {
    unsafe {
        match ((*(ent.add(0x28) as *const u32) >> 6) & 0xF) {
            3 => ent.add(0x780) as u32,
            2 => ent.add(0xDB8) as u32,
            4 => ent.add(0x264) as u32,
            _ => 0,
        }
    }
});
