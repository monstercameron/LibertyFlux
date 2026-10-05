// original: 0x00a3c6e0 vehicle_limits_reset (proposed)

/// Reset a vehicle object's limit fields to the global defaults.
///
/// Clears the words at `+0x170` and `+0x174`, then copies three global
/// float words to `+0x180`/`+0x184`/`+0x188` and again to
/// `+0x190`/`+0x194`/`+0x198` (bit copies, exact for any fill).
/// Thiscall, no stack arguments, returns 0 (EAX is zeroed first and no
/// later instruction sets it).
lf_checker_rt::export!(thiscall, rw_00a3c6e0(obj: u32) -> u32 {
    unsafe {
        const G0: u32 = 0x0110_DAE0;
        const G1: u32 = 0x0110_DAE4;
        const G2: u32 = 0x0110_DAE8;
        core::ptr::write_unaligned((obj + 0x170) as *mut u32, 0);
        core::ptr::write_unaligned((obj + 0x174) as *mut u32, 0);
        let g0 = core::ptr::read(lf_checker_rt::global::<u32>(G0));
        let g1 = core::ptr::read(lf_checker_rt::global::<u32>(G1));
        let g2 = core::ptr::read(lf_checker_rt::global::<u32>(G2));
        core::ptr::write_unaligned((obj + 0x180) as *mut u32, g0);
        core::ptr::write_unaligned((obj + 0x184) as *mut u32, g1);
        core::ptr::write_unaligned((obj + 0x188) as *mut u32, g2);
        core::ptr::write_unaligned((obj + 0x190) as *mut u32, g0);
        core::ptr::write_unaligned((obj + 0x194) as *mut u32, g1);
        core::ptr::write_unaligned((obj + 0x198) as *mut u32, g2);
        0
    }
});
