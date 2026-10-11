// original: 0x00C2F510 NativeImpl_SET_CAR_DOOR_LATCHED_2

/// Reset the door-latch state words at byte offsets `0x14` through `0x24`,
/// clear bits 9 through 15 of the flags at `0x10`, and set bit 8. If the
/// original flags also have bit 18 set, ask the latch predicate callee about
/// the supplied vehicle pointer; when it returns zero, call the latch setter.
/// For a positive signed byte at `this + 5`, forward that count to the object
/// reached through the supplied vehicle's pointer at offset `0xdc4`. The
/// thiscall method has one stack pointer argument and returns no value.
lf_checker_rt::export!(thiscall, rw_00c2f510(
    this: *mut u8,
    vehicle: *mut u8,
) -> () {
    const FLAGS_OFFSET: usize = 0x10;
    const DOOR_COUNT_OFFSET: usize = 0x05;
    const VEHICLE_DOOR_INTERFACE_OFFSET: usize = 0xdc4;
    const CLEAR_FLAGS_MASK: u32 = 0xffff_01ff;
    const SET_FLAGS_MASK: u32 = 0x0000_0100;
    const LATCH_PREDICATE: u32 = 1;
    const SET_LATCH_STATE: u32 = 2;
    const APPLY_DOOR_COUNT: u32 = 3;

    unsafe {
        let old_flags = this.add(FLAGS_OFFSET).cast::<u32>().read_unaligned();
        let updated_flags = (old_flags & CLEAR_FLAGS_MASK) | SET_FLAGS_MASK;
        this.add(FLAGS_OFFSET).cast::<u32>().write_unaligned(updated_flags);
        for offset in [0x14usize, 0x18, 0x1c, 0x20, 0x24] {
            this.add(offset).cast::<u32>().write_unaligned(0);
        }

        if old_flags & (1 << 18) != 0 {
            let predicate_result: u32 = lf_checker_rt::callee_thiscall!(
                LATCH_PREDICATE,
                u32,
                this as u32,
                vehicle as u32,
            );
            if predicate_result as u8 == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    SET_LATCH_STATE,
                    u32,
                    this as u32,
                    vehicle as u32,
                );
            }

            let door_count = this.add(DOOR_COUNT_OFFSET).cast::<i8>().read_unaligned();
            if door_count > 0 {
                let interface = vehicle
                    .add(VEHICLE_DOOR_INTERFACE_OFFSET)
                    .cast::<u32>()
                    .read_unaligned();
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    APPLY_DOOR_COUNT,
                    u32,
                    interface,
                    door_count as u32,
                );
            }
        }
    }
});
