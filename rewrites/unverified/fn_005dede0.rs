// original: 0x005DEDE0 CTaskComplexMedicPassenger::vf6

/// Return the passenger task's mode-dependent f32 setting.
///
/// A nonzero byte at object offset 0x48 selects the first configured f32;
/// zero selects the second. The constants are loaded and returned without
/// arithmetic, preserving their exact bit patterns through the x87 result
/// channel. The unused stack word is still consumed by thiscall cleanup.
lf_checker_rt::export!(thiscall, rw_005dede0(this: u32, _unused: u32) -> f32 {
    passenger_mode_value(this, false)
});

fn passenger_mode_value(this: u32, reverse_selection: bool) -> f32 {
    const MODE_OFFSET: u32 = 0x48;
    const ACTIVE_VALUE: u32 = 0x00FE8B40;
    const INACTIVE_VALUE: u32 = 0x00FE8D94;

    let active = unsafe { (this.wrapping_add(MODE_OFFSET) as *const u8).read() != 0 };
    let selected = if active ^ reverse_selection { ACTIVE_VALUE } else { INACTIVE_VALUE };
    let bits = unsafe { lf_checker_rt::global::<u32>(selected).read_unaligned() };
    f32::from_bits(bits)
}
