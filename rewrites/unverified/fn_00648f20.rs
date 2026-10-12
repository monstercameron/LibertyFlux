// original: 0x00648f20 rage::rmPtfxShaderVar_Texture::vf1

/// Push the texture variable into a parameter block through the shared
/// writer: the block cursor is `param + 0x14`, the target object is at
/// `param + 0x18`, sent with the binding count at `this + 0x0c` and the
/// texture entry at `this + 0x20`. Returns the writer's result. Thiscall,
/// one stack argument.
lf_checker_rt::export!(thiscall, rw_00648f20(this: u32, param: u32) -> u32 {
    unsafe {
        const BINDING_COUNT: u32 = 0x0c;
        const TEXTURE_ENTRY: u32 = 0x20;
        const BLOCK_CURSOR: u32 = 0x14;
        const TARGET_OBJECT: u32 = 0x18;
        const WRITER_CALLEE: u32 = 1;
        let entry =
            ((this.wrapping_add(TEXTURE_ENTRY)) as *const u32).read_unaligned();
        let count =
            ((this.wrapping_add(BINDING_COUNT)) as *const u32).read_unaligned();
        let block = param.wrapping_add(BLOCK_CURSOR);
        let target =
            ((param.wrapping_add(TARGET_OBJECT)) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(WRITER_CALLEE, u32, target, block, count, entry)
    }
});
