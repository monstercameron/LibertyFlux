// original: 0x00CC85A0 fragInstNMGta::vf24

/// Prepares the four search parameters with the shared helper, then scans the
/// receiver's nested record when its type is 2 through 4, its count at `+0x1f8`
/// is nonzero, and marker `0x40` at `+0x1f4` is set. The signed half-open range
/// `[start, end)` indexes an object-pointer array. Each object's descriptor at
/// `+0xa0` chooses a value at `+0x38` when its `+0x34` word equals `key`, or
/// `+0x34` otherwise. A nested-record helper receives that value and `1`; a
/// nonzero AL clears bit zero of the object's byte at `+0xa5`. The start slot
/// is used as a loop counter by the original; its stack comparison is disabled
/// and its effects are observed through array selection and heap writes.
lf_checker_rt::export!(thiscall, rw_00cc85a0(
    this: u32,
    objects: u32,
    key: u32,
    start: u32,
    end: u32
) -> () {
    const PREPARE_SEARCH: u32 = 1;
    const TEST_MATCH: u32 = 2;
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

    let _ = lf_checker_rt::callee_thiscall!(
        PREPARE_SEARCH,
        u32,
        this,
        objects,
        key,
        start,
        end
    );

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
    if !(2..5).contains(&kind) || record_count == 0 || record_flags & ACTIVE_BIT == 0 {
        return;
    }

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
});
