// original: 0x00be3970 CTaskComplexUseMobilePhoneAndMovement::vf19 (symbols)

/// Build this task's phone-use subtask, directly or as a movement pair.
///
/// `arg + FLAG_OFF` (0x26c) bit `DIRECT_BIT` (4) selects the shape. When set,
/// a pool slot is taken and the phone subtask is constructed there with this
/// task's parameter (`this + PARAM_OFF`, 0x14); the new subtask is returned,
/// or zero when the pool is empty.
///
/// When clear, a coordinator slot is taken first (zero when empty), then a
/// second slot for the phone half (a miss leaves a null half), then a third
/// slot for the movement half, which is base-constructed and stamped with
/// this task's vtable pointer, its parameter-table pointer
/// (`PARAM_TABLE_FILE_VA`) and a zeroed counter (`+0x20`); a miss there
/// leaves a null half. The coordinator is then constructed over
/// (movement-or-null, phone-or-null, 1, 0) and returned.
///
/// Original: 0x00be3970 (thiscall, one stack word: the incoming argument).
lf_checker_rt::export!(thiscall, rw_00be3970(this: u32, arg: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0x26c;
        const DIRECT_BIT: u8 = 4;
        const PARAM_OFF: u32 = 0x14;
        const POOL_GLOBAL: u32 = 0x0167e2a0;
        const VTABLE_FILE_VA: u32 = 0x00e98794;
        const PARAM_TABLE_FILE_VA: u32 = 0x00e987e8;
        const COUNT_OFF: u32 = 0x20;
        const ALLOC_DIRECT: u32 = 1;
        const CONSTRUCT_DIRECT: u32 = 2;
        const ALLOC_COORD: u32 = 3;
        const ALLOC_PHONE: u32 = 4;
        const CONSTRUCT_PHONE: u32 = 5;
        const ALLOC_MOVE: u32 = 6;
        const BASE_CTOR: u32 = 7;
        const CONSTRUCT_COORD: u32 = 8;
        let flags = (arg.wrapping_add(FLAG_OFF) as *const u8).read();
        let pool = (lf_checker_rt::global::<u32>(POOL_GLOBAL) as *const u32).read_unaligned();
        let param = (this.wrapping_add(PARAM_OFF) as *const u32).read_unaligned();
        if flags & DIRECT_BIT != 0 {
            let slot = lf_checker_rt::callee_thiscall!(ALLOC_DIRECT, u32, pool);
            if slot == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(CONSTRUCT_DIRECT, u32, slot, param);
        }
        let coord = lf_checker_rt::callee_thiscall!(ALLOC_COORD, u32, pool);
        if coord == 0 {
            return 0;
        }
        let phone = lf_checker_rt::callee_thiscall!(ALLOC_PHONE, u32, pool);
        let phone = if phone == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(CONSTRUCT_PHONE, u32, phone, param)
        };
        let movement = lf_checker_rt::callee_thiscall!(ALLOC_MOVE, u32, pool);
        let movement = if movement == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, movement, 1);
            (movement as *mut u32)
                .write_unaligned(lf_checker_rt::relocated(VTABLE_FILE_VA));
            (movement.wrapping_add(PARAM_OFF) as *mut u32)
                .write_unaligned(lf_checker_rt::relocated(PARAM_TABLE_FILE_VA));
            (movement.wrapping_add(COUNT_OFF) as *mut u32).write_unaligned(0);
            movement
        };
        lf_checker_rt::callee_thiscall!(CONSTRUCT_COORD, u32, coord, movement, phone, 1, 0)
    }
});
