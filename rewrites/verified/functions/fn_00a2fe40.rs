// original: 0x00A2FE40 CEntity::vf43

/// Detaches an entity's physics instance when the instance is initialized.
///
/// The thiscall entry receives the entity pointer in ECX. It reads the
/// instance pointer at entity byte offset 0x38 and returns that pointer in EAX
/// when the instance or its status word is zero. Otherwise it calls the
/// entity vtable's byte slot 0xB4 with the entity and integer one, then calls
/// the detach helper with the current global manager, the instance, and zero.
/// The helper's return value remains in EAX. The pointer width and vtable
/// layout used here match this 32-bit game structure.
lf_checker_rt::export!(thiscall, rw_00a2fe40(entity: u32) -> u32 {
    const INSTANCE_WORD: usize = 14;
    const VTABLE_SLOT_WORD: usize = 45;
    const DETACH_MANAGER: u32 = 0x012B_9C7C;
    const VTABLE_METHOD: u32 = 1;
    const DETACH_HELPER: u32 = 2;

    unsafe {
        let entity_words = entity as *const u32;
        let instance = entity_words.add(INSTANCE_WORD).read_unaligned();
        if instance == 0 {
            return 0;
        }
        let instance_status = (instance as *const u32).add(1).read_unaligned();
        if instance_status == 0 {
            return instance;
        }

        let vtable = entity_words.read_unaligned();
        let method_address = (vtable as *const u32)
            .add(VTABLE_SLOT_WORD)
            .read_unaligned();
        let method: unsafe extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(method_address as usize);
        let _method_result = method(entity, 1);

        let manager = lf_checker_rt::global::<u32>(DETACH_MANAGER).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            DETACH_HELPER,
            u32,
            manager,
            instance,
            0u32
        )
    }
});
