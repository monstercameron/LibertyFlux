// original: 0x00ca4e60 propose: destroy_event_handler_members
/// Member teardown: destroys the members at +8 and +4 (in that order), then
/// tail-destroys the member at +0x10. Proposed name: no merged symbol.
lf_rs75_rt::export!(thiscall, rw_00ca4e60(this: u32) -> u32 {
    let _: u32 = lf_rs75_rt::callee_thiscall!(1, u32, this);
    let _: u32 = lf_rs75_rt::callee_thiscall!(2, u32, this);
    lf_rs75_rt::callee_thiscall!(3, u32, this)
});
