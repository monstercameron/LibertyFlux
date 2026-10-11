// original: 0x006A3150 reset_grc_setup_slots
/// Selects one byte-sized mode, passes that mode and a second byte to the
/// setup helper, then clears four object slots.
///
/// The first global gate forces the selected mode to zero when nonzero. If
/// that does not force zero, the second gate forces the mode to one when
/// nonzero; otherwise the low byte of the first stack argument is used. The
/// low byte of the second stack argument is forwarded in EDX. The entry EDX
/// is not part of this function's calling convention, so the proof fixes its
/// upper bits to zero while comparing the full helper register arguments.
/// The gates are read in order, the helper runs before the four zero stores,
/// and the helper's EAX result is returned unchanged.
const FIRST_SETUP_GATE: u32 = 0x0111_02F4;
const SECOND_SETUP_GATE: u32 = 0x0111_02DC;
const MODE_SLOT: usize = 0x20;
const AUX_SLOT: usize = 0x24;
const CLOCK_SLOT: usize = 0x28;
const RESOURCE_SLOT: usize = 0x2C;

lf_checker_rt::export!(thiscall, rw_006a3150(this: u32, first_flag: u32, second_flag: u32) -> u32 {
    let first_global = unsafe {
        core::ptr::read_volatile(lf_checker_rt::global::<u32>(FIRST_SETUP_GATE))
    };
    let second_global = unsafe {
        core::ptr::read_volatile(lf_checker_rt::global::<u32>(SECOND_SETUP_GATE))
    };
    let selected_mode = if first_global != 0 {
        0
    } else if second_global != 0 {
        1
    } else {
        u32::from(first_flag as u8)
    };
    let helper_result = lf_checker_rt::callee_fastcall!(
        1,
        u32,
        selected_mode,
        u32::from(second_flag as u8),
    );
    let object = this as *mut u8;
    unsafe {
        core::ptr::write_volatile(object.add(RESOURCE_SLOT).cast(), 0);
        core::ptr::write_volatile(object.add(CLOCK_SLOT).cast(), 0);
        core::ptr::write_volatile(object.add(MODE_SLOT).cast(), 0);
        core::ptr::write_volatile(object.add(AUX_SLOT).cast(), 0);
    }
    helper_result
});
