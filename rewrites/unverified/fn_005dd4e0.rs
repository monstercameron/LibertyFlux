// original: 0x005dd4e0 CTaskComplexTreatAccident::CTaskComplexTreatAccident_2
/// Construct a treat-accident task in place: base-construct, store the
/// patient handle, stamp the vtable, attach to the patient when set.
export!(thiscall, rw_005dd4e0(this_ptr: *mut u8, patient: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this_ptr as u32);
        *(this_ptr as *mut u32) = relocated(0xFE0DD4);
        *((this_ptr as *mut u8).add(0x14) as *mut u32) = patient;
        if patient != 0 {
            callee_thiscall!(2, u32, patient, (this_ptr as u32).wrapping_add(0x14));
        }
        this_ptr as u32
    }
});
