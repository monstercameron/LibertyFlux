// original: 0x005E1610 CRenderPhasePhoneModel::vf8
/// Builds a phone-model clear command unless the global render guard is set.
/// The receiver byte at `0x490` selects a zero scalar or the shared render
/// scalar. The allocator receives `[0x18, 0, scalar_bits]`; a nonnull command
/// then constructs a clear-target command with six fixed words and the scalar,
/// while a null allocation passes zero to the cancel helper. Both active paths
/// finish through the same post-effect dispatch tail. The method has only its
/// ECX receiver and no meaningful return value.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_phone_model_render(this: *mut u8) {
    const GLOBAL_RENDER_GUARD_VA: u32 = 0x0116_09F6;
    const SHARED_RENDER_SCALAR_VA: u32 = 0x00FE_88E8;
    const PHONE_MODEL_READY: usize = 0x490;
    const ALLOCATOR_WORDS: [u32; 2] = [0x18, 0];
    const CLEAR_FLAGS: [u32; 5] = [0, 0, 1, 1, 0];

    let guard = unsafe {
        core::ptr::read_unaligned(lf_checker_rt::global::<u8>(GLOBAL_RENDER_GUARD_VA))
    };
    if guard != 0 {
        return;
    }

    let model_ready = unsafe {
        core::ptr::read_unaligned(this.add(PHONE_MODEL_READY))
    };
    let scalar_bits = if model_ready != 0 {
        0
    } else {
        unsafe { core::ptr::read_unaligned(lf_checker_rt::global::<u32>(SHARED_RENDER_SCALAR_VA)) }
    };

    let allocation = lf_checker_rt::callee_cdecl!(
        1,
        u32,
        ALLOCATOR_WORDS[0],
        ALLOCATOR_WORDS[1],
        scalar_bits
    );
    if allocation != 0 {
        let _constructed = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            allocation,
            CLEAR_FLAGS[0],
            CLEAR_FLAGS[1],
            CLEAR_FLAGS[2],
            scalar_bits,
            CLEAR_FLAGS[3],
            CLEAR_FLAGS[4]
        );
        let _cancelled = lf_checker_rt::callee_cdecl!(3, u32, _constructed);
    } else {
        let _cancelled = lf_checker_rt::callee_cdecl!(3, u32, 0);
    }

    let _dispatch_result = lf_checker_rt::callee_cdecl!(4, u32);
}
