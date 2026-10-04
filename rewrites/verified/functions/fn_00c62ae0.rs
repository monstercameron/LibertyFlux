// original: 0x00c62ae0 anim_player_detach
/// Player detach: releases the active binding and re-registers the hook.
///
/// For binding kinds 1 and 2, looks a non-trivial handle up in the registry
/// (a missing handle still runs the release hook), runs the release hook stored
/// in the object with this player, and re-registers the default hook for the
/// kind, returning the registrar's answer. Any other kind returns unchanged.
export!(thiscall, rw_00c62ae0(this: u32) -> u32 {
    unsafe {
        let kind = *((this + 0x1C) as *const u32);
        if kind != 1 && kind != 2 {
            return kind;
        }
        let handle = *((this + 0x28) as *const u32);
        let mut run_hook = handle == 0xFFFF_FFFF;
        if !run_hook {
            let registry = *global::<u32>(0x167E2A0);
            let found: u32 = callee_thiscall!(0, u32, registry, handle);
            run_hook = found != 0;
        }
        if run_hook {
            let extra = *((this + 0x24) as *const u32);
            let target = *((this + 0x20) as *const u32);
            let hook: extern "cdecl" fn(u32, u32) -> u32 =
                core::mem::transmute::<u32, extern "cdecl" fn(u32, u32) -> u32>(target);
            let _: u32 = hook(this, extra);
        }
        let which = if kind == 2 { 2 } else { 1 };
        callee_thiscall!(2, u32, this, which, relocated(0x4016A0), 0)
    }
});
