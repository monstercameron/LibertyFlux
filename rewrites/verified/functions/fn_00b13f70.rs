// original: 0x00B13F70 grid_lookup_at_position

/// Find the grid entry for a position and return the address of its payload.
///
/// `this` points to a grid whose four sign-selected cell records begin at
/// offsets 0, 12, 24 and 36. The position helper writes two sign flags and a
/// four-word coordinate scratch value; the quantizer supplies an unsigned
/// key. A zero cell count returns null. Otherwise the key selects a bucket by
/// unsigned remainder and is searched through that bucket's linked list. A
/// match returns the node address plus four. The original is thiscall with
/// two stack arguments; the second is popped but not read.
lf_checker_rt::export!(thiscall, rw_00B13F70(this: u32, position: u32, _ignored: u32) -> u32 {
    unsafe {
        const GRID_CELL_COUNT_OFFSET: u32 = 4;
        const GRID_NEXT_CELL_STRIDE: u32 = 4;
        const CELL_KEY_OFFSET: u32 = 0;
        const CELL_NEXT_OFFSET: u32 = 0x10;
        const PAYLOAD_OFFSET: u32 = 4;

        let mut x_negative = 0u32;
        let mut y_negative = 0u32;
        let mut coordinates = [0u32; 4];
        let x_negative_ptr = &mut x_negative as *mut u32 as u32;
        let y_negative_ptr = &mut y_negative as *mut u32 as u32;
        let coordinates_ptr = coordinates.as_mut_ptr() as u32;

        let _splitter_result = lf_checker_rt::callee_stdcall!(
            1, u32, position, x_negative_ptr, y_negative_ptr, coordinates_ptr,
        );
        let key = lf_checker_rt::callee_thiscall!(2, u32, this, coordinates_ptr);

        let cell_index = y_negative
            .wrapping_add(x_negative.wrapping_mul(2))
            .wrapping_mul(3);
        let cell_address = this.wrapping_add(cell_index.wrapping_mul(GRID_NEXT_CELL_STRIDE));
        let count_address = cell_address.wrapping_add(GRID_CELL_COUNT_OFFSET);
        let bucket_count = (count_address as *const u16).read_unaligned();
        if bucket_count == 0 {
            return 0;
        }

        let entries = (cell_address as *const u32).read_unaligned();
        let bucket = key % u32::from(bucket_count);
        let mut node = (entries.wrapping_add(bucket.wrapping_mul(4)) as *const u32).read_unaligned();
        while node != 0 {
            let node_key = (node.wrapping_add(CELL_KEY_OFFSET) as *const u32).read_unaligned();
            if node_key == key {
                return node.wrapping_add(PAYLOAD_OFFSET);
            }
            node = (node.wrapping_add(CELL_NEXT_OFFSET) as *const u32).read_unaligned();
        }
        0
    }
});
