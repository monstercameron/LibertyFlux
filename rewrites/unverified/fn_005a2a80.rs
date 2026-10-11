// original: 0x005A2A80 mainloop_frame_update

/// Advance one main-loop frame's timing and resource state.
///
/// The routine has no explicit parameters. It runs the frame hooks in order,
/// publishes sampled values to shared state, marks the current owner object,
/// processes two bounded resource registries, and performs optional cleanup
/// and notification work selected by global flags. Calls into other engine
/// routines are intercepted by the checker and receive the same scripted
/// answers on both sides.
///
/// The registry records use a base pointer, a flag-byte array, a signed count,
/// a row stride, a lowest-pending index, and a remaining-item count. The
/// cleanup callbacks run only for a matching, unmarked row. A pending-task
/// scan compares each pointed-to record's generation before dispatching it.
///
/// This implementation reads and writes globals through the worker's
/// relocated image base. It returns the last frame hook's EAX value.
lf_checker_rt::export!(cdecl, rw_005A2A80() -> u32 {
    unsafe {
        const OWNER_CONTEXT: u32 = 0x019D2EE0;
        const SERVICE_CONTEXT: u32 = 0x011737D0;
        const SERVICE_OBJECT: u32 = 0x01981A4C;
        const OWNER_OBJECT: u32 = 0x0118D808;
        const SERVICE_OBJECT_GLOBAL: u32 = 0x0118D804;
        const REGISTRY_GLOBAL: u32 = 0x011764C0;
        const RESOURCE_GLOBAL: u32 = 0x018B6E84;
        const CLEANUP_GATE: u32 = 0x01160C3A;
        const REFRESH_GATE: u32 = 0x01160C39;
        const OWNER_NOTIFY_GATE: u32 = 0x018B6C93;
        const INIT_MODE_GLOBAL: u32 = 0x011F7060;
        const INIT_STATE_GLOBAL: u32 = 0x012088B4;
        const INIT_STAGE_GLOBAL: u32 = 0x01037720;
        const FLOAT_SOURCE: u32 = 0x0106C290;
        const FLOAT_SAMPLE: u32 = 0x018B6E9C;
        const FLOAT_PUBLISHED: u32 = 0x0118F4B8;
        const FLOAT_TICK: u32 = 0x011609CC;
        const VECTOR_SOURCE: u32 = 0x01BB37B0;
        const VECTOR_PUBLISHED: u32 = 0x01191260;
        const RESOURCE_COUNT_GLOBAL: u32 = 0x018B6C84;
        const RESOURCE_ARRAY_GLOBAL: u32 = 0x0118D7F0;
        const RESOURCE_ARRAY_LENGTH: u32 = 0x0118D7F4;
        const NOTIFY_A: u32 = 0x01160C35;
        const NOTIFY_B: u32 = 0x01160C36;
        const NOTIFY_KIND: u32 = 0x01160C40;
        const STATE_BYTE: u32 = 0x018E51CC;
        const STATE_GATE: u32 = 0x01160C3D;
        const DEFERRED_CLEAR: u32 = 0x018B6C8C;
        const REFRESH_SOURCE: u32 = 0x0105C67E;
        const REFRESH_DEST: u32 = 0x011A28F0;
        const REFRESH_CLEAR: u32 = 0x0105C6B6;
        const OUTPUT_RESOURCE_COUNT: u32 = 0x0106C2BC;
        const OUTPUT_RESOURCE_LIMIT: u32 = 0x0106C2C4;
        const OWNER_FLAG: u32 = 0x0118F4BC;
        const RELEASE_CONTEXT: u32 = 0x01032F58;
        const TASK_DESCRIPTOR_A: u32 = 0x0114E350;
        const TASK_DESCRIPTOR_B: u32 = 0x0114E3A0;

        #[inline(always)]
        unsafe fn read32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn read16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn global32(file_address: u32) -> u32 {
            unsafe { read32(lf_checker_rt::relocated(file_address)) }
        }
        #[inline(always)]
        unsafe fn global8(file_address: u32) -> u8 {
            unsafe { read8(lf_checker_rt::relocated(file_address)) }
        }
        #[inline(always)]
        unsafe fn set_global32(file_address: u32, value: u32) {
            unsafe { write32(lf_checker_rt::relocated(file_address), value) }
        }
        #[inline(always)]
        unsafe fn set_global8(file_address: u32, value: u8) {
            unsafe { write8(lf_checker_rt::relocated(file_address), value) }
        }
        #[inline(always)]
        unsafe fn call_method0(receiver: u32, slot_offset: u32) -> u32 {
            let table = unsafe { read32(receiver) };
            let target = unsafe { read32(table.wrapping_add(slot_offset)) };
            let method: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            method(receiver)
        }
        #[inline(always)]
        unsafe fn call_method1(receiver: u32, slot_offset: u32, argument: u32) -> u32 {
            let table = unsafe { read32(receiver) };
            let target = unsafe { read32(table.wrapping_add(slot_offset)) };
            let method: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            method(receiver, argument)
        }

        macro_rules! sweep_registry {
            ($handle:expr, $release_id:literal, $notify_id:literal, $destroy_id:literal) => {{
                let mut registry = global32(REGISTRY_GLOBAL);
                let row_count = read32(registry.wrapping_add(8)) as i32;
                if row_count > 0 {
                    let flags = read32(registry.wrapping_add(4));
                    let rows = read32(registry);
                    let stride = read32(registry.wrapping_add(12));
                    let mut index = 0u32;
                    let mut matched = false;
                    while (index as i32) < row_count {
                        if read8(flags.wrapping_add(index)) & 0x80 == 0 {
                            let row = rows.wrapping_add(index.wrapping_mul(stride));
                            if row != 0 && read32(row.wrapping_add(8)) == $handle {
                                matched = true;
                                break;
                            }
                        }
                        index = index.wrapping_add(1);
                    }
                    if matched && index != u32::MAX {
                        let mut row = rows.wrapping_add(index.wrapping_mul(stride));
                        let remaining = read32(row.wrapping_add(4)).wrapping_sub(1);
                        write32(row.wrapping_add(4), remaining);
                        if (remaining as i32) <= 0 {
                            let released: u32 = lf_checker_rt::callee_cdecl!(
                                $release_id,
                                u32,
                                index,
                                global32(RELEASE_CONTEXT)
                            );
                            if (released & 0xFF) == 0 {
                                let _: u32 = lf_checker_rt::callee_cdecl!($notify_id, u32, index);
                            }
                        }
                        registry = global32(REGISTRY_GLOBAL);
                        let flags = read32(registry.wrapping_add(4));
                        if read8(flags.wrapping_add(index)) & 0x80 != 0 {
                            row = 0;
                        } else {
                            let rows = read32(registry);
                            let stride = read32(registry.wrapping_add(12));
                            row = rows.wrapping_add(index.wrapping_mul(stride));
                        }
                        if read32(row) != 0 {
                            let _: u32 = lf_checker_rt::callee_cdecl!($destroy_id, u32, index);
                        }
                        let rows = read32(registry);
                        let stride = read32(registry.wrapping_add(12));
                        let flags = read32(registry.wrapping_add(4));
                        let slot = row.wrapping_sub(rows).wrapping_div(stride);
                        let flag_address = flags.wrapping_add(slot);
                        write8(flag_address, read8(flag_address) | 0x80);
                        let minimum = read32(registry.wrapping_add(16));
                        if (slot as i32) < (minimum as i32) {
                            write32(registry.wrapping_add(16), slot);
                        }
                        let left = read32(registry.wrapping_add(20)).wrapping_sub(1);
                        write32(registry.wrapping_add(20), left);
                    }
                }
            }};
        }

        // The first hook carries the saved register context and refreshes
        // optional frame state before the regular service callbacks begin.
        set_global8(0x018B6E80, 0);
        let frame_context = 0u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            lf_checker_rt::relocated(OWNER_CONTEXT),
            0,
            0,
            &frame_context as *const u32 as u32,
            0
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            lf_checker_rt::relocated(SERVICE_CONTEXT),
            0
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
        let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, 0);
        let _: u32 = lf_checker_rt::callee_cdecl!(5, u32,);
        if global8(REFRESH_GATE) != 0 {
            set_global8(REFRESH_DEST, global8(REFRESH_SOURCE));
            set_global8(REFRESH_CLEAR, 0);
            let _: u32 = lf_checker_rt::callee_cdecl!(6, u32,);
            set_global8(REFRESH_GATE, 0);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(7, u32,);
        let service_object = global32(SERVICE_OBJECT_GLOBAL);
        let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, service_object);

        let owner_object = global32(OWNER_OBJECT);
        let _: u32 = call_method0(owner_object, 8);
        if global32(INIT_MODE_GLOBAL) != 1
            && global32(INIT_STATE_GLOBAL) == u32::MAX
            && global32(INIT_STAGE_GLOBAL) != 0x12
        {
            let selection = global32(0x0118EA08);
            let entry_table = lf_checker_rt::relocated(0x0118E7F8);
            let selected = read32(entry_table.wrapping_add(selection.wrapping_mul(4)));
            let mut prepared_scalar = 0x3F51EB85u32;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                10,
                u32,
                0,
                0,
                selected.wrapping_add(0x18),
                selected.wrapping_add(0x20),
                &mut prepared_scalar as *mut u32 as u32
            );
        }
        let _: u32 = call_method0(owner_object, 4);
        let _: u32 = lf_checker_rt::callee_stdcall!(12, u32, global32(FLOAT_SOURCE), 0);
        set_global32(FLOAT_PUBLISHED, global32(FLOAT_SAMPLE));
        set_global32(FLOAT_TICK, global32(FLOAT_SOURCE));
        for word in 0..4u32 {
            let value = global32(VECTOR_SOURCE.wrapping_add(word * 4));
            set_global32(VECTOR_PUBLISHED.wrapping_add(word * 4), value);
        }
        let owner_flags = owner_object.wrapping_add(0x558);
        write8(owner_flags, read8(owner_flags) | 1);
        set_global8(OWNER_FLAG, 1);

        let cleanup_requested = global8(CLEANUP_GATE) != 0;
        let pending_resource = global32(RESOURCE_GLOBAL);
        set_global32(0x0118E90C, global32(0x018B6DA8));
        set_global32(0x0118E910, global32(0x018B6DAC));
        set_global32(0x0118E914, global32(0x018B6DB0));
        set_global32(0x0118E918, global32(0x018B6DB4));
        set_global32(0x0118E91C, global32(0x018B6DB8));
        set_global8(0x011609F7, 0);
        if cleanup_requested {
            set_global8(CLEANUP_GATE, 0);
            if pending_resource != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(13, u32, pending_resource);
                let tls_root = lf_checker_rt::tls_slot(0);
                let finalizer = read32(tls_root.wrapping_add(8));
                let _: u32 = call_method1(finalizer, 12, pending_resource);
                set_global32(RESOURCE_GLOBAL, 0);
            }
        }
        if global8(OWNER_NOTIFY_GATE) != 0 {
            if global8(0x01160C33) == 0
                && global32(INIT_MODE_GLOBAL) != 1
                && global32(INIT_STATE_GLOBAL) == u32::MAX
                && global32(INIT_STAGE_GLOBAL) != 0x12
            {
                let _: u32 = lf_checker_rt::callee_cdecl!(15, u32,);
            }
            set_global8(OWNER_NOTIFY_GATE, 0);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(16, u32,);

        let first_handle: u32 = lf_checker_rt::callee_cdecl!(
            17,
            u32,
            lf_checker_rt::relocated(0x018B6E00)
        );
        sweep_registry!(first_handle, 18, 19, 20);
        let second_handle: u32 = lf_checker_rt::callee_cdecl!(
            21,
            u32,
            lf_checker_rt::relocated(0x00F87810)
        );
        sweep_registry!(second_handle, 22, 23, 24);

        let pending_count = global32(RESOURCE_COUNT_GLOBAL) as i32;
        if pending_count > 0 {
            let entry_count = read16(lf_checker_rt::relocated(RESOURCE_ARRAY_LENGTH)) as u32;
            let entries = global32(RESOURCE_ARRAY_GLOBAL);
            let mut matching_entry = 0u32;
            let mut index = 0u32;
            while index < entry_count {
                let record = read32(entries.wrapping_add(index.wrapping_mul(4)));
                if read32(record.wrapping_add(0x53C)) == pending_count as u32 {
                    matching_entry = record;
                    break;
                }
                index = index.wrapping_add(1);
            }
            let handled: u32 = lf_checker_rt::callee_cdecl!(
                25,
                u32,
                matching_entry,
                0,
                lf_checker_rt::relocated(TASK_DESCRIPTOR_A),
                lf_checker_rt::relocated(TASK_DESCRIPTOR_B),
                0
            );
            if handled != 0 {
                let target = read32(handled.wrapping_add(0x764));
                write8(target.wrapping_add(0x20), 0);
            }
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            26,
            u32,
            lf_checker_rt::relocated(0x011F6310)
        );

        let resource_after_scan = global32(RESOURCE_GLOBAL);
        if resource_after_scan != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(27, u32, resource_after_scan);
            let tls_root = lf_checker_rt::tls_slot(0);
            let finalizer = read32(tls_root.wrapping_add(8));
            let _: u32 = call_method1(finalizer, 12, resource_after_scan);
            set_global32(RESOURCE_GLOBAL, 0);
        }
        let first_query: u32 = lf_checker_rt::callee_cdecl!(29, u32,);
        if first_query != 0 {
            let second_query: u32 = lf_checker_rt::callee_cdecl!(30, u32,);
            let payload = read32(second_query.wrapping_add(0x228));
            let flags = read32(payload.wrapping_add(0x4C8));
            write32(payload.wrapping_add(0x4C8), flags & !0x400);
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            31,
            u32,
            lf_checker_rt::relocated(0x0128E94C)
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(32, u32, 0);
        let _: u32 = lf_checker_rt::callee_cdecl!(33, u32, 1);
        let _: u32 = lf_checker_rt::callee_cdecl!(34, u32,);

        let mut state = global8(STATE_BYTE);
        if global8(STATE_GATE) != 0 {
            state = 0;
        }
        set_global8(STATE_BYTE, state);
        if global8(NOTIFY_A) != 0 || global8(NOTIFY_B) != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                35,
                u32,
                lf_checker_rt::relocated(SERVICE_CONTEXT),
                0
            );
            let notification: u32 = lf_checker_rt::callee_thiscall!(
                36,
                u32,
                lf_checker_rt::relocated(SERVICE_OBJECT),
                0x3FAA283B
            );
            if notification != 0 {
                let _: u32 = call_method1(notification, 8, 1);
                set_global32(DEFERRED_CLEAR, 0);
                set_global8(NOTIFY_B, 0);
            }
        }
        if global32(NOTIFY_KIND) == 0x39 {
            set_global8(0x018B6E8D, 0);
            set_global8(0x018B6E97, 0);
            set_global8(0x018B6E94, 0);
            let ready: u32 = lf_checker_rt::callee_thiscall!(
                38,
                u32,
                lf_checker_rt::relocated(SERVICE_OBJECT),
                0x3FAA283B
            );
            if (ready & 0xFF) != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    39,
                    u32,
                    lf_checker_rt::relocated(SERVICE_CONTEXT),
                    0
                );
                let notification: u32 = lf_checker_rt::callee_thiscall!(
                    40,
                    u32,
                    lf_checker_rt::relocated(SERVICE_OBJECT),
                    0x3FAA283B
                );
                if notification != 0 {
                    let _: u32 = call_method1(notification, 8, 1);
                }
            }
        }
        set_global32(OUTPUT_RESOURCE_COUNT, 0x49);
        set_global32(OUTPUT_RESOURCE_LIMIT, 0x49);
        lf_checker_rt::callee_cdecl!(42, u32,)
    }
});
