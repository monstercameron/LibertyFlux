// original: 0x00B233A0 CHeli::vf43

/// Dispatches a vehicle's physics event when its physics record is present.
///
/// `this` is an entity whose vtable supplies the slot-43 operations, whose
/// placement-matrix pointer is at `+0x20`, whose entity flag word is at
/// `+0x24`, and whose rigid-body record pointer is at `+0x38`. The record's
/// word at `+4` gates the event. The method first invokes two virtual hooks;
/// a nonzero low byte from the second hook stops the work. It then asks the
/// transform helper to process the placement, routes the record to one of
/// three service operations according to bits in the entity flag word, and
/// calls the final cleanup helper only when the status hook returns zero.
///
/// The class-slot name is a merged vtable placeholder shared by several
/// entity classes, not a semantic function name. This implementation keeps
/// all object accesses as 32-bit offsets so its data logic is pointer-width
/// independent; the checked ABI remains thiscall on the original 32-bit
/// build.
lf_checker_rt::export!(thiscall, rw_b233a0(this: u32) -> () {
    const VTABLE_PTR: u32 = 0x00;
    const PLACEMENT_MATRIX: u32 = 0x20;
    const ENTITY_FLAGS: u32 = 0x24;
    const PHYSICS_RECORD: u32 = 0x38;
    const RECORD_READY: u32 = 0x04;
    const VT_SLOT_QUERY: u32 = 0xA0;
    const VT_SLOT_NOTIFY: u32 = 0xB4;
    const SERVICE_GLOBAL: u32 = 0x12B9_C7C;
    const EVENT_FLAG: u32 = 0x04;
    const EVENT_MASK: u32 = 0x18;
    const EVENT_READY: u32 = 0x01;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn call_vtable(this: u32, slot: u32, arg: Option<u32>) -> u32 {
        unsafe {
            let vtable = read_word(this.wrapping_add(VTABLE_PTR));
            let target = read_word(vtable.wrapping_add(slot));
            match arg {
                Some(value) => {
                    let method: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(target as usize);
                    method(this, value)
                }
                None => {
                    let method: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(target as usize);
                    method(this)
                }
            }
        }
    }

    unsafe {
        let record = read_word(this.wrapping_add(PHYSICS_RECORD));
        if record == 0 {
            return;
        }

        let _notify_result = call_vtable(this, VT_SLOT_NOTIFY, Some(1));
        if call_vtable(this, VT_SLOT_QUERY, None) != 0 {
            return;
        }
        if read_word(record.wrapping_add(RECORD_READY)) == 0 {
            return;
        }

        let placement = read_word(this.wrapping_add(PLACEMENT_MATRIX));
        let _ = lf_checker_rt::callee_stdcall!(3, u32, placement);

        let flags = read_word(this.wrapping_add(ENTITY_FLAGS));
        let service = lf_checker_rt::global::<u32>(SERVICE_GLOBAL).read_unaligned();
        if flags & EVENT_FLAG != 0 {
            let _ = lf_checker_rt::callee_thiscall!(4, u32, service, record, 0);
        } else if (flags >> 1) & 1 != 0 && flags & EVENT_MASK == 0 && flags & EVENT_READY != 0 {
            let _ = lf_checker_rt::callee_thiscall!(5, u32, service, record, 0, 0);
        } else {
            let _ = lf_checker_rt::callee_thiscall!(6, u32, service, record, 0);
        }

        if (lf_checker_rt::callee_thiscall!(7, u32, this) as u8) == 0 {
            let _ = lf_checker_rt::callee_cdecl!(8, u32, this);
        }
    }
});
