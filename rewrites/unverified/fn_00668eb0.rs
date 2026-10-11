// original: 0x00668EB0 rage::ptxEventEmitter::vf5

/// The function makes no call when the inner object pointer at `this + 0x48` is null or either required stack argument is zero. Otherwise it loads vtable slot 0x38 from that object and invokes it with stack argument `arg_2`, stack argument `arg_0`, stack argument `arg_1`, the word at `this + 0x4c`, the word at `this + 0x0c`. The third incoming word is forwarded as raw single-precision bits, first among the callee's stack arguments, without arithmetic.
/// The method is thiscall with 3 stack arguments and callee cleanup. The original leaves EAX unspecified when it takes the no-call path, so the contract does not compare the return channel; it compares the heap, stack, call log, and faults.
lf_checker_rt::export!(thiscall, rw_00668EB0(this: u32, arg_0: u32, arg_1: u32, arg_2: u32) -> u32 {
    const INNER_OBJECT: u32 = 0x48;
    const FORWARDED_MEMBER: u32 = 0x4c;
    const VTABLE_SLOT: u32 = 0x38;
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    let inner_object = unsafe { read_u32(this.wrapping_add(INNER_OBJECT)) };
    if inner_object == 0 || arg_0 == 0 || arg_1 == 0 {
        return 0;
    }
    let member_value = unsafe { read_u32(this.wrapping_add(FORWARDED_MEMBER)) };
    const EVENT_POINTER: u32 = 0x0c;
    let event_pointer = unsafe { read_u32(this.wrapping_add(EVENT_POINTER)) };
    let vtable = unsafe { read_u32(inner_object) };
    let target = unsafe { read_u32(vtable.wrapping_add(VTABLE_SLOT)) };
    let method: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 = unsafe { core::mem::transmute(target as usize) };
    let _ignored_result = method(inner_object, arg_2, arg_0, arg_1, member_value, event_pointer);
    0
});
