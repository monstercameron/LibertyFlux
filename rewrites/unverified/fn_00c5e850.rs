// original: 0x00C5E850 CTaskComplexGangDriveby::vf18

/// Selects the next child for a gang-driveby task after the character is
/// eligible. The driveby seat helper supplies a nine-word result record;
/// only its first and third words are forwarded when the character state is
/// eight. Other eligible states create the gang-driveby task using the
/// current task's member fields. The child type check, optional character
/// gate, allocator and constructors retain their original call order.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_00c5e850(this: u32, ped: u32) -> u32 {
    unsafe {
        const CHILD_OFFSET: u32 = 0x08;
        const PED_VEHICLE_OFFSET: u32 = 0x0b30;
        const PED_ACTION_FLAGS_OFFSET: u32 = 0x026c;
        const PED_STATE_OFFSET: u32 = 0x02b0;
        const PED_GATE_A_OFFSET: u32 = 0x0218;
        const PED_GATE_B_OFFSET: u32 = 0x0219;
        const CHILD_TYPE_SLOT: u32 = 0x0c;
        const TASK_TYPE_DRIVEBY: u32 = 0x346;
        const SPECIAL_STATE: u32 = 8;
        const OPTIONAL_MEMBER_FLAG: u32 = 0x30;
        const OPTIONAL_MEMBER: u32 = 0x20;
        const CONSTRUCTOR_ARG_A: u32 = 0x14;
        const CONSTRUCTOR_FLOAT_BITS: u32 = 0x34;
        const CONSTRUCTOR_ARG_D: u32 = 0x38;
        const GLOBAL_MANAGER: u32 = 0x0167e2a0;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        if read_u32(ped.wrapping_add(PED_VEHICLE_OFFSET)) == 0
            || read_u8(ped.wrapping_add(PED_ACTION_FLAGS_OFFSET)) & 4 == 0
        {
            return 0;
        }

        let task_state = read_u32(ped.wrapping_add(PED_STATE_OFFSET));
        let mut seat_result = [0u32; 9];
        let helper_accepts: u8 = lf_checker_rt::callee_cdecl!(
            2,
            u8,
            ped,
            seat_result.as_mut_ptr() as u32,
            task_state
        );

        if read_u8(ped.wrapping_add(PED_GATE_A_OFFSET)) == 0
            && read_u8(ped.wrapping_add(PED_GATE_B_OFFSET)) != 0
        {
            let may_continue: u8 = lf_checker_rt::callee_cdecl!(3, u8, ped);
            if may_continue == 0 {
                return 0;
            }
        }
        if helper_accepts == 0 {
            return 0;
        }

        let child = read_u32(this.wrapping_add(CHILD_OFFSET));
        let child_vtable = read_u32(child);
        type TaskTypeCall = extern "thiscall" fn(u32) -> u32;
        let task_type_call: TaskTypeCall = core::mem::transmute(
            read_u32(child_vtable.wrapping_add(CHILD_TYPE_SLOT)) as usize,
        );
        if task_type_call(child) != TASK_TYPE_DRIVEBY {
            return 0;
        }

        let manager = lf_checker_rt::global::<u32>(GLOBAL_MANAGER).read_unaligned();
        let allocated = lf_checker_rt::callee_thiscall!(4, u32, manager);
        if allocated == 0 {
            return 0;
        }

        if task_state == SPECIAL_STATE {
            return lf_checker_rt::callee_thiscall!(
                5,
                u32,
                allocated,
                seat_result[0],
                seat_result[2]
            );
        }

        let optional_member = if read_u8(this.wrapping_add(OPTIONAL_MEMBER_FLAG)) != 0 {
            this.wrapping_add(OPTIONAL_MEMBER)
        } else {
            0
        };
        lf_checker_rt::callee_thiscall!(
            6,
            u32,
            allocated,
            read_u32(this.wrapping_add(CONSTRUCTOR_ARG_A)),
            optional_member,
            read_u32(this.wrapping_add(CONSTRUCTOR_FLOAT_BITS)),
            read_u32(this.wrapping_add(CONSTRUCTOR_ARG_D))
        )
    }
}
