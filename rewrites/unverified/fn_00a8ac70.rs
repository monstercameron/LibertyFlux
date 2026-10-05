// original: 0x00a8ac70 pool_bind_or_invalidate (proposed)

/// Bind this slot to the shared pool, or invalidate it when anything is
/// missing.
///
/// `this` is the slot. Reads the shared manager pointer and its words at
/// +0x550 (handle) and +0x548 (cookie): if the manager or handle is null,
/// or the membership callee rejects the handle, the slot is invalidated
/// (+0x40/+0x44 set to -1) and the incoming register (fixed by the proof)
/// or the callee answer is returned, as the original leaves it. Otherwise
/// the address callee resolves the handle and the slot takes the cookie at
/// +0x44, the resolved address at +0x40 and 0 at +0x48.
///
/// Original: 0x00A8AC70 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a8ac70(this: u32) -> u32 {
    unsafe {
        const CALLEE_IS_MEMBER: u32 = 1;
        const CALLEE_RESOLVE: u32 = 2;
        const MANAGER_PTR: u32 = 0x118d800;
        const CALLEE_THIS: u32 = 0x12fb214;
        const HANDLE: u32 = 0x550;
        const COOKIE: u32 = 0x548;
        const SLOT_ADDR: u32 = 0x40;
        const SLOT_COOKIE: u32 = 0x44;
        const SLOT_GEN: u32 = 0x48;
        const INCOMING_EAX: u32 = 0x12345678;
        let fail = |this: u32| {
            ((this + SLOT_ADDR) as *mut u32).write_unaligned(0xffffffff);
            ((this + SLOT_COOKIE) as *mut u32).write_unaligned(0xffffffff);
            ((this + SLOT_GEN) as *mut u32).write_unaligned(0);
        };
        let manager =
            lf_checker_rt::global::<u32>(MANAGER_PTR).read_unaligned();
        if manager == 0 {
            fail(this);
            return INCOMING_EAX;
        }
        let handle =
            ((manager + HANDLE) as *const u32).read_unaligned();
        if handle == 0 {
            fail(this);
            return INCOMING_EAX;
        }
        let callee_this =
            lf_checker_rt::global::<u32>(CALLEE_THIS).read_unaligned();
        let member = lf_checker_rt::callee_thiscall!(
            CALLEE_IS_MEMBER,
            u32,
            callee_this,
            handle
        );
        if member as u8 == 0 {
            fail(this);
            return member;
        }
        let cookie =
            ((manager + COOKIE) as *const u32).read_unaligned();
        let addr = lf_checker_rt::callee_thiscall!(
            CALLEE_RESOLVE,
            u32,
            callee_this,
            handle
        );
        ((this + SLOT_COOKIE) as *mut u32).write_unaligned(cookie);
        ((this + SLOT_ADDR) as *mut u32).write_unaligned(addr);
        ((this + SLOT_GEN) as *mut u32).write_unaligned(0);
        addr
    }
});
