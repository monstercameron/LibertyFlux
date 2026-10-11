// original: 0x00AE5EA0 streaming_table_two_count_sum

/// Return the sum of two 16-bit counters from one streaming table entry.
///
/// The cdecl argument is an unsigned entry index. Entries use a `0x54`-byte
/// stride in the fixed global table; the function zero-extends and adds the
/// words at offsets `+0x14` and `+0x1c` with 32-bit wrapping semantics.
///
/// The global table address is relocated from the executable's preferred
/// address, so this implementation does not depend on the image load base.
export!(cdecl, rw_00ae5ea0(index: u32) -> u32 {
    unsafe {
        const TABLE_FILE_VA: u32 = 0x01614c90;
        const ENTRY_STRIDE: u32 = 0x54;
        const FIRST_COUNTER: u32 = 0x14;
        const SECOND_COUNTER: u32 = 0x1c;

        let entry = TABLE_FILE_VA.wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
        let first = core::ptr::read_unaligned(global::<u16>(entry.wrapping_add(FIRST_COUNTER)));
        let second = core::ptr::read_unaligned(global::<u16>(entry.wrapping_add(SECOND_COUNTER)));
        u32::from(first).wrapping_add(u32::from(second))
    }
});
