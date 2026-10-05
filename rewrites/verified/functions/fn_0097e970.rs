// original: 0x0097E970 ped_task_set_attachment (proposed)

/// Attach a ped (or nothing) to a task object, caching the last triple.
///
/// `this` is the task object; the arguments are a selector, a ped object
/// (or null) and a parameter. Callee 1 first classifies the selector's
/// middle byte and the verdict is latched in the flag byte at `+0x1A0`.
///
/// A ped whose kind bits (`+0x28 & 0x3C0`) read pedestrian with state 2
/// at `+0x1304` takes an early path that only refreshes the resolved
/// word at `+0xB8` from its global. Otherwise, if the cached triple at
/// `+0xAC`/`+0xB4`/`+0xB0` already equals the arguments, nothing happens.
/// If not, a resolver word is picked: callee 2 (this = fixed manager)
/// from the ped's `+0x38` link and the parameter when both exist, else a
/// table word indexed by the selector's low byte. The old ped link at
/// `+0xB4` is released through callee 3 when set. A null resolver clears
/// the cache and resolves the table's first word; otherwise the resolved
/// word is the resolver, overridden by its global when the ped reads
/// pedestrian with state 1, and the new triple is stored with callee 4
/// linking the ped.
///
/// Original: 0x0097E970 (thiscall, three stack arguments, no return).
lf_checker_rt::export!(thiscall, rw_0097E970(this: u32, sel: u32, ped: u32, param: u32) -> u32 {
    unsafe {
        const MGR_CLASSIFY: u32 = 0x016C8FB0;
        const MGR_RESOLVE: u32 = 0x012202E0;
        const G_RESOLVED_FAST: u32 = 0x01231304;
        const G_TABLE_BASE: u32 = 0x01231290;
        const G_RESOLVED_ALT: u32 = 0x01231308;
        const KIND_MASK: u32 = 0x3C0;
        const KIND_PED: u32 = 0x80;
        const OFF_FLAG: u32 = 0x1A0;
        const OFF_RESOLVED: u32 = 0xB8;
        const OFF_SEL: u32 = 0xAC;
        const OFF_PED: u32 = 0xB4;
        const OFF_PARAM: u32 = 0xB0;
        const PED_KIND: u32 = 0x28;
        const PED_STATE: u32 = 0x1304;
        const PED_LINK: u32 = 0x38;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn gget(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }

        let class: u32 = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            lf_checker_rt::relocated(MGR_CLASSIFY),
            (sel >> 8) & 0xFF
        );
        (this.wrapping_add(OFF_FLAG) as *mut u8).write(if (class & 0xFF) == 0 { 0 } else { 1 });
        if ped != 0
            && rd32(ped.wrapping_add(PED_KIND)) & KIND_MASK == KIND_PED
            && rd32(ped.wrapping_add(PED_STATE)) == 2
        {
            wr32(this.wrapping_add(OFF_RESOLVED), gget(G_RESOLVED_FAST));
            return 0;
        }
        if rd32(this.wrapping_add(OFF_SEL)) == sel
            && rd32(this.wrapping_add(OFF_PED)) == ped
            && rd32(this.wrapping_add(OFF_PARAM)) == param
        {
            return 0;
        }
        let link = if ped != 0 { rd32(ped.wrapping_add(PED_LINK)) } else { 0 };
        let mut resolver = 0u32;
        let mut have = false;
        if ped != 0 && link != 0 {
            resolver = lf_checker_rt::callee_thiscall!(
                2,
                u32,
                lf_checker_rt::relocated(MGR_RESOLVE),
                link,
                param
            );
            have = resolver != 0;
        }
        if !have {
            let base = gget(G_TABLE_BASE);
            resolver = rd32(base.wrapping_add((sel & 0xFF).wrapping_mul(4)));
        }
        let ped_slot = this.wrapping_add(OFF_PED);
        if rd32(ped_slot) != 0 {
            let _: u32 = lf_checker_rt::callee_stdcall!(3, u32, ped_slot);
            wr32(ped_slot, 0);
        }
        if resolver == 0 {
            wr32(this.wrapping_add(OFF_SEL), 0);
            wr32(this.wrapping_add(OFF_PARAM), 0);
            wr32(this.wrapping_add(OFF_RESOLVED), rd32(gget(G_TABLE_BASE)));
            return 0;
        }
        let mut resolved = resolver;
        if ped != 0
            && rd32(ped.wrapping_add(PED_KIND)) & KIND_MASK == KIND_PED
            && rd32(ped.wrapping_add(PED_STATE)) == 1
        {
            resolved = gget(G_RESOLVED_ALT);
        }
        wr32(this.wrapping_add(OFF_RESOLVED), resolved);
        wr32(this.wrapping_add(OFF_SEL), sel);
        wr32(ped_slot, ped);
        if ped != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, ped, ped_slot);
        }
        wr32(this.wrapping_add(OFF_PARAM), param);
        0
    }
});
