// original: 0x00a09e20 mission_cleanup_release_flagged
/// Release flagged mission-cleanup records for the seven supported entry
/// kinds. The manager stores 0x2c-byte entries at +4; a nonzero kind is
/// processed only when the word at +0x0c is exactly one. Kinds 1, 2, and 3
/// resolve their handle through the corresponding entity, pedestrian, or
/// object pool and dispatch only for a live object. Kind 4 resolves the
/// handle with mode eight and dispatches only for a nonnegative result. Kinds
/// 5 through 7 have no kind-specific dispatch. Every selected record is
/// reset with (handle, kind, 0, 1), even when lookup returns null. The manager
/// is armed by a no-argument thiscall helper; a false result leaves it alone.
/// The method uses thiscall with no stack arguments and returns no value.
lf_checker_rt::export!(thiscall, rw_00a09e20(this: u32) -> () {
    unsafe {
        const ENTRY_BASE: u32 = 4;
        const ENTRY_STRIDE: u32 = 0x2c;
        const ENTRY_COUNT: u32 = 256;
        const ENTRY_KIND: u32 = 0;
        const ENTRY_HANDLE: u32 = 4;
        const ENTRY_FLAG: u32 = 0x0c;

        const ENTITY_POOL_GLOBAL: u32 = 0x012e22a4;
        const PED_POOL_GLOBAL: u32 = 0x018b6f1c;
        const OBJECT_POOL_GLOBAL: u32 = 0x01632c60;

        const IS_ARMED: u32 = 0;
        const POOL_GET: u32 = 1;
        const DISPATCH_ENTITY: u32 = 2;
        const DISPATCH_PED: u32 = 3;
        const DISPATCH_OBJECT: u32 = 4;
        const FIND_SPECIAL_INDEX: u32 = 5;
        const APPLY_SPECIAL_INDEX: u32 = 6;
        const RESET_ENTRY: u32 = 7;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_global_pointer(file_va: u32) -> u32 {
            let slot = lf_checker_rt::global::<u32>(file_va);
            unsafe { slot.read_unaligned() }
        }

        if lf_checker_rt::callee_thiscall!(IS_ARMED, u8, this) == 0 {
            return;
        }

        for index in 0..ENTRY_COUNT {
            let entry = this
                .wrapping_add(ENTRY_BASE)
                .wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            let kind = read_u8(entry.wrapping_add(ENTRY_KIND));
            if kind == 0 || read_u32(entry.wrapping_add(ENTRY_FLAG)) != 1 {
                continue;
            }

            let handle = read_u32(entry.wrapping_add(ENTRY_HANDLE));
            match kind {
                1 => {
                    let pool = read_global_pointer(ENTITY_POOL_GLOBAL);
                    let object = lf_checker_rt::callee_thiscall!(POOL_GET, u32, pool, handle);
                    if object != 0 {
                        let _ = lf_checker_rt::callee_cdecl!(DISPATCH_ENTITY, u32, object);
                    }
                }
                2 => {
                    let pool = read_global_pointer(PED_POOL_GLOBAL);
                    let object = lf_checker_rt::callee_thiscall!(POOL_GET, u32, pool, handle);
                    if object != 0 {
                        let _ = lf_checker_rt::callee_cdecl!(DISPATCH_PED, u32, object);
                    }
                }
                3 => {
                    let pool = read_global_pointer(OBJECT_POOL_GLOBAL);
                    let object = lf_checker_rt::callee_thiscall!(POOL_GET, u32, pool, handle);
                    if object != 0 {
                        let _ = lf_checker_rt::callee_cdecl!(DISPATCH_OBJECT, u32, object);
                    }
                }
                4 => {
                    let special_index =
                        lf_checker_rt::callee_cdecl!(FIND_SPECIAL_INDEX, i32, handle, 8);
                    if special_index >= 0 {
                        let _ = lf_checker_rt::callee_cdecl!(
                            APPLY_SPECIAL_INDEX,
                            u32,
                            special_index as u32,
                            1
                        );
                    }
                }
                _ => {}
            }

            let _ = lf_checker_rt::callee_thiscall!(
                RESET_ENTRY,
                u32,
                this,
                handle,
                kind as u32,
                0,
                1
            );
        }
    }
});
