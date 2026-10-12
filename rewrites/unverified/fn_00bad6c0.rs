// original: 0x00BAD6C0 NativeImpl_SET_NM_MESSAGE_VEC3_2
/// Forward a three-component native-message vector when its manager is active.
///
/// The first cdecl argument selects a word in the native-message table. The
/// function reads the manager pointer and returns immediately when it is null.
/// Otherwise it passes the selected table word and the three raw float words
/// to a thiscall helper, preserving the float bit patterns.
lf_checker_rt::export!(cdecl, rw_bad6c0(
    message_index: u32,
    x: u32,
    y: u32,
    z: u32,
) -> () {
    const MANAGER_VA: u32 = 0x0167F0A8;
    const MESSAGE_TABLE_VA: u32 = 0x01051CC0;
    let manager = unsafe { lf_checker_rt::global::<u32>(MANAGER_VA).read_unaligned() };
    if manager == 0 {
        return;
    }
    let message_table = unsafe { lf_checker_rt::global::<u32>(MESSAGE_TABLE_VA) };
    let message = unsafe { message_table.wrapping_add(message_index as usize).read_unaligned() };
    let vector: [u32; 3] = [x, y, z];
    let _ = lf_checker_rt::callee_thiscall!(1, u32, manager, message, vector[0], vector[1], vector[2]);
});
