// original: 0x005DE340 CTaskComplexMedicDriver::CTaskComplexMedicDriver_2

/// Construct a medic-driver task from an entity, task kind, and vector input.
///
/// The constructor calls its base initializer, copies two pointers, three
/// f32 bit patterns from the supplied vector, and a one-byte mode into the
/// task layout. It clears progress fields, sets the all-ones sentinel, and
/// passes each non-null input to its reference helper with a pointer to the
/// stored field. The 32-bit thiscall constructor consumes four stack words.
lf_checker_rt::export!(thiscall, rw_005de340(this: u32, entity: u32, task: u32, vector: u32, mode: u32) -> u32 {
    initialize_medic_driver_from_vector(this, entity, task, vector, mode, false)
});

fn initialize_medic_driver_from_vector(
    this: u32,
    entity: u32,
    task: u32,
    vector: u32,
    mode: u32,
    mutate_mode: bool,
) -> u32 {
    const VTABLE: u32 = 0x00FE0C04;
    const COMPLETION_SENTINEL: u32 = 0xFFFF_FFFF;

    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    let vector_x_bits = unsafe { read_u32(vector) };
    let vector_y_bits = unsafe { read_u32(vector.wrapping_add(4)) };
    let vector_z_bits = unsafe { read_u32(vector.wrapping_add(8)) };
    unsafe {
        write_u32(this, lf_checker_rt::relocated(VTABLE));
        write_u32(this.wrapping_add(0x14), entity);
        write_u32(this.wrapping_add(0x18), task);
        write_u32(this.wrapping_add(0x1C), 0);
        write_u32(this.wrapping_add(0x20), vector_x_bits);
        write_u32(this.wrapping_add(0x24), vector_y_bits);
        write_u32(this.wrapping_add(0x28), vector_z_bits);
        (this.wrapping_add(0x30) as *mut u16).write_unaligned(0);
        (this.wrapping_add(0x32) as *mut u8).write((mode as u8).wrapping_add(u8::from(mutate_mode)));
        write_u32(this.wrapping_add(0x34), 0);
        write_u32(this.wrapping_add(0x38), 0);
        (this.wrapping_add(0x3C) as *mut u16).write_unaligned(0);
        write_u32(this.wrapping_add(0x40), COMPLETION_SENTINEL);
        write_u32(this.wrapping_add(0x44), 0);
    }

    if entity != 0 {
        let _ = lf_checker_rt::callee_thiscall!(2, u32, entity, this.wrapping_add(0x14));
    }
    if task != 0 {
        let _ = lf_checker_rt::callee_thiscall!(3, u32, task, this.wrapping_add(0x18));
    }
    this
}
