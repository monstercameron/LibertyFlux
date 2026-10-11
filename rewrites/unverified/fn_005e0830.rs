// original: 0x005E0830 CTaskComplexDriveFireTruck::~CTaskComplexDriveFireTruck__deleting_2

/// Query the current child task's virtual type and create a successor only
/// for the signed-comparison result 0x2c6 or the special type 0x6d. The
/// supplied character object must also have bit 2 set in its byte at +0x26c.
/// When both tests pass, this calls the task's virtual slot at +0x4c with the
/// character pointer and returns that method's result. Every other path
/// returns zero. Both calls remain virtual calls through the fabricated
/// tables used by the contract.
lf_checker_rt::export!(thiscall, rw_005e0830(this: u32, character: u32) -> u32 {
    unsafe {
        const CURRENT_TASK: u32 = 0x08;
        const TASK_TYPE_SLOT: u32 = 0x0c;
        const CREATE_NEXT_SLOT: u32 = 0x4c;
        const CHARACTER_FLAGS: u32 = 0x26c;
        const NEEDS_SUCCESSOR_TYPE: u32 = 0x2c6;
        const CAR_SET_TYPE: u32 = 0x6d;
        type GetTaskType = extern "thiscall" fn(u32) -> u32;
        type CreateNextTask = extern "thiscall" fn(u32, u32) -> u32;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        let current_task = read_u32(this.wrapping_add(CURRENT_TASK));
        let task_vtable = read_u32(current_task);
        let get_task_type: GetTaskType = core::mem::transmute(
            read_u32(task_vtable.wrapping_add(TASK_TYPE_SLOT)) as usize,
        );
        let task_type = get_task_type(current_task);

        let should_create_successor = task_type == NEEDS_SUCCESSOR_TYPE || task_type == CAR_SET_TYPE;
        let has_required_character_flag = read_u8(character.wrapping_add(CHARACTER_FLAGS)) & 4 != 0;
        if should_create_successor && has_required_character_flag {
            let task_vtable = read_u32(this);
            let create_next: CreateNextTask = core::mem::transmute(
                read_u32(task_vtable.wrapping_add(CREATE_NEXT_SLOT)) as usize,
            );
            return create_next(this, character);
        }
        0
    }
});
