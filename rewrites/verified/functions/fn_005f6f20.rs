// original: 0x005F6F20 rage::fragInst::vf48

/// Check a fragment record's table slot and, when its nested record is active,
/// call the two update helpers in order.
///
/// The record index is an unsigned 16-bit word at `+0x08`; `0xffff` is a
/// sentinel. The global manager supplies an unsigned 16-bit limit at `+0x26`
/// and an eight-byte table at `+0x70`. Since both compared values are
/// zero-extended 16-bit words, the original signed 32-bit bound branches have
/// the same result as this unsigned comparison, and its negative-index branch
/// is unreachable. The instance resource is at `+0x64`; its nested record is
/// at resource offset `+0x160`, and its enabled flag is at `+0x174`. If that
/// flag is nonzero, the indexed entry's byte at `+4` must have either low bit
/// set. The method then calls the nested update helper with fields at `+0x08`
/// and `+0x14`, followed by the instance helper with the nested value, zero,
/// and a word read through the nested record's pointer at `+0x04`. This
/// thiscall method returns no value and has no stack arguments.
lf_checker_rt::export!(thiscall, rw_005f6f20(this: u32) -> () {
    const RECORD_INDEX: u32 = 0x08;
    const RESOURCE_POINTER: u32 = 0x64;
    const RESOURCE_SUBOBJECT: u32 = 0x100;
    const NESTED_RECORD: u32 = 0x60;
    const RESOURCE_ENABLED: u32 = 0x74;
    const NESTED_NEXT: u32 = 0x04;
    const NESTED_FIRST: u32 = 0x08;
    const NESTED_VALUE: u32 = 0x14;
    const TABLE_LIMIT: u32 = 0x26;
    const TABLE_POINTER: u32 = 0x70;
    const TABLE_FLAGS: u32 = 0x04;
    const MANAGER_GLOBAL: u32 = 0x018B_896C;

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
        let index = u32::from(read_u16(this.wrapping_add(RECORD_INDEX)));
        if index == u32::from(u16::MAX) {
            return;
        }

        let manager = core::ptr::read_unaligned(lf_checker_rt::global::<u32>(MANAGER_GLOBAL));
        let limit = u32::from(read_u16(manager.wrapping_add(TABLE_LIMIT)));
        if index >= limit {
            return;
        }

        let resource = read_u32(this.wrapping_add(RESOURCE_POINTER));
        if resource == 0 {
            return;
        }
        let subobject = resource.wrapping_add(RESOURCE_SUBOBJECT);
        let nested = read_u32(subobject.wrapping_add(NESTED_RECORD));
        if read_u32(subobject.wrapping_add(RESOURCE_ENABLED)) != 0 {
            let table = read_u32(manager.wrapping_add(TABLE_POINTER));
            let flags = read_u8(
                table
                    .wrapping_add(index.wrapping_mul(8))
                    .wrapping_add(TABLE_FLAGS),
            );
            if flags & 3 == 0 {
                return;
            }
        }
        if nested == 0 {
            return;
        }

        let nested_value = read_u32(nested.wrapping_add(NESTED_VALUE));
        let nested_next = read_u32(nested.wrapping_add(NESTED_NEXT));
        let _ = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            nested,
            read_u32(nested.wrapping_add(NESTED_FIRST)),
            nested_value,
        );
        let final_value = u32::from(read_u16(nested_next.wrapping_add(NESTED_VALUE)));
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, nested_value, 0, final_value);
    }
});
