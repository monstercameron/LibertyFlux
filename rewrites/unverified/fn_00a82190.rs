// original: 0x00A82190 CSimpleNMFlinchTaskInfo::get_flinch_entity (proposed)

/// Return the cached flinch entity, resolving it through the network registrar when needed.
///
/// The receiver stores the cached entity at `+0x30` and its 16-bit network ID
/// at `+0x36`. A null cache with a nonzero ID calls the mission entity
/// registrar and then virtual slot one on the resolved network object. The
/// resulting pointer, including null, is cached. Both external calls use
/// checker stand-ins.
lf_checker_rt::export!(thiscall, rw_00a82190(info: u32) -> u32 {
    const CACHED_ENTITY: u32 = 0x30;
    const NETWORK_ID: u32 = 0x36;
    const MISSION_ENTITY_REGISTRAR_VA: u32 = 0x0190_8ef0;
    const NETWORK_ID_LOOKUP: u32 = 1;
    const ENTITY_RESOLVER_SLOT_ONE: u32 = 2;

    let cached_entity = unsafe { ((info.wrapping_add(CACHED_ENTITY)) as *const u32).read_unaligned() };
    if cached_entity == 0 {
        let network_id = unsafe { ((info.wrapping_add(NETWORK_ID)) as *const u16).read_unaligned() as u32 };
        if network_id != 0 {
            let registrar = lf_checker_rt::global::<u32>(MISSION_ENTITY_REGISTRAR_VA) as u32;
            let network_object = lf_checker_rt::callee_thiscall!(NETWORK_ID_LOOKUP, u32, registrar, network_id, 0);
            if network_object != 0 {
                let entity = lf_checker_rt::callee_thiscall!(ENTITY_RESOLVER_SLOT_ONE, u32, network_object);
                unsafe { ((info.wrapping_add(CACHED_ENTITY)) as *mut u32).write_unaligned(entity); }
            }
        }
    }

    unsafe { ((info.wrapping_add(CACHED_ENTITY)) as *const u32).read_unaligned() }
});
