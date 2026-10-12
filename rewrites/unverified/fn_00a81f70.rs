// original: 0x00A81F70 CSimpleNMBraceTaskInfo::get_brace_entity (proposed)

/// Return the cached brace entity, resolving and caching it from its network ID when needed.
///
/// The receiver stores the cached entity pointer at `+0x18` and the 16-bit
/// network ID at `+0x1E`. A null cache with a nonzero ID queries the mission
/// entity registrar, then calls the returned object's virtual slot one to
/// obtain its entity. A null lookup leaves the cache unchanged; a successful
/// lookup stores the virtual result, including null. Both callees are modeled
/// through checker runtime stand-ins.
lf_checker_rt::export!(thiscall, rw_00a81f70(info: u32) -> u32 {
    const CACHED_ENTITY: u32 = 0x18;
    const NETWORK_ID: u32 = 0x1e;
    const MISSION_ENTITY_REGISTRAR_VA: u32 = 0x0190_8ef0;
    const NETWORK_ID_LOOKUP: u32 = 1;
    const ENTITY_RESOLVER_SLOT_ONE: u32 = 2;

    let cached_entity = unsafe { rd32(info.wrapping_add(CACHED_ENTITY)) };
    if cached_entity == 0 {
        let network_id = unsafe { rd16(info.wrapping_add(NETWORK_ID)) };
        if network_id != 0 {
            let registrar = lf_checker_rt::global::<u32>(MISSION_ENTITY_REGISTRAR_VA) as u32;
            let network_object = lf_checker_rt::callee_thiscall!(
                NETWORK_ID_LOOKUP,
                u32,
                registrar,
                network_id,
                0
            );
            if network_object != 0 {
                let entity = lf_checker_rt::callee_thiscall!(
                    ENTITY_RESOLVER_SLOT_ONE,
                    u32,
                    network_object
                );
                unsafe { wr32(info.wrapping_add(CACHED_ENTITY), entity); }
            }
        }
    }

    unsafe { rd32(info.wrapping_add(CACHED_ENTITY)) }
});

#[inline(always)]
unsafe fn rd16(address: u32) -> u32 {
    unsafe { (address as *const u16).read_unaligned() as u32 }
}

#[inline(always)]
unsafe fn rd32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
}
