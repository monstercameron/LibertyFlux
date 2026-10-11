// original: 0x006596B0 stream_find_entry_parameter

/// Search a collection's entry view for the supplied selector. The view
/// pointer is stored at `this + 0x24`; it contains an entry array at `+4`
/// and an unsigned 16-bit count at `+8`. The selector and zero are first
/// passed to the resource helper. If the count is zero, return `u32::MAX`.
/// Otherwise pass the temporary output word, entry array, count, eight-byte
/// element width, and fixed comparator to the lookup helper. A null result,
/// or a result whose `+4` address wraps to null, also returns `u32::MAX`;
/// success returns the dword at result `+4`. The thiscall callee removes its
/// one stack word.
lf_checker_rt::export!(thiscall, rw_006596B0(this: u32, selector: u32) -> u32 {
    unsafe {
        const ENTRY_VIEW: u32 = 0x24;
        const ENTRY_ARRAY: u32 = 0x04;
        const ENTRY_COUNT: u32 = 0x08;
        const ELEMENT_BYTES: u32 = 8;
        const COMPARATOR_VA: u32 = 0x0041_2950;
        const RESOURCE_HELPER: u32 = 1;
        const LOOKUP_HELPER: u32 = 2;

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u32 {
            unsafe { core::ptr::read_unaligned(address as *const u16) as u32 }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { core::ptr::read_unaligned(address as *const u32) }
        }

        let entry_view = read_u32(this.wrapping_add(ENTRY_VIEW));
        let setup_result = lf_checker_rt::callee_cdecl!(
            RESOURCE_HELPER,
            u32,
            selector,
            0,
        );
        let mut output_word = setup_result;
        let output_pointer = &mut output_word as *mut u32 as u32;

        let entry_count = read_u16(entry_view.wrapping_add(ENTRY_COUNT));
        if entry_count == 0 {
            return u32::MAX;
        }

        let entries = read_u32(entry_view.wrapping_add(ENTRY_ARRAY));
        let comparator = lf_checker_rt::relocated(COMPARATOR_VA);
        let match_pointer = lf_checker_rt::callee_cdecl!(
            LOOKUP_HELPER,
            u32,
            output_pointer,
            entries,
            entry_count,
            ELEMENT_BYTES,
            comparator,
        );

        let value_address = match_pointer.wrapping_add(4);
        if match_pointer == 0 || value_address == 0 {
            u32::MAX
        } else {
            read_u32(value_address)
        }
    }
});
