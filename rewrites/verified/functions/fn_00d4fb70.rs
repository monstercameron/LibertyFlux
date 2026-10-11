// original: 0x00D4FB70 CTaskSimplePutDownObject::vf17
/// Update the putdown task and release its active resource. A set completion
/// byte at `+0x2c` returns true immediately. Otherwise a subobject constructor
/// receives the pointer at `+0x18`; a missing task link at `+0x1c` triggers the
/// secondary handler and returns false. When the task link's flag word at
/// `+0x74` has bit four set and the argument object's target at `+0x20` exists,
/// two exact f32 fields are converted to doubles, passed to a scripted helper,
/// and the resulting f32 is sent to two task methods. The resource pointer at
/// `+0x14` is then released and cleared. The function is thiscall with one
/// stack pointer argument and returns its boolean in AL.
lf_checker_rt::export!(thiscall, rw_00d4fb70(this: u32, argument: u32) -> u8 {
    const COMPLETE: u32 = 0x2c;
    const RESOURCE: u32 = 0x14;
    const RESOURCE_ARG: u32 = 0x18;
    const TASK_LINK: u32 = 0x1c;
    const SUBOBJECT: u32 = 0x20;
    const PED_TARGET: u32 = 0x20;
    const PED_FALLBACK: u32 = 0x1c;
    const TASK_FLAGS: u32 = 0x74;
    const TARGET_X: u32 = 0x10;
    const TARGET_Y: u32 = 0x14;
    const RESOURCE_SLOT: u32 = 1;
    const SECONDARY_HANDLER: u32 = 2;
    const DOUBLE_HELPER: u32 = 3;
    const UPDATE_PRIMARY: u32 = 4;
    const UPDATE_SECONDARY: u32 = 5;
    const UPDATE_BLEND: u32 = 6;
    const RELEASE_RESOURCE: u32 = 7;
    const SIGN_MASK_GLOBAL: u32 = 0x00fe_8fa0;

    let complete = unsafe { ((this + COMPLETE) as *const u8).read() };
    if complete != 0 {
        return 1;
    }

    let resource_argument = unsafe { ((this + RESOURCE_ARG) as *const u32).read_unaligned() };
    let _: u32 = lf_checker_rt::callee_thiscall!(
        RESOURCE_SLOT, u32, this.wrapping_add(SUBOBJECT), resource_argument
    );

    let task_link = unsafe { ((this + TASK_LINK) as *const u32).read_unaligned() };
    if task_link == 0 {
        let _: u8 = lf_checker_rt::callee_thiscall!(SECONDARY_HANDLER, u8, this, argument);
        return 0;
    }

    let resource_slot = this + RESOURCE;
    let resource = unsafe { (resource_slot as *const u32).read_unaligned() };
    if resource == 0 {
        return 0;
    }
    let task_flags = unsafe { ((task_link + TASK_FLAGS) as *const u32).read_unaligned() };
    if task_flags & 0x10 == 0 {
        return 0;
    }

    let target = unsafe { ((argument + PED_TARGET) as *const u32).read_unaligned() };
    let force_bits = if target != 0 {
        let target_x = unsafe { ((target + TARGET_X) as *const u32).read_unaligned() };
        let target_y = unsafe { ((target + TARGET_Y) as *const u32).read_unaligned() };
        let sign_mask = unsafe {
            lf_checker_rt::global::<[u32; 4]>(SIGN_MASK_GLOBAL).read_unaligned()
        };
        let signed_x = f32::from_bits(target_x ^ sign_mask[0]);
        let y_value = f32::from_bits(target_y);
        let x_as_double = core::hint::black_box(f64::from(signed_x)).to_bits();
        let y_as_double = core::hint::black_box(f64::from(y_value)).to_bits();
        let combined = lf_checker_rt::callee_cdecl!(
            DOUBLE_HELPER,
            u64,
            x_as_double as u32,
            (x_as_double >> 32) as u32,
            y_as_double as u32,
            (y_as_double >> 32) as u32
        );
        let rounded = core::hint::black_box(f64::from_bits(combined)) as f32;
        rounded.to_bits()
    } else {
        unsafe { ((argument + PED_FALLBACK) as *const u32).read_unaligned() }
    };

    let _: u32 = lf_checker_rt::callee_thiscall!(UPDATE_PRIMARY, u32, argument, force_bits);
    let _: u32 = lf_checker_rt::callee_thiscall!(UPDATE_SECONDARY, u32, argument, force_bits);
    let _: u32 = lf_checker_rt::callee_thiscall!(
        UPDATE_BLEND, u32, argument.wrapping_add(0x2b0), argument, 1
    );
    let _: u32 = lf_checker_rt::callee_thiscall!(
        RELEASE_RESOURCE, u32, resource, resource_slot
    );
    unsafe { (resource_slot as *mut u32).write_unaligned(0) };
    0
});
