// original: 0x00a71300 CTaskComplexPlayerOnFoot::vf18
/// Select the next player task from its event type. The first task interface
/// returns an unsigned task id; the gates against 0x13c and 0x11d are signed,
/// while the nested 0xd3..=0x1b1 dispatch range is unsigned. The selected
/// factory receives the owner, event record and task id. Event 6 also looks
/// up a secondary record and marks its id word when its interface reports 2.
/// Event 0x207 sets the owner's mode word. Event 0x416 updates the action byte
/// and checks an entity predicate before choosing the 0xd3 task. All exits
/// copy the relocated world-mode global into the last-mode global and return
/// the selected task pointer (or zero). This is a thiscall with one stack
/// argument; indirect and direct calls are replayed through the checker.

const TASK_EVENT: u32 = 0x08;
const TASK_SUBOBJECT: u32 = 0x18;
const TASK_RECORD_ID: u32 = 0x20;
const RECORD_OBJECT: u32 = 0x224;
const RECORD_TASK_ID: u32 = 0xb30;
const RECORD_FLAGS: u32 = 0x26c;
const PLAYER_ACTION: u32 = 0x14;
const PLAYER_MODE: u32 = 0x50;
const PLAYER_VALUE: u32 = 0x7c;
const NEW_TASK_RECORD: u16 = 0x0100;
const WORLD_MODE: u32 = 0x011735b4;
const LAST_WORLD_MODE: u32 = 0x012fa6b8;

const C_EVENT_TYPE: u32 = 1;
const C_SUBTYPE: u32 = 2;
const C_CREATED_TYPE: u32 = 3;
const C_LOOKUP: u32 = 4;
const C_SELECT: u32 = 5;
const C_PREPARE: u32 = 6;
const C_GET_ENTITY: u32 = 7;
const C_IS_READY: u32 = 8;
const C_BUILD_TASK: u32 = 9;

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

unsafe fn player_on_foot_event(this: u32, event_record: u32, mutant: bool) -> u32 {
    unsafe {
        let event_task = read_u32(this + TASK_EVENT);
        let event_id = lf_checker_rt::callee_thiscall!(C_EVENT_TYPE, u32, event_task);
        let mut selected = 0u32;

        if (event_id as i32) > 0x13c {
            if (event_id as i32) > 0x207 {
                if event_id == 0x2de {
                    if read_u32(event_record + RECORD_TASK_ID) == 0
                        || read_u8(event_record + RECORD_FLAGS) & 4 == 0
                    {
                        selected = lf_checker_rt::callee_thiscall!(
                            C_BUILD_TASK,
                            u32,
                            this,
                            event_record,
                            8
                        );
                    }
                } else if event_id == 0x416 {
                    write_u8(this + PLAYER_ACTION, 1);
                    selected = lf_checker_rt::callee_thiscall!(
                        C_PREPARE,
                        u32,
                        this,
                        event_record,
                        0,
                        1,
                        0
                    );
                    if selected == 0 {
                        let entity = lf_checker_rt::callee_thiscall!(
                            C_GET_ENTITY,
                            u32,
                            event_record
                        );
                        if read_u8(this + PLAYER_MODE) == 0 && entity != 0 {
                            let ready = lf_checker_rt::callee_thiscall!(
                                C_IS_READY,
                                u32,
                                entity.wrapping_add(0x26b8)
                            );
                            let task_id = if ready as u8 != 0 { 0xd3 } else { 8 };
                            selected = lf_checker_rt::callee_thiscall!(
                                C_BUILD_TASK,
                                u32,
                                this,
                                event_record,
                                task_id
                            );
                        } else if read_u8(this + PLAYER_MODE) == 0 && entity == 0 {
                            selected = lf_checker_rt::callee_thiscall!(
                                C_BUILD_TASK,
                                u32,
                                this,
                                event_record,
                                8
                            );
                        }
                    }
                } else if event_id == 0x207 {
                    if !mutant {
                        write_u32(this + PLAYER_VALUE, 0x3f80_0000);
                    }
                    selected = lf_checker_rt::callee_thiscall!(
                        C_BUILD_TASK,
                        u32,
                        this,
                        event_record,
                        8
                    );
                } else if event_id == 0x1af || event_id == 0x1b1 {
                    selected = lf_checker_rt::callee_thiscall!(
                        C_BUILD_TASK,
                        u32,
                        this,
                        event_record,
                        8
                    );
                }
            } else if event_id == 0x207 {
                if !mutant {
                    write_u32(this + PLAYER_VALUE, 0x3f80_0000);
                }
                selected = lf_checker_rt::callee_thiscall!(
                    C_BUILD_TASK,
                    u32,
                    this,
                    event_record,
                    8
                );
            } else if event_id == 0x1af || event_id == 0x1b1 {
                selected = lf_checker_rt::callee_thiscall!(
                    C_BUILD_TASK,
                    u32,
                    this,
                    event_record,
                    8
                );
            }
        } else if event_id == 0x13c
            || event_id == 0x11f
            || (event_id as i32) > 0x138 && (event_id as i32) <= 0x13b
            || event_id == 7
            || event_id == 0x73
        {
            selected = lf_checker_rt::callee_thiscall!(
                C_BUILD_TASK,
                u32,
                this,
                event_record,
                8
            );
        } else if event_id == 0x11d {
            let subtype_task = read_u32(event_task + TASK_SUBOBJECT);
            let subtype = if subtype_task == 0 {
                0xc8
            } else {
                lf_checker_rt::callee_thiscall!(C_SUBTYPE, u32, subtype_task)
            };
            let table_index = subtype.wrapping_sub(0xd3);
            if table_index <= 0xde {
                if subtype == 0xd3 || subtype == 0x13f || subtype == 0x1af || subtype == 0x1b1 {
                    selected = lf_checker_rt::callee_thiscall!(
                        C_BUILD_TASK,
                        u32,
                        this,
                        event_record,
                        8
                    );
                } else if subtype == 0x146 {
                    selected = lf_checker_rt::callee_thiscall!(
                        C_SELECT,
                        u32,
                        this,
                        event_record
                    );
                    if selected == 0 {
                        selected = lf_checker_rt::callee_thiscall!(
                            C_BUILD_TASK,
                            u32,
                            this,
                            event_record,
                            8
                        );
                    }
                }
            }
        } else if event_id == 6 {
            let lookup_owner = read_u32(event_record + RECORD_OBJECT).wrapping_add(0x44);
            let created = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, lookup_owner, 1);
            if created != 0 {
                let created_id = lf_checker_rt::callee_thiscall!(
                    C_CREATED_TYPE,
                    u32,
                    created
                );
                if created_id == 2 {
                    write_u16(created + TASK_RECORD_ID, NEW_TASK_RECORD);
                }
            }
            selected = lf_checker_rt::callee_thiscall!(
                C_BUILD_TASK,
                u32,
                this,
                event_record,
                8
            );
        }

        let world_mode = lf_checker_rt::global::<u32>(WORLD_MODE).read_unaligned();
        lf_checker_rt::global::<u32>(LAST_WORLD_MODE).write_unaligned(world_mode);
        selected
    }
}

#[inline(always)]
unsafe fn write_u8(address: u32, value: u8) {
    unsafe { (address as *mut u8).write(value) }
}

lf_checker_rt::export!(thiscall, rw_00a71300(this: u32, event_record: u32) -> u32 {
    unsafe { player_on_foot_event(this, event_record, false) }
});
