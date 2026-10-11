// original: 0x00D212A0 classify_vehicle_tick_slot_probe
/// Checks the vehicle's tick-slot class, then asks the slot classifier for a
/// result. The two helper answers are scripted by the proof contract; the
/// output starts at 1.0 and is updated only by the classifier stand-in.
/// Arguments are a vehicle pointer and a writable fraction pointer.
lf_checker_rt::export!(cdecl, rw_00d212a0(vehicle: u32, fraction_out: u32) -> u8 {
    const INITIAL_FRACTION_BITS: u32 = 0x3f80_0000;
    const FIRST_CLASS: u32 = 3;
    const SECOND_CLASS: u32 = 5;
    const PRIMARY_MODE: u32 = 3;
    const FALLBACK_MODE: u32 = 2;

    unsafe { (fraction_out as *mut u32).write_unaligned(INITIAL_FRACTION_BITS); }
    let first_class = lf_checker_rt::callee_thiscall!(1, u32, vehicle);
    if first_class == FIRST_CLASS { return 0; }
    let second_class = lf_checker_rt::callee_thiscall!(1, u32, vehicle);
    if second_class == SECOND_CLASS { return 0; }

    let matched = lf_checker_rt::callee_cdecl!(2, u8, vehicle, fraction_out, PRIMARY_MODE, PRIMARY_MODE);
    if matched != 0 { return 1; }
    lf_checker_rt::callee_cdecl!(2, u8, vehicle, fraction_out, FALLBACK_MODE, FALLBACK_MODE)
});
