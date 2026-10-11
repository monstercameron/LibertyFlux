// original: 0x00DA3550 ped_attractor_register_item (proposed)

/// Register an item with a pedestrian attractor. The signed result of the
/// validation helper controls whether the pending item is appended to the
/// four-byte collection; the collection is then refreshed through its member
/// helpers. The object's virtual slot at `+8` selects the subsequent manager
/// path, and slot zero refines that path when slot `+8` accepts the item.
///
/// The method reads the vector fields at `+0x24..+0x2c` on the append path,
/// the object's vtable at `+0`, and the manager pointer stored in the global
/// data slot. The manager, constructors, collection helpers and virtual
/// methods are outgoing calls. It takes one 32-bit stack argument and returns
/// the low byte value one after the manager notification path completes.
lf_checker_rt::export!(thiscall, rw_00da3550(self_ptr: u32, item: u32) -> u8 {
    unsafe {
        const VTABLE_POINTER: u32 = 0x00;
        const VTABLE_SLOT_ACCEPT: u32 = 0x08;
        const VTABLE_SLOT_KIND: u32 = 0x00;
        const VECTOR_END: u32 = 0x28;
        const VECTOR_CAPACITY: u32 = 0x2c;
        const GLOBAL_MANAGER_SLOT: u32 = 0x0167_e2a0;
        const KIND_WITH_ALT_CONSTRUCTOR: u32 = 8;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let status = lf_checker_rt::callee_thiscall!(1, u32, self_ptr, item);
        if (status as i32) < 0 {
            let end = read_u32(self_ptr.wrapping_add(VECTOR_END));
            let capacity = read_u32(self_ptr.wrapping_add(VECTOR_CAPACITY));
            if end != capacity {
                if end != 0 {
                    unsafe { (end as *mut u32).write_unaligned(item) };
                }
                unsafe { ((self_ptr + VECTOR_END) as *mut u32).write_unaligned(end.wrapping_add(4)) };
            } else {
                let mut local_range = 0u32;
                let mut local_output = 0u32;
                let _ = lf_checker_rt::callee_thiscall!(
                    10,
                    u32,
                    self_ptr.wrapping_add(0x24),
                    end,
                    (&mut local_range as *mut u32) as u32,
                    (&mut local_output as *mut u32) as u32,
                    1,
                    1
                );
            }
            let _ = lf_checker_rt::callee_thiscall!(8, u32, self_ptr, item);
            let _ = lf_checker_rt::callee_thiscall!(9, u32, self_ptr);
        }

        let table = read_u32(self_ptr.wrapping_add(VTABLE_POINTER));
        let accepts = read_u32(table.wrapping_add(VTABLE_SLOT_ACCEPT));
        let accept: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(accepts as usize);
        let accepted = accept(self_ptr, item) as u8 != 0;

        let kind = if accepted {
            let target = read_u32(table.wrapping_add(VTABLE_SLOT_KIND));
            let get_kind: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            get_kind(self_ptr)
        } else {
            0
        };

        let manager = unsafe {
            (lf_checker_rt::global::<u32>(GLOBAL_MANAGER_SLOT) as *const u32).read_unaligned()
        };
        let manager_result = lf_checker_rt::callee_thiscall!(4, u32, manager);
        let notification = if manager_result == 0 {
            0
        } else if accepted && kind != KIND_WITH_ALT_CONSTRUCTOR {
            lf_checker_rt::callee_thiscall!(5, u32, manager_result)
        } else {
            let second_status = lf_checker_rt::callee_thiscall!(1, u32, self_ptr, item);
            lf_checker_rt::callee_thiscall!(6, u32, manager_result, self_ptr, second_status)
        };
        let _ = lf_checker_rt::callee_thiscall!(7, u32, self_ptr, item, notification);
        1
    }
});
