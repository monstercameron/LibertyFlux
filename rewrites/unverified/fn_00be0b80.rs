// original: 0x00BE0B80 CTaskSimpleTriggerLookAt::vf1
/// Clone a `CTaskSimpleTriggerLookAt` task (vtable slot 1).
///
/// Loads the allocator from the global at `ALLOCATOR` and allocates a fresh
/// object; when allocation fails (null) returns 0. Otherwise runs the copy
/// constructor as a thiscall with the fresh object in ECX and arg0 = +0x30, arg1 = ebx, arg2 = edi (see prelude), arg3 = this+0x20, arg4 = +0x48, arg5 = float bits +0x44, arg6 = +0x40, arg7 = sign-extended byte +0x3c,
/// and returns the constructor's result. All float fields move as bits
/// (the original uses movss, never arithmetic). Original is thiscall with
/// no stack words.
lf_checker_rt::export!(thiscall, rw_00be0b80(this: u32) -> u32 {
    const ALLOC: u32 = 1;
    const CTOR: u32 = 2;
    const ALLOCATOR: u32 = 0x0167E2A0;
    const F_30: u32 = 0x30;
    const P_20: u32 = 0x20;
    const F_48: u32 = 0x48;
    const FLT_44: u32 = 0x44;
    const F_40: u32 = 0x40;
    const B_3C: u32 = 0x3C;
    const EDI_OFF: u32 = 0x38;
    const EBX_OFF: u32 = 0x34;
    const GATE_OFF: u32 = 0x30;
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read() }
    }
    unsafe {

    // Prelude: edi = [this+0x38] and ebx = [this+0x34], adjusted when the
    // signed edi is above -1 while [this+0x30] is zero (then edi = -1 and
    // ebx = 100). The original compares edi as SIGNED (jle).
    let mut edi = rd32(this + EDI_OFF) as i32;
    let mut ebx = rd32(this + EBX_OFF);
    if edi > -1 && rd32(this + GATE_OFF) == 0 {
        edi = -1;
        ebx = 100;
    }
        let allocator = lf_checker_rt::global::<u32>(ALLOCATOR).read();
        let fresh = lf_checker_rt::callee_thiscall!(ALLOC, u32, allocator);
        if fresh == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CTOR, u32, fresh, rd32(this + F_30), ebx, edi as u32, this.wrapping_add(P_20), rd32(this + F_48), rd32(this + FLT_44), rd32(this + F_40), rd8(this + B_3C) as i8 as i32 as u32)
    }
});
