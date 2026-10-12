// original: 0x00A6CD80 CTaskComplexPlayerOnFoot::CTaskComplexPlayerOnFoot

/// Initialize the fixed fields and shared task slots for a player-on-foot
/// complex task. The ECX receiver is a 32-bit task object; the method is
/// thiscall and returns the receiver in EAX. It first invokes the base task
/// constructor, then two task setup helpers. It resets four shared task
/// indices and flags while visiting four 16-byte global task records, invokes
/// the record helper once more for the embedded task at offset 0x18, and
/// initializes the remaining object fields. The fixed walk makes the cold
/// overflow branch unreachable. The final scale field is the bit pattern for
/// 10.0f. The object layout is inherently 32-bit.
lf_checker_rt::export!(thiscall, rw_00A6CD80(task: u32) -> u32 {
    unsafe {
        const OBJECT_VTABLE: u32 = 0x00e9_ee14;
        const GLOBAL_INDEX_ARRAY: u32 = 0x012f_a6bc;
        const GLOBAL_FLAG_ARRAY: u32 = 0x012f_a6cc;
        const GLOBAL_TASK_RECORDS: u32 = 0x012f_a6f8;
        const GLOBAL_RECORD_END: u32 = 0x012f_a738;
        const BASE_CONSTRUCTOR: u32 = 1;
        const TASK_SETUP: u32 = 2;
        const RECORD_HELPER: u32 = 3;

        #[inline(always)]
        unsafe fn write_u32(base: u32, offset: usize, value: u32) {
            unsafe {
                (base as usize as *mut u8)
                    .add(offset)
                    .cast::<u32>()
                    .write_unaligned(value);
            }
        }

        #[inline(always)]
        unsafe fn write_u8(base: u32, offset: usize, value: u8) {
            unsafe {
                (base as usize as *mut u8).add(offset).write(value);
            }
        }

        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, task);
        (task as usize as *mut u32).write_unaligned(lf_checker_rt::relocated(OBJECT_VTABLE));
        write_u8(task, 0x14, 0);
        let _ = lf_checker_rt::callee_thiscall!(TASK_SETUP, u32, task.wrapping_add(0x18));
        write_u32(task, 0x70, 0);
        write_u32(task, 0x80, 0);
        write_u32(task, 0x84, 0);

        let mut record = GLOBAL_TASK_RECORDS;
        let mut index = 0u32;
        while record < GLOBAL_RECORD_END {
            let _ = lf_checker_rt::callee_thiscall!(
                RECORD_HELPER,
                u32,
                lf_checker_rt::relocated(record),
            );
            (lf_checker_rt::relocated(GLOBAL_INDEX_ARRAY + index * 4) as *mut u32)
                .write_unaligned(u32::MAX);
            if index >= 4 {
                break;
            }
            (lf_checker_rt::relocated(GLOBAL_FLAG_ARRAY + index) as *mut u8)
                .write_unaligned(0);
            record = record.wrapping_add(0x10);
            index += 1;
        }

        let _ = lf_checker_rt::callee_thiscall!(
            RECORD_HELPER,
            u32,
            task.wrapping_add(0x18),
        );

        write_u32(task, 0x28, u32::MAX);
        write_u8(task, 0x2c, 0);
        write_u32(task, 0x30, 0);
        write_u32(task, 0x34, 0);
        write_u32(task, 0x38, 0);
        write_u8(task, 0x40, 0);
        write_u32(task, 0x44, 0);
        write_u32(task, 0x48, 0);
        write_u32(task, 0x4c, 0);
        write_u8(task, 0x50, 0);
        write_u32(task, 0x60, 0);
        write_u32(task, 0x64, 0);
        write_u32(task, 0x68, 0);
        write_u32(task, 0x74, 0);
        write_u32(task, 0x78, 0);
        write_u32(task, 0x7c, 0);
        write_u32(task, 0x90, 0);
        write_u32(task, 0x94, 0);
        write_u32(task, 0x98, 0);
        write_u32(task, 0xa0, 0x4120_0000);

        task
    }
});
