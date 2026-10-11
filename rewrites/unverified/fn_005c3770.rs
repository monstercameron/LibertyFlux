// original: 0x005C3770 CPedFactoryNY::vf5

/// Creates a ped task through the factory's dispatch slot, then marks and
/// optionally finalizes the returned task object.
///
/// `this` is a factory object whose vtable entry at `+0x18` is used by the
/// failed-transition path. `input` points to a small request record and its
/// byte at `+1` rejects the request when nonzero. `task_index` is signed for
/// the initial negative check; nonnegative values index the global task
/// descriptor table. A descriptor's kind at `+0x90` selects the factory
/// routine. `options` contributes its low byte to the optional transition
/// call. The final two stack words are accepted by the ABI but unused.
///
/// A successful factory result has bit 0 set in its flags at `+0x260`. The
/// function may then run a transition check and dispatch the factory's
/// `+0x18` vtable entry, or call the task finalizer. If the finalizer's float
/// result equals the configured cutoff and it accepts the object, byte
/// `+0x63` is cleared. It returns the created object, or zero for rejected, missing, or
/// destroyed objects. The signed index check precedes the table access.
///
/// Original ABI: thiscall with five stack words; the callee removes all five.
lf_checker_rt::export!(thiscall, rw_005c3770(
    this: u32,
    input: u32,
    _request_kind: u32,
    task_index: u32,
    options: u32,
    _unused: u32,
) -> u32 {
    unsafe {
        const TASK_TABLE_VA: u32 = 0x1295_CD8;
        const TASK_KIND: u32 = 0x90;
        const CREATED_FLAGS: u32 = 0x260;
        const CREATED_VALUE: u32 = 0x21C;
        const CREATED_STATE: u32 = 0x63;
        const FACTORY_TRANSITION_SLOT: u32 = 0x18;
        const REQUEST_CONFIG_VA: u32 = 0x17F5_FC7;
        const TRANSITION_SERVICE_VA: u32 = 0x1908_EF0;
        const TASK_FINALIZER_VA: u32 = 0x128E_310;
        const FLOAT_CUTOFF_VA: u32 = 0x00FE_8628;
        const KIND_SPECIAL: u32 = 2;
        const MODE_OFF: u8 = 0;
        const MODE_LOW: u32 = 8;
        const MODE_HIGH: u32 = 10;
        const REQUIRED_FLAG: u32 = 1;

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
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        if read_u8(input.wrapping_add(1)) != 0 {
            return 0;
        }

        let signed_task_index = task_index as i32;
        if signed_task_index < 0 {
            return 0;
        }

        let table_entry_va = TASK_TABLE_VA.wrapping_add(task_index.wrapping_mul(4));
        let task_descriptor = read_u32(lf_checker_rt::relocated(table_entry_va));
        if task_descriptor == 0 {
            return 0;
        }

        // The original clears the low half of its second incoming word before
        // forwarding that word. Keep that value explicit in the rewrite.
        let forwarded_request_kind = _request_kind & 0xFFFF_0000;
        let mut input_slot_value = input;
        let input_slot = (&mut input_slot_value as *mut u32) as u32;
        let task_kind = read_u32(task_descriptor.wrapping_add(TASK_KIND));

        let resolved_task = if task_kind == KIND_SPECIAL {
            lf_checker_rt::callee_thiscall!(2, u32, this, input_slot)
        } else {
            lf_checker_rt::callee_thiscall!(1, u32, this, input_slot, task_index)
        };
        if resolved_task == 0 {
            return 0;
        }

        let mut created = if task_kind == KIND_SPECIAL {
            lf_checker_rt::callee_thiscall!(4, u32, this, task_index, forwarded_request_kind, resolved_task)
        } else {
            lf_checker_rt::callee_thiscall!(3, u32, this, task_index, forwarded_request_kind, resolved_task)
        };
        if created == 0 {
            return 0;
        }

        let old_flags = read_u32(created.wrapping_add(CREATED_FLAGS));
        write_u32(created.wrapping_add(CREATED_FLAGS), old_flags | REQUIRED_FLAG);

        if task_index as u8 != 0 {
            let may_transition = lf_checker_rt::callee_cdecl!(5, u32,) as u8 != 0;
            if may_transition {
                let option_byte = options as u8;
                let transition_mode = if option_byte == MODE_OFF {
                    0
                } else {
                    let request_config = lf_checker_rt::relocated(REQUEST_CONFIG_VA);
                    if read_u8(request_config) != 0 { MODE_HIGH } else { MODE_LOW }
                };
                let transition_ok = lf_checker_rt::callee_thiscall!(
                    6,
                    u32,
                    lf_checker_rt::relocated(TRANSITION_SERVICE_VA),
                    created,
                    0,
                    transition_mode,
                    0,
                ) as u8 != 0;

                if !transition_ok && option_byte == MODE_OFF {
                    let factory_vtable = read_u32(this);
                    let dispatch = read_u32(factory_vtable.wrapping_add(FACTORY_TRANSITION_SLOT));
                    let transition: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(dispatch as usize);
                    let _ = transition(this, created);
                    created = 0;
                }
            }
        }

        if created != 0 {
            let value = read_u32(created.wrapping_add(CREATED_VALUE));
            lf_checker_rt::callee_cdecl!(
                7,
                u32,
                created,
                created.wrapping_add(0x80),
                value,
                0,
                0,
                0,
            );
        }

        let finalized_value: f32 = lf_checker_rt::callee_thiscall!(
            8,
            f32,
            lf_checker_rt::relocated(TASK_FINALIZER_VA),
        );
        let cutoff = f32::from_bits(read_u32(lf_checker_rt::relocated(FLOAT_CUTOFF_VA)));
        if created != 0 && finalized_value == cutoff {
            let clear_state = lf_checker_rt::callee_thiscall!(9, u32, created, 0) as u8 != 0;
            if clear_state {
                write_u8(created.wrapping_add(CREATED_STATE), 0);
            }
        }

        created
    }
});
