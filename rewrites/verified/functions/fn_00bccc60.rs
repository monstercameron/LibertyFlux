// original: 0x00BCCC60 physics_check_status_flags
/// Return one when a resolved physics object's 32-bit status at byte offset
/// 0x1304 equals 4 and any enabled helper accepts it. The cdecl inputs are a
/// context handle and three flag words; only each flag's low byte is tested.
/// The manager resolver returns the object. For a matching status, the
/// function checks the three flags in order, calls the corresponding
/// thiscall helper only for a nonzero low byte, and returns immediately when
/// that helper's low result byte is nonzero. Otherwise it returns zero. The
/// proof compares AL, scripts edge returns with both zero and nonzero low
/// bytes, and pins status values around the equality boundary.
lf_checker_rt::export!(
    cdecl,
    rw_00BCCC60(
        context_handle: u32,
        first_flag: u32,
        second_flag: u32,
        third_flag: u32,
    ) -> u32 {
        const PHYSICS_MANAGER_SLOT: u32 = 0x012e_22a4;
        const STATUS_FIELD_OFFSET: u32 = 0x1304;
        const FLAG_MASK: u32 = 0xff;
        let manager = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<u32>(PHYSICS_MANAGER_SLOT))
        };
        let object = lf_checker_rt::callee_thiscall!(1, u32, manager, context_handle);
        let status = unsafe {
            core::ptr::read_unaligned((object + STATUS_FIELD_OFFSET) as *const u32)
        };
        if status != 4 {
            return 0;
        }
        if first_flag & FLAG_MASK != 0 {
            let accepted = lf_checker_rt::callee_thiscall!(2, u32, object);
            if accepted & FLAG_MASK != 0 {
                return 1;
            }
        }
        if second_flag & FLAG_MASK != 0 {
            let accepted = lf_checker_rt::callee_thiscall!(3, u32, object);
            if accepted & FLAG_MASK != 0 {
                return 1;
            }
        }
        if third_flag & FLAG_MASK != 0 {
            let accepted = lf_checker_rt::callee_thiscall!(4, u32, object);
            if accepted & FLAG_MASK != 0 {
                return 1;
            }
        }
        0
    }
);
