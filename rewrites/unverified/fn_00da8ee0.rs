// original: 0x00DA8EE0 CTaskComplexStuckInAir::vf18

/// Choose the first task for a stuck-in-air task from the active subtask kind
/// and request context. The virtual query supplies an unsigned task-kind ID.
/// Kinds `0xCB`, `0xD0`, `0xD3`, and `0x38E` select helper requests; the
/// context's 16-bit state at `+0x224 -> +0x244` and optional target at `+0x228`
/// choose among them. Other kinds return null. The helper is stdcall with two
/// stack arguments, pushed as `(kind, request)` in that order.
lf_checker_rt::export!(thiscall, rw_00DA8EE0(this: u32, request: u32) -> u32 {
    unsafe {
        const SUBTASK: u32 = 0x08;
        const TYPE_QUERY_SLOT: u32 = 0x0C;
        const STATE_REFERENCE: u32 = 0x224;
        const STATE_VALUE: u32 = 0x244;
        const OPTIONAL_TARGET: u32 = 0x228;
        const TARGET_FIELD: u32 = 0x70;
        const KIND_CB: u32 = 0xCB;
        const KIND_D0: u32 = 0xD0;
        const KIND_D3: u32 = 0xD3;
        const KIND_38E: u32 = 0x38E;
        const REQUEST_HELPER: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn state_is_zero(request: u32) -> bool {
            let state_block = unsafe { read_u32(request.wrapping_add(STATE_REFERENCE)) };
            unsafe { read_u16(state_block.wrapping_add(STATE_VALUE)) == 0 }
        }

        #[inline(always)]
        unsafe fn request_task(kind: u32, request: u32) -> u32 {
            lf_checker_rt::callee_stdcall!(REQUEST_HELPER, u32, kind, request)
        }

        let subtask = read_u32(this.wrapping_add(SUBTASK));
        let vtable = read_u32(subtask);
        let query_address = read_u32(vtable.wrapping_add(TYPE_QUERY_SLOT));
        let query: unsafe extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(query_address as usize);
        let task_kind = query(subtask);
        match task_kind {
            KIND_D0 => {
                let state_zero = state_is_zero(request);
                request_task(if state_zero { 0x516 } else { KIND_CB }, request)
            }
            KIND_CB => {
                let state_zero = state_is_zero(request);
                if state_zero {
                    request_task(0x516, request)
                } else {
                    let target = read_u32(request.wrapping_add(OPTIONAL_TARGET));
                    if target != 0 && target.wrapping_add(TARGET_FIELD) != 0 {
                        request_task(KIND_CB, request)
                    } else {
                        request_task(KIND_D3, request)
                    }
                }
            }
            KIND_D3 => {
                let state_zero = state_is_zero(request);
                if !state_zero {
                    request_task(KIND_CB, request)
                } else {
                    let target = read_u32(request.wrapping_add(OPTIONAL_TARGET));
                    if target != 0 && target.wrapping_add(TARGET_FIELD) != 0 {
                        request_task(0x516, request)
                    } else {
                        request_task(KIND_38E, request)
                    }
                }
            }
            KIND_38E => {
                let state_zero = state_is_zero(request);
                request_task(if state_zero { 0x516 } else { KIND_CB }, request)
            }
            _ => 0,
        }
    }
});
