// original: 0x005DE260 CTaskComplexMedicDriver::CTaskComplexMedicDriver

/// Initialize a medic-driver task from its two referenced inputs.
///
/// The constructor calls the base initializer, stores both input pointers,
/// copies three f32 constants from the shared configuration data, and clears
/// the task's progress fields while setting its sentinel to all ones. Each
/// non-null input is passed by reference to the matching 32-bit thiscall
/// helper. It returns the initialized object and consumes two stack words.
lf_checker_rt::export!(thiscall, rw_005de260(this: u32, entity: u32, task: u32) -> u32 {
    initialize_medic_driver(this, entity, task, false)
});

fn initialize_medic_driver(this: u32, entity: u32, task: u32, wrong_sentinel: bool) -> u32 {
    const FIRST_CONFIGURATION: u32 = 0x01B4B2A0;
    const SECOND_CONFIGURATION: u32 = 0x01B4B2A4;
    const THIRD_CONFIGURATION: u32 = 0x01B4B2A8;
    const VTABLE: u32 = 0x00FE0C04;
    const COMPLETION_SENTINEL: u32 = 0xFFFF_FFFF;

    // The direct base initializer is intercepted by the checker.
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn write_u16(address: u32, value: u16) {
        unsafe { (address as *mut u16).write_unaligned(value) }
    }

    unsafe {
        write_u32(this, lf_checker_rt::relocated(VTABLE));
        write_u32(this.wrapping_add(0x14), entity);
        write_u32(this.wrapping_add(0x18), task);
        write_u32(this.wrapping_add(0x1C), 0);
        write_u32(this.wrapping_add(0x20), lf_checker_rt::global::<u32>(FIRST_CONFIGURATION).read_unaligned());
        write_u32(this.wrapping_add(0x24), lf_checker_rt::global::<u32>(SECOND_CONFIGURATION).read_unaligned());
        write_u32(this.wrapping_add(0x28), lf_checker_rt::global::<u32>(THIRD_CONFIGURATION).read_unaligned());
        write_u16(this.wrapping_add(0x30), 0);
        (this.wrapping_add(0x32) as *mut u8).write(0);
        write_u32(this.wrapping_add(0x34), 0);
        write_u32(this.wrapping_add(0x38), 0);
        write_u16(this.wrapping_add(0x3C), 0);
        write_u32(this.wrapping_add(0x40), if wrong_sentinel { 0 } else { COMPLETION_SENTINEL });
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
