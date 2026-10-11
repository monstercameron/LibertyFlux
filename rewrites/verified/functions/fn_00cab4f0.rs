// original: 0x00cab4f0 CTaskComplexGoToPointAiming::CTaskComplexGoToPointAiming

/// Constructs an aiming go-to-point task. It records the task kind, target
/// reference, point metadata and byte flag, and replaces the stored target
/// vector and two scalar settings only if any of the five float comparisons
/// is unequal or unordered. `target_vector` points to four words; `point`
/// supplies the three words copied to `+0x20..+0x28`. The entity reference
/// uses the task's pointer field at `+0x18`. Float comparisons follow IEEE
/// equality, so signed zeros compare equal and NaNs trigger the update.
lf_checker_rt::export!(thiscall, rw_00cab4f0(
    this: u32,
    reference_id: u32,
    target_vector: u32,
    entity: u32,
    point: u32,
    setting_a: u32,
    setting_b: u32,
    byte_flag: u32
) -> u32 {
    const BASE_CTOR: u32 = 1;
    const ENTITY_REF_SET: u32 = 2;
    const TASK_VTABLE_VA: u32 = 0x00ed_82cc;
    const TASK_KIND: u32 = 0x14;
    const ENTITY_REFERENCE: u32 = 0x18;
    const POINT_X: u32 = 0x20;
    const TARGET_X: u32 = 0x30;
    const TARGET_Y: u32 = 0x34;
    const TARGET_Z: u32 = 0x38;
    const TARGET_METADATA: u32 = 0x3c;
    const SETTING_A: u32 = 0x40;
    const SETTING_B: u32 = 0x44;
    const BYTE_FLAG: u32 = 0x48;
    const TARGET_CHANGED: u32 = 0x49;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    fn same_float_bits(left: u32, right: u32) -> bool {
        f32::from_bits(left) == f32::from_bits(right)
    }

    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
    unsafe {
        write_word(this, lf_checker_rt::relocated(TASK_VTABLE_VA));
        write_word(this + TASK_KIND, reference_id);
        write_word(this + ENTITY_REFERENCE, entity);

        let point_x = read_word(point);
        let point_y = read_word(point + 4);
        let point_z = read_word(point + 8);
        write_word(this + POINT_X, point_x);
        write_word(this + POINT_X + 4, point_y);
        write_word(this + POINT_X + 8, point_z);
        (this.wrapping_add(BYTE_FLAG) as *mut u8).write(byte_flag as u8);

        if entity != 0 {
            let _ = lf_checker_rt::callee_thiscall!(
                ENTITY_REF_SET,
                u32,
                entity,
                this + ENTITY_REFERENCE
            );
        }

        let target_x = read_word(target_vector);
        let target_y = read_word(target_vector + 4);
        let target_z = read_word(target_vector + 8);
        let target_metadata = read_word(target_vector + 12);
        let unchanged = same_float_bits(read_word(this + TARGET_X), target_x)
            && same_float_bits(read_word(this + TARGET_Y), target_y)
            && same_float_bits(read_word(this + TARGET_Z), target_z)
            && same_float_bits(read_word(this + SETTING_A), setting_a)
            && same_float_bits(read_word(this + SETTING_B), setting_b);

        if !unchanged {
            write_word(this + TARGET_X, target_x);
            write_word(this + TARGET_Y, target_y);
            write_word(this + TARGET_Z, target_z);
            write_word(this + TARGET_METADATA, target_metadata);
            let changed_flag = (this.wrapping_add(TARGET_CHANGED) as *mut u8).read();
            (this.wrapping_add(TARGET_CHANGED) as *mut u8).write(changed_flag | 1);
            write_word(this + SETTING_A, setting_a);
            write_word(this + SETTING_B, setting_b);
        }
    }
    this
});
