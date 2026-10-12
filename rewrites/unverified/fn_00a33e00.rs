// original: 0x00A33E00 CEntity::attach_physics_instance

/// Attach a physics object to this entity. Store the reciprocal pointers,
/// enable the object's CCD mode through its helper, and return when the second
/// stack argument's low byte is zero. Otherwise sign-extend the entity's model
/// id at `+0x2e`, read that model's info pointer from the relocated model table,
/// and call the model helper only when its signed 16-bit status at `+0x52` is
/// nonnegative. A separate `+0x24` flag controls the final model callback.
/// The two stack arguments are the physics pointer and a byte-like switch.
lf_checker_rt::export!(thiscall, rw_00a33e00(this: u32, physics: u32, should_release: u32) -> () {
    const ENTITY_PHYSICS: u32 = 0x38;
    const ENTITY_FLAGS: u32 = 0x24;
    const ENTITY_MODEL_ID: u32 = 0x2e;
    const PHYSICS_OWNER: u32 = 0x0c;
    const MODEL_STATUS: u32 = 0x52;
    const MODEL_TABLE_VA: u32 = 0x0129_5cd8;
    const MODEL_ACTION_FLAG: u32 = 0x0400_0000;
    const SET_OBJECT_CCD: u32 = 1;
    const RELEASE_MODEL_IDE: u32 = 2;
    const ADD_MODEL_REF: u32 = 3;

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_i16(address: u32) -> i16 {
        i16::from_ne_bytes(unsafe { (address as *const u16).read_unaligned().to_ne_bytes() })
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        write_u32(this.wrapping_add(ENTITY_PHYSICS), physics);
        write_u32(physics.wrapping_add(PHYSICS_OWNER), this);
        let _ = lf_checker_rt::callee_thiscall!(SET_OBJECT_CCD, u32, physics, 0x40u32, 1u32);

        if should_release & 0xff == 0 {
            return;
        }

        let model_id = read_i16(this.wrapping_add(ENTITY_MODEL_ID));
        let table_slot = lf_checker_rt::relocated(MODEL_TABLE_VA)
            .wrapping_add(i32::from(model_id).wrapping_mul(4) as u32);
        let model_info = read_u32(table_slot);
        let model_status = read_i16(model_info.wrapping_add(MODEL_STATUS));
        if model_status > -1 {
            let _ = lf_checker_rt::callee_thiscall!(RELEASE_MODEL_IDE, u32, model_info);
        }

        if read_u32(this.wrapping_add(ENTITY_FLAGS)) & MODEL_ACTION_FLAG != 0 {
            let _ = lf_checker_rt::callee_thiscall!(ADD_MODEL_REF, u32, model_info);
        }
    }
});
