// original: 0x00A33C50 CEntity::vf8

/// Detach the entity's pool row when both its 32-bit handle and signed-byte
/// kind pass their `-1` sentinel checks. The pool pointer is read from the
/// global slot at virtual address `0x012fb214`. The pool lookup receives the
/// handle; a non-null row is then passed with this entity to the cleanup
/// helper. Both helpers use thiscall and the entity layout uses 32-bit
/// pointers.
lf_checker_rt::export!(thiscall, rw_00a33c50(this: u32) -> () {
    const ROW_KIND: u32 = 0x40;
    const ROW_HANDLE: u32 = 0x48;
    const INVALID_HANDLE: i32 = -1;
    const INVALID_KIND: i8 = -1;
    const POOL_POINTER_VA: u32 = 0x012f_b214;
    const POOL_LOOKUP: u32 = 1;
    const DETACH_ROW: u32 = 2;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let handle = read_u32(this.wrapping_add(ROW_HANDLE));
        if (handle as i32) <= INVALID_HANDLE {
            return;
        }

        let kind = read_u8(this.wrapping_add(ROW_KIND));
        if (kind as i8) <= INVALID_KIND {
            return;
        }

        let pool_pointer = lf_checker_rt::relocated(POOL_POINTER_VA);
        let pool = read_u32(pool_pointer);
        let row = lf_checker_rt::callee_thiscall!(POOL_LOOKUP, u32, pool, handle);
        if row != 0 {
            let _ = lf_checker_rt::callee_thiscall!(DETACH_ROW, u32, row, this);
        }
    }
});
