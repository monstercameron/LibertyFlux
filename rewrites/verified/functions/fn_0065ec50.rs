// original: 0x0065EC50 rage::snMsgJoinResponse::AutoIdDesc__::AutoIdDesc__

/// Registers the static auto-ID descriptor for `rage::snMsgJoinResponse::AutoIdDesc__::AutoIdDesc__` and returns
/// the address of its descriptor record. The record stores its schema
/// version, message ID, message name, registry table, prior registry link,
/// and a one-byte registration flag. The helper returns a registry node;
/// its prior link is saved back into the descriptor, both registration
/// counters advance with 32-bit wrapping, and the unsigned high-water mark
/// becomes `message_id + 1` only when the ID exceeds the previous mark.
///
/// This is a cdecl function with no stack arguments. The registry helper is
/// intercepted by the checker; its scripted return points to a 32-bit node
/// whose words at offsets 0, 8, 12, and 16 are the link and counters.
lf_checker_rt::export!(cdecl, rw_0065ec50() -> u32 {
    unsafe {
        const DESCRIPTOR_VA: u32 = 0x019f09d4;
        const REGISTERED_FLAG_VA: u32 = 0x019f09f0;
        const SCHEMA_VERSION_VA: u32 = 0x019f09d8;
        const MESSAGE_ID_VA: u32 = 0x019f09dc;
        const MESSAGE_NAME_VA: u32 = 0x019f09e4;
        const REGISTRY_TABLE_VA: u32 = 0x019f09e8;
        const PREVIOUS_NODE_VA: u32 = 0x019f09ec;
        const INITIALIZER_SLOT_VA: u32 = 0x00fcaff8;
        const MESSAGE_ID: u32 = 0x0000002a;
        const MESSAGE_NAME: u32 = 0x00fe306c;
        const TYPE_NAME: u32 = 0x00fe321c;
        const REGISTRY_TABLE: u32 = 0x019f3984;
        const FLAG_REGISTERED: u8 = 1;
        let flag = lf_checker_rt::global::<u8>(REGISTERED_FLAG_VA);

        #[repr(C)]
        struct RegistryNode {
            previous: u32,
            reserved: u32,
            descriptor_count: u32,
            type_count: u32,
            next_message_id: u32,
        }

        flag.write(flag.read() | FLAG_REGISTERED);
        lf_checker_rt::global::<u32>(DESCRIPTOR_VA)
            .write(lf_checker_rt::relocated(INITIALIZER_SLOT_VA));
        lf_checker_rt::global::<u32>(SCHEMA_VERSION_VA).write(1);
        lf_checker_rt::global::<u32>(MESSAGE_ID_VA).write(MESSAGE_ID);
        lf_checker_rt::global::<u32>(MESSAGE_NAME_VA)
            .write(lf_checker_rt::relocated(MESSAGE_NAME));
        lf_checker_rt::global::<u32>(REGISTRY_TABLE_VA)
            .write(lf_checker_rt::relocated(REGISTRY_TABLE));
        lf_checker_rt::global::<u32>(PREVIOUS_NODE_VA).write(0);

        let node_address = lf_checker_rt::callee_cdecl!(1, u32,);
        let node = &mut *(node_address as *mut RegistryNode);
        lf_checker_rt::global::<u32>(PREVIOUS_NODE_VA).write(node.previous);
        node.previous = lf_checker_rt::relocated(DESCRIPTOR_VA);
        if MESSAGE_ID > node.next_message_id {
            node.next_message_id = MESSAGE_ID.wrapping_add(1);
        }
        node.descriptor_count = node.descriptor_count.wrapping_add(1);
        node.type_count = node.type_count.wrapping_add(1);

        lf_checker_rt::global::<u32>(DESCRIPTOR_VA).write(lf_checker_rt::relocated(TYPE_NAME));
        lf_checker_rt::relocated(DESCRIPTOR_VA)
    }
});
