// original: 0x00602BA0 rage::rmPtfxManager::spawn_effect_by_name (proposed)

/// Look up an effect rule and spawn one live instance of it.
///
/// `this` in ECX with three stack arguments: the rule name (`name`), a
/// spawn parameter (`param`) and a lookup scope (`scope`). The rule is
/// found first (callee 1, thiscall with `this`, `name` and `scope`); a
/// null answer means no rule and the function returns 0. Otherwise one
/// instance is spawned (callee 2, thiscall with `this`, the rule and
/// `param`) and its address is returned. Thiscall.
lf_checker_rt::export!(thiscall, rw_00602BA0(this: u32, name: u32, param: u32, scope: u32) -> u32 {
    unsafe {
        const FIND_CALLEE: u32 = 1;
        const SPAWN_CALLEE: u32 = 2;

        let rule: u32 = lf_checker_rt::callee_thiscall!(FIND_CALLEE, u32, this, name, scope);
        if rule == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(SPAWN_CALLEE, u32, this, rule, param)
    }
});
