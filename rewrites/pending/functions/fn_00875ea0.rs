// original: 0x00875ea0 rage::crmtNodeExtrapolate::vf4
/// Dispatch an extrapolate sample: probe, then run, resume or push.
///
/// thiscall/1 (`rage::crmtNodeExtrapolate::vf4`). Probes the node through
/// slot 7 of its own vtable; when the probe succeeds, the argument is run
/// with a callback frame through slot 7 of its vtable. Otherwise the
/// stored rate is compared against the floor: above it the argument is
/// resumed with a different callback frame, otherwise the pending value
/// is pushed through slot 10.
export!(thiscall, rw_00875ea0(this: *mut u8, arg: u32) -> () {
    unsafe {
        let own_vtable = *(this as *const u32);
        let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            *((own_vtable as *const u8).add(0x1c) as *const u32) as usize,
        );
        let ready = probe(this as u32);
        let arg_vtable = *(arg as *const u32);
        if ready != 0 {
            let run: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(
                    *((arg_vtable as *const u8).add(0x1c) as *const u32)
                        as usize,
                );
            let frame = [
                relocated(0x00875f30),
                0,
                this as u32,
                relocated(0x00404b80),
            ];
            run(arg, frame.as_ptr() as u32, 1);
        } else {
            let rate = *(this.add(0x30) as *const f32);
            let floor = *global::<f32>(0x00fe8628);
            if rate > floor {
                let run: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(
                        *((arg_vtable as *const u8).add(0x1c) as *const u32)
                            as usize,
                    );
                let frame = [
                    relocated(0x00876020),
                    0,
                    this as u32,
                    relocated(0x00404b80),
                ];
                run(arg, frame.as_ptr() as u32, 0);
            } else {
                let push: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(
                        *((arg_vtable as *const u8).add(0x28) as *const u32)
                            as usize,
                    );
                push(arg, *(this.add(0x24) as *const u32));
            }
        }
    }
});
