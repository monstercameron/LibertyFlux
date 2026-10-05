// original: 0x00c6c660 anim_dict_ctor (proposed)

/// Construct an animation dictionary: base part, two member tables,
/// then register every entry of the second table.
///
/// Runs the base constructor on `this`, tags the vtable slot, builds
/// the member at `+0x10` and the member at `+0x18` from the same
/// argument, then passes each entry address of the second member's
/// array to the registrar. Returns `this`.
///
/// Original: thiscall with one stack word, four call sites.
lf_checker_rt::export!(thiscall, rw_00c6c660(this: u32, arg: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EC_C104;
        const MEMBER_A: u32 = 0x10;
        const MEMBER_B: u32 = 0x18;
        const COUNT_OFF: u32 = 0x4;
        const BASE_CTOR: u32 = 1;
        const MEMBER_CTOR: u32 = 2;
        const REGISTER: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }

        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this);
        unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE)) };
        lf_checker_rt::callee_thiscall!(MEMBER_CTOR, u32, arg, this.wrapping_add(MEMBER_A));
        let member_b = this.wrapping_add(MEMBER_B);
        lf_checker_rt::callee_thiscall!(MEMBER_CTOR, u32, arg, member_b);
        let mut i = 0u32;
        while i < rd16(member_b.wrapping_add(COUNT_OFF)) {
            let arr = rd32(member_b);
            lf_checker_rt::callee_cdecl!(REGISTER, u32, arr.wrapping_add(i.wrapping_mul(4)));
            i = i.wrapping_add(1);
        }
        this
    }
});
