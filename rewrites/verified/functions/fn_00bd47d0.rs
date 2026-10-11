// original: 0x00bd47d0 NativeImpl_CREATE_CHECKPOINT

/// Forward seven coordinate or checkpoint words and the leading handle word to the checkpoint constructor. If its signed result is nonnegative, fetch the current script thread and register the checkpoint with the script manager. Return the constructor result on both paths.
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn rw_00bd47d0(
    handle_word: u32,
    arg_2: u32,
    arg_3: u32,
    arg_4: u32,
    arg_5: u32,
    arg_6: u32,
    arg_7: u32,
    arg_8: u32,
) -> u32 {
    const CREATE_CHECKPOINT: u32 = 1;
    const GET_CURRENT_THREAD: u32 = 2;
    const REGISTER_CHECKPOINT: u32 = 3;
    const SCRIPT_MANAGER: u32 = 0x012b_d0c4;

    unsafe {
        let checkpoint_id = lf_checker_rt::callee_cdecl!(
            CREATE_CHECKPOINT,
            u32,
            arg_2,
            arg_3,
            arg_4,
            arg_5,
            arg_6,
            arg_7,
            arg_8,
            handle_word,
        );
        if (checkpoint_id as i32) >= 0 {
            let thread_id = lf_checker_rt::callee_stdcall!(GET_CURRENT_THREAD, u32, 0, 0);
            let manager = lf_checker_rt::global::<u32>(SCRIPT_MANAGER).read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(
                REGISTER_CHECKPOINT,
                u32,
                manager,
                checkpoint_id,
                11,
                thread_id,
            );
        }
        checkpoint_id
    }
}
