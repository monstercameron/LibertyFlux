// original: 0x00A73ED0 CTaskSimpleMovePlayer::vf17
/// Run the simple player move task. A clear context state returns success
/// immediately; otherwise the task updates its movement bytes, marks the
/// referenced pool record, and selects one of two movement helpers. Mode
/// globals and masked entity bytes then determine whether to notify the
/// movement system and set bit 1 of the context output flags. Byte XOR tests
/// compare as unsigned values against 0x7f. This is a thiscall with one
/// context pointer argument; helper calls are replayed by the checker.

#[inline(always)]
unsafe fn read_u8(address: u32) -> u8 {
    unsafe { (address as *const u8).read() }
}

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn write_u16(address: u32, value: u16) {
    unsafe { (address as *mut u16).write_unaligned(value) }
}

#[inline(always)]
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn write_u8(address: u32, value: u8) {
    unsafe { (address as *mut u8).write(value) }
}

const MOVE_PHASE: u32 = 0x20;
const MOVE_PENDING: u32 = 0x21;
const MOVE_CONTEXT_STATE: u32 = 0x219;
const MOVE_POOL_OWNER: u32 = 0xa80;
const MOVE_POOL_FLAGS: u32 = 0x50;
const MOVE_OUTPUT_FLAGS: u32 = 0xbe0;
const MOVE_OBJECT_STATE: u32 = 0x328d;
const MOVE_OBJECT_PRIMARY: u32 = 0x27dc;
const MOVE_OBJECT_SECONDARY: u32 = 0x27de;
const MOVE_OBJECT_TERTIARY: u32 = 0x27df;
const MOVE_PED_MODE: u32 = 0x1e2;
const MOVE_PED_TYPE: u32 = 0x24;
const MOVE_MODE_GLOBAL: u32 = 0x011d6fd4;
const MOVE_OPTION_GLOBAL: u32 = 0x01160ec0;

const M_ENTITY: u32 = 1;
const M_MOVE_WITH_ACTION: u32 = 2;
const M_MOVE_WITHOUT_ACTION: u32 = 3;
const M_STATUS_FIRST: u32 = 4;
const M_STATUS_SECOND: u32 = 5;
const M_STATUS_THIRD: u32 = 6;
const M_NOTIFY_FIRST: u32 = 7;
const M_SHOULD_NOTIFY: u32 = 8;
const M_NOTIFY_SECOND: u32 = 9;

/// Run the simple player move task. An active context returns success
/// immediately; otherwise the task updates its movement bytes, marks the
/// referenced pool record, and selects one of two movement helpers. Mode
/// globals and masked entity bytes then determine whether to notify the
/// movement system and set bit 1 of the context output flags. Byte XOR tests
/// compare as unsigned values against 0x7f. This is a thiscall with one
/// context pointer argument; helper calls are replayed by the checker.
unsafe fn move_player(this: u32, context: u32, mutant: bool) -> u8 {
    unsafe {
        if read_u8(context + MOVE_CONTEXT_STATE) == 0 {
            return 1;
        }

        let entity = lf_checker_rt::callee_thiscall!(M_ENTITY, u32, context);
        let pool_record = read_u32(context + MOVE_POOL_OWNER);
        write_u32(
            pool_record + MOVE_POOL_FLAGS,
            read_u32(pool_record + MOVE_POOL_FLAGS) | 0x2000,
        );

        if read_u8(this + MOVE_PENDING) == 0 {
            write_u8(this + MOVE_PHASE, 0);
        }
        let movement_active = read_u8(this + MOVE_PHASE) != 0;
        write_u8(this + MOVE_PENDING, 0);
        if movement_active {
            lf_checker_rt::callee_thiscall!(M_MOVE_WITH_ACTION, u32, this, context);
        } else {
            lf_checker_rt::callee_thiscall!(M_MOVE_WITHOUT_ACTION, u32, this, context);
        }

        let mode = lf_checker_rt::global::<u32>(MOVE_MODE_GLOBAL).read_unaligned();
        let option = lf_checker_rt::global::<u32>(MOVE_OPTION_GLOBAL).read_unaligned();
        let skip_object_checks = mode == 2
            && (read_u8(context + MOVE_PED_MODE) & 0x0f) >= 2
            && ((read_u32(context + MOVE_PED_TYPE) >> 5) & 1) == 0;

        if !skip_object_checks {
            if read_u8(entity + MOVE_OBJECT_STATE) != 0 && option != 0 {
                let first_status = lf_checker_rt::callee_thiscall!(M_STATUS_FIRST, u8, context);
                if first_status == 0 {
                    let axis_delta = read_u8(entity + MOVE_OBJECT_SECONDARY)
                        ^ read_u8(entity + MOVE_OBJECT_PRIMARY);
                    if axis_delta > 0x7f {
                        lf_checker_rt::callee_thiscall!(
                            M_NOTIFY_SECOND,
                            u32,
                            context,
                            1,
                            u32::MAX
                        );
                    } else {
                        notify_when_axis_is_close(context, entity, M_NOTIFY_SECOND, mutant);
                        return 0;
                    }
                } else {
                    notify_when_axis_is_close(context, entity, M_NOTIFY_SECOND, mutant);
                    return 0;
                }
            } else {
                let primary_delta = read_u8(entity + MOVE_OBJECT_SECONDARY)
                    ^ read_u8(entity + MOVE_OBJECT_PRIMARY);
                let tertiary_delta = read_u8(entity + MOVE_OBJECT_TERTIARY)
                    ^ read_u8(entity + MOVE_OBJECT_PRIMARY);
                if primary_delta > 0x7f && tertiary_delta <= 0x7f {
                    let status = lf_checker_rt::callee_thiscall!(M_STATUS_THIRD, u8, context);
                    if status != 0 {
                        lf_checker_rt::callee_thiscall!(
                            M_NOTIFY_FIRST,
                            u32,
                            context,
                            0,
                            u32::MAX
                        );
                    } else {
                        let should_notify =
                            lf_checker_rt::callee_thiscall!(M_SHOULD_NOTIFY, u8, context);
                        if should_notify != 0 {
                            lf_checker_rt::callee_thiscall!(
                                M_NOTIFY_SECOND,
                                u32,
                                context,
                                1,
                                u32::MAX
                            );
                        }
                    }
                }
            }
        }

        if !mutant {
            write_u32(
                context + MOVE_OUTPUT_FLAGS,
                read_u32(context + MOVE_OUTPUT_FLAGS) | 2,
            );
        }
        0
    }
}

#[inline(always)]
unsafe fn notify_when_axis_is_close(context: u32, entity: u32, notify_id: u32, mutant: bool) {
    unsafe {
        let second_status = lf_checker_rt::callee_thiscall!(M_STATUS_SECOND, u8, context);
        if second_status != 0 {
            let axis_delta = read_u8(entity + MOVE_OBJECT_SECONDARY)
                ^ read_u8(entity + MOVE_OBJECT_PRIMARY);
            if axis_delta <= 0x7f {
                lf_checker_rt::callee_thiscall!(notify_id, u32, context, 0, u32::MAX);
            }
        }
        if !mutant {
            write_u32(
                context + MOVE_OUTPUT_FLAGS,
                read_u32(context + MOVE_OUTPUT_FLAGS) | 2,
            );
        }
    }
}

lf_checker_rt::export!(thiscall, rw_00a73ed0(this: u32, context: u32) -> u8 {
    unsafe { move_player(this, context, false) }
});

