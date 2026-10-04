// original: 0x00d44730 ambient_table_get (proposed)

/// Return one entry of the ambient object table by index.
///
/// Reads the table base pointer from its static slot and returns the dword at
/// `base + index * 4`. No bounds check is performed: an out-of-range index
/// reads whatever the address yields or faults, exactly as the original.
///
/// Original: cdecl, one stack word (index), caller cleans up.
lf_checker_rt::export!(cdecl, rw_00d44730(index: u32) -> u32 {
    unsafe {
        const TABLE_SLOT: u32 = 0x017208f0;
        let base = (lf_checker_rt::global::<u32>(TABLE_SLOT) as *const u32).read_unaligned();
        ((base.wrapping_add(index.wrapping_mul(4))) as *const u32).read_unaligned()
    }
});
