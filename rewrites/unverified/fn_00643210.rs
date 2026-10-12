// original: 0x00643210 rage::ptxDomain::init_as_kind

/// Dispatch the domain initializer for kind values 0, 1, 2 and 3, then store
/// that kind's relocated vtable pointer at object offset zero. Other kind
/// values leave the object unchanged. The second fastcall argument is passed
/// as the sole stack argument to the initializer.
lf_checker_rt::export!(fastcall, rw_00643210(this: *mut u8, context: u32) -> () {
    unsafe {
        const KIND: usize = 8;
        const INITIALIZER: u32 = 1;
        let kind = (this.add(KIND) as *const u32).read_unaligned();
        let vtable = match kind {
            0 => Some(lf_checker_rt::relocated(0x00FE293C)),
            1 => Some(lf_checker_rt::relocated(0x00FE28FC)),
            2 => Some(lf_checker_rt::relocated(0x00FE291C)),
            3 => Some(lf_checker_rt::relocated(0x00FE295C)),
            _ => None,
        };
        if let Some(vtable) = vtable {
            let _ = lf_checker_rt::callee_thiscall!(INITIALIZER, u32, this as u32, context);
            (this as *mut u32).write_unaligned(vtable);
        }
    }
});
