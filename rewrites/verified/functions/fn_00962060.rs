// original: 0x00962060 teardown_ref_owner
/// Release every owned reference held by this object, then reset its state.
///
/// The object owns up to six references, each cleared to null (or -1) after
/// release: a refcounted pointer whose count lives at offset 0x20 of the
/// target (destroyed through helper 1 and freed only when the count reaches
/// zero), a polymorphic object destroyed through its own deleting
/// destructor, two more objects destroyed through helpers 4 and 5 and freed,
/// and two slotted objects sharing one release rule (a 16-bit count at
/// offset 0xA is decremented when nonzero; reaching zero destroys through
/// the deleting destructor only for kinds 2 and 4). A handle at offset 0x18
/// is closed through helper 7 unless already -1. Three trailing dwords,
/// three trailing words, one dword and three flag bytes are then zeroed.
export!(thiscall, rw_00962060(this: u32) -> u32 {
    unsafe {
        const REFCOUNT_OFF: u32 = 0x20;
        const KIND_OFF: u32 = 8;
        const SLOT_COUNT_OFF: u32 = 0x0A;
        const NO_HANDLE: u32 = 0xFFFF_FFFF;

        let primary = *(this as *const u32);
        if primary != 0 {
            let count = (primary.wrapping_add(REFCOUNT_OFF)) as *mut u32;
            *count = (*count).wrapping_sub(1);
            if *count == 0 {
                callee_thiscall!(1, u32, primary);
                callee_cdecl!(2, u32, primary);
            }
            *(this as *mut u32) = 0;
        }

        let secondary = *((this.wrapping_add(4)) as *const u32);
        if secondary != 0 {
            fn1_deleting_dtor(secondary);
            *((this.wrapping_add(4)) as *mut u32) = 0;
        }

        let third = *((this.wrapping_add(0x10)) as *const u32);
        if third != 0 {
            callee_thiscall!(4, u32, third);
            callee_cdecl!(2, u32, third);
            *((this.wrapping_add(0x10)) as *mut u32) = 0;
        }

        let fourth = *((this.wrapping_add(0x14)) as *const u32);
        if fourth != 0 {
            callee_thiscall!(5, u32, fourth);
            callee_cdecl!(2, u32, fourth);
            *((this.wrapping_add(0x14)) as *mut u32) = 0;
        }

        for slot in [8u32, 12u32] {
            let cell = (this.wrapping_add(slot)) as *mut u32;
            let obj = *cell;
            if obj != 0 {
                let count_cell = (obj.wrapping_add(SLOT_COUNT_OFF)) as *mut u16;
                let count = *count_cell;
                if count != 0 {
                    let kind = *((obj.wrapping_add(KIND_OFF)) as *const u8);
                    let next = count.wrapping_sub(1);
                    *count_cell = next;
                    if next == 0 && (kind == 2 || kind == 4) {
                        fn1_deleting_dtor(obj);
                    }
                }
                *cell = 0;
            }
        }

        let handle = *((this.wrapping_add(0x18)) as *const u32);
        if handle != NO_HANDLE {
            callee_cdecl!(7, u32, handle);
            *((this.wrapping_add(0x18)) as *mut u32) = NO_HANDLE;
        }

        let mut dword = this.wrapping_add(0xA0);
        let mut word = this.wrapping_add(0xAC);
        for _ in 0..3 {
            *(dword as *mut u32) = 0;
            *(word as *mut u16) = 0;
            dword = dword.wrapping_add(4);
            word = word.wrapping_add(2);
        }
        *((this.wrapping_add(0x1C)) as *mut u32) = 0;
        *((this.wrapping_add(0x20)) as *mut u8) = 0;
        *((this.wrapping_add(0x40)) as *mut u8) = 0;
        *((this.wrapping_add(0x60)) as *mut u8) = 0;
        0
    }
});

/// Call the deleting destructor (vtable slot 0, flag 1) of a heap object.
unsafe fn fn1_deleting_dtor(obj: u32) -> u32 {
    unsafe {
        let vtable = *(obj as *const u32);
        let target = *(vtable as *const u32);
        let dtor: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        dtor(obj, 1)
    }
}
