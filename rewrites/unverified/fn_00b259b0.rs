// original: 0x00B259B0 CHeli::vf44

/// Runs an entity's optional post-physics work. The low nibble of the byte at
/// `+0x1e2` gates an entity vtable call at `+0x114`. A physics record pointer
/// at `+0x38` gates record tests, record vtable queries at `+0x04`, an optional
/// indexed service call, and a final entity check with optional cleanup. The
/// category comparison against thirteen is signed; the record index is an
/// unsigned 16-bit value and `0xffff` suppresses its service call.
lf_checker_rt::export!(thiscall, rw_b259b0(this: u32) -> () {
    const ENTITY_VTABLE_SLOT: u32 = 0x114;
    const ENTITY_KIND: u32 = 0x1E2;
    const PHYSICS_RECORD: u32 = 0x38;
    const RECORD_FLAGS: u32 = 0x0A;
    const RECORD_INDEX: u32 = 0x08;
    const RECORD_VTABLE_SLOT: u32 = 0x04;
    const SERVICE_GLOBAL_RVA: u32 = 0x0EB9_C7C;

    const ENTITY_QUERY: u32 = 1;
    const RECORD_CATEGORY: u32 = 2;
    const PRE_DISPATCH: u32 = 3;
    const INDEXED_SERVICE: u32 = 4;
    const FINAL_CHECK: u32 = 5;
    const CLEANUP: u32 = 6;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let kind = read_u8(this.wrapping_add(ENTITY_KIND)) & 0x0F;
        let record = read_u32(this.wrapping_add(PHYSICS_RECORD));

        if kind >= 2 {
            let vtable = read_u32(this);
            let _target = read_u32(vtable.wrapping_add(ENTITY_VTABLE_SLOT));
            let _ = lf_checker_rt::callee_thiscall!(ENTITY_QUERY, u32, this, 0);
        }

        if record != 0 {
            if read_u16(record.wrapping_add(RECORD_FLAGS)) & 0x2000 != 0 {
                let _ = lf_checker_rt::callee_thiscall!(PRE_DISPATCH, u32, this, record);
            }

            let record_vtable = read_u32(record);
            let _target = read_u32(record_vtable.wrapping_add(RECORD_VTABLE_SLOT));
            let mut category = lf_checker_rt::callee_thiscall!(RECORD_CATEGORY, u32, record);
            if category != 3 {
                let _target = read_u32(record_vtable.wrapping_add(RECORD_VTABLE_SLOT));
                category = lf_checker_rt::callee_thiscall!(RECORD_CATEGORY, u32, record);
                if (category as i32) < 13 {
                    let index = read_u16(record.wrapping_add(RECORD_INDEX));
                    if index != u16::MAX {
                        let service = lf_checker_rt::global::<u32>(SERVICE_GLOBAL_RVA).read_unaligned();
                        let _ = lf_checker_rt::callee_thiscall!(INDEXED_SERVICE, u32, service, index as u32, 0);
                    }
                }
            }
        }

        if record != 0 && (lf_checker_rt::callee_thiscall!(FINAL_CHECK, u32, this) as u8) != 0 {
            let _ = lf_checker_rt::callee_cdecl!(CLEANUP, u32, this);
        }
    }
});
