// original: 0x0066c590 rage::rmPtfxShaderVar_Texture::load_and_set_texture

/// Load or find a texture through the manager pointer stored at the global
/// texture-manager slot, passing the incoming identifier unchanged. Then call
/// virtual slot `+0x2c` on this shader variable with the loaded texture and
/// return that method's result. This is a thiscall method with one stack
/// argument; both outgoing thiscalls are compared through checker stubs.
lf_checker_rt::export!(thiscall, rw_0066c590(this: u32, texture_id: u32) -> u32 {
    unsafe {
        const TEXTURE_MANAGER_FILE_VA: u32 = 0x01bb6674;
        const TEXTURE_SETTER_SLOT: u32 = 0x2c;
        const TEXTURE_LOADER_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let texture_manager = read_word(lf_checker_rt::relocated(TEXTURE_MANAGER_FILE_VA));
        let texture = lf_checker_rt::callee_thiscall!(
            TEXTURE_LOADER_CALLEE,
            u32,
            texture_manager,
            texture_id
        );

        let object_vtable = read_word(this);
        let setter_entry = read_word(object_vtable + TEXTURE_SETTER_SLOT);
        let set_texture: unsafe extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(setter_entry as usize);
        set_texture(this, texture)
    }
});
