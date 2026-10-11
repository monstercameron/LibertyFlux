// original: 0x00B45010 CEventCarUndriveable::CEventCarUndriveable
/// Construct an undriveable-car event with one optional referenced vehicle.
/// The base event constructor runs first. The field is installed before an
/// optional call through its address to the reference helper; `this` is returned.
lf_checker_rt::export!(thiscall, rw_00b45010(this: u32, referenced_vehicle: u32) -> u32 {
    unsafe {
        const EVENT_VTABLE: u32 = 0x00eae334;
        const VEHICLE_FIELD: usize = 0x0c;
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        let object = this as *mut u8;
        let vehicle_field = object.add(VEHICLE_FIELD);
        (object as *mut u32).write_unaligned(lf_checker_rt::relocated(EVENT_VTABLE));
        (vehicle_field as *mut u32).write_unaligned(referenced_vehicle);
        if referenced_vehicle != 0 {
            let _ = lf_checker_rt::callee_stdcall!(2, u32, vehicle_field as u32);
        }
        this
    }
});
