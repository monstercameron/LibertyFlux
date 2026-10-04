// original: 0x00e5c430 node_register_d510_then_register
/// Pushes the fixed node at 0x018dd510 onto the intrusive list headed
/// at 0x017f58f4 (old head saved in the link slot at 0x018dd564),
/// then registers tag 0x00e6e350. Returns the registrar's answer.
export!(cdecl, rw_00e5c430() -> u32 {
    unsafe {
        const HEAD: u32 = 0x017F58F4;
        const NODE: u32 = 0x018DD510;
        const LINK_SLOT: u32 = 0x018DD564;
        const TAG: u32 = 0x00E6E350;
        let head = global::<u32>(HEAD);
        let old = *head;
        *global::<u32>(LINK_SLOT) = old;
        *head = relocated(NODE);
        callee_cdecl!(1, u32, relocated(TAG))
    }
});
