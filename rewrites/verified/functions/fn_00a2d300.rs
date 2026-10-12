// original: 0x00A2D300 CPlayerPed::change_model

/// Check the selected model's state. If it is inactive, return the default
/// initializer result. Otherwise preload the player model, initialize it,
/// revive the player with the existing-slot flag, and load its diff bundle.
lf_checker_rt::export!(thiscall, rw_a2d300(this: u32, model_id: u32) -> u32 {
    const MODEL_TABLE_VA: u32 = 0x0129_5CD8;
    const LOOKUP: u32 = 1;
    const PRELOAD: u32 = 2;
    const REVIVE: u32 = 3;
    const LOAD_BUNDLE: u32 = 4;

    let model_entry = unsafe {
        let entry_va = MODEL_TABLE_VA.wrapping_add(model_id.wrapping_mul(4));
        (lf_checker_rt::global::<u32>(entry_va) as *const u32).read_unaligned()
    };
    let model_state = unsafe { (model_entry.wrapping_add(0x8c) as *const u8).read() };

    if model_state == 0 {
        return lf_checker_rt::callee_thiscall!(LOOKUP, u32, this, model_id);
    }

    let _ = lf_checker_rt::callee_cdecl!(PRELOAD, u32, model_id);
    let _ = lf_checker_rt::callee_thiscall!(LOOKUP, u32, this, model_id);

    let current_slot = unsafe { (this.wrapping_add(0x68) as *const u32).read_unaligned() };
    let revive_with_slot = if current_slot == u32::MAX { 0 } else { 1 };
    let _ = lf_checker_rt::callee_thiscall!(REVIVE, u32, this, revive_with_slot);

    let bundle_context = unsafe { (this.wrapping_add(0x21c) as *const u32).read_unaligned() };
    lf_checker_rt::callee_cdecl!(LOAD_BUNDLE, u32, this, bundle_context)
});
