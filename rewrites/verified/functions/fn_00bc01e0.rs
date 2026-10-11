// original: 0x00BC01E0 GIVING_PED_SEQUENCE

/// Attach the selected sequence task to a ped when its handle is valid. The
/// two cdecl parameters are the ped handle and script context. A live existing
/// task with byte +0x0e set is returned unchanged; sequence indexes above the
/// unsigned bound 0x7f are rejected. Otherwise the handler builds the optional
/// text description, allocates a sequence task, and forwards it with task id
/// 0x1d. Pool and task-pool pointers come from their global pointer slots.
lf_checker_rt::export!(cdecl, rw_00BC01E0(ped_handle: u32, script_context: u32) -> u32 {
    const PED_POOL_GLOBAL: u32 = 0x018b_6f1c;
    const TASK_POOL_GLOBAL: u32 = 0x0167_e2a0;
    const PED_TASK: u32 = 0x6c;
    const TASK_READY: u32 = 0x0e;
    const TASK_ID_SEQUENCE: u32 = 0x1d;
    const MAX_SEQUENCE_INDEX: u32 = 0x7f;
    const TEXT_BUFFER: u32 = 0x0193_5fa8;
    const STRING_TABLE: u32 = 0x018e_51e8;
    const STRING_FORMAT: u32 = 0x00eb_7020;
    const STRING_TOKEN_FORMAT: u32 = 0x00eb_70a4;
    const STRING_TOKEN_SUFFIX: u32 = 0x00eb_7090;
    const POOL_FIRST: u32 = 1;
    const SCRIPT_INDEX: u32 = 2;
    const POOL_SECOND: u32 = 3;
    const TEXT_MODE: u32 = 4;
    const FORMAT_INITIAL: u32 = 5;
    const FORMAT_TOKEN: u32 = 6;
    const STRING_LENGTH: u32 = 7;
    const STRING_TOKEN: u32 = 8;
    const TASK_ALLOC: u32 = 9;
    const TASK_CONSTRUCTOR: u32 = 10;
    const ASSIGN_TASK: u32 = 11;

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
        let ped = lf_checker_rt::callee_thiscall!(POOL_FIRST, u32, ped_pool, ped_handle);
        let current_task = unsafe { read_word(ped.wrapping_add(PED_TASK)) };
        if current_task != 0 && unsafe { read_byte(current_task.wrapping_add(TASK_READY)) } != 0 {
            return current_task;
        }
    }

    let sequence_index = lf_checker_rt::callee_cdecl!(SCRIPT_INDEX, u32, script_context, 4_u32);
    if sequence_index > MAX_SEQUENCE_INDEX {
        return sequence_index;
    }

    let ped = lf_checker_rt::callee_thiscall!(POOL_SECOND, u32, ped_pool, ped_handle);
    if ped != 0 {
        let current_task = unsafe { read_word(ped.wrapping_add(PED_TASK)) };
        if current_task != 0 {
            let text_buffer = lf_checker_rt::relocated(TEXT_BUFFER);
            let string_table = lf_checker_rt::relocated(STRING_TABLE);
            let format = lf_checker_rt::relocated(STRING_FORMAT);
            let _ = lf_checker_rt::callee_thiscall!(TEXT_MODE, u32, text_buffer, 0_u32);
            let _ = lf_checker_rt::callee_cdecl!(
                FORMAT_INITIAL,
                u32,
                text_buffer,
                2_u32,
                format,
            );
            let _ = lf_checker_rt::callee_thiscall!(TEXT_MODE, u32, text_buffer, 1_u32);
            let string_length = lf_checker_rt::callee_thiscall!(STRING_LENGTH, u32, string_table);
            let signed_key = (string_length as u8 as i8 as i32) as u32;
            let token = lf_checker_rt::callee_thiscall!(
                STRING_TOKEN,
                u32,
                current_task,
                sequence_index,
                signed_key,
            );
            let _ = lf_checker_rt::callee_cdecl!(
                FORMAT_TOKEN,
                u32,
                lf_checker_rt::relocated(TEXT_BUFFER),
                2_u32,
                lf_checker_rt::relocated(STRING_TOKEN_FORMAT),
                lf_checker_rt::relocated(STRING_TOKEN_SUFFIX),
                token,
            );
        }
    }

    let task_pool = unsafe {
        lf_checker_rt::global::<u32>(TASK_POOL_GLOBAL).read_unaligned()
    };
    let allocation = lf_checker_rt::callee_thiscall!(TASK_ALLOC, u32, task_pool);
    let task = if allocation != 0 {
        lf_checker_rt::callee_thiscall!(TASK_CONSTRUCTOR, u32, allocation, sequence_index)
    } else {
        0
    };
    lf_checker_rt::callee_cdecl!(ASSIGN_TASK, u32, ped_handle, task, TASK_ID_SEQUENCE)
});
