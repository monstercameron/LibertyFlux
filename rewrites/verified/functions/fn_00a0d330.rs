// original: 0x00A0D330 CEntity::get_position
/// Copies four dwords of position data to the output. A non-null transform
/// pointer selects its position at +0x30; otherwise the entity's inline
/// position at +0x10 is used. The fourth copied word is returned in EAX.
lf_checker_rt::export!(thiscall, rw_00A0D330(entity: u32, output: u32) -> u32 {
    unsafe {
        const TRANSFORM: u32 = 0x20;
        const INLINE_POSITION: u32 = 0x10;
        const TRANSFORM_POSITION: u32 = 0x30;
        let transform = (entity.wrapping_add(TRANSFORM) as *const u32).read_unaligned();
        let source = if transform == 0 { entity.wrapping_add(INLINE_POSITION) } else { transform.wrapping_add(TRANSFORM_POSITION) };
        let mut last = 0;
        for offset in [0u32, 4, 8, 12] {
            last = (source.wrapping_add(offset) as *const u32).read_unaligned();
            (output.wrapping_add(offset) as *mut u32).write_unaligned(last);
        }
        last
    }
});
