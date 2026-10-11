// original: 0x00CC9DF0 ARTFeedbackInterfaceGta::vf5

/// Finds a render object through the receiver's `+0x190` parent and nested
/// record. A nonnull resource at nested `+0x6c` with a nonzero byte at `+0x0e`
/// is passed to the direct helper at `+0x808`; otherwise a fallback helper is
/// called with nested `+0x224`, then its result and three constants are passed
/// to the creation helper. If the selected helper returns an object, virtual
/// slot `+0x5c` receives the nested record and original receiver, and the method
/// returns `1`; missing objects return `0`. The helper implementations and
/// virtual target are scripted by the contract.
lf_checker_rt::export!(thiscall, rw_00cc9df0(this: u32) -> u32 {
    const PARENT_LINK: u32 = 0x190;
    const NESTED_LINK: u32 = 0x0c;
    const RESOURCE_LINK: u32 = 0x6c;
    const RESOURCE_READY: u32 = 0x0e;
    const DIRECT_RESOURCE_OFFSET: u32 = 0x808;
    const FALLBACK_LINK: u32 = 0x224;
    const VTABLE_SLOT: u32 = 0x5c;
    const FALLBACK_DESCRIPTOR_A_VA: u32 = 0x01112778;
    const FALLBACK_DESCRIPTOR_B_VA: u32 = 0x011128a8;
    const DIRECT_HELPER: u32 = 1;
    const FALLBACK_HELPER: u32 = 2;
    const CREATE_HELPER: u32 = 3;

    let parent = unsafe { (this.wrapping_add(PARENT_LINK) as *const u32).read_unaligned() };
    if parent == 0 {
        return 0;
    }
    let nested = unsafe { (parent.wrapping_add(NESTED_LINK) as *const u32).read_unaligned() };
    if nested == 0 {
        return 0;
    }

    let resource = unsafe {
        (nested.wrapping_add(RESOURCE_LINK) as *const u32).read_unaligned()
    };
    let render_object = if resource != 0
        && unsafe { (resource.wrapping_add(RESOURCE_READY) as *const u8).read() } != 0
    {
        lf_checker_rt::callee_thiscall!(
            DIRECT_HELPER,
            u32,
            resource.wrapping_add(DIRECT_RESOURCE_OFFSET)
        )
    } else {
        let fallback = unsafe {
            (nested.wrapping_add(FALLBACK_LINK) as *const u32).read_unaligned()
        };
        let helper_data = lf_checker_rt::callee_thiscall!(FALLBACK_HELPER, u32, fallback);
        let descriptor_a = lf_checker_rt::relocated(FALLBACK_DESCRIPTOR_A_VA);
        let descriptor_b = lf_checker_rt::relocated(FALLBACK_DESCRIPTOR_B_VA);
        lf_checker_rt::callee_cdecl!(
            CREATE_HELPER,
            u32,
            helper_data,
            0,
            descriptor_a,
            descriptor_b,
            0
        )
    };

    if render_object == 0 {
        return 0;
    }
    let vtable = unsafe { (render_object as *const u32).read_unaligned() };
    let method_address = unsafe {
        (vtable.wrapping_add(VTABLE_SLOT) as *const u32).read_unaligned()
    };
    let draw_method: extern "thiscall" fn(u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(method_address as usize) };
    let _ = draw_method(render_object, nested, this);
    1
});
