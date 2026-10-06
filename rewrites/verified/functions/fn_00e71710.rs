// original: 0x00e71710 audio_reinit_1710
/// Release the block at 0x012202e8, reinstall the vtable at 0x012202e0, tail-call init.
///
/// Pushes the pointer from the global slot and calls the release callee
/// (cdecl/1, id 1), clears the slot, stores the vtable address 0x00e83134
/// into the object header (a relocated absolute: the stored immediate
/// carries a relocation entry, so the worker's original stores the
/// relocated address too), then tail-calls the init routine (id 2,
/// thiscall/0) with the object in ECX and returns its answer. cdecl/0.
export!(cdecl, rw_00e71710() -> u32 {
    unsafe {
        const PTR_SLOT: u32 = 0x012202E8;
        const OBJ: u32 = 0x012202E0;
        const VTABLE: u32 = 0x00E83134;
        let p = lf_checker_rt::global::<u32>(PTR_SLOT).read_unaligned();
        lf_checker_rt::callee_cdecl!(1, u32, p);
        lf_checker_rt::global::<u32>(PTR_SLOT).write_unaligned(0);
        lf_checker_rt::global::<u32>(OBJ).write_unaligned(relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(2, u32, relocated(OBJ))
    }
});
