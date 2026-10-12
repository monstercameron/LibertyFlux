// original: 0x00602B00 rage::rmPtfxManager::spawn_effect_instance (proposed)

/// Spawn one live instance of an effect rule.
///
/// `this` in ECX, the rule address (`rule`) and a spawn parameter
/// (`param`) on the stack. A null rule spawns nothing and returns 0.
/// Otherwise a node is allocated (callee 1, thiscall with `this`, no
/// stack arguments); a null node also returns 0. The rule is recorded at
/// `NODE_RULE` (+0x78) on the node, the node is initialised (callee 2,
/// thiscall with the node) and `param` is stored at `NODE_PARAM`
/// (+0x254). When the rule's copy flag (`RULE_COPY` at +0x10e) is set,
/// the eight transform words at `SRC_XFORM` (+0x28) on the manager are
/// copied to `DST_XFORM` (+0x10) on the node, except the hole where the
/// last two go to +0x60 and +0x68 instead of +0x40 and +0x48. Returns
/// the node address. Thiscall.
lf_checker_rt::export!(thiscall, rw_00602B00(this: u32, rule: u32, param: u32) -> u32 {
    unsafe {
        const NODE_RULE: u32 = 0x78;
        const NODE_PARAM: u32 = 0x254;
        const RULE_COPY: u32 = 0x10e;
        const SRC_XFORM: u32 = 0x28;
        const DST_XFORM: u32 = 0x10;
        const DST_TAIL: u32 = 0x60;
        const ALLOC_CALLEE: u32 = 1;
        const INIT_CALLEE: u32 = 2;

        if rule == 0 {
            return 0;
        }
        let node: u32 = lf_checker_rt::callee_thiscall!(ALLOC_CALLEE, u32, this);
        if node == 0 {
            return 0;
        }
        ((node + NODE_RULE) as *mut u32).write(rule);
        let _: u32 = lf_checker_rt::callee_thiscall!(INIT_CALLEE, u32, node);
        ((node + NODE_PARAM) as *mut u32).write(param);
        if ((rule + RULE_COPY) as *const u8).read() != 0 {
            for i in 0..6u32 {
                let w = ((this + SRC_XFORM + i * 8) as *const u64).read_unaligned();
                ((node + DST_XFORM + i * 8) as *mut u64).write_unaligned(w);
            }
            for i in 0..2u32 {
                let w = ((this + SRC_XFORM + 48 + i * 8) as *const u64).read_unaligned();
                ((node + DST_TAIL + i * 8) as *mut u64).write_unaligned(w);
            }
        }
        node
    }
});
