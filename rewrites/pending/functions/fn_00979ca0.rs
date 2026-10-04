// original: 0x00979ca0 audCollisionAudioEntity::vf2
/// Release a collision audio entity's secondary buffer.
///
/// Runs the entity's first cleanup step, frees the buffer held at +0x8
/// through the global allocator and clears the slot.
export!(thiscall, rw_00979ca0(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        callee_cdecl!(2, u32, *((this.add(8)) as *const u32));
        *((this.add(8)) as *mut u32) = 0;
        0
    }
});
