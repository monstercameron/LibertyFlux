// original: 0x00E715B0 destroy_audio_entity_static_instance
/// Write the relocated audio-entity vtable to its static instance slot and tail-call the instance destructor.
lf_checker_rt::export!(cdecl, rw_fn_00E715B0() -> u32 {
    unsafe {
        const STATIC_OBJECT: u32 = 0x012142B8;
        const AUDIO_ENTITY_VTABLE: u32 = 0x00E83134;
        let object = lf_checker_rt::relocated(STATIC_OBJECT);
        lf_checker_rt::global::<u32>(STATIC_OBJECT).write(lf_checker_rt::relocated(AUDIO_ENTITY_VTABLE));
        lf_checker_rt::callee_thiscall!(1, u32, object)
    }
});
