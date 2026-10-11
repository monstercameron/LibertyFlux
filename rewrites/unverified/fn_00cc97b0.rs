// original: 0x00CC97B0 fragInstGta::vf21

/// Tests whether the receiver's nested physics record should reject an
/// associated object. The record kind must be 2 through 4 and flag bit zero at
/// `+0x24` must be set. It rejects an object whose nested pointer aliases this
/// record, and also rejects a matching linked record when the packed type
/// nibble is 3 or 7 and bit 7 is set. A nonzero count at `+0x1f8` invokes a
/// matcher; nonzero AL rejects. For kind bits `0x80` and type 3, a manager
/// lookup is checked against the same fields. The result is returned in AL;
/// both helper implementations are scripted by the contract.
lf_checker_rt::export!(thiscall, rw_00cc97b0(this: u32, other: u32) -> u8 {
    const NESTED_RECORD: u32 = 0x0c;
    const KIND: u32 = 0x28;
    const RECORD_FLAGS: u32 = 0x24;
    const LINKED_RECORD: u32 = 0x1bc;
    const PACKED_TYPE: u32 = 0x1e2;
    const RECORD_COUNT: u32 = 0x1f8;
    const TYPE_CODE: u32 = 0x1304;
    const REQUIRED_FLAG: u8 = 1;
    const MATCHER: u32 = 1;
    const MANAGER_LOOKUP: u32 = 2;

    let nested = unsafe { (this.wrapping_add(NESTED_RECORD) as *const u32).read_unaligned() };
    if nested == 0 {
        return 1;
    }
    let kind_word = unsafe { (nested.wrapping_add(KIND) as *const u32).read_unaligned() };
    let kind = (kind_word >> 6) & 0x0f;
    if !(2..5).contains(&kind) {
        return 1;
    }
    let record_flags = unsafe { (nested.wrapping_add(RECORD_FLAGS) as *const u8).read() };
    if record_flags & REQUIRED_FLAG == 0 {
        return 0;
    }

    if other != 0 {
        let other_nested = unsafe {
            (other.wrapping_add(NESTED_RECORD) as *const u32).read_unaligned()
        };
        if nested == other_nested {
            return 0;
        }
    }

    let linked = unsafe {
        (nested.wrapping_add(LINKED_RECORD) as *const u32).read_unaligned()
    };
    if linked != 0 {
        let linked_owner = unsafe { (linked.wrapping_add(0x38) as *const u32).read_unaligned() };
        if linked_owner == other {
            let packed = unsafe {
                (nested.wrapping_add(PACKED_TYPE) as *const u16).read_unaligned()
            };
            let nibble = packed & 0x0f;
            if (nibble == 3 || nibble == 7) && packed & 0x80 != 0 {
                return 0;
            }
        }
    }

    let record_count = unsafe {
        (nested.wrapping_add(RECORD_COUNT) as *const u32).read_unaligned()
    };
    if record_count != 0
        && lf_checker_rt::callee_thiscall!(MATCHER, u8, nested, other, 0) != 0
    {
        return 0;
    }

    if kind_word & 0x3c0 == 0x80 {
        let nested_type = unsafe {
            (nested.wrapping_add(TYPE_CODE) as *const u32).read_unaligned()
        };
        if nested_type == 3 {
            let manager = lf_checker_rt::callee_cdecl!(MANAGER_LOOKUP, u32, other);
            if manager != 0 {
                let manager_kind = unsafe {
                    (manager.wrapping_add(KIND) as *const u32).read_unaligned()
                };
                let manager_type = unsafe {
                    (manager.wrapping_add(TYPE_CODE) as *const u32).read_unaligned()
                };
                if manager_kind & 0x3c0 == 0x80 && manager_type == 3 {
                    return 0;
                }
            }
        }
    }
    1
});
