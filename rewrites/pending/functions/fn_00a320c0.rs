// original: 0x00a320c0 entity_placed_on_static_matrix
/// Placement test: reports whether the entity's placement matrix pointer
/// (`placement_matrix34` at +0x20) is the shared static matrix. The compared
/// address is relocated with the image, so it is derived, never literal.
export!(thiscall, rw_00a320c0(ent: *const u8) -> u32 {
    unsafe {
        u32::from(*(ent.add(0x20) as *const u32) == relocated(0x12DDF20))
    }
});
