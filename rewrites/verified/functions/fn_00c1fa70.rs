// original: 0x00C1FA70 streaming_test_hash_bits

/// Test two independent low-bit patterns against the supplied value and the
/// global hash seed. The first pattern mixes the low byte of `input * 8` with
/// the low seed byte and tests mask `0x60`; the second mixes the low byte of
/// `seed >> 11` with the low input byte and tests mask `0x03`. Returns one if
/// either pattern is nonzero. The argument is one cdecl word; the seed is
/// read from the global hash slot.
lf_checker_rt::export!(cdecl, rw_00c1fa70(input: u32) -> u32 {
    unsafe {
        const HASH_SEED_VA: u32 = 0x011735B4;
        const FIRST_MASK: u8 = 0x60;
        const SECOND_MASK: u8 = 0x03;

        let seed = lf_checker_rt::global::<u32>(HASH_SEED_VA).read();
        let first_byte = (input.wrapping_shl(3) as u8) ^ (seed as u8);
        let second_byte = ((seed >> 11) as u8) ^ (input as u8);
        u32::from(first_byte & FIRST_MASK != 0 || second_byte & SECOND_MASK != 0)
    }
});
