// original: 0x00ca4e10 CEventHandler::~CEventHandler
/// Destructor fragment: installs this object's vtable, destroys the member
/// at +0x34, then tail-destroys the base object at +0x20 (the original ends
/// in a jump chain; the rewrite forwards through the checker's tail stub).
lf_rs75_rt::export!(thiscall, rw_00ca4e10(this: u32) -> u32 {
    unsafe {
        *(this as *mut u32) = lf_rs75_rt::relocated(0x00ED7B54);
        let _: u32 =
            lf_rs75_rt::callee_thiscall!(1, u32, this.wrapping_add(0x34));
        lf_rs75_rt::callee_thiscall!(2, u32, this.wrapping_add(0x20))
    }
});
