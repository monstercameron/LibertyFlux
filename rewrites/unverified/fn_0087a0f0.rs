// original: 0x0087A0F0 rage::crmtComposerOptimized::sync_vehicle_transforms

/// Count active child entries and dispatch vehicle transform synchronization.
///
/// `this` is a composer with its child-list pointer at `+0x0c`, remap object
/// pointer at `+0x1c`, and 16-bit operation counter at `+0x72`. The child-list
/// pointer refers to a track array at `+0x04` and a current count at `+0x14`.
/// The stack argument points to an inline child list: its signed count is at
/// `+0x0c`, and the first child pointer is at `+0x08`. For each child whose
/// first byte is nonzero, the composer counter advances by one. For each
/// positive child count, the helper receives the child pointer in ECX and the
/// current track and remap value as stack words. The remap value is returned.
///
/// Original: thiscall with one stack pointer; the transform helper is
/// intercepted. Counts above one fault when the count word is read as the
/// second child pointer, so those trials do not prove a completed list walk.
lf_checker_rt::export!(thiscall, rw_0087a0f0(this: u32, child_list: u32) -> u32 {
    const COMPOSER_LIST_OFFSET: u32 = 0x0c;
    const TRACK_ARRAY_OFFSET: u32 = 0x04;
    const TRACK_COUNT_OFFSET: u32 = 0x14;
    const REMAP_OBJECT_OFFSET: u32 = 0x1c;
    const OPERATION_COUNT_OFFSET: u32 = 0x72;
    const CHILD_ARRAY_OFFSET: u32 = 0x08;
    const CHILD_COUNT_OFFSET: u32 = 0x0c;
    const REMAP_VALUE_OFFSET: u32 = 0x04;
    const SYNC_TRANSFORMS: u32 = 1;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u16(address: u32, value: u16) {
        unsafe { (address as *mut u16).write_unaligned(value) }
    }

    unsafe {
        let child_count = read_u32(child_list.wrapping_add(CHILD_COUNT_OFFSET)) as i32;
        let mut active_count = 0u32;
        if child_count > 0 {
            let mut index = 0u32;
            while index < child_count as u32 {
                let child = read_u32(
                    child_list
                        .wrapping_add(CHILD_ARRAY_OFFSET)
                        .wrapping_add(index.wrapping_mul(4)),
                );
                if (child as *const u8).read() != 0 {
                    active_count = active_count.wrapping_add(1);
                }
                index = index.wrapping_add(1);
            }
        }

        let counter_address = this.wrapping_add(OPERATION_COUNT_OFFSET);
        let operation_count = read_u16(counter_address);
        write_u16(counter_address, operation_count.wrapping_add(active_count as u16));

        let composer_list = read_u32(this.wrapping_add(COMPOSER_LIST_OFFSET));
        let track_count = read_u32(composer_list.wrapping_add(TRACK_COUNT_OFFSET));
        let tracks = read_u32(composer_list.wrapping_add(TRACK_ARRAY_OFFSET));
        let current_track = read_u32(tracks.wrapping_add(track_count.wrapping_mul(4)));
        let remap_object = read_u32(this.wrapping_add(REMAP_OBJECT_OFFSET));
        let remap_value = read_u32(remap_object.wrapping_add(REMAP_VALUE_OFFSET));

        if child_count > 0 {
            let mut index = 0u32;
            while index < child_count as u32 {
                let child = read_u32(
                    child_list
                        .wrapping_add(CHILD_ARRAY_OFFSET)
                        .wrapping_add(index.wrapping_mul(4)),
                );
                let _ = lf_checker_rt::callee_thiscall!(
                    SYNC_TRANSFORMS,
                    u32,
                    child,
                    current_track,
                    remap_value
                );
                index = index.wrapping_add(1);
            }
        }
        remap_value
    }
});
