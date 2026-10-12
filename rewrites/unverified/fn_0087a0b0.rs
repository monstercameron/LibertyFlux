// original: 0x0087A0B0 rage::crmtComposerOptimized::remap_animation_track

/// Dispatch a remap request for the track at a composer's current list count.
///
/// `this` points to a composer with its list at `+0x0c`, a remap object at
/// `+0x1c`, and a 16-bit operation counter at `+0x72`. The list's track array
/// pointer is at `+0x04` and its unsigned count is at `+0x14`. The helper
/// receives a two-word record containing `tracks[count]` and the word at
/// `remap_object+0x04`, plus the caller's operation value. The helper's EAX
/// answer is returned unchanged.
///
/// Original: thiscall with one stack word; the helper is intercepted.
lf_checker_rt::export!(thiscall, rw_0087a0b0(this: u32, operation: u32) -> u32 {
    const LIST_OFFSET: u32 = 0x0c;
    const TRACK_ARRAY_OFFSET: u32 = 0x04;
    const TRACK_COUNT_OFFSET: u32 = 0x14;
    const REMAP_OBJECT_OFFSET: u32 = 0x1c;
    const OPERATION_COUNT_OFFSET: u32 = 0x72;
    const REMAP_FIELD_OFFSET: u32 = 0x04;
    const REMAP_DISPATCH: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn increment_u16(address: u32) {
        unsafe {
            let value = (address as *const u16).read_unaligned();
            (address as *mut u16).write_unaligned(value.wrapping_add(1));
        }
    }

    unsafe {
        increment_u16(this.wrapping_add(OPERATION_COUNT_OFFSET));
        let list = read_u32(this.wrapping_add(LIST_OFFSET));
        let count = read_u32(list.wrapping_add(TRACK_COUNT_OFFSET));
        let tracks = read_u32(list.wrapping_add(TRACK_ARRAY_OFFSET));
        let selected_track = read_u32(tracks.wrapping_add(count.wrapping_mul(4)));
        let remap_object = read_u32(this.wrapping_add(REMAP_OBJECT_OFFSET));
        let remap_value = read_u32(remap_object.wrapping_add(REMAP_FIELD_OFFSET));
        let record = [selected_track, remap_value];
        let record_address = record.as_ptr() as u32;

        lf_checker_rt::callee_thiscall!(
            REMAP_DISPATCH,
            u32,
            record_address,
            operation
        )
    }
});
