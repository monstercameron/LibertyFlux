// original: 0x00D3E520 task_request_create_for_actor (proposed)
/// Builds a task request in the eight-byte result object. The actor links an
/// auxiliary value object and a virtual-call interface; the interface's
/// function at slot 0x44 supplies a value combined with the auxiliary word,
/// unless the fifth stack argument overrides it. A ten-word cdecl factory
/// call receives that value and the seven input words. The result stores the
/// returned task pointer and a byte indicating whether it is non-null. The
/// float argument is forwarded by its original 32-bit representation.
lf_checker_rt::export!(thiscall, rw_d3e520(
    result_out: u32,
    actor: u32,
    argument_1: u32,
    argument_2: u32,
    float_bits: u32,
    override_value: u32,
    argument_5: u32,
    argument_6: u32,
) -> u8 {
    const ACTOR_VALUE_LINK: usize = 0x224;
    const ACTOR_INTERFACE_LINK: usize = 0xA80;
    const AUXILIARY_WORD: usize = 0x284;
    const VTABLE_SLOT_44: usize = 0x44;
    const INDIRECT_QUERY: u32 = 1;
    const TASK_FACTORY: u32 = 2;
    const RESULT_POINTER_OFFSET: usize = 4;

    unsafe { (result_out as *mut u8).write(0); }
    let auxiliary = unsafe { ((actor + ACTOR_VALUE_LINK as u32) as *const u32).read_unaligned() };
    let auxiliary_value = unsafe { ((auxiliary + AUXILIARY_WORD as u32) as *const u32).read_unaligned() };
    let interface = unsafe { ((actor + ACTOR_INTERFACE_LINK as u32) as *const u32).read_unaligned() };
    let vtable = unsafe { (interface as *const u32).read_unaligned() };
    let method = unsafe { ((vtable + VTABLE_SLOT_44 as u32) as *const u32).read_unaligned() };
    let _ = method;
    let virtual_value = lf_checker_rt::callee_thiscall!(INDIRECT_QUERY, u32, interface);
    let factory_value = if override_value != 0 {
        override_value
    } else {
        virtual_value | auxiliary_value
    };
    let task = lf_checker_rt::callee_cdecl!(
        TASK_FACTORY,
        u32,
        argument_1,
        argument_2,
        argument_1,
        float_bits,
        factory_value,
        0,
        argument_5,
        argument_6,
        actor,
        0,
    );
    unsafe { ((result_out + RESULT_POINTER_OFFSET as u32) as *mut u32).write_unaligned(task); }
    if task != 0 {
        unsafe { (result_out as *mut u8).write(1); }
    }
    unsafe { (result_out as *const u8).read() }
});
