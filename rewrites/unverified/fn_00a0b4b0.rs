// original: 0x00a0b4b0 entity_apply_room_key
/// Apply a room key to an entity. The kind is the four-bit field at byte
/// offset 0x28, bits 6 through 9. Kinds 2 and 3 configure the embedded audio
/// interface and then notify its two helpers and the entity vtable. Kind 4
/// looks up the supplied handle in the room pool, releases the streaming
/// entity, optionally resolves the context, updates entity timing, and
/// advances streaming state. Other kinds return without calls. The lookup
/// result is treated as a sentinel only when it is exactly `u32::MAX`.
/// Stdcall with three stack words; the return value is unspecified.
lf_checker_rt::export!(stdcall, rw_00a0b4b0(
    entity: u32,
    context: u32,
    handle: u32
) -> () {
    unsafe {
        const KIND_OFFSET: u32 = 0x28;
        const ROOM_DATA_OFFSET: u32 = 0x20;
        const AUDIO_INTERFACE_OFFSET: u32 = 0x80;
        const ENTITY_VTABLE_SLOT: u32 = 0x18;
        const ROOM_POOL_VA: u32 = 0x012fb214;

        const POOL_GET_AT_HANDLE: u32 = 0;
        const RELEASE_STREAMING_ENTITY: u32 = 1;
        const POOL_FORWARD_LOOKUP: u32 = 2;
        const SET_STATE_TIMING: u32 = 3;
        const ADVANCE_STREAMING: u32 = 4;
        const AUDIO_CONFIGURE_TABLE: u32 = 5;
        const AUDIO_CONFIGURE: u32 = 6;
        const AUDIO_RESOLVE_VOICE: u32 = 7;
        const ENTITY_VTABLE_ACTION: u32 = 8;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let kind = (read_u32(entity.wrapping_add(KIND_OFFSET)) >> 6) & 0x0f;

        if kind == 2 || kind == 3 {
            let room_data = read_u32(entity.wrapping_add(ROOM_DATA_OFFSET));
            let audio_context = if room_data == 0 {
                entity.wrapping_add(0x10)
            } else {
                room_data.wrapping_add(0x30)
            };
            let interface_field = entity.wrapping_add(AUDIO_INTERFACE_OFFSET);
            let interface_object = read_u32(interface_field);
            let configure_target = read_u32(interface_object.wrapping_add(4));
            let configure_table: unsafe extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(configure_target);
            let _ = configure_table(interface_field, audio_context, 0);

            let _ = lf_checker_rt::callee_thiscall!(
                AUDIO_CONFIGURE,
                u32,
                interface_field,
                context,
                1
            );
            let _ = lf_checker_rt::callee_thiscall!(
                AUDIO_RESOLVE_VOICE,
                u32,
                interface_field,
                context
            );

            let vtable = read_u32(entity);
            let action_target = read_u32(vtable.wrapping_add(ENTITY_VTABLE_SLOT));
            let entity_action: unsafe extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(action_target);
            let _ = entity_action(entity);
        } else if kind == 4 {
            let pool_pointer = lf_checker_rt::global::<u32>(ROOM_POOL_VA).read();
            let pool_entity = lf_checker_rt::callee_thiscall!(
                POOL_GET_AT_HANDLE,
                u32,
                pool_pointer,
                handle
            );
            if pool_entity == 0 {
                return;
            }

            let _ = lf_checker_rt::callee_cdecl!(
                RELEASE_STREAMING_ENTITY,
                u32,
                entity,
                1
            );

            let mut resolved_index = 0;
            if context != 0 {
                let lookup = lf_checker_rt::callee_thiscall!(
                    POOL_FORWARD_LOOKUP,
                    u32,
                    pool_entity,
                    context
                );
                resolved_index = if lookup == u32::MAX { 0 } else { lookup };
            }

            let _ = lf_checker_rt::callee_thiscall!(
                SET_STATE_TIMING,
                u32,
                entity,
                handle,
                resolved_index,
                u32::MAX
            );
            let _ = lf_checker_rt::callee_cdecl!(ADVANCE_STREAMING, u32, entity, 1);
        }
    }
});
