// original: 0x00B6D6F0 CTaskComplexGoToCarDoorAndStandStill_dtor
/// Destroy a go-to-car-door task: release the owned handle, stamp the
/// vtable, forward the auxiliary handle through the shared dispatcher while
/// set, then run the base destructor.
export!(thiscall, rw_00b6d6f0(this: *mut u8) -> u32 {
    unsafe {
        let field = *((this.add(0x14)) as *mut u32);
        *(this as *mut u32) = relocated(0xEB1A3C);
        if field != 0 {
            callee_thiscall!(1, u32, field, (this.add(0x14)) as u32);
        }
        let aux = *((this.add(0x7c)) as *mut u32);
        if aux != 0 {
            let disp = *(global::<u32>(0x179D114));
            callee_thiscall!(2, u32, disp, aux);
        }
        callee_thiscall!(3, u32, this as u32)
    }
});
