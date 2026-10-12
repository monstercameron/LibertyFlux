// original: 0x008C1390 rage::audOcclusionGroupManager::~audOcclusionGroupManager__deleting

/// Destroy an occlusion group manager, freeing it when asked.
///
/// Writes the class vtable, then when bit 0 of `flags` is set releases the
/// object through the audio allocator. Returns the object pointer.
/// Thiscall, object in ECX, flags popped from the stack.
lf_checker_rt::export!(thiscall, rw_008C1390(this: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00E7_E850;
        const DELETE: u32 = 1;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if flags & DELETE != 0 {
            lf_checker_rt::callee_cdecl!(1, u32, this);
        }
        this
    }
});
