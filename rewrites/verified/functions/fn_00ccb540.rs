use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

const TASK_FLAGS_OFFSET: usize = 0x268;
const TASK_COMPONENT_OFFSET: usize = 0x2b0;
const TASK_SUBJECT_OFFSET: usize = 0x2c4;
const TASK_MODE_OFFSET: usize = 0x219;
const SUBJECT_RANDOM_FLAG_OFFSET: usize = 0x210;
const RANDOM_FLAG_MASK: u32 = 0x0200_0000;

/// Proof scope: subject and factory result are nonnull; accepted mode zero
/// is excluded from the tested fixture. Helper bodies are scripted; temporary
/// pointer identities are skipped while their declared pointee outputs are
/// compared. Return and full data-section comparisons are disabled, and the
/// checker uses zero stack fill. Null-factory fault diagnostics add no proof.
///
/// Update the selected subject flag, validate the component, then run its
/// task interface or the fallback. The helper-four call precedes the task
/// interface read even when the factory returns null.
unsafe fn update_task_flag(owner_value: u32) {
    let owner = owner_value as *mut u8;
    let task_flags = unsafe { owner.add(TASK_FLAGS_OFFSET).read() };
    let subject = unsafe { owner.add(TASK_SUBJECT_OFFSET).cast::<u32>().read_unaligned() };

    if task_flags & 0x08 == 0 && subject != 0 {
        let random = lf_checker_rt::callee_cdecl!(1, u32,);
        let desired = if (random as i32) < 0x3fff { RANDOM_FLAG_MASK } else { 0 };
        let flag = unsafe {
            (subject as *mut u8)
                .add(SUBJECT_RANDOM_FLAG_OFFSET)
                .cast::<u32>()
                .read_unaligned()
        };
        let updated = flag ^ ((flag ^ desired) & RANDOM_FLAG_MASK);
        unsafe {
            (subject as *mut u8)
                .add(SUBJECT_RANDOM_FLAG_OFFSET)
                .cast::<u32>()
                .write_unaligned(updated)
        };
    }

    let subject = unsafe { owner.add(TASK_SUBJECT_OFFSET).cast::<u32>().read_unaligned() };
    let component = owner_value.wrapping_add(TASK_COMPONENT_OFFSET as u32);
    let accepted = callee_thiscall!(2, u32, component, subject);
    if accepted as u8 != 0 {
        let mode = unsafe { owner.add(TASK_MODE_OFFSET).read() };
        let enable = u32::from(mode == 0);
        let task = callee_thiscall!(3, u32, component, owner_value, enable);
        let mut first = [0u32; 1];
        let mut second = [0u32; 1];
        let mut third = [0u32; 1];
        let _ = callee_thiscall!(
            4,
            u32,
            owner_value,
            first.as_mut_ptr() as u32,
            second.as_mut_ptr() as u32,
            third.as_mut_ptr() as u32
        );
        let _ = unsafe { core::ptr::read_volatile(task as *const u32) };
        let _ = callee_thiscall!(5, u32, task, first.as_mut_ptr() as u32, 0, 0);
    } else {
        let _ = callee_cdecl!(6, u32, owner_value);
    }
}

export!(stdcall, rw_00ccb540(owner: u32) -> () {
    unsafe { update_task_flag(owner) }
});
