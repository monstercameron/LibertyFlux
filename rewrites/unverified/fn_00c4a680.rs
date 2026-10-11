// original: 0x00C4A680 camera_set_timed_parameter

/// Dispatch one timed parameter through the object's third virtual slot.
///
/// The receiver in ECX points to a nested object at byte offset 0x160; the
/// nested object's first word is its table pointer, and byte offset 8 in that
/// table holds the method. The three stack arguments are a helper receiver,
/// a forwarded word, and one float bit pattern. The method receives a
/// 16-byte local workspace, the forwarded word, the float both as a stack
/// word and in XMM0, then two zeros; its x87 float result is discarded. A
/// second intercepted helper receives the first stack argument and the same
/// workspace pointer, and its 32-bit result is returned. The table lookup and
/// both outgoing calls remain observable in the checker contract.
lf_checker_rt::export!(thiscall, rw_c4a680(this: u32, helper_receiver: u32, forwarded: u32, value_bits: u32) -> u32 {
    const NESTED_OBJECT_FIELD: u32 = 0x160;
    const VTABLE_POINTER_FIELD: u32 = 0;
    const VIRTUAL_SLOT_OFFSET: u32 = 8;
    const ZERO_ARGUMENT: u32 = 0;
    const FINAL_HELPER: u32 = 2;

    #[repr(align(16))]
    struct AlignedWorkspace([u32; 4]);

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    let nested_object = unsafe { read_word(this.wrapping_add(NESTED_OBJECT_FIELD)) };
    let vtable = unsafe { read_word(nested_object.wrapping_add(VTABLE_POINTER_FIELD)) };
    let method_address = unsafe { read_word(vtable.wrapping_add(VIRTUAL_SLOT_OFFSET)) };
    let mut workspace = AlignedWorkspace([0; 4]);
    let workspace_pointer = core::ptr::addr_of_mut!(workspace.0[0]) as usize as u32;

    type TimedMethod = unsafe extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> f32;
    let method: TimedMethod = unsafe { core::mem::transmute(method_address as usize) };
    let _discarded_value = unsafe {
        method(
            nested_object,
            workspace_pointer,
            forwarded,
            value_bits,
            ZERO_ARGUMENT,
            ZERO_ARGUMENT,
        )
    };

    lf_checker_rt::callee_thiscall!(
        FINAL_HELPER,
        u32,
        helper_receiver,
        workspace_pointer,
    )
});
