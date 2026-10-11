// original: 0x005DAB50 CVehicleFactoryNY::vf4

/// Return the vehicle factory's static class entry selected by `class_index`.
///
/// The caller supplies the unsigned table index on the stack. The table is an
/// array of process-local pointers beginning at the original image's virtual
/// address `0x0110e8a0`, with one four-byte pointer per entry. This method does
/// no bounds check, uses the value as an array index, returns the stored
/// pointer in EAX, and removes four argument bytes using stdcall cleanup.
lf_checker_rt::export!(stdcall, rw_005dab50(class_index: u32) -> *mut u8 {
    const VEHICLE_CLASS_TABLE: u32 = 0x0110_e8a0;

    unsafe { lf_checker_rt::global::<*mut u8>(VEHICLE_CLASS_TABLE).add(class_index as usize).read_unaligned() }
});
