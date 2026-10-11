// original: 0x00DCB8F0 BIKE_CRASH_DRIVEN

/// Selects a native context pointer from an entity's task classification.
///
/// The cdecl function takes an entity pointer and a selector word. When the
/// selector's low byte is nonzero, the entity's signedness is irrelevant: its
/// unsigned type value maps to one of three context pointers. With a zero
/// selector, a null entity or an unrecognized type returns the default
/// context; type zero also asks a helper about the entity's embedded task
/// context, while type one selects its dedicated context.
lf_checker_rt::export!(cdecl, rw_00dcb8f0(entity: u32, selector: u32) -> u32 {
    unsafe {
        const ENTITY_TYPE: u32 = 0x1304;
        const EMBEDDED_CONTEXT: u32 = 0x210;
        const IS_CONTEXT_ACTIVE: u32 = 1;
        const TYPE_ZERO_CONTEXT: u32 = 0x00EF5098;
        const TYPE_ONE_CONTEXT: u32 = 0x00EF50AC;
        const DEFAULT_CONTEXT: u32 = 0x00EF50C0;
        const SELECTOR_TYPE_ZERO_CONTEXT: u32 = 0x00EF50E8;
        const SELECTOR_TYPE_ONE_CONTEXT: u32 = 0x00EF50F4;
        const SELECTOR_TYPE_THREE_OR_FOUR_CONTEXT: u32 = 0x00EF5100;
        const SELECTOR_FALLBACK_CONTEXT: u32 = 0x00EF5110;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        if selector as u8 != 0 {
            if entity == 0 {
                return lf_checker_rt::relocated(SELECTOR_FALLBACK_CONTEXT);
            }
            return match read_u32(entity.wrapping_add(ENTITY_TYPE)) {
                0 => lf_checker_rt::relocated(SELECTOR_TYPE_ZERO_CONTEXT),
                1 => lf_checker_rt::relocated(SELECTOR_TYPE_ONE_CONTEXT),
                3 | 4 => lf_checker_rt::relocated(SELECTOR_TYPE_THREE_OR_FOUR_CONTEXT),
                _ => lf_checker_rt::relocated(SELECTOR_FALLBACK_CONTEXT),
            };
        }

        if entity == 0 {
            return lf_checker_rt::relocated(DEFAULT_CONTEXT);
        }
        let entity_type = read_u32(entity.wrapping_add(ENTITY_TYPE));
        if entity_type == 0 {
            let active = lf_checker_rt::callee_thiscall!(
                IS_CONTEXT_ACTIVE,
                u32,
                entity.wrapping_add(EMBEDDED_CONTEXT)
            );
            if active as u8 != 0 {
                return lf_checker_rt::relocated(TYPE_ZERO_CONTEXT);
            }
        }
        if entity_type == 1 {
            return lf_checker_rt::relocated(TYPE_ONE_CONTEXT);
        }
        lf_checker_rt::relocated(DEFAULT_CONTEXT)
    }
});
