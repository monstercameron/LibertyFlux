// original: 0x00936090 update_network_generation_if_current

/// Compare `current_generation` with a source global and copy
/// `next_generation` to the destination global only when the values match.
/// The function uses two cdecl stack arguments and writes one four-byte
/// destination global; the comparison and copy use exact 32-bit values.
lf_checker_rt::export!(cdecl, rw_00936090(current_generation: u32, next_generation: u32) -> () {
    unsafe {
        const SOURCE_GLOBAL: u32 = 0x011a_4ddc;
        const DESTINATION_GLOBAL: u32 = 0x011a_4de4;

        let stored_generation = unsafe {
            lf_checker_rt::global::<u32>(SOURCE_GLOBAL).read_unaligned()
        };
        if current_generation == stored_generation {
            unsafe {
                lf_checker_rt::global::<u32>(DESTINATION_GLOBAL)
                    .write_unaligned(next_generation);
            }
        }
    }
});
