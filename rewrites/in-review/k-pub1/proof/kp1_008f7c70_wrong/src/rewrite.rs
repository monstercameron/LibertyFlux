// Gate byte cleared after the real refresh (it always sets 1).
lf_checker_rt::export!(thiscall, rw_008f7c70(obj: u32) -> u32 {
    let r = rw_008f7c70(obj);
    unsafe { ((obj + 0x471) as *mut u8).write(0) };
    r
});
