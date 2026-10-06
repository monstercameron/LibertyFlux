// original: 0x008f4cb0 input_unit_arg_store (proposed)

/// Copy the shared input-unit argument word into this device object.
///
/// `obj` is the input device object (the sampler block ends at `+0x3A7C`,
/// sibling lanes read axes at `+0x3A70`). The function loads the global
/// unit-argument word and stores it at `+0x3A6C`. The loaded value is also
/// left in EAX, so the contract compares the full return register.
///
/// Thiscall: object in ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f4cb0(obj: u32) -> u32 {
    unsafe {
        const UNIT_ARG_SLOT: u32 = 0x3a6c;
        const G_UNIT_ARG: u32 = 0x11735b4;
        let v = (lf_checker_rt::global::<u32>(G_UNIT_ARG)).read_unaligned();
        ((obj + UNIT_ARG_SLOT) as *mut u32).write_unaligned(v);
        v
    }
});
