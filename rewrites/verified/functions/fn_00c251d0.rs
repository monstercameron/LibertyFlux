// original: 0x00c251d0 cam_mode_vtable_install (proposed)
/// Install one camera-mode handler pointer: `which` (0-4) selects which
/// handler address is stored to `obj + index*4 + 0x170`; any other `which`
/// stores nothing. Returns `index` on the storing paths, `which` otherwise
/// (the leftover in eax, deterministic from the inputs).
///
/// The stored values are link-time code addresses with relocation entries,
/// so they are read through `relocated`, matching the relocated image.
///
/// Original: 0x00c251d0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c251d0(obj: u32, index: u32, which: u32) -> u32 {
    unsafe {
        const SLOT_BASE: u32 = 0x170;
        const H0: u32 = 0x00c24960;
        const H1: u32 = 0x00c24970;
        const H2: u32 = 0x00c24820;
        const H3: u32 = 0x00c24930;
        const H4: u32 = 0x00c24860;
        let val = match which {
            0 => lf_checker_rt::relocated(H0),
            1 => lf_checker_rt::relocated(H1),
            2 => lf_checker_rt::relocated(H2),
            3 => lf_checker_rt::relocated(H3),
            4 => lf_checker_rt::relocated(H4),
            _ => return which,
        };
        let slot = obj.wrapping_add(index.wrapping_mul(4)).wrapping_add(SLOT_BASE);
        (slot as *mut u32).write_unaligned(val);
        index
    }
});
