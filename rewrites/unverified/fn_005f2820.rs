// original: 0x005f2820 InitializeRenderPhaseHtml

/// Initializes a render phase's vtable, marks the shared render state active,
/// and invokes up to two methods on the TLS callback object. Each method is
/// gated by a nonzero 16-bit field and a non-null phase pointer. Finally it
/// tail-dispatches to the base update method and returns that method's result.
lf_checker_rt::export!(thiscall, rw_005f2820(this: u32) -> u32 {
    const PHASE_VTABLE: u32 = 0x00fe_15ec;
    const CALLBACK_OBJECT: u32 = 0x08;
    const CALLBACK_METHOD: u32 = 0x0c;
    const SHARED_RENDER_STATE: u32 = 0x0118_d808;
    const ACTIVE_RENDER_FLAG: u32 = 0x558;
    const FIRST_POINTER: u32 = 0x940;
    const FIRST_ENABLED: u32 = 0x946;
    const SECOND_POINTER: u32 = 0x948;
    const SECOND_ENABLED: u32 = 0x94e;

    unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(PHASE_VTABLE)) };
    let _ = lf_checker_rt::callee_cdecl!(1, u32, 0u32, u32::MAX);

    let render_state = unsafe {
        lf_checker_rt::global::<u32>(SHARED_RENDER_STATE).read_unaligned()
    };
    unsafe {
        let flag = render_state.wrapping_add(ACTIVE_RENDER_FLAG) as *mut u8;
        flag.write(flag.read() | 1);
    }

    let tls_block = lf_checker_rt::tls_slot(0);
    let callback_object = unsafe {
        (tls_block.wrapping_add(CALLBACK_OBJECT) as *const u32).read_unaligned()
    };
    let vtable = unsafe { (callback_object as *const u32).read_unaligned() };
    let target = unsafe {
        (vtable.wrapping_add(CALLBACK_METHOD) as *const u32).read_unaligned()
    };
    // SAFETY: the contract plants the intercepted method stub in this TLS object's table.
    let callback: extern "thiscall" fn(u32, u32) -> u32 = unsafe {
        core::mem::transmute(target as usize)
    };

    let second_enabled = unsafe {
        (this.wrapping_add(SECOND_ENABLED) as *const u16).read_unaligned()
    };
    let second_pointer = unsafe {
        (this.wrapping_add(SECOND_POINTER) as *const u32).read_unaligned()
    };
    if second_enabled != 0 && second_pointer != 0 {
        let _ = callback(callback_object, second_pointer);
    }

    let first_enabled = unsafe {
        (this.wrapping_add(FIRST_ENABLED) as *const u16).read_unaligned()
    };
    let first_pointer = unsafe {
        (this.wrapping_add(FIRST_POINTER) as *const u32).read_unaligned()
    };
    if first_enabled != 0 && first_pointer != 0 {
        let _ = callback(callback_object, first_pointer);
    }

    lf_checker_rt::callee_thiscall!(3, u32, this)
});
