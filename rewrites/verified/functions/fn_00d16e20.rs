// original: 0x00d16e20 CTaskSimplePlaySwatSignalAnim::vf17
/// Advances the signal animation state until one frame makes no transition.
/// State transitions use the engine helper's AL result; state 3 invokes the
/// finalizer and completes, while unsigned states above three return false.
/// The state-zero path copies the engine mode global to its result global.

const ACTION_KIND: u32 = 0x8B;
const FOUR_SECONDS_BITS: u32 = 0x4080_0000;
const ENGINE_MODE_VA: u32 = 0x0117_35B4;
const SIGNAL_MODE_VA: u32 = 0x0171_DE4C;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    (address as *const u32).read_unaligned()
}

lf_checker_rt::export!(thiscall, rw_00d16e20(task: u32, entity: u32) -> u32 {
    unsafe {
        let starting_state = read_u32(task + 0x14);
        loop {
            let state = read_u32(task + 0x14);
            match state {
                0 => {
                    let mode = lf_checker_rt::global::<u32>(ENGINE_MODE_VA).read_unaligned();
                    lf_checker_rt::global::<u32>(SIGNAL_MODE_VA)
                        .write_unaligned(mode);
                    let ready = lf_checker_rt::callee_cdecl!(
                        1,
                        u32,
                        entity,
                        task + 0x24
                    );
                    let next_state = if ready & 0xFF != 0 { 1 } else { 3 };
                    (task as *mut u32).add(5).write_unaligned(next_state);
                }
                1 => {
                    let ready = lf_checker_rt::callee_thiscall!(
                        2,
                        u32,
                        task + 0x18,
                        read_u32(task + 0x24)
                    );
                    if ready & 0xFF != 0 {
                        let _factory_result = lf_checker_rt::callee_thiscall!(
                            3,
                            u32,
                            task,
                            entity,
                            read_u32(task + 0x24),
                            read_u32(task + 0x28),
                            FOUR_SECONDS_BITS,
                            1
                        );
                        (task as *mut u32).add(5).write_unaligned(2);
                    }
                }
                2 => {
                    if read_u32(task + 0x10) == 0 {
                        (task as *mut u32).add(5).write_unaligned(3);
                    }
                }
                3 => {
                    let _finished = lf_checker_rt::callee_thiscall!(
                        2,
                        u32,
                        task + 0x18,
                        u32::MAX
                    );
                    let _finalizer = lf_checker_rt::callee_thiscall!(4, u32, task);
                    return 1;
                }
                _ => return 0,
            }

            if read_u32(task + 0x14) == starting_state {
                return 0;
            }
        }
    }
});
