// original: 0x00CC8650 phInstGta::vf24

/// Scans the nested physics record's signed half-open index range when its type
/// is 2 through 4, its count at `+0x1f8` is nonzero, and marker `0x40` at
/// `+0x1f4` is set. Each object descriptor at `+0xa0` selects a value at
/// `+0x38` when its word at `+0x34` matches the search key, or `+0x34` otherwise.
/// The nested-record matcher receives that value and `1`; a nonzero AL clears
/// bit zero of the object's byte at `+0xa5`. When the nested kind bits equal
/// `0xc0`, the final helper receives the array, key, loop-updated start index,
/// and end bound. The original
/// uses the incoming `start` slot as its loop counter, so stack comparison is
/// disabled; the index is observed through heap accesses and writes.
lf_checker_rt::export!(thiscall, rw_00cc8650(
    this: u32,
    objects: u32,
    key: u32,
    start: u32,
    end: u32
) -> () {
    const TEST_MATCH: u32 = 1;
    const FINAL_HANDLER: u32 = 2;
    const NESTED_RECORD: u32 = 0x0c;
    const TYPE_BITS: u32 = 0x28;
    const RECORD_COUNT: u32 = 0x1f8;
    const RECORD_FLAGS: u32 = 0x1f4;
    const ACTIVE_BIT: u8 = 0x40;
    const OBJECT_DESCRIPTOR: u32 = 0xa0;
    const OBJECT_ACTIVE: u32 = 0xa5;
    const DESCRIPTOR_PRIMARY: u32 = 0x34;
    const DESCRIPTOR_MATCHED: u32 = 0x38;
    const OBJECT_BIT: u8 = 1;

    let nested = unsafe { (this.wrapping_add(NESTED_RECORD) as *const u32).read_unaligned() };
    if nested == 0 {
        return;
    }
    let kind_word = unsafe { (nested.wrapping_add(TYPE_BITS) as *const u32).read_unaligned() };
    let kind = (kind_word >> 6) & 0x0f;
    let record_count = unsafe {
        (nested.wrapping_add(RECORD_COUNT) as *const u32).read_unaligned()
    };
    let record_flags = unsafe { (nested.wrapping_add(RECORD_FLAGS) as *const u8).read() };

    let mut updated_start = start;
    if (2..5).contains(&kind) && record_count != 0 && record_flags & ACTIVE_BIT != 0 {
        let mut object_index = start as i32;
        let end_index = end as i32;
        while object_index < end_index {
            let slot = objects.wrapping_add((object_index as u32).wrapping_mul(4));
            let object = unsafe { (slot as *const u32).read_unaligned() };
            let descriptor = unsafe {
                (object.wrapping_add(OBJECT_DESCRIPTOR) as *const u32).read_unaligned()
            };
            let primary = unsafe {
                (descriptor.wrapping_add(DESCRIPTOR_PRIMARY) as *const u32).read_unaligned()
            };
            let tested_value = if key == primary {
                unsafe {
                    (descriptor.wrapping_add(DESCRIPTOR_MATCHED) as *const u32).read_unaligned()
                }
            } else {
                primary
            };
            let found = lf_checker_rt::callee_thiscall!(TEST_MATCH, u8, nested, tested_value, 1);
            if found != 0 {
                let active_byte = object.wrapping_add(OBJECT_ACTIVE) as *mut u8;
                unsafe { active_byte.write(active_byte.read() & !OBJECT_BIT) };
            }
            object_index = object_index.wrapping_add(1);
        }
        updated_start = object_index as u32;
    }

    if kind_word & 0x3c0 == 0xc0 {
        let _ = lf_checker_rt::callee_thiscall!(
            FINAL_HANDLER,
            u32,
            nested,
            objects,
            key,
            updated_start,
            end
        );
    }
});
