// original: 0x00BC0C30 build_and_attach_two_position_tasks

/// Build two position tasks from the owner's position inputs and attach the
/// resulting task list. The cdecl arguments are the owner word, three raw
/// position words, and a signed integer amount. The first constructor observes
/// the three position words. The second observes a stack record beginning with
/// the amount converted to f32 and scaled, followed by the first two position
/// words; its separate constructor arguments are both zero. Lookup,
/// constructor, list-update and attach calls retain their original order.
lf_checker_rt::export!(cdecl, rw_00BC0C30(
    owner: u32,
    position_x: u32,
    position_y: u32,
    position_z: u32,
    signed_amount: u32,
) -> u32 {
    const MANAGER_GLOBAL: u32 = 0x0167_e2a0;
    const SCALE_FACTOR: u32 = 0x00fe_86b4;
    const FIRST_LOOKUP: u32 = 1;
    const GET_LIST: u32 = 2;
    const SECOND_LOOKUP: u32 = 3;
    const FIRST_CONSTRUCTOR: u32 = 4;
    const FIRST_LIST_UPDATE: u32 = 5;
    const THIRD_LOOKUP: u32 = 6;
    const SECOND_CONSTRUCTOR: u32 = 7;
    const SECOND_LIST_UPDATE: u32 = 8;
    const ATTACH_TASK_LIST: u32 = 9;
    const TASK_KIND: u32 = 0x5d;

    let manager = unsafe {
        lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned()
    };
    let position = [position_x, position_y, position_z];
    let first_entity = lf_checker_rt::callee_thiscall!(FIRST_LOOKUP, u32, manager,);
    let task_list = if first_entity != 0 {
        lf_checker_rt::callee_thiscall!(GET_LIST, u32, first_entity,)
    } else {
        0
    };

    let first_target = unsafe {
        lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned()
    };
    let first_target = lf_checker_rt::callee_thiscall!(SECOND_LOOKUP, u32, first_target,);
    let first_task = if first_target != 0 {
        lf_checker_rt::callee_thiscall!(
            FIRST_CONSTRUCTOR,
            u32,
            first_target,
            0_u32,
            position.as_ptr() as u32,
            0_u32,
            1_u32,
            0x4040_0000_u32,
        )
    } else {
        0
    };
    let _first_list_update = lf_checker_rt::callee_thiscall!(
        FIRST_LIST_UPDATE,
        u32,
        task_list,
        first_task,
    );

    let second_target = unsafe {
        lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned()
    };
    let second_target = lf_checker_rt::callee_thiscall!(THIRD_LOOKUP, u32, second_target,);
    let amount = (signed_amount as i32) as f32;
    let scale = unsafe {
        f32::from_bits(lf_checker_rt::global::<u32>(SCALE_FACTOR).read_unaligned())
    };
    let scaled_amount =
        (core::hint::black_box(amount) * core::hint::black_box(scale)).to_bits();
    let second_position = [scaled_amount, position_x, position_y];
    let second_task = if second_target != 0 {
        lf_checker_rt::callee_thiscall!(
            SECOND_CONSTRUCTOR,
            u32,
            second_target,
            0_u32,
            second_position.as_ptr() as u32,
            0_u32,
            0_u32,
        )
    } else {
        0
    };
    let _second_list_update = lf_checker_rt::callee_thiscall!(
        SECOND_LIST_UPDATE,
        u32,
        task_list,
        second_task,
    );

    lf_checker_rt::callee_cdecl!(ATTACH_TASK_LIST, u32, owner, task_list, TASK_KIND)
});
