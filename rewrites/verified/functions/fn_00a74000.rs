// original: 0x00A74000 CTaskSimpleMoveSwim::vf17
/// Advance the simple swimming move task. It marks movement state, probes
/// the active event item, and conditionally requests a replacement task.
/// Event type 0x10c selects the special path; a component type of 2 skips
/// allocation, while other values create and assign a task. The final item
/// is asked for its next task and then activated. The active predicate uses
/// the context's flags, and the range check receives the value word, index
/// 0x3b and the exact single-precision bits for -10.0. This is a thiscall
/// with one context pointer argument; all helper and virtual calls are
/// checked through scripted callees.

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

const SWIM_TASK_OBJECT: u32 = 0x04;
const SWIM_TASK_ACTIVE: u32 = 0x88;
const SWIM_CONTEXT_MODE: u32 = 0x219;
const SWIM_CONTEXT_SLOT: u32 = 0x224;
const SWIM_CONTEXT_ACTIONS: u32 = 0x26c;
const SWIM_CONTEXT_READY: u32 = 0x29c;
const SWIM_CONTEXT_STYLE: u32 = 0x2a0;
const SWIM_CONTEXT_VALUE: u32 = 0x78;
const SWIM_POOL_OWNER: u32 = 0xa80;
const SWIM_POOL_FLAGS: u32 = 0x50;
const SWIM_COMPONENT_VTABLE_TYPE: u32 = 0x0c;
const SWIM_COMPONENT_NEXT: u32 = 0x30;
const SWIM_COMPONENT_SET: u32 = 0x10;
const SWIM_POOL_POINTER: u32 = 0x0167e2a0;

const W_IS_ACTIVE: u32 = 1;
const W_CHECK_RANGE: u32 = 2;
const W_START_TASK: u32 = 3;
const W_GET_EVENT_ITEM: u32 = 4;
const W_CONFIGURE_EVENT: u32 = 5;
const W_CHECK_TASK_ITEM: u32 = 6;
const W_GET_TASK_ITEM: u32 = 7;
const W_ALLOC_TASK: u32 = 8;
const W_INIT_TASK: u32 = 9;
const W_ASSIGN_TASK: u32 = 10;
const W_GET_FINAL_ITEM: u32 = 11;
const W_FINISH: u32 = 12;
const W_EVENT_TYPE: u32 = 13;
const W_TASK_TYPE: u32 = 14;
const W_NEXT_TASK: u32 = 15;
const W_SET_NEXT_TASK: u32 = 16;

/// Advance the simple swimming move task. It marks movement state, probes
/// the active event item, and conditionally requests a replacement task.
/// Event type 0x10c selects the special path; a component type of 2 skips
/// allocation, while other values create and assign a task. The final item
/// is asked for its next task and then activated. The active predicate uses
/// the context's flags, and the range check receives the value word, index
/// 0x3b and the exact single-precision bits for -10.0. This is a thiscall
/// with one context pointer argument; all helper and virtual calls are
/// checked through scripted callees.
unsafe fn move_swim(this: u32, context: u32, mutant: bool) -> u8 {
    unsafe {
        write_u32(
            context + SWIM_CONTEXT_STYLE,
            read_u32(context + SWIM_CONTEXT_STYLE) | 0x800,
        );

        let active = lf_checker_rt::callee_thiscall!(W_IS_ACTIVE, u8, this, context);
        if active != 0 {
            let _ = lf_checker_rt::callee_cdecl!(
                W_CHECK_RANGE,
                u8,
                read_u32(context + SWIM_CONTEXT_VALUE),
                0x3b,
                0xc1200000
            );
            return 1;
        }

        lf_checker_rt::callee_thiscall!(W_START_TASK, u32, this, context);
        write_u32(
            context + SWIM_CONTEXT_ACTIONS,
            read_u32(context + SWIM_CONTEXT_ACTIONS) & !0x2000,
        );
        write_u32(
            context + 0x118,
            read_u32(context + 0x118) | 1,
        );
        let component_record = read_u32(context + SWIM_CONTEXT_SLOT);
        write_u8(component_record + 0x246, 2);

        let mut build_default = false;
        let mut special_event = false;
        if read_u8(context + SWIM_CONTEXT_MODE) != 0 {
            let event_owner = read_u32(this + SWIM_TASK_OBJECT);
            if event_owner != 0 {
                let event_type = lf_checker_rt::callee_thiscall!(W_EVENT_TYPE, u32, event_owner);
                if event_type == 0x10c {
                    build_default = true;
                    special_event = true;
                    let pool_record = read_u32(context + SWIM_POOL_OWNER);
                    write_u32(
                        pool_record + SWIM_POOL_FLAGS,
                        read_u32(pool_record + SWIM_POOL_FLAGS) | 0x2000,
                    );
                    let event_item = lf_checker_rt::callee_thiscall!(
                        W_GET_EVENT_ITEM,
                        u32,
                        component_record.wrapping_add(0x44),
                        1
                    );
                    if read_u8(this + SWIM_TASK_ACTIVE) != 0 {
                        if event_item != 0 {
                            lf_checker_rt::callee_thiscall!(
                                W_CONFIGURE_EVENT,
                                u32,
                                event_item,
                                context,
                                2,
                                0
                            );
                        }
                    } else {
                        let first_item = lf_checker_rt::callee_thiscall!(
                            W_CHECK_TASK_ITEM,
                            u32,
                            component_record.wrapping_add(0x44),
                            1
                        );
                        if first_item != 0 {
                            let task_item = lf_checker_rt::callee_thiscall!(
                                W_GET_TASK_ITEM,
                                u32,
                                component_record.wrapping_add(0x44),
                                1
                            );
                            let task_type = lf_checker_rt::callee_thiscall!(
                                W_TASK_TYPE,
                                u32,
                                task_item
                            );
                            if task_type == 2 {
                                build_default = false;
                            }
                        }
                    }
                }
            }
        }

        if special_event {
            if build_default {
                let pool = lf_checker_rt::global::<u32>(SWIM_POOL_POINTER).read_unaligned();
                let allocated = lf_checker_rt::callee_thiscall!(W_ALLOC_TASK, u32, pool);
                let initialized = if allocated != 0 {
                    lf_checker_rt::callee_thiscall!(W_INIT_TASK, u32, allocated)
                } else {
                    0
                };
                lf_checker_rt::callee_thiscall!(
                    W_ASSIGN_TASK,
                    u32,
                    component_record.wrapping_add(0x44),
                    initialized,
                    1
                );
            }

            let final_item = lf_checker_rt::callee_thiscall!(
                W_GET_FINAL_ITEM,
                u32,
                component_record.wrapping_add(0x44),
                1
            );
            let next_item = lf_checker_rt::callee_thiscall!(W_NEXT_TASK, u32, final_item);
            lf_checker_rt::callee_thiscall!(W_SET_NEXT_TASK, u32, next_item, 1);
        }
        lf_checker_rt::callee_thiscall!(W_FINISH, u32, this, context);
        write_u8(this + SWIM_TASK_ACTIVE, 0);
        if !mutant {
            write_u32(
                context + SWIM_CONTEXT_READY,
                read_u32(context + SWIM_CONTEXT_READY) | 0x2000,
            );
        }
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00a74000(this: u32, context: u32) -> u8 {
    unsafe { move_swim(this, context, false) }
});

