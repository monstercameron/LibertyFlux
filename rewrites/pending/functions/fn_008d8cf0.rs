// original: 0x008D8CF0 load_cycle_runner

/// Load-cycle runner: clamped start, four poll/report/fold cycles.
pub fn run_cycle(this: u32, mode: u32) -> u32 {
    unsafe {
        let mgr = relocated(0x011D4EC8);
        let slot = this as *mut u8;
        if mode == 1 && global::<u8>(0x017ED8C1).read() != 0 {
            let w1 = (global::<u32>(0x017ED954).read() as *mut u8).add(0xA) as *mut u16;
            if w1.read() > 1 {
                w1.write(1);
            }
            let w2 = (global::<u32>(0x017EDDA4).read() as *mut u8).add(0xA) as *mut u16;
            if w2.read() > 1 {
                w2.write(1);
            }
            callee_cdecl!(0, u32,);
        }
        for _ in 0..4 {
            let v = callee_thiscall!(1, u32, mgr);
            callee_thiscall!(2, u32, mgr);
            let mut cur = v;
            if v != 0 {
                callee_cdecl!(3, u32, v);
            }
            callee_thiscall!(4, u32, mgr);
            if slot.add(0xFCA).read() == 0 {
                cur = 1;
            } else {
                let tag = (slot.add(0xFB4) as *const u32).read();
                callee_cdecl!(5, u32, tag);
                let keep = global::<u8>(0x017ED8D2).read();
                global::<u8>(0x017ED8D2).write(if mode == 2 { 1 } else { keep });
                let h = (slot.add(0xFB8) as *const u32).read();
                if (callee_cdecl!(6, u32, h, 0xFA0u32) as u8) == 0 {
                    loop {
                        global::<u8>(0x017ED8C1).write(0);
                        if (callee_cdecl!(6, u32, h, 0xFA0u32) as u8) != 0 {
                            break;
                        }
                    }
                }
            }
            callee_thiscall!(7, u32, relocated(0x01175C58));
            callee_thiscall!(8, u32, mgr);
            let g = global::<u32>(0x011D6F10);
            g.write(cur.wrapping_sub(g.read()));
            callee_thiscall!(9, u32, mgr);
            let c = slot.add(0xFC0) as *mut u32;
            c.write(cur.wrapping_sub(c.read()));
        }
        callee_thiscall!(10, u32, mgr)
    }
}
