// original: 0x00D4E9A0 CTaskComplexPickUpObject::ctor (proposed)

// Constructor of CTaskComplexPickUpObject: chains base constructors, then links the target.
///
/// Runs intercepted callee 1 (base) on `this`, installs the class vtable
/// pointer, stores `arg0` at `+0x14` and `arg1` at `+0x18`, constructs the
/// sub-objects at `+0x30` and `+0x3c` through intercepted callees 2 and 3,
/// and returns `this`. When `arg0` is non-null it is additionally linked: a
/// pointer to the `+0x14` slot goes to intercepted callee 4, vtable slot
/// `+0x50` of the target runs with a pointer to `+0x20` (intercepted callee
/// 5), and intercepted callee 6 is consulted. A zero answer there, or a
/// non-zero word at target `+0x6c`, ends the linking. Otherwise the target
/// goes with (target, 0, 0, 1) and a fixed address to intercepted callee 7; a
/// zero answer unlinks again through intercepted callee 8 and clears `+0x14`.
///
/// Original: 0x00D4E9A0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00d4e9a0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EE5004;
        const FIXED_ADDR: u32 = 0x01908EF0;
        const BASE_CTOR: u32 = 1;
        const SUB_A: u32 = 2;
        const SUB_B: u32 = 3;
        const LINK: u32 = 4;
        const TARGET_HOOK: u32 = 5;
        const CHECK: u32 = 6;
        const REGISTER: u32 = 7;
        const UNLINK: u32 = 8;
        const HOOK_SLOT: u32 = 0x50;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + 0x14) as *mut u32).write_unaligned(arg0);
        ((this + 0x18) as *mut u32).write_unaligned(arg1);
        lf_checker_rt::callee_thiscall!(SUB_A, u32, this.wrapping_add(0x30));
        lf_checker_rt::callee_thiscall!(SUB_B, u32, this.wrapping_add(0x3c));
        let target = ((this + 0x14) as *const u32).read_unaligned();
        if target == 0 {
            return this;
        }
        lf_checker_rt::callee_thiscall!(LINK, u32, target, this.wrapping_add(0x14));
        let target2 = ((this + 0x14) as *const u32).read_unaligned();
        let vt = (target2 as *const u32).read_unaligned();
        let addr = ((vt + HOOK_SLOT) as *const u32).read_unaligned();
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(addr as usize);
        hook(target2, this.wrapping_add(0x20));
        // Callee 6 keeps no register setup from the original (ECX holds stub
        // scratch on both sides, as it holds callee leftovers in the game),
        // so it is declared cdecl/0 and ECX is not compared.
        let ok: u32 = lf_checker_rt::callee_cdecl!(CHECK, u32,);
        if ok as u8 == 0 {
            return this;
        }
        let target3 = ((this + 0x14) as *const u32).read_unaligned();
        if ((target3 + 0x6c) as *const u32).read_unaligned() != 0 {
            return this;
        }
        let reg: u32 = lf_checker_rt::callee_thiscall!(
            REGISTER, u32, lf_checker_rt::relocated(FIXED_ADDR), target3, 0, 0, 1);
        if reg as u8 != 0 {
            return this;
        }
        let target4 = ((this + 0x14) as *const u32).read_unaligned();
        if target4 != 0 {
            lf_checker_rt::callee_thiscall!(UNLINK, u32, target4, this.wrapping_add(0x14));
        }
        ((this + 0x14) as *mut u32).write_unaligned(0);
        this
    }
});
