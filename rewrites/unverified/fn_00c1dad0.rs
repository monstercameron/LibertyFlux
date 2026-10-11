// original: 0x00C1DAD0 m:/textures/coronas

/// Initialize the corona texture registry through seven scripted collaborators,
/// store the final returned handle in the registry global, clear 768 sparse
/// four-byte state slots at a 0x40-byte stride, then tail-call the follow-up
/// initializer. The internal clear helper's writes are reproduced directly so
/// the full sparse global output remains observable. The tail target takes no
/// stack arguments.
lf_checker_rt::export!(cdecl, rw_00c1dad0() -> u32 {
    unsafe {
        const LABEL_OPEN: u32 = 0x00EC5354;
        const LABEL_CONFIG: u32 = 0x00EC535C;
        const LABEL_RESOURCE: u32 = 0x00EC533C;
        const LABEL_ATTACH: u32 = 0x00EC5378;
        const REGISTRY_HANDLE_VA: u32 = 0x0169EAC0;
        const PENDING_HANDLE_VA: u32 = 0x0169EAC4;
        const SPARSE_STATE_VA: u32 = 0x016AAAF0;
        const STATE_SLOT_COUNT: u32 = 768;
        const STATE_SLOT_STRIDE: u32 = 0x40;

        let registry = lf_checker_rt::callee_cdecl!(
            1, u32, lf_checker_rt::relocated(LABEL_OPEN),
        );
        let _ = lf_checker_rt::callee_cdecl!(
            2, (), lf_checker_rt::relocated(LABEL_CONFIG), registry,
        );
        let _ = lf_checker_rt::callee_cdecl!(3, (), registry);

        let resource = lf_checker_rt::callee_cdecl!(
            4, u32, lf_checker_rt::relocated(LABEL_RESOURCE), 0,
        );
        let attachment = lf_checker_rt::callee_cdecl!(
            5, u32, lf_checker_rt::relocated(LABEL_ATTACH), resource,
        );
        let instance = lf_checker_rt::callee_cdecl!(6, u32, attachment);
        let registry_handle = lf_checker_rt::callee_thiscall!(7, u32, instance);
        (lf_checker_rt::relocated(REGISTRY_HANDLE_VA) as *mut u32)
            .write_unaligned(registry_handle);

        let state_base = lf_checker_rt::relocated(SPARSE_STATE_VA);
        for slot in 0..STATE_SLOT_COUNT {
            let slot_address = state_base.wrapping_add(slot.wrapping_mul(STATE_SLOT_STRIDE));
            (slot_address as *mut u32).write_unaligned(0);
        }
        (lf_checker_rt::relocated(PENDING_HANDLE_VA) as *mut u32).write_unaligned(0);

        lf_checker_rt::callee_cdecl!(8, u32,)
    }
});
