// original: 0x00ca6c60 CEventHandler::~CEventHandler_3
/// List-search event slot: walks the node list hung off the context object
/// looking first for a node tagged 0x40C, then for one tagged 0x417, with a
/// priority comparison that can cut either walk short. On a hit it builds a
/// fresh initialised object through the factory and stores it at this+0xC.
/// (Merged name guesses a destructor; the code is a find-or-create slot.)
lf_rs75_rt::export!(thiscall, rw_00ca6c60(this: u32, _a: u32, _b: u32, _c: u32) -> u32 {
    unsafe {
        let a = *((this + 4) as *const u32);
        let d = *((a + 0x224) as *const u32);
        if *((d + 0x50) as *const u32) != 0 {
            return a;
        }
        let head = *((d + 0x2E0) as *const u32);
        let mut eax = head;
        let mut hit = false;
        if eax != 0 {
            let mut esi = ((*((eax + 8) as *const u32) >> 1) & 7);
            loop {
                let ecx = ((*((eax + 8) as *const u32) >> 1) & 7);
                if esi < ecx && esi >= 2 {
                    break;
                }
                if *((eax + 4) as *const u32) == 0x40C {
                    hit = true;
                    break;
                }
                esi = ecx;
                eax = *((eax + 0xC) as *const u32);
                if eax == 0 {
                    break;
                }
            }
        }
        if !hit {
            let mut edx = head;
            if edx == 0 {
                return eax;
            }
            let mut ecx = ((*((edx + 8) as *const u32) >> 1) & 7);
            loop {
                let e = ((*((edx + 8) as *const u32) >> 1) & 7);
                if ecx < e && ecx >= 2 {
                    return e;
                }
                if *((edx + 4) as *const u32) == 0x417 {
                    hit = true;
                    break;
                }
                ecx = e;
                edx = *((edx + 0xC) as *const u32);
                if edx == 0 {
                    return e;
                }
            }
        }
        if !hit {
            // Unreachable: both loops only exit via return or hit.
            return eax;
        }
        let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
        let obj: u32 = lf_rs75_rt::callee_thiscall!(1, u32, mgr);
        if obj == 0 {
            *((this + 0xC) as *mut u32) = 0;
            return 0;
        }
        let ans: u32 = lf_rs75_rt::callee_thiscall!(2, u32, obj);
        *(obj as *mut u32) = lf_rs75_rt::relocated(0x00EB391C);
        *((obj + 0x14) as *mut u32) = 1;
        *((obj + 0x18) as *mut u8) = 0;
        *((obj + 0x1C) as *mut u32) = 0;
        *((this + 0xC) as *mut u32) = obj;
        ans
    }
});
