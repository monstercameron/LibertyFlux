// original: 0x00b666b0 veh_visit_bound_set
/// Visit a bound element set, reconfigure it, and release the binding.
///
/// `this+0x14` points at a binding record (null, or bit 0 of its `+0x24`
/// word clear, returns the entry EAX at once; that exit is excluded from the
/// proof). Otherwise reads a manager from a global, enlists up to 8 element
/// words through it into a frame buffer, visits each nonzero element,
/// binding `+0x1e2`, reconfigures through `[binding+0x38]`, emits through
/// vtable slot `+0xdc` of the binding, and clears bit 0 of binding `+0x24`.
/// Returns the binding pointer. The stack word is ignored.
///
/// Original: thiscall, ECX plus one stack word.
lf_checker_rt::export!(thiscall, rw_00b666b0(this: u32, _a0: u32) -> u32 {
    unsafe {
        const ENLIST: u32 = 1;
        const VISIT: u32 = 2;
        const CONFIG: u32 = 3;
        const EMIT: u32 = 4;
        const MGR_VA: u32 = 0x012b9c7c;
        const VT_EMIT: u32 = 0xdc;
        let d = ((this + 0x14) as *const u32).read_unaligned();
        if d == 0 {
            return 0; // unreachable: pinned (original returns entry EAX)
        }
        if ((d + 0x24) as *const u8).read() & 1 == 0 {
            return 0; // unreachable: pinned (original returns entry EAX)
        }
        let mgr = lf_checker_rt::global::<u32>(MGR_VA).read_unaligned();
        let b = ((mgr + 8) as *const u32).read_unaligned();
        let key = ((d + 0x38) as *const u32).read_unaligned();
        
        let mut buf = [0u32; 10];
        let n: u32 = lf_checker_rt::callee_thiscall!(
            ENLIST, u32, b, key, buf.as_mut_ptr() as u32, 8);
        let mut i = 0u32;
        while (i as i32) < (n as i32) {
            let e = buf[i as usize];
            if e != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(VISIT, u32, b, e);
            }
            i = i.wrapping_add(1);
        }
        let w = ((d + 0x1e2) as *const u16).read_unaligned();
        ((d + 0x1e2) as *mut u16).write_unaligned(w & 0xffdf);
        let w2 = ((d + 0x1e2) as *const u16).read_unaligned();
        ((d + 0x1e2) as *mut u16).write_unaligned(w2 & 0xffbf);
        let _: u32 = lf_checker_rt::callee_thiscall!(CONFIG, u32, key, 0x2000, 0);
        let vt = (d as *const u32).read_unaligned();
        let slot = ((vt + VT_EMIT) as *const u32).read_unaligned();
        let emit: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let _ = emit(d);
        let flags = ((d + 0x24) as *const u32).read_unaligned();
        ((d + 0x24) as *mut u32).write_unaligned(flags & 0xfffffffe);
        d
    }
});

/// Wrong version of rw_00b666b0: the final release of bit 0 is omitted.
lf_checker_rt::export!(thiscall, mut_00b666b0(this: u32, _a0: u32) -> u32 {
    unsafe {
        const ENLIST: u32 = 1;
        const VISIT: u32 = 2;
        const CONFIG: u32 = 3;
        const EMIT: u32 = 4;
        const MGR_VA: u32 = 0x012b9c7c;
        const VT_EMIT: u32 = 0xdc;
        let d = ((this + 0x14) as *const u32).read_unaligned();
        if d == 0 {
            return 0; // unreachable: pinned (original returns entry EAX)
        }
        if ((d + 0x24) as *const u8).read() & 1 == 0 {
            return 0; // unreachable: pinned (original returns entry EAX)
        }
        let mgr = lf_checker_rt::global::<u32>(MGR_VA).read_unaligned();
        let b = ((mgr + 8) as *const u32).read_unaligned();
        let key = ((d + 0x38) as *const u32).read_unaligned();
        
        let mut buf = [0u32; 10];
        let n: u32 = lf_checker_rt::callee_thiscall!(
            ENLIST, u32, b, key, buf.as_mut_ptr() as u32, 8);
        let mut i = 0u32;
        while (i as i32) < (n as i32) {
            let e = buf[i as usize];
            if e != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(VISIT, u32, b, e);
            }
            i = i.wrapping_add(1);
        }
        let w = ((d + 0x1e2) as *const u16).read_unaligned();
        ((d + 0x1e2) as *mut u16).write_unaligned(w & 0xffdf);
        let w2 = ((d + 0x1e2) as *const u16).read_unaligned();
        ((d + 0x1e2) as *mut u16).write_unaligned(w2 & 0xffbf);
        let _: u32 = lf_checker_rt::callee_thiscall!(CONFIG, u32, key, 0x2000, 0);
        let vt = (d as *const u32).read_unaligned();
        let slot = ((vt + VT_EMIT) as *const u32).read_unaligned();
        let emit: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let _ = emit(d);
        // MUTANT: final release of bit 0 omitted.
        d
    }
});
