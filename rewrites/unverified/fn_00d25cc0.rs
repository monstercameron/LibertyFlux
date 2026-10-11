// original: 0x00D25CC0 get_burnt_vehicle_texture
/// Looks up the burnt-out vehicle texture by calling the name and texture-slot helpers, stores
/// the returned texture handle in a shared global word, and returns a success byte. The function
/// is cdecl with one stack argument; each helper result is scripted and its call arguments are
/// compared.
lf_checker_rt::export!(cdecl, rw_00d25cc0(vehicle_name: u32) -> u8 {
    const TEXTURE_DICTIONARY_VA: u32 = 0x00EE10F8;
    const TEXTURE_HANDLE_GLOBAL_VA: u32 = 0x0171F9AC;
    unsafe {
        let global_slot = lf_checker_rt::global::<u32>(TEXTURE_HANDLE_GLOBAL_VA);
        global_slot.write_unaligned(0);
        let name_context = lf_checker_rt::callee_cdecl!(1, u32, vehicle_name);
        let dictionary_name = lf_checker_rt::relocated(TEXTURE_DICTIONARY_VA);
        let texture_slot = lf_checker_rt::callee_cdecl!(2, u32, dictionary_name, 0);
        let texture_handle = lf_checker_rt::callee_thiscall!(3, u32, name_context, texture_slot);
        global_slot.write_unaligned(texture_handle);
        1
    }
});
