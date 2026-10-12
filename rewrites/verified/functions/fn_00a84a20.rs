// original: 0x00A84A20 CComplexGangDrivebyTaskInfo::deserialize

/// Deserializes the common task state, reads a flag, then either parses the
/// network identifier or clears it. A nonzero identifier is resolved through
/// the mission registrar and its object's vtable slot 1; the stored link is
/// detached, replaced and attached when the resolved link changes, or cleared
/// when lookup fails. The final two stream reads fill byte fields at offsets
/// 0x31 and 0x32. The method is thiscall, consumes one stream pointer, and
/// returns the final byte-reader's EAX value.
lf_checker_rt::export!(thiscall, rw_00A84A20(task_info: u32, stream: u32) -> u32 {
    const BASE_DESERIALIZE: u32 = 1;
    const READ_BYTE: u32 = 2;
    const READ_LINK_ID: u32 = 3;
    const READ_MODE_AND_FLAG: u32 = 4;
    const GET_STREAM_BASE: u32 = 5;
    const READ_MODE_VALUE: u32 = 6;
    const READ_TASK_KIND: u32 = 7;
    const FIND_NET_OBJECT: u32 = 8;
    const DETACH_LINK: u32 = 10;
    const ATTACH_LINK: u32 = 11;

    const LINK_ID_OFFSET: u32 = 0x18;
    const LINK_POINTER_OFFSET: u32 = 0x1C;
    const TASK_KIND_OFFSET: u32 = 0x20;
    const FINAL_FLAG_OFFSET: u32 = 0x31;
    const FINAL_MODE_OFFSET: u32 = 0x32;
    const TASK_KIND: u32 = 0x13;
    const MODE_BYTES: u32 = 0x20;
    const REGISTRAR_VA: u32 = 0x0190_8EF0;

    let call_link_vtable_slot_one = |receiver: u32| {
        // SAFETY: the contract plants the object's vtable pointer and the
        // slot's recorder stub in the same fabricated heap on both sides.
        unsafe {
            let table = core::ptr::read_unaligned(receiver as *const u32);
            let target = core::ptr::read_unaligned(table.wrapping_add(4) as *const u32);
            let call: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
            call(receiver)
        }
    };

    let _ = lf_checker_rt::callee_thiscall!(BASE_DESERIALIZE, u32, task_info, stream);

    // The original uses its incoming stream slot as a one-byte scratch value.
    // Initializing this word from the stream pointer preserves the snapshot
    // seen by the reader before it writes the flag.
    let mut stream_flag_word = stream;
    let _ = lf_checker_rt::callee_thiscall!(
        READ_BYTE,
        u32,
        stream,
        (&mut stream_flag_word as *mut u32 as usize) as u32,
    );

    if stream_flag_word as u8 != 0 {
        let _ = lf_checker_rt::callee_thiscall!(
            READ_LINK_ID,
            u32,
            stream,
            task_info.wrapping_add(LINK_ID_OFFSET),
        );
        let _ = lf_checker_rt::callee_thiscall!(
            READ_MODE_AND_FLAG,
            u32,
            stream,
            MODE_BYTES,
            1,
        );
        let stream_base = lf_checker_rt::callee_stdcall!(GET_STREAM_BASE, u32, 1);
        let mode_value = stream_base.wrapping_sub(0x2E);
        let _ = lf_checker_rt::callee_thiscall!(READ_MODE_VALUE, u32, stream, mode_value);
    } else {
        unsafe {
            core::ptr::write_unaligned(
                task_info.wrapping_add(LINK_ID_OFFSET) as *mut u16,
                0,
            );
        }
        let _ = lf_checker_rt::callee_thiscall!(
            READ_TASK_KIND,
            u32,
            stream,
            task_info.wrapping_add(TASK_KIND_OFFSET),
            TASK_KIND,
        );
    }

    let link_id = unsafe {
        core::ptr::read_unaligned(task_info.wrapping_add(LINK_ID_OFFSET) as *const u16)
    };
    let mut retain_existing_link = false;
    if link_id != 0 {
        let registrar = lf_checker_rt::global::<u32>(REGISTRAR_VA) as u32;
        let net_object = lf_checker_rt::callee_thiscall!(
            FIND_NET_OBJECT,
            u32,
            registrar,
            u32::from(link_id),
            0,
        );

        if net_object != 0 {
            let active = call_link_vtable_slot_one(net_object);
            if active != 0 {
                let new_link = call_link_vtable_slot_one(net_object);
                let link_field = task_info.wrapping_add(LINK_POINTER_OFFSET);
                let old_link = unsafe { core::ptr::read_unaligned(link_field as *const u32) };
                if old_link == new_link {
                    retain_existing_link = true;
                } else {
                    if old_link != 0 {
                        let _ = lf_checker_rt::callee_thiscall!(
                            DETACH_LINK,
                            u32,
                            old_link,
                            link_field,
                        );
                    }
                    let replacement = call_link_vtable_slot_one(net_object);
                    unsafe { core::ptr::write_unaligned(link_field as *mut u32, replacement) };
                    let _ = lf_checker_rt::callee_thiscall!(
                        ATTACH_LINK,
                        u32,
                        replacement,
                        link_field,
                    );
                    retain_existing_link = true;
                }
            }
        }
    }

    if !retain_existing_link {
        let link_field = task_info.wrapping_add(LINK_POINTER_OFFSET);
        let old_link = unsafe { core::ptr::read_unaligned(link_field as *const u32) };
        if old_link != 0 {
            let _ = lf_checker_rt::callee_thiscall!(DETACH_LINK, u32, old_link, link_field);
            unsafe { core::ptr::write_unaligned(link_field as *mut u32, 0) };
        }
    }

    let final_flag = task_info.wrapping_add(FINAL_FLAG_OFFSET);
    let _ = lf_checker_rt::callee_thiscall!(READ_BYTE, u32, stream, final_flag);
    let final_mode = task_info.wrapping_add(FINAL_MODE_OFFSET);
    lf_checker_rt::callee_thiscall!(READ_BYTE, u32, stream, final_mode)
});
