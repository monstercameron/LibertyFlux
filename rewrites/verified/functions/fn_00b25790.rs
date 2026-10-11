// original: 0x00B25790 CPhysical::vf64

/// Queries a physics object three times to derive a byte-sized notification
/// value. A null first result leaves it false; otherwise the second result's
/// field at `+0x18` must be nonzero before the third query. The third result's
/// field at `+0x18` points to a nested field at `+0x18`; a final value of zero
/// or two makes the byte true. It updates entity flags at `+0x118` with the original wrapping
/// expression, then clears bit zero when the optional object at `+0x6c` is
/// null or has a zero byte at `+0x0e`. The notification helper receives only the low
/// byte of the local boolean; the source's upper call-word bytes are stale
/// saved-register data. Finally it dispatches through vtable slot `+0x11c`.
lf_checker_rt::export!(thiscall, rw_b25790(this: u32) -> () {
    const VTABLE_PTR: u32 = 0;
    const OBJECT_LOOKUP_1: u32 = 1;
    const OBJECT_LOOKUP_2: u32 = 2;
    const OBJECT_LOOKUP_3: u32 = 3;
    const NOTIFY_HELPER: u32 = 4;
    const FINAL_DISPATCH: u32 = 5;
    const PHYSICS_FLAGS: u32 = 0x118;
    const OPTIONAL_OBJECT: u32 = 0x6C;
    const OPTIONAL_BYTE: u32 = 0x0E;
    const LINK_FIELD: u32 = 0x18;
    const VTABLE_SLOT: u32 = 0x11C;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let mut notify = 0u32;
        if lf_checker_rt::callee_thiscall!(OBJECT_LOOKUP_1, u32, this) != 0 {
            let second = lf_checker_rt::callee_thiscall!(OBJECT_LOOKUP_2, u32, this);
            if read_u32(second.wrapping_add(LINK_FIELD)) != 0 {
                let third = lf_checker_rt::callee_thiscall!(OBJECT_LOOKUP_3, u32, this);
                let link = read_u32(third.wrapping_add(LINK_FIELD));
                let state = read_u32(link.wrapping_add(LINK_FIELD));
                if state == 0 || state == 2 {
                    notify = 1;
                }
            }
        }

        let flags_address = this.wrapping_add(PHYSICS_FLAGS);
        let old_flags = read_u32(flags_address);
        let doubled_difference = old_flags.wrapping_add(old_flags) ^ old_flags;
        let mut new_flags = (doubled_difference & 2) ^ old_flags;

        let optional = read_u32(this.wrapping_add(OPTIONAL_OBJECT));
        if optional == 0 || read_u8(optional.wrapping_add(OPTIONAL_BYTE)) == 0 {
            new_flags &= !1;
        }
        write_u32(flags_address, new_flags);

        let _ = lf_checker_rt::callee_thiscall!(NOTIFY_HELPER, u32, this, notify);
        let vtable = read_u32(this.wrapping_add(VTABLE_PTR));
        let _target = read_u32(vtable.wrapping_add(VTABLE_SLOT));
        let _ = lf_checker_rt::callee_thiscall!(FINAL_DISPATCH, u32, this);
    }
});
