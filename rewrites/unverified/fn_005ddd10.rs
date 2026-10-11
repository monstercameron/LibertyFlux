// original: 0x005DDD10 select_treatment_task_variant

/// Select and initialize the treatment task selected by the manager.
///
/// The method first checks the task's active entity and the supplied
/// pedestrian's eligibility flag. Eligible entities are passed to one of
/// two manager constructors according to the task's one-byte mode; otherwise
/// the manager's fallback constructor is used. All manager and constructor
/// calls use the 32-bit thiscall convention and return their scripted result.
lf_checker_rt::export!(thiscall, rw_005ddd10(this: u32, pedestrian: u32) -> u32 {
    select_treatment_task_variant(this, pedestrian, false)
});

fn select_treatment_task_variant(this: u32, pedestrian: u32, mutate_mode: bool) -> u32 {
    const TASK_MANAGER: u32 = 0x0167E2A0;
    const ENTITY_OFFSET: u32 = 0x14;
    const TASK_KIND_OFFSET: u32 = 0x18;
    const MODE_OFFSET: u32 = 0x1C;
    const VECTOR_OFFSET: u32 = 0x30;
    const VARIANT_OFFSET: u32 = 0x42;
    const PED_FLAGS_OFFSET: u32 = 0x26C;
    const PED_TASK_OFFSET: u32 = 0xB30;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    let entity = unsafe { read_u32(this.wrapping_add(ENTITY_OFFSET)) };
    let ped_task = unsafe { read_u32(pedestrian.wrapping_add(PED_TASK_OFFSET)) };
    let ped_flags = unsafe { read_u32(pedestrian.wrapping_add(PED_FLAGS_OFFSET)) };
    let eligible = entity != 0 && ped_task != 0 && ped_flags & 4 != 0;
    let manager = unsafe { lf_checker_rt::global::<u32>(TASK_MANAGER).read_unaligned() };
    let factory = lf_checker_rt::callee_thiscall!(1, u32, manager);

    if factory == 0 {
        return 0;
    }

    if eligible {
        let mode = unsafe { (this.wrapping_add(MODE_OFFSET) as *const u8).read() };
        let variant = unsafe { (this.wrapping_add(VARIANT_OFFSET) as *const u8).read() } as u32;
        let variant = variant.wrapping_add(u32::from(mutate_mode));
        let task_kind = unsafe { read_u32(this.wrapping_add(TASK_KIND_OFFSET)) };
        let vector = this.wrapping_add(VECTOR_OFFSET);
        if mode != 0 {
            lf_checker_rt::callee_thiscall!(
                2, u32, factory, entity, task_kind, vector, variant,
            )
        } else {
            lf_checker_rt::callee_thiscall!(
                3, u32, factory, entity, task_kind, vector, variant,
            )
        }
    } else {
        lf_checker_rt::callee_thiscall!(4, u32, factory)
    }
}
