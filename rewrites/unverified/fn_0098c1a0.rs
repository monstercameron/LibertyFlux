// original: 0x0098C1A0 audVehicleAudioEntity::ensure_heli_config

/// When the global configuration gate is set, obtains the current model
/// configuration name and asks the shared registry to find it. A zero lookup
/// result triggers a second registry call with the fallback resource name.
/// With the gate clear, the function returns zero without making calls.
lf_checker_rt::export!(thiscall, rw_audvehicleaudioentity_ensure_heli_config(this: u32) -> u32 {
    const CONFIG_GATE: u32 = 0x115D_CE6;
    const REGISTRY: u32 = 0x115D_9A0;
    const FALLBACK_RESOURCE: u32 = 0x00E8_F510;

    unsafe {
        if lf_checker_rt::global::<u8>(CONFIG_GATE).read() == 0 {
            return 0;
        }

        let model_name: unsafe extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(lf_checker_rt::callee_addr(1));
        let lookup: unsafe extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(lf_checker_rt::callee_addr(2));

        let name = model_name(this);
        let result = lookup(REGISTRY, name);
        if result != 0 {
            return result;
        }
        lookup(REGISTRY, FALLBACK_RESOURCE)
    }
});
