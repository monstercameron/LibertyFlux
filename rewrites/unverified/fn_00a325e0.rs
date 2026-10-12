// original: 0x00A325E0 CEntity::update_physics_instance

/// Query the physics instance referenced by `CEntity::physics_rigid_body_record`
/// at byte offset `0x38`. A null instance returns immediately. Otherwise its
/// function table supplies a zero-argument classification method at byte
/// offset `0x04`; the method is called again only when the first signed result
/// is below 13, and processing continues only when that second result equals
/// 3. The instance method at byte offset `0x08` then receives the caller's
/// single 32-bit argument. The original uses thiscall, with the entity in ECX,
/// one stack argument, and callee cleanup. It has no meaningful return value.
lf_checker_rt::export!(thiscall, rw_00a325e0(entity: u32, command: u32) -> u32 {
    unsafe {
        const PHYSICS_INSTANCE: u32 = 0x38;
        const CLASSIFY_SLOT: u32 = 0x04;
        const UPDATE_SLOT: u32 = 0x08;
        const CLASSIFY_THRESHOLD: i32 = 13;
        const ACCEPTED_CLASS: u32 = 3;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let instance = read_u32(entity.wrapping_add(PHYSICS_INSTANCE));
        if instance == 0 {
            return 0;
        }

        let vtable = read_u32(instance);
        let classify_address = read_u32(vtable.wrapping_add(CLASSIFY_SLOT));
        let classify: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(classify_address as usize);

        let first_class = classify(instance);
        if (first_class as i32) < CLASSIFY_THRESHOLD {
            let second_class = classify(instance);
            if second_class != ACCEPTED_CLASS {
                return 0;
            }
        }

        let update_address = read_u32(vtable.wrapping_add(UPDATE_SLOT));
        let update: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(update_address as usize);
        let _ignored_result = update(instance, command);
        0
    }
});
