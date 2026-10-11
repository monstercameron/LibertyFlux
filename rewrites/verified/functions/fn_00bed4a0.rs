// original: 0x00BED4A0 assign_task_selection_result

/// Resolve a task selection for `task_index`, where comparisons against the
/// signed task count and signed table result follow the original's signed
/// conditions. The task object holds a vtable at `+0`, the task-record array
/// pointer at `+0xF88`, its signed count at `+0xF8C`, a signed table class at
/// `+0x2E`, and a mode at `+0x1304`. Records are `0x34` bytes; their key is at
/// `+0` and flags at `+0x10`. The runtime class table is rooted at
/// `0x01295CD8`; its selected descriptor points at a key map at `+0xCC`.
/// Success writes type `0x11` and the low task-index byte to `this`; mode three
/// also copies a helper field at `+0x34` to `this + 4`. Direct helpers are
/// scripted thiscalls, while the two virtual lookups execute through the
/// supplied vtable entries. The original is thiscall with task and index
/// arguments.
lf_checker_rt::export!(thiscall, rw_00BED4A0(this: u32, task: u32, task_index: u32) -> u32 {
    const VTABLE_POINTER: u32 = 0;
    const FIRST_QUERY_SLOT: u32 = 0xA0;
    const RESULT_QUERY_SLOT: u32 = 0xE0;
    const OBJECT_RESULT: u32 = 0x100;
    const TASK_ARRAY: u32 = 0xF88;
    const TASK_COUNT: u32 = 0xF8C;
    const TABLE_CLASS: u32 = 0x2E;
    const TABLE_BASE_VA: u32 = 0x01295CD8;
    const TABLE_MAP: u32 = 0xCC;
    const RECORD_STRIDE: u32 = 0x34;
    const RECORD_FLAGS: u32 = 0x10;
    const MODE: u32 = 0x1304;
    const SUCCESS_TYPE: u8 = 0x11;
    const SPECIAL_MODE: u32 = 3;
    const SPECIAL_RECORD_KIND: u8 = 4;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn read_i16(address: u32) -> i16 {
        unsafe { (address as *const i16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u8(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }
    #[inline(always)]
    unsafe fn call_virtual(object: u32, slot: u32) -> u32 {
        let table = unsafe { read_u32(object + VTABLE_POINTER) };
        let entry = unsafe { read_u32(table + slot) };
        let method: unsafe extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(entry) };
        unsafe { method(object) }
    }

    unsafe {
        let first_query = call_virtual(task, FIRST_QUERY_SLOT);
        let mut eax = if first_query != 0 {
            let query_object = call_virtual(task, FIRST_QUERY_SLOT);
            call_virtual(query_object, RESULT_QUERY_SLOT)
        } else {
            read_u32(task + OBJECT_RESULT)
        };

        if eax == 0 {
            return eax & 0xFFFF_FF00;
        }

        let signed_index = task_index as i32;
        let signed_count = read_u32(task + TASK_COUNT) as i32;
        let array_base = read_u32(task + TASK_ARRAY);
        let flag_record = if signed_index < signed_count {
            array_base.wrapping_add(task_index.wrapping_mul(RECORD_STRIDE))
        } else {
            0
        };
        eax = read_u32(flag_record.wrapping_add(RECORD_FLAGS)) >> 8;
        if eax & 1 != 0 {
            return eax & 0xFFFF_FF00;
        }

        let key_record = if signed_index < signed_count {
            array_base.wrapping_add(task_index.wrapping_mul(RECORD_STRIDE))
        } else {
            0
        };
        let signed_class = read_i16(task + TABLE_CLASS) as i32;
        let table_slot = TABLE_BASE_VA.wrapping_add((signed_class as u32).wrapping_mul(4));
        let class_descriptor = lf_checker_rt::global::<u32>(table_slot).read_unaligned();
        let key_map = read_u32(class_descriptor.wrapping_add(TABLE_MAP));
        let task_key = read_u32(key_record);
        let mapped_value = read_u32(key_map.wrapping_add(task_key.wrapping_mul(4)));
        if (mapped_value as i32) <= -1 {
            return key_map & 0xFFFF_FF00;
        }

        write_u8(this, SUCCESS_TYPE);
        write_u8(this + 1, task_index as u8);
        if read_u32(task + MODE) != SPECIAL_MODE {
            let helper_result = lf_checker_rt::callee_thiscall!(3, u32, task, mapped_value);
            eax = lf_checker_rt::callee_thiscall!(4, u32, this, helper_result);
            return (eax & 0xFFFF_FF00) | 1;
        }

        let current_record = if signed_index < signed_count {
            array_base.wrapping_add(task_index.wrapping_mul(RECORD_STRIDE))
        } else {
            0
        };
        if current_record == 0 {
            return (key_map & 0xFFFF_FF00) | 1;
        }
        eax = read_u32(current_record.wrapping_add(RECORD_FLAGS));
        let record_kind = (eax as u8) & 0x0F;
        eax = (eax & 0xFFFF_FF00) | u32::from(record_kind);
        if record_kind != SPECIAL_RECORD_KIND {
            return (eax & 0xFFFF_FF00) | 1;
        }

        let helper_object = lf_checker_rt::callee_thiscall!(5, u32, task, mapped_value);
        eax = read_u32(helper_object.wrapping_add(0x34));
        write_u32(this + 4, eax);
        (eax & 0xFFFF_FF00) | 1
    }
});
