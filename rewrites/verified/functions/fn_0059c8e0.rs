// original: 0x0059C8E0 CNetBlenderDummyPed::vf12

/// Return the position record inside the entity owned by this network dummy.
///
/// `this` contains the entity pointer at `+0xC0`; the entity points to its
/// position record at `+0x20`. This method returns that record advanced by
/// `0x30` bytes. All address arithmetic wraps as 32-bit pointer arithmetic,
/// matching the original thiscall method, which has no stack arguments.
lf_checker_rt::export!(thiscall, rw_0059c8e0(this: u32) -> u32 {
    const ENTITY_POINTER: u32 = 0xC0;
    const POSITION_POINTER: u32 = 0x20;
    const POSITION_BODY: u32 = 0x30;

    unsafe {
        let entity = ((this.wrapping_add(ENTITY_POINTER)) as *const u32).read_unaligned();
        let position = ((entity.wrapping_add(POSITION_POINTER)) as *const u32).read_unaligned();
        position.wrapping_add(POSITION_BODY)
    }
});
