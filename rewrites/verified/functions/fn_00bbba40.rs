// original: 0x00bbba40 NativeImpl_EXTEND_PATROL_ROUTE
/// Extend a patrol route unless the new leg is already covered.
///
/// Compares the leg name against the constant word "NONE". When they differ,
/// copies the leg name and the route name into scratch buffers and hands the
/// route record (the three coordinates, padding, then the leg name) to the
/// route builder together with the leg name; when they match, both
/// buffers stay empty. Runs the frame-cookie check afterwards and returns the
/// builder's answer.
/// The second scratch copy lands in a part of the frame no call observes, so
/// the rewrite only replays its reads (a null route name still faults there);
/// everything the builder observes is reproduced exactly.
export!(cdecl, rw_00bbba40(f0: u32, f1: u32, f2: u32, s1: u32, s2: u32) -> u32 {
    unsafe {
        let lit = relocated(0xEB71E4) as *const u8;
        let mut p = s1 as *const u8;
        let mut q = lit;
        let same = loop {
            let a = *p;
            let b = *q;
            if a != b {
                break false;
            }
            if a == 0 {
                break true;
            }
            p = p.add(1);
            q = q.add(1);
        };
        // Route record as the original lays it out on its frame: the three
        // coordinates, two zero words, then the leg-name buffer.
        let mut rec = [0u32; 8];
        rec[0] = f0;
        rec[1] = f1;
        rec[2] = f2;
        if !same {
            let dst = rec.as_mut_ptr().add(5) as *mut u8;
            let mut sp = s1 as *const u8;
            let mut i = 0usize;
            loop {
                let b = *sp;
                *dst.add(i) = b;
                if b == 0 {
                    break;
                }
                sp = sp.add(1);
                i += 1;
            }
            // Replay the unobserved second copy's reads for fault parity.
            let mut qp = s2 as *const u8;
            while *qp != 0 {
                qp = qp.add(1);
            }
        }
        let r: u32 = callee_thiscall!(1, u32, relocated(0x171BC20),
            rec.as_mut_ptr() as u32, rec.as_mut_ptr().add(5) as u32);
        // Frame-cookie check: preserved registers, answer already in `r`.
        let _: u32 = callee_cdecl!(2, u32,);
        r
    }
});
