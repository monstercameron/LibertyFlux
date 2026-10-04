// original: 0x00ca65d0 CEventHandler::vf61
/// Guarded event slot: probes the event through its vtable slot 13, checks
/// the probe result against a registry, then requires event id 0x76C, a
/// non-null probe and marker bits in the probe's status word before
/// converting through the factory; a registry release call runs on every
/// converted event. Stores the conversion at this+0xC.
lf_rs75_rt::export!(thiscall, rw_00ca65d0(this: u32, ev: u32, _b: u32, _c: u32) -> u32 {
    unsafe {
        let vtbl = *(ev as *const u32);
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vtbl + 0x34) as *const u32) as usize);
        let ebx = *((ev + 0x10) as *const u32);
        let found = probe(ev);
        let ctx = *((this + 4) as *const u32);
        let reg = *((ctx + 0x224) as *const u32);
        let r2: u32 = lf_rs75_rt::callee_thiscall!(2, u32, reg, ebx);
        if r2 != 0 {
            return r2;
        }
        if ebx != 0x76C {
            return 0;
        }
        if found == 0 {
            return 0;
        }
        let bits = *((found + 0x28) as *const u32) & 0x3C0;
        if bits != 0xC0 {
            return bits;
        }
        let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
        let h: u32 = lf_rs75_rt::callee_thiscall!(3, u32, mgr);
        let conv: u32 = if h == 0 {
            0
        } else {
            lf_rs75_rt::callee_thiscall!(4, u32, h, found, 0)
        };
        *((this + 0xC) as *mut u32) = conv;
        lf_rs75_rt::callee_thiscall!(5, u32, reg, found, 1)
    }
});
