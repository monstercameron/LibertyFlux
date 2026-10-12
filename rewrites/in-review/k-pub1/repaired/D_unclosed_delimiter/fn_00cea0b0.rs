// original: 0x00CEA0B0 GENERIC_CURSE

/// Resets the task byte, validates and selects a task style, optionally plays the matching profile, creates and attaches the behavior object, updates timer and orbit state, and animates a matching owner-list node.

unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

unsafe fn read_f32(address: u32) -> f32 {
    f32::from_bits(unsafe { read_u32(address) })
}

unsafe fn advance_generic_curse(this: u32, owner: u32, mutant: bool) {
    const STYLE: u32 = 0x28;
    const TASK_KIND: u32 = 0x2C;
    const STYLE_OVERRIDE: u32 = 0x38;
    const RESET_BYTE: u32 = 0x3D;
    const BEHAVIOUR_OBJECT: u32 = 0x40;
    const TIMER: u32 = 0x4C;
    const FLAG_BYTE: u32 = 0x44;
    const TIMER_ORBIT: u32 = 0x50;
    const OWNER_INFO: u32 = 0x218;
    const OWNER_ORBIT: u32 = 0x219;
    const OWNER_MANAGER: u32 = 0x78;
    const OWNER_LOOKUP: u32 = 0x228;
    const OWNER_LIST: u32 = 0x224;
    const GLOBAL_LOOKUP_INDEX: u32 = 0x01036F14;
    const GLOBAL_LOOKUP_TABLE: u32 = 0x011A8808;
    const CAL_VALIDATE: u32 = 1;
    const CAL_STATE_COUNT: u32 = 2;
    const CAL_PLAY_PROFILE: u32 = 3;
    const CAL_CREATE_BEHAVIOUR: u32 = 4;
    const CAL_ATTACH_BEHAVIOUR: u32 = 5;
    const CAL_READ_TIMER: u32 = 6;
    const CAL_SPECIAL_STATE: u32 = 7;
    const CAL_ANIMATE: u32 = 8;

    (this.wrapping_add(RESET_BYTE) as *mut u8).write(0);
    let initial_kind = read_u32(this.wrapping_add(TASK_KIND));
    if initial_kind != 3 && initial_kind != 4 {
        return;
    }

    let mut style = 0x66u32;
    let mut task_kind = if initial_kind == 4 { 6u32 } else { 5u32 };
    write_u32(this.wrapping_add(STYLE), style);
    write_u32(this.wrapping_add(TASK_KIND), task_kind);
    let requested_style = read_u32(this.wrapping_add(STYLE_OVERRIDE));
    if requested_style != u32::MAX {
        style = requested_style;
        write_u32(this.wrapping_add(STYLE), style);
    }

    let mut valid = lf_checker_rt::callee_cdecl!(CAL_VALIDATE, u32, task_kind, style);
    while valid == 0 {
        if style == 0x65 {
            style = 0x67;
        } else if style == 0x67 {
            style = 0x66;
        } else if style == 0x66 {
            return;
        }
        write_u32(this.wrapping_add(STYLE), style);
        valid = lf_checker_rt::callee_cdecl!(CAL_VALIDATE, u32, task_kind, style);
    }

    let owner_info = (owner.wrapping_add(OWNER_INFO) as *const u8).read();
    let owner_orbit = (owner.wrapping_add(OWNER_ORBIT) as *const u8).read();
    if owner_info == 0 && owner_orbit != 0 {
        let lookup_index = lf_checker_rt::global::<u32>(GLOBAL_LOOKUP_INDEX).read();
        if lookup_index != u32::MAX {
            let lookup_slot = GLOBAL_LOOKUP_TABLE.wrapping_add(lookup_index.wrapping_mul(4));
            let lookup_object = lf_checker_rt::global::<u32>(lookup_slot).read();
            if lookup_object != 0 && read_u32(lookup_object.wrapping_add(0x4C8)) == 0 {
                let lookup_entity = read_u32(owner.wrapping_add(OWNER_LOOKUP));
                let receiver = if lookup_entity == 0 {
                    0
                } else {
                    lookup_entity.wrapping_add(0x70)
                };
                let state_count = lf_checker_rt::callee_thiscall!(
                    CAL_STATE_COUNT,
                    u32,
                    receiver
                );
                let passes_signed_minimum = (state_count as i32) >= 1;
                let passes_tested_minimum = if mutant {
                    state_count >= 1
                } else {
                    passes_signed_minimum
                };
                if passes_tested_minimum {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        CAL_PLAY_PROFILE,
                        u32,
                        owner.wrapping_add(0x570),
                        lf_checker_rt::relocated(0x00EDC07C),
                        0,
                        0,
                        0,
                        0xFFFF_FFFF,
                        0,
                        0,
                        0x3F80_0000,
                        0,
                        0
                    );
                }
            }
        }
    }

    let manager = read_u32(owner.wrapping_add(OWNER_MANAGER));
    let behaviour_object = lf_checker_rt::callee_thiscall!(
        CAL_CREATE_BEHAVIOUR,
        u32,
        manager,
        task_kind,
        style,
        0x4100_0000,
        u32::MAX
    );
    write_u32(this.wrapping_add(BEHAVIOUR_OBJECT), behaviour_object);
    let _: u32 = lf_checker_rt::callee_thiscall!(
        CAL_ATTACH_BEHAVIOUR,
        u32,
        behaviour_object,
        1,
        lf_checker_rt::relocated(0x00CD_BE40),
        this
    );

    if (owner.wrapping_add(OWNER_ORBIT) as *const u8).read() != 0 {
        let timer: f32 = lf_checker_rt::callee_thiscall!(CAL_READ_TIMER, f32, behaviour_object);
        let base = read_f32(lf_checker_rt::relocated(0x00FE8828));
        let adjusted = core::hint::black_box(timer) - core::hint::black_box(base);
        write_u32(this.wrapping_add(TIMER), adjusted.to_bits());

        if style == 0x64 {
            let enabled: u8 = lf_checker_rt::callee_cdecl!(CAL_SPECIAL_STATE, u8,);
            if enabled != 0 {
                write_u32(behaviour_object.wrapping_add(0x54), 0x3F8C_CCCD);
            }
        }
    } else {
        let timer: f32 = lf_checker_rt::callee_thiscall!(CAL_READ_TIMER, f32, behaviour_object);
        let base = read_f32(lf_checker_rt::relocated(0x00FE87E8));
        let adjusted = core::hint::black_box(timer) - core::hint::black_box(base);
        write_u32(this.wrapping_add(TIMER), adjusted.to_bits());
    }

    let list_owner = read_u32(owner.wrapping_add(OWNER_LIST));
    if list_owner != 0 {
        let mut node = read_u32(list_owner.wrapping_add(0x2E0));
        while node != 0 {
            if read_u32(node.wrapping_add(4)) == 0x76C {
                let enabled: u8 = lf_checker_rt::callee_thiscall!(
                    CAL_ANIMATE,
                    u8,
                    behaviour_object,
                    0x4000,
                    this.wrapping_add(0x48),
                    0,
                    0x3F80_0000
                );
                (this.wrapping_add(FLAG_BYTE) as *mut u8).write(enabled);
                break;
            }
            node = read_u32(node.wrapping_add(0x0C));
        }
    }
}

lf_checker_rt::export!(thiscall, rw_00cea0b0(this: u32, owner: u32) -> u32 {
    unsafe { advance_generic_curse(this, owner, false) }
    0
});
