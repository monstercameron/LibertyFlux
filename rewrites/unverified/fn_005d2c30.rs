// original: 0x005D2C30 blip_slot_detach

/// Detach the object currently stored in a blip slot, then clear the slot's
/// object pointer and 16-bit timing key.
///
/// The thiscall receiver is the slot. If it contains an object, the function
/// conditionally clears that object's timing value when the object's active
/// state is not either of the two protected values and its timing mode is
/// `2`. It then notifies the object's timing subsystem with the slot pointer.
/// All offsets are byte offsets; comparisons of the 32-bit state values are
/// unsigned equality tests. The function returns zero and has no stack args.
lf_checker_rt::export!(thiscall, rw_005d2c30(this: u32) -> u32 {
    unsafe {
        const SLOT_OBJECT: usize = 0;
        const SLOT_TIMING_KEY: usize = 0x40;
        const OBJECT_ENABLED: usize = 0x210;
        const OBJECT_STATE: usize = 0xA74;
        const OBJECT_TIMING_MODE_PTR: usize = 0x21C;
        const OBJECT_TIMING_MODE: usize = 0x224;
        const OBJECT_TIMING_TARGET_PTR: usize = 0x22C;
        const TIMING_MODE_VALUE: u32 = 2;
        const CLEARED_TIMING_VALUE: u32 = u32::MAX;

        let slot = this as *mut u8;
        let object = (slot as *mut u32).read_unaligned();
        if object != 0 {
            let entity = object as *mut u8;
            if entity.add(OBJECT_ENABLED).read() != 0 {
                let state = (entity.add(OBJECT_STATE) as *mut u32).read_unaligned();
                if state != 1 && state != 2 {
                    let mode = (entity.add(OBJECT_TIMING_MODE) as *mut u32).read_unaligned();
                    if mode != 0 {
                        let mode_object = (entity.add(OBJECT_TIMING_MODE_PTR) as *mut u32).read_unaligned();
                        let mode_value = (mode_object as *const u8)
                            .add(0x12C)
                            .cast::<u32>()
                            .read_unaligned();
                        if mode_value == TIMING_MODE_VALUE {
                            let target = (entity.add(OBJECT_TIMING_TARGET_PTR) as *mut u32).read_unaligned();
                            (target as *mut u8).add(0x54).cast::<u32>().write_unaligned(CLEARED_TIMING_VALUE);
                        }
                    }
                }
            }
            let _ = lf_checker_rt::callee_thiscall!(1, u32, object, this);
        }
        (slot as *mut u32).write_unaligned(0);
        slot.add(SLOT_TIMING_KEY).cast::<u16>().write_unaligned(0);
        0
    }
});
