// original: 0x00BC0A10 build_and_attach_scaled_position_task

/// Build and attach a scaled position task for the supplied owner. The cdecl
/// arguments are the owner word, three position words, and a signed integer
/// amount. The position words pass unchanged to the lookup. After that call,
/// the constructor receives a pointer to a three-word stack area the original
/// leaves uninitialized; its contract defines those words as zero. The signed
/// amount is converted to f32 and multiplied by the stored scale factor in that order.
/// A nonzero low byte from the readiness helper returns immediately; a null
/// lookup skips construction, and the final cdecl helper returns the result.
lf_checker_rt::export!(cdecl, rw_00BC0A10(
    owner: u32,
    position_x: u32,
    position_y: u32,
    position_z: u32,
    signed_amount: u32,
) -> u32 {
    const SCALE_FACTOR: u32 = 0x00fe_86b4;
    const MANAGER_GLOBAL: u32 = 0x0167_e2a0;
    const READY_CHECK: u32 = 1;
    const POSITION_LOOKUP: u32 = 2;
    const TASK_CONSTRUCTOR: u32 = 3;
    const OWNER_ASSIGN: u32 = 4;
    const TASK_KIND: u32 = 0x74;

    let ready = lf_checker_rt::callee_cdecl!(READY_CHECK, u32,);
    if ready as u8 != 0 {
        return ready;
    }

    let manager = unsafe {
        lf_checker_rt::global::<u32>(MANAGER_GLOBAL).read_unaligned()
    };
    let entity = lf_checker_rt::callee_thiscall!(
        POSITION_LOOKUP,
        u32,
        manager,
        position_x,
        position_y,
        position_z,
    );

    let task = if entity != 0 {
        let constructor_vector = [0_u32; 3];
        let amount = (signed_amount as i32) as f32;
        let scale = unsafe {
            f32::from_bits(lf_checker_rt::global::<u32>(SCALE_FACTOR).read_unaligned())
        };
        let scaled_amount =
            (core::hint::black_box(amount) * core::hint::black_box(scale)).to_bits();
        lf_checker_rt::callee_thiscall!(
            TASK_CONSTRUCTOR,
            u32,
            entity,
            0_u32,
            constructor_vector.as_ptr() as u32,
            scaled_amount,
            1_u32,
        )
    } else {
        0
    };

    lf_checker_rt::callee_cdecl!(OWNER_ASSIGN, u32, owner, task, TASK_KIND)
});
