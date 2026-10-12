// original: 0x00880260 append_utf16_range

/// Append the UTF-16 range `[source_begin, source_end)` to the receiver's
/// small or allocated buffer. The range length is derived from the wrapping
/// byte difference and an arithmetic shift by one. Up to fifteen code units
/// use inline storage; larger ranges allocate `count + 1` units. The range
/// over the maximum unit count follows the fatal reporter path and is excluded
/// from the proof because the original reaches `int3` there.
lf_checker_rt::export!(thiscall, rw_00880260(this: u32, source_begin: u32, source_end: u32) -> u32 {
    unsafe {
        const DATA_BASE_OFFSET: u32 = 0x00;
        const CURSOR_OFFSET: u32 = 0x20;
        const LIMIT_OFFSET: u32 = 0x24;
        const INLINE_LIMIT_UNITS: u32 = 16;
        const MAX_UNIT_COUNT: u32 = 0x7FFF_FFFE;
        const ALLOCATE_CALLEE: u32 = 1;
        const APPEND_CALLEE: u32 = 2;
        const FATAL_CALLEE: u32 = 3;

        let byte_difference = source_end.wrapping_sub(source_begin);
        let unit_count = ((byte_difference as i32) >> 1) as u32;
        let requested_units = unit_count.wrapping_add(1);
        let copy_bytes = unit_count.wrapping_add(unit_count);
        let data_base_slot = this.wrapping_add(DATA_BASE_OFFSET) as *mut u32;
        let cursor_slot = this.wrapping_add(CURSOR_OFFSET) as *mut u32;
        let limit_slot = this.wrapping_add(LIMIT_OFFSET) as *mut u32;
        let inline_limit = this.wrapping_add(CURSOR_OFFSET);

        if requested_units <= INLINE_LIMIT_UNITS {
            limit_slot.write_unaligned(inline_limit);
            cursor_slot.write_unaligned(this);
            let _ = lf_checker_rt::callee_cdecl!(
                APPEND_CALLEE, u32, this, source_begin, copy_bytes
            );
            let data_base = if limit_slot.read_unaligned() != inline_limit {
                data_base_slot.read_unaligned()
            } else {
                this
            };
            let new_cursor = data_base.wrapping_add(copy_bytes);
            cursor_slot.write_unaligned(new_cursor);
            new_cursor
        } else {
            if unit_count > MAX_UNIT_COUNT {
                let _ = lf_checker_rt::callee_cdecl!(FATAL_CALLEE, u32,);
                std::process::abort();
            }

            let allocated = lf_checker_rt::callee_stdcall!(
                ALLOCATE_CALLEE, u32, requested_units, requested_units
            );
            data_base_slot.write_unaligned(allocated);
            cursor_slot.write_unaligned(allocated);
            limit_slot.write_unaligned(allocated.wrapping_add(requested_units.wrapping_mul(2)));

            if limit_slot.read_unaligned() != inline_limit {
                if source_end != source_begin {
                    let destination = data_base_slot.read_unaligned();
                    let copied_to = lf_checker_rt::callee_cdecl!(
                        APPEND_CALLEE, u32, destination, source_begin, byte_difference
                    );
                    let new_cursor = copied_to.wrapping_add(byte_difference);
                    cursor_slot.write_unaligned(new_cursor);
                    new_cursor
                } else {
                    let new_cursor = this.wrapping_add(copy_bytes);
                    cursor_slot.write_unaligned(new_cursor);
                    new_cursor
                }
            } else {
                let _ = lf_checker_rt::callee_cdecl!(
                    APPEND_CALLEE, u32, this, source_begin, copy_bytes
                );
                let data_base = if limit_slot.read_unaligned() != inline_limit {
                    data_base_slot.read_unaligned()
                } else {
                    this
                };
                let new_cursor = data_base.wrapping_add(copy_bytes);
                cursor_slot.write_unaligned(new_cursor);
                new_cursor
            }
        }
    }
});
