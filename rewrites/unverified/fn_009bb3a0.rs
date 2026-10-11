// original: 0x009BB3A0 camera_script_ref_acquire (proposed)

/// Acquire one script-camera reference and report the new byte count.
///
/// `this + 0x530` is an 8-bit reference count; incrementing it wraps at 255.
/// The new byte is written through the first stack pointer, the active byte
/// at `+0x531` is set, and the timer word at `+0x528` becomes 1000. The
/// service object is looked up from the global service slot and its vtable
/// method at `+0x18` is called when present. A nonzero second stack byte
/// clears the global camera flag; zero preserves it. EAX returns that final
/// flag byte. The object-method call is made through the planted vtable slot.
lf_checker_rt::export!(thiscall, rw_009bb3a0(this: u32, count_out: u32, clear_flag: u32) -> u32 {
    unsafe {
        const REFCOUNT: u32 = 0x530;
        const ACTIVE: u32 = 0x531;
        const TIMER: u32 = 0x528;
        const SERVICE_SLOT: u32 = 0x0103_e498;
        const GLOBAL_FLAG: u32 = 0x016d_8b41;
        const VTABLE_METHOD: u32 = 0x18;
        const GET_SERVICE: u32 = 1;

        let count_slot = this.wrapping_add(REFCOUNT) as *mut u8;
        let count = count_slot.read().wrapping_add(1);
        count_slot.write(count);
        (this.wrapping_add(ACTIVE) as *mut u8).write(1);
        (count_out as *mut u8).write(count);
        (this.wrapping_add(TIMER) as *mut u32).write_unaligned(1000);

        let service = lf_checker_rt::callee_thiscall!(
            GET_SERVICE,
            u32,
            lf_checker_rt::relocated(SERVICE_SLOT),
        );
        if service != 0 {
            let vtable = (service as *const u32).read_unaligned();
            let target = ((vtable.wrapping_add(VTABLE_METHOD)) as *const u32).read_unaligned();
            let call: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
            let _ = call(service);
        }

        let flag = lf_checker_rt::global::<u8>(GLOBAL_FLAG);
        if clear_flag as u8 != 0 {
            flag.write(0);
        }
        u32::from(flag.read())
    }
});
