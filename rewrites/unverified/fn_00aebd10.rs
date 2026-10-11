// original: 0x00AEBD10 apply_timer_filter_mask

/// Combines the active mask at `settings+8` with the inverse of the excluded
/// mask at `settings+0x0c`, then retains its low 24 bits. It ORs that result
/// into the manager's filter word at `+0x54` and, when present, the linked
/// manager's filter word at the same offset from the link at `+0x4c`.
///
/// Calling convention: cdecl with a manager pointer, an unused word, and a
/// pointer to the two-mask settings layout. All object pointers and fields
/// preserve the original 32-bit layout.
lf_checker_rt::export!(cdecl, rw_00aebd10(manager: u32, _unused: u32, settings: u32) -> () {
    unsafe {
        const MANAGER_LINK: u32 = 0x4c;
        const FILTER_WORD: u32 = 0x54;
        const ACTIVE_MASK: u32 = 8;
        const EXCLUDED_MASK: u32 = 0x0c;
        const FILTER_BITS: u32 = 0x00ff_ffff;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn or_u32(address: u32, bits: u32) {
            let current = unsafe { read_u32(address) };
            unsafe { (address as *mut u32).write_unaligned(current | bits) };
        }

        let active = read_u32(settings.wrapping_add(ACTIVE_MASK));
        let excluded = read_u32(settings.wrapping_add(EXCLUDED_MASK));
        let filter = !excluded & active & FILTER_BITS;
        or_u32(manager.wrapping_add(FILTER_WORD), filter);

        let linked_manager = read_u32(manager.wrapping_add(MANAGER_LINK));
        if linked_manager != 0 {
            or_u32(linked_manager.wrapping_add(FILTER_WORD), filter);
        }
    }
});
