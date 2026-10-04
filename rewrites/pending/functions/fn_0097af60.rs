// original: 0x0097af60 audPedAudioEntity::vf0
/// Destroy a ped audio entity, freeing it when the flag asks.
///
/// Runs the entity destructor, then frees the object through the global
/// allocator when the low bit of `flags` is set. Returns the object.
export!(thiscall, rw_0097af60(this: *mut u8, flags: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        if flags & 1 != 0 {
            callee_cdecl!(2, u32, this as u32);
        }
        this as u32
    }
});
