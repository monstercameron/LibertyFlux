// original: 0x00CC14A0 update_anim_blend_state

/// Partial proof stage for `update_anim_blend_state`.
///
/// This stage covers the entity-gated update path when the entity's blend
/// control bit is set, the required-task helper reports false, and the task
/// tick reports complete. It clamps the current blend value to the object's
/// maximum, runs the aim update and task tick callbacks, sets the active-blend
/// flag on completion, and returns true. The remaining route, euphoria,
/// camera, and descriptor paths are not represented by this stage.
///
/// The object uses 32-bit fields because this checked entry point is a
/// 32-bit `thiscall`; offsets are relative to the pointed-to object.
lf_checker_rt::export!(thiscall, rw_00CC14A0(this: u32, task: u32, _update_delta_bits: u32) -> u32 {
    unsafe {
        const ENTITY_POINTER: u32 = 0x24;
        const ENTITY_BLEND_CONTROL: u32 = 0x118;
        const ENTITY_TASK_MASKS: u32 = 0x268;
        const ENTITY_TASK_LINK: u32 = 0x6c;
        const LINK_ACTIVE_BYTE: u32 = 0x0e;
        const CURRENT_BLEND: u32 = 0x08;
        const SECONDARY_BLEND: u32 = 0x10;
        const MAXIMUM_BLEND: u32 = 0x70;
        const BLEND_FLAGS: u32 = 0x50;
        const REQUIRED_TASK_MASK: u32 = 0x0000_0800;
        const ACTIVE_BLEND_FLAG: u32 = 0x0004_0000;
        const UNIT: f32 = f32::from_bits(0x3f80_0000);

        const HAS_REQUIRED_TASK: u32 = 1;
        const TASK_AIM_UPDATE: u32 = 2;
        const PED_TASK_TICK_A: u32 = 3;
        const SECURITY_COOKIE: u32 = 4;

        #[inline(always)]
        unsafe fn field(base: u32, offset: u32) -> u32 {
            base.wrapping_add(offset)
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let entity = read_u32(field(this, ENTITY_POINTER));

        // The complementary speed-easing branch makes an additional math
        // call and continues into later logic. This stage proves the set-bit
        // branch only; the contract pins that bit for every completed trial.
        if read_u8(field(entity, ENTITY_BLEND_CONTROL)) & 1 == 0 {
            return 0;
        }

        let has_required_task =
            lf_checker_rt::callee_cdecl!(HAS_REQUIRED_TASK, u32, entity);
        if has_required_task != 0
            && read_u32(field(entity, ENTITY_TASK_MASKS)) & REQUIRED_TASK_MASK == 0
        {
            let current = f32::from_bits(read_u32(field(this, CURRENT_BLEND)));
            if !(UNIT > current) {
                write_u32(field(this, CURRENT_BLEND), UNIT.to_bits());
            }

            let linked_task = read_u32(field(entity, ENTITY_TASK_LINK));
            if linked_task == 0 || read_u8(field(linked_task, LINK_ACTIVE_BYTE)) == 0 {
                let secondary = f32::from_bits(read_u32(field(this, SECONDARY_BLEND)));
                if !(UNIT > secondary) {
                    write_u32(field(this, SECONDARY_BLEND), UNIT.to_bits());
                }
            }
        }

        let current = f32::from_bits(read_u32(field(this, CURRENT_BLEND)));
        let maximum = f32::from_bits(read_u32(field(this, MAXIMUM_BLEND)));
        if !(maximum > current) {
            write_u32(field(this, CURRENT_BLEND), maximum.to_bits());
        }

        let _aim_update =
            lf_checker_rt::callee_thiscall!(TASK_AIM_UPDATE, u32, this, task);
        let task_complete =
            lf_checker_rt::callee_thiscall!(PED_TASK_TICK_A, u32, this, task);
        if task_complete == 0 {
            // The caller continues into the unimplemented stages here. The
            // stage contract never takes this path; do not promote this file
            // as a complete rewrite until that continuation is implemented.
            return 0;
        }

        let flags = read_u32(field(this, BLEND_FLAGS));
        write_u32(field(this, BLEND_FLAGS), flags | ACTIVE_BLEND_FLAG);
        let _cookie_check = lf_checker_rt::callee_cdecl!(SECURITY_COOKIE, u32,);
        1
    }
});
