// original: 0x00BC03B0 assign_coordinate_task_to_ped

/// Allocate and attach the coordinate-driven task for a ped. The cdecl
/// arguments are the ped handle, a second pool handle, three coordinate
/// words, and a task option word. A current task whose byte at +0x0e is set
/// is returned as-is. Otherwise the coordinate words are copied into a local
/// three-word record, passed to the constructor with task type 0x48, and the
/// result is assigned to the ped with task id 0x65.
lf_checker_rt::export!(cdecl, rw_00BC03B0(
    ped_handle: u32,
    object_handle: u32,
    x_bits: u32,
    y_bits: u32,
    z_bits: u32,
    task_option: u32,
) -> u32 {
    const PED_POOL_GLOBAL: u32 = 0x018b_6f1c;
    const OBJECT_POOL_GLOBAL: u32 = 0x0163_2c60;
    const TASK_POOL_GLOBAL: u32 = 0x0167_e2a0;
    const PED_TASK: u32 = 0x6c;
    const TASK_READY: u32 = 0x0e;
    const TASK_TYPE: u32 = 0x48;
    const TASK_ID_COORDINATE: u32 = 0x65;
    const POOL_PED: u32 = 1;
    const POOL_OBJECT: u32 = 2;
    const TASK_ALLOC: u32 = 3;
    const TASK_CONSTRUCTOR: u32 = 4;
    const ASSIGN_TASK: u32 = 5;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_byte(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    let ped_pool = unsafe {
        lf_checker_rt::global::<u32>(PED_POOL_GLOBAL).read_unaligned()
    };
    if ped_handle != 0 {
        let ped = lf_checker_rt::callee_thiscall!(POOL_PED, u32, ped_pool, ped_handle);
        let current_task = unsafe { read_word(ped.wrapping_add(PED_TASK)) };
        if current_task != 0 && unsafe { read_byte(current_task.wrapping_add(TASK_READY)) } != 0 {
            return current_task;
        }
    }

    let object_pool = unsafe {
        lf_checker_rt::global::<u32>(OBJECT_POOL_GLOBAL).read_unaligned()
    };
    let object = lf_checker_rt::callee_thiscall!(POOL_OBJECT, u32, object_pool, object_handle);
    let task_pool = unsafe {
        lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL).read_unaligned()
    };
    let allocation = lf_checker_rt::callee_thiscall!(TASK_ALLOC, u32, task_pool);
    let task = if allocation != 0 {
        let coordinate_words = [x_bits, y_bits, z_bits];
        let coordinate_pointer = coordinate_words.as_ptr() as u32;
        lf_checker_rt::callee_thiscall!(
            TASK_CONSTRUCTOR,
            u32,
            allocation,
            object,
            TASK_TYPE,
            coordinate_pointer,
            task_option,
        )
    } else {
        0
    };
    lf_checker_rt::callee_cdecl!(ASSIGN_TASK, u32, ped_handle, task, TASK_ID_COORDINATE)
});
