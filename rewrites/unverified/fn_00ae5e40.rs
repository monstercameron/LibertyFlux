// original: 0x00AE5E40 streaming_packed_bounds_query

/// Decode a packed rectangle and query one of the two global streaming grids.
///
/// The cdecl pointer references a record with packed coordinates at `+0x04`
/// and a grid selector at byte `+0x24`. The packed word stores biased 11-bit
/// left and top coordinates, then 5-bit width and height increments. Each
/// base coordinate is reduced by `0x400`; the far coordinates add the packed
/// increments with 32-bit wrapping. Selector bit `0x40` chooses the 120-column
/// grid; a clear bit chooses the 48-column grid. The method call receives
/// left, top, right and bottom in that order and its EAX result is returned.
/// Both query methods are intercepted and scripted by the proof contract.
export!(cdecl, rw_00ae5e40(record: *const u8) -> u32 {
    unsafe {
        const PACKED_COORDINATES: usize = 0x04;
        const GRID_SELECTOR: usize = 0x24;
        const LOW_FIELD_MASK: u32 = 0x7ff;
        const COORDINATE_BIAS: i32 = 0x400;
        const WIDTH_FIELD_MASK: u32 = 0x1f;
        const WIDTH_FIELD_SHIFT: u32 = 22;
        const HEIGHT_FIELD_SHIFT: u32 = 27;
        const GRID_120_FILE_VA: u32 = 0x015b30f0;
        const GRID_48_FILE_VA: u32 = 0x015c1610;

        let packed = core::ptr::read_unaligned(record.add(PACKED_COORDINATES).cast::<u32>());
        let left = ((packed & LOW_FIELD_MASK) as i32).wrapping_sub(COORDINATE_BIAS);
        let top = (((packed >> 11) & LOW_FIELD_MASK) as i32).wrapping_sub(COORDINATE_BIAS);
        let right = left.wrapping_add(((packed >> WIDTH_FIELD_SHIFT) & WIDTH_FIELD_MASK) as i32);
        let bottom = top.wrapping_add((packed >> HEIGHT_FIELD_SHIFT) as i32);

        if core::ptr::read(record.add(GRID_SELECTOR)) & 0x40 != 0 {
            callee_thiscall!(
                1,
                u32,
                relocated(GRID_120_FILE_VA),
                left as u32,
                top as u32,
                right as u32,
                bottom as u32,
            )
        } else {
            callee_thiscall!(
                2,
                u32,
                relocated(GRID_48_FILE_VA),
                left as u32,
                top as u32,
                right as u32,
                bottom as u32,
            )
        }
    }
});
