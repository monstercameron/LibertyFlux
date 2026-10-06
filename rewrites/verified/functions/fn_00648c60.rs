// original: 0x00648c60 rage::rmPtfxShaderVar_Float::vf1
/// Push this float variable's level into a target slot through the writer.
///
/// Loads the level from this object and the source word beside its header,
/// then calls the writer with the target's dispatch word in the object
/// register, the target field address, the source word, a pointer to a
/// frame slot holding the level, and the constant words 4, 1, 2.
/// Returns the writer's answer.
export!(thiscall, rw_00648c60(this_: *const u8, target: *const u8) -> u32 {
    unsafe {
        let level = *(this_.add(0x20) as *const f32);
        let source = *(this_.add(0x0c) as *const u32);
        let mut slot = level;
        let dispatch = *(target.add(0x18) as *const u32);
        callee_thiscall!(
            1,
            u32,
            dispatch,
            target.add(0x14) as u32,
            source,
            &mut slot as *mut f32 as u32,
            4,
            1,
            2
        )
    }
});
