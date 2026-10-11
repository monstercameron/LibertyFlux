// original: 0x00CD8960 CTaskComplexSeekEntityAnyMeans<CEntitySeekPosCalculatorXYOffset>::vf19

/// Refresh the task's timer fields, ask its vector resolver for three raw
/// float words, then create a follow task through the shared intelligence
/// context. If no ped is available the method returns zero; otherwise it
/// forwards the creator's result. The helper arguments preserve their stack
/// order, including the three-word vector produced in a temporary buffer.
///
/// The period is read at byte offset `0x18`, the float word at `0x1C`, and
/// the timer is loaded from the shared current-time slot. The routine writes
/// time, period and the active-state byte at offsets `0x20`, `0x24` and
/// `0x28` respectively. Float values are copied as exact 32-bit words.
///
/// Calling convention: thiscall with one 32-bit stack argument. All three
/// outgoing helpers are thiscall; the final creator receives nine stack
/// words and returns the result unchanged.
lf_checker_rt::export!(thiscall, rw_00cd8960(this: u32, request: u32) -> u32 {
    const INTELLIGENCE_CONTEXT: u32 = 0x0167_E2A0;
    const CURRENT_TIME: u32 = 0x0117_35B4;
    const PERIOD: u32 = 0x18;
    const FLOAT_WORD: u32 = 0x1C;
    const STORED_TIME: u32 = 0x20;
    const DURATION: u32 = 0x24;
    const ACTIVE_STATE: u32 = 0x28;
    const BUILD_VECTOR: u32 = 1;
    const FIND_PED: u32 = 2;
    const CREATE_TASK: u32 = 3;
    const TASK_KIND: u32 = 0x1E;
    const TASK_SLOT: u32 = 0x14;

    unsafe {
        let mut vector_words = [0u32; 3];
        let vector_pointer = vector_words.as_mut_ptr() as usize as u32;
        let _ = lf_checker_rt::callee_thiscall!(BUILD_VECTOR, u32, this, request, vector_pointer);

        let period = ((this + PERIOD) as *const u32).read_unaligned();
        let float_bits = ((this + FLOAT_WORD) as *const u32).read_unaligned();
        let now = lf_checker_rt::global::<u32>(CURRENT_TIME).read();
        ((this + STORED_TIME) as *mut u32).write_unaligned(now);
        ((this + DURATION) as *mut u32).write_unaligned(period);
        ((this + ACTIVE_STATE) as *mut u8).write(1);

        let context = lf_checker_rt::global::<u32>(INTELLIGENCE_CONTEXT).read();
        let ped = lf_checker_rt::callee_thiscall!(FIND_PED, u32, context);
        if ped == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(
                CREATE_TASK,
                u32,
                ped,
                0,
                vector_pointer,
                TASK_KIND,
                0,
                u32::MAX,
                float_bits,
                0,
                TASK_SLOT,
                1
            )
        }
    }
});
