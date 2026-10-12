// original: 0x00DF6820 UIFileViewer::first_matching_child

/// Return the first child whose activity query reports a nonzero byte.
///
/// Four child pointers are checked in field order at offsets `+0x1e0`,
/// `+0x1e4`, `+0x1e8` and `+0x1ec`. Each child uses vtable slot `+0x124`;
/// the first positive result returns that child pointer, and four negative
/// results return null. The original dereferences each child without a null
/// check.
lf_checker_rt::export!(thiscall, rw_df6820(this: u32) -> u32 {
    unsafe {
        const CHILD_FIELDS: [u32; 4] = [0x1E0, 0x1E4, 0x1E8, 0x1EC];
        for field_offset in CHILD_FIELDS {
            let child = ((this.wrapping_add(field_offset)) as *const u32).read_unaligned();
            let active = lf_checker_rt::callee_thiscall!(1, u32, child);
            if active as u8 != 0 {
                return child;
            }
        }
        0
    }
});
