// original: 0x00a8d670 interior_release_room_records

/// Prepare the room-record list when required, then release or retain each
/// non-null object. The byte flags are at `+0x73`, `+0x75`, and `+0x76`; the
/// list pointer and 16-bit record count are at `+0xec` and `+0xf0`. An object
/// owned through `+0xb4` is first checked; a nonzero low-byte result keeps it.
/// Otherwise an owner match at `+0xbc` clears that word and writes `u32::MAX`
/// at `+0xc0`; a non-match calls the object's secondary and primary vtable
/// methods with no argument and with argument 1. The list is released after
/// an enabled scan. Thiscall takes only the object pointer in ECX.
lf_checker_rt::export!(thiscall, rw_00a8d670(this: u32) -> u32 {
    unsafe {
        const INIT_IF_NEEDED: u32 = 5;
        const CHECK_OWNER: u32 = 1;
        const VTABLE_SECONDARY: u32 = 2;
        const VTABLE_PRIMARY: u32 = 3;
        const FREE_LIST: u32 = 4;
        const INITIALIZED: u32 = 0x73;
        const NEEDS_INIT: u32 = 0x75;
        const ENABLED: u32 = 0x76;
        const RECORD_LIST: u32 = 0xec;
        const RECORD_COUNT: u32 = 0xf0;
        const OWNER_A: u32 = 0xb4;
        const OWNER_B: u32 = 0xbc;
        const RELEASE_SENTINEL: u32 = 0xc0;

        let initialized = ((this + INITIALIZED) as *const u8).read_unaligned();
        let needs_init = ((this + NEEDS_INIT) as *const u8).read_unaligned();
        if initialized != 0 || needs_init == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(INIT_IF_NEEDED, u32, this);
        }

        let enabled = ((this + ENABLED) as *const u8).read_unaligned();
        if enabled == 0 {
            return 0;
        }
        let list = ((this + RECORD_LIST) as *const u32).read_unaligned();
        let count = ((this + RECORD_COUNT) as *const u16).read_unaligned() as u32;
        let mut index = 0u32;
        while index < count {
            let object = ((list + index.wrapping_mul(4)) as *const u32).read_unaligned();
            if object != 0 {
                let owner_a = ((object + OWNER_A) as *const u32).read_unaligned();
                if owner_a == this {
                    let keep = lf_checker_rt::callee_thiscall!(CHECK_OWNER, u32, object);
                    if keep as u8 != 0 {
                        index += 1;
                        continue;
                    }
                }

                let owner_b = ((object + OWNER_B) as *const u32).read_unaligned();
                if owner_b == this {
                    ((object + OWNER_B) as *mut u32).write_unaligned(0);
                    ((object + RELEASE_SENTINEL) as *mut u32).write_unaligned(u32::MAX);
                } else {
                    let _: u32 = lf_checker_rt::callee_thiscall!(VTABLE_SECONDARY, u32, object);
                    let _: u32 = lf_checker_rt::callee_thiscall!(VTABLE_PRIMARY, u32, object, 1);
                }
            }
            index += 1;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(FREE_LIST, u32, list);
        ((this + RECORD_LIST) as *mut u32).write_unaligned(0);
        ((this + RECORD_COUNT) as *mut u32).write_unaligned(0);
        ((this + ENABLED) as *mut u8).write_unaligned(0);
        0
    }
});
