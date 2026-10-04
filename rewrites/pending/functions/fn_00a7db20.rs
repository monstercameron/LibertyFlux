// original: 0x00a7db20 CSimpleCarDriveTaskInfo::CSimpleCarDriveTaskInfo
/// Construct a car-drive task info. Forwards one argument to the base
/// constructor, then folds a flag bit plus several status bits read from
/// the given vehicle-ish object into the flag word at +0x1c. A null
/// object pointer skips the status merge.
export!(thiscall, rw_00a7db20(this: u32, vehicle: u32, base_arg: u32, flags_arg: u32) -> u32 {
    const VTABLE: u32 = 0x00EA13A4;
    callee_thiscall!(2, u32, this, base_arg);
    unsafe {
        let obj = this as *mut u8;
        let base_flags = (obj.add(0x1c) as *const u32).read_unaligned();
        let mut flags = (base_flags & 0xFFFF_FFC0) | (flags_arg & 1);
        (obj as *mut u32).write_unaligned(relocated(VTABLE));
        (obj.add(0x1c) as *mut u32).write_unaligned(flags);
        if vehicle != 0 {
            let status_a = *((vehicle.wrapping_add(0xF14)) as *const u8) as u32;
            flags = (flags & !2) | ((status_a >> 2) & 2);
            (obj.add(0x1c) as *mut u32).write_unaligned(flags);
            let status_b = (vehicle.wrapping_add(0x12EC) as *const u32).read_unaligned();
            flags = (flags & !4) | (if status_b != 0 { 4 } else { 0 });
            (obj.add(0x1c) as *mut u32).write_unaligned(flags);
            let status_c = *((vehicle.wrapping_add(0xF19)) as *const u8) as u32;
            flags = (flags & !0x10) | ((status_c >> 3) & 0x1FFF_FFF0);
            (obj.add(0x1c) as *mut u32).write_unaligned(flags);
            let status_d = *((vehicle.wrapping_add(0xF1E)) as *const u8) as u32;
            let latched = (status_d & 0x20) != 0 && (status_c & 0x10) != 0 && (status_c & 0x20) == 0;
            flags = (flags & !8) | (if latched { 8 } else { 0 });
            (obj.add(0x1c) as *mut u32).write_unaligned(flags);
            flags = (flags & !0x20) | ((status_c << 4) & 0x20);
            (obj.add(0x1c) as *mut u32).write_unaligned(flags);
        }
    }
    this
});
