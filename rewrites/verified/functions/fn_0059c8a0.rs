// original: 0x0059C8A0 CNetBlenderDummyPed::vf4

/// Send the two blend buffers and the entity position to three virtual slots.
///
/// The `this` object holds its vtable at `+0`, the entity at `+0xC0`, the
/// float argument bits at `+0xCC`, and a byte flag at `+0xB4`. Slots `+0x38`
/// and `+0x3C` receive pointers to the buffers at `+0x50` and `+0x60`; the
/// entity's slot `+0x0C` receives the stored float bits. The flag is cleared
/// before the last call, whose return value is returned unchanged.
lf_checker_rt::export!(thiscall, rw_0059c8a0(this: u32) -> u32 {
    const VTABLE_POINTER: u32 = 0x00;
    const FIRST_BUFFER: u32 = 0x50;
    const SECOND_BUFFER: u32 = 0x60;
    const ENTITY_POINTER: u32 = 0xC0;
    const READY_FLAG: u32 = 0xB4;
    const FLOAT_BITS: u32 = 0xCC;
    const FIRST_VTABLE_SLOT: u32 = 0x38;
    const SECOND_VTABLE_SLOT: u32 = 0x3C;
    const ENTITY_VTABLE_SLOT: u32 = 0x0C;
    const FIRST_MODE: u32 = 1;
    const CLEARED: u8 = 0;

    unsafe {
        let first_vtable = (this.wrapping_add(VTABLE_POINTER) as *const u32).read_unaligned();
        let first_slot = (first_vtable.wrapping_add(FIRST_VTABLE_SLOT) as *const u32).read_unaligned();
        let _ = core::hint::black_box(first_slot);
        let first_buffer = this.wrapping_add(FIRST_BUFFER);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, first_buffer, FIRST_MODE);

        let second_vtable = (this.wrapping_add(VTABLE_POINTER) as *const u32).read_unaligned();
        let second_slot = (second_vtable.wrapping_add(SECOND_VTABLE_SLOT) as *const u32).read_unaligned();
        let _ = core::hint::black_box(second_slot);
        let second_buffer = this.wrapping_add(SECOND_BUFFER);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, second_buffer);

        let entity = (this.wrapping_add(ENTITY_POINTER) as *const u32).read_unaligned();
        let entity_vtable = (entity as *const u32).read_unaligned();
        let entity_slot = (entity_vtable.wrapping_add(ENTITY_VTABLE_SLOT) as *const u32).read_unaligned();
        let _ = core::hint::black_box(entity_slot);
        let float_bits = (this.wrapping_add(FLOAT_BITS) as *const u32).read_unaligned();
        (this.wrapping_add(READY_FLAG) as *mut u8).write_unaligned(CLEARED);
        lf_checker_rt::callee_thiscall!(3, u32, entity, float_bits)
    }
});
