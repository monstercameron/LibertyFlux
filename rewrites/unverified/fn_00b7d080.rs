// original: 0x00B7D080 CTaskComplexCarDriveMissionFleeScene::vf18

/// Select the next child-task request for a car-drive flee-scene task.
///
/// `this` is the task object and `ped` is the actor context passed as the one
/// stack argument. The method asks the current child task for its type through
/// virtual slot `+0x0c`. The type dispatch is an unsigned window beginning at
/// `0x2c5`; only `0x2c5`, `0x2ca`, `0x2cd`, `0x2de`, `0x340`, and `0x396` have
/// special entries. Other types in the window and every type outside it return
/// zero; event `0x396` is the explicit table entry that requests event `0x516`
/// from the task builder.
///
/// The special cases either call virtual slots `+0x4c` or `+0x58`, request a
/// child from the builder, or replace the child pointer at `this + 0x40`.
/// Replacing a non-null child calls the release helper first, stores the new
/// pointer, then calls the retain helper with the address of that slot. The
/// fallback comparison at `this + 0x44` is signed. This is a 32-bit thiscall
/// method with one pointer argument and a 32-bit result.
lf_checker_rt::export!(thiscall, rw_00b7d080(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_POINTER: u32 = 0x08;
        const CURRENT_CHILD: u32 = 0x40;
        const SIGNED_STATE: u32 = 0x44;
        const HAS_EVENT: u32 = 0x48;
        const PED_FLAGS: u32 = 0x26c;
        const PED_CHILD: u32 = 0xb30;
        const FIRST_TYPE: u32 = 0x2c5;
        const LAST_TYPE_OFFSET: u32 = 0xd1;
        const DEFAULT_EVENT: u32 = 0x516;
        const RELEASE_CHILD: u32 = 5;
        const RETAIN_CHILD: u32 = 6;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let child = read_u32(this + TASK_POINTER);
        let task_type: u32 = lf_checker_rt::callee_thiscall!(1, u32, child);
        let type_offset = task_type.wrapping_sub(FIRST_TYPE);
        let build_child = |event: u32| -> u32 {
            lf_checker_rt::callee_thiscall!(4, u32, this, event, ped)
        };

        if type_offset > LAST_TYPE_OFFSET {
            return 0;
        }

        match task_type {
            0x2c5 => lf_checker_rt::callee_thiscall!(2, u32, this, ped),
            0x2ca => {
                if read_u32(this + CURRENT_CHILD) != 0 {
                    build_child(0x2de)
                } else if read_u8(this + HAS_EVENT) != 0 {
                    build_child(0x2cd)
                } else {
                    build_child(DEFAULT_EVENT)
                }
            }
            0x2cd => {
                let flags = read_u8(ped + PED_FLAGS);
                let next_child = read_u32(ped + PED_CHILD);
                if flags & 4 != 0 && next_child != 0 {
                    let slot = this + CURRENT_CHILD;
                    let old_child = read_u32(slot);
                    if old_child != 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            RELEASE_CHILD,
                            u32,
                            old_child,
                            slot
                        );
                    }
                    write_u32(slot, next_child);
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        RETAIN_CHILD,
                        u32,
                        next_child,
                        slot
                    );
                    build_child(0x2c5)
                } else if (read_u32(this + SIGNED_STATE) as i32) >= 0 {
                    build_child(0x340)
                } else {
                    lf_checker_rt::callee_thiscall!(3, u32, this)
                }
            }
            0x2de => {
                if read_u8(ped + PED_FLAGS) & 4 != 0 && read_u32(ped + PED_CHILD) != 0 {
                    build_child(0x2c5)
                } else {
                    build_child(DEFAULT_EVENT)
                }
            }
            0x340 => {
                let flags = read_u8(ped + PED_FLAGS);
                let next_child = read_u32(ped + PED_CHILD);
                if flags & 4 != 0 && next_child != 0 {
                    let slot = this + CURRENT_CHILD;
                    let old_child = read_u32(slot);
                    if old_child != 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            RELEASE_CHILD,
                            u32,
                            old_child,
                            slot
                        );
                    }
                    write_u32(slot, next_child);
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        RETAIN_CHILD,
                        u32,
                        next_child,
                        slot
                    );
                    build_child(0x2c5)
                } else {
                    lf_checker_rt::callee_thiscall!(3, u32, this)
                }
            }
            0x396 => build_child(DEFAULT_EVENT),
            _ => 0,
        }
    }
});
