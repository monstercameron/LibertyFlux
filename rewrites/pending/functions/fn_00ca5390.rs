// original: 0x00ca5390 CEventHandler::vf56
/// Two-type event slot. A null payload pointer or an unknown type word
/// exits quietly; type 0x38F converts through the factory with two global
/// parameter words and a global float, marks the new object, and stores it
/// at this+0xC (when the factory is empty the original stores null and then
/// faults writing the mark through the null pointer; the rewrite faults
/// the same way and the checker compares the faults). Type 0x2D6 first
/// consults a flag on the context object and either falls into the 0x38F
/// path or runs a probe-plus-convert sequence instead.
lf_rs75_rt::export!(thiscall, rw_00ca5390(this: u32, ev: u32, _b: u32, _c: u32) -> u32 {
    unsafe {
        let pay = *((ev + 0x18) as *const u32);
        if pay == 0 {
            return ev;
        }
        let t = *((ev + 0x10) as *const u32);
        let mut via_38f = t == 0x38F;
        if t == 0x2D6 {
            let ctx = *((this + 4) as *const u32);
            if (*((ctx + 0x26C) as *const u8) & 4) == 0 {
                via_38f = true;
            } else {
                let inner = *((ctx + 0xB30) as *const u32);
                let probe: u32 = lf_rs75_rt::callee_thiscall!(2, u32, inner, ctx);
                if probe & 0xFF == 0 {
                    *((this + 0xC) as *mut u32) = 0;
                    return probe;
                }
                let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
                let h: u32 = lf_rs75_rt::callee_thiscall!(3, u32, mgr);
                if h == 0 {
                    *((this + 0xC) as *mut u32) = 0;
                    return 0;
                }
                let ans: u32 = lf_rs75_rt::callee_thiscall!(4, u32, h, inner, 0);
                *((this + 0xC) as *mut u32) = ans;
                return ans;
            }
        }
        if !via_38f {
            return t;
        }
        let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
        let h: u32 = lf_rs75_rt::callee_thiscall!(1, u32, mgr);
        if h == 0 {
            *((this + 0xC) as *mut u32) = 0;
            // Original keeps going and faults on this store; fault likewise.
            *((h + 0x39) as *mut u8) = 1;
            return 0;
        }
        const CEIL: u32 = 0x47C35000; // float bits
        let f = *lf_rs75_rt::global::<u32>(0x00EEF94C);
        let g48 = *lf_rs75_rt::global::<u32>(0x00EEF948);
        let g44 = *lf_rs75_rt::global::<u32>(0x00EEF944);
        let ans: u32 = lf_rs75_rt::callee_thiscall!(5, u32, h, pay, 1, CEIL, g44, g48, f, 0);
        *((this + 0xC) as *mut u32) = ans;
        *((ans + 0x39) as *mut u8) = 1;
        ans
    }
});
