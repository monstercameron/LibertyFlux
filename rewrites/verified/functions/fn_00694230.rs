// original: 0x00694230 blend_shape_component_copy_00694230 (proposed)

/// Copy the fixed component fields and initialize the embedded base component.
///
/// The destination is in ECX and the source is the single stack argument. The
/// result is the destination pointer. The copied words are at offsets `+4`,
/// `+8`, and `+0x14` through `+0x2c`; the two embedded base fields at `+0x0c`
/// and `+0x10` are cleared. The embedded base initializer is called with the
/// destination and source subobject pointers.
lf_checker_rt::export!(thiscall, rw_00694230(destination: u32, source: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FE38DC;
        const BASE_OFFSET: u32 = 0x0C;
        const FIRST_TAIL_OFFSET: u32 = 0x14;
        const TAIL_WORDS: u32 = 7;

        let destination_bytes = destination as *mut u8;
        let source_bytes = source as *const u8;
        (destination_bytes as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE));
        *((destination_bytes.add(4)) as *mut u32) = *((source_bytes.add(4)) as *const u32);
        *((destination_bytes.add(8)) as *mut u32) = *((source_bytes.add(8)) as *const u32);
        *((destination_bytes.add(BASE_OFFSET as usize)) as *mut u32) = 0;
        *((destination_bytes.add((BASE_OFFSET + 4) as usize)) as *mut u32) = 0;

        lf_checker_rt::callee_thiscall!(1, u32, destination + BASE_OFFSET, source + BASE_OFFSET);

        let mut word_index = 0;
        while word_index < TAIL_WORDS {
            let offset = FIRST_TAIL_OFFSET + word_index * 4;
            *((destination_bytes.add(offset as usize)) as *mut u32) =
                *((source_bytes.add(offset as usize)) as *const u32);
            word_index += 1;
        }
        destination
    }
});
