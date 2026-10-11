// original: 0x00B42780 append_overlapping_entities

/// Append registered child entities whose signed 16-bit bounds intersect a
/// query box.
///
/// `query_box` contains min-X, max-X, min-Y, max-Y, min-Z and max-Z as six
/// signed words. The result descriptor contains an item pointer at `+0`, a
/// 16-bit count at `+4` and a 16-bit capacity at `+6`. A registry root stores
/// its aggregate bounds at `+0x0c` and its first child at `+0x18`; child
/// entities link through `+0x258` and store bounds at `+0x260`. All six
/// comparisons are signed and inclusive, so touching a boundary intersects.
/// A matching child is appended unless count equals capacity, in which case
/// the function returns immediately. The count wraps as a 16-bit value. The
/// third cdecl argument is unused.
///
/// The registry pointer and live count are read from the game globals through
/// the checker runtime. The i686 proof preserves the original 32-bit object
/// graph; pointer fields use the target's native pointer width in Rust.
#[allow(unused_doc_comments)]
lf_checker_rt::export!(cdecl, rw_00b42780(query_box: *const u8, result_list: *mut u8, _unused: u32) -> () {
    const REGISTRY_POINTER_VA: u32 = 0x0166_541C;
    const REGISTRY_COUNT_VA: u32 = 0x0166_5420;
    const ROOT_CHILD_HEAD: usize = 0x18;
    const ROOT_BOUNDS: usize = 0x0C;
    const ENTITY_NEXT: usize = 0x258;
    const ENTITY_BOUNDS: usize = 0x260;
    const RESULT_COUNT: usize = 4;
    const RESULT_CAPACITY: usize = 6;

    unsafe fn read_i16(base: *const u8, offset: usize) -> i16 {
        core::ptr::read_unaligned(base.add(offset).cast::<i16>())
    }

    unsafe fn intersects(
        query: *const u8,
        candidate: *const u8,
        bounds_offset: usize,
    ) -> bool {
        read_i16(candidate, bounds_offset) <= read_i16(query, 2)
            && read_i16(candidate, bounds_offset + 4) <= read_i16(query, 6)
            && read_i16(candidate, bounds_offset + 8) <= read_i16(query, 10)
            && read_i16(candidate, bounds_offset + 2) >= read_i16(query, 0)
            && read_i16(candidate, bounds_offset + 6) >= read_i16(query, 4)
            && read_i16(candidate, bounds_offset + 10) >= read_i16(query, 8)
    }

    unsafe {
        let registry_count = lf_checker_rt::global::<u16>(REGISTRY_COUNT_VA);
        let mut live_count = core::ptr::read_unaligned(registry_count);
        if live_count == 0 {
            return;
        }

        let mut root_index = 0usize;
        while root_index < usize::from(live_count) {
            let registry: *const usize = core::ptr::read_unaligned(
                lf_checker_rt::global::<*const usize>(REGISTRY_POINTER_VA),
            );
            let root_address = core::ptr::read_unaligned(registry.add(root_index));
            let root = root_address as *const u8;
            let mut entity_address = core::ptr::read_unaligned(
                root.add(ROOT_CHILD_HEAD).cast::<usize>(),
            );

            if entity_address != 0 && intersects(query_box, root, ROOT_BOUNDS) {
                while entity_address != 0 {
                    let entity = entity_address as *const u8;
                    if intersects(query_box, entity, ENTITY_BOUNDS) {
                        let count_pointer = result_list.add(RESULT_COUNT).cast::<u16>();
                        let capacity_pointer = result_list.add(RESULT_CAPACITY).cast::<u16>();
                        let count = core::ptr::read_unaligned(count_pointer);
                        let capacity = core::ptr::read_unaligned(capacity_pointer);
                        if count == capacity {
                            return;
                        }

                        let items: *mut usize = core::ptr::read_unaligned(result_list.cast());
                        core::ptr::write_unaligned(items.add(usize::from(count)), entity_address);
                        core::ptr::write_unaligned(count_pointer, count.wrapping_add(1));
                    }
                    entity_address = core::ptr::read_unaligned(
                        entity.add(ENTITY_NEXT).cast::<usize>(),
                    );
                }
            }

            root_index += 1;
            live_count = core::ptr::read_unaligned(registry_count);
        }
    }
});
