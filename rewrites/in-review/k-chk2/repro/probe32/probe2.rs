use std::ffi::c_void;
#[link(name = "kernel32")]
unsafe extern "system" {
    fn VirtualAlloc(addr: *mut c_void, size: usize, atype: u32, prot: u32) -> *mut c_void;
    fn VirtualFree(addr: *mut c_void, size: usize, ftype: u32) -> i32;
    fn GetLastError() -> u32;
    fn GetSystemInfo(info: *mut [u8; 36]);
}
fn main() {
    let mut si = [0u8; 36];
    unsafe { GetSystemInfo(&mut si) };
    let gran = u32::from_le_bytes([si[16+4-4+0],si[16+4-4+1],si[16+4-4+2],si[16+4-4+3]]);
    println!("pid={} pageSize-ish bytes={:?}", std::process::id(), &si[4..8]);
    for a in [0x10000usize, 0x100000, 0x200000, 0x300000, 0x3f0000, 0x400000, 0x410000, 0x500000, 0x700000, 0x7a0000, 0x800000, 0x1000000, 0x2000000, 0x3000000, 0x4000000, 0x20000000] {
        let p = unsafe { VirtualAlloc(a as *mut c_void, 0x10000, 0x2000, 0x01) };
        let e = unsafe { GetLastError() };
        println!("  reserve {a:#010x} 64K: {} err={}", if p.is_null() {"FAIL"} else {"ok"}, e);
        if !p.is_null() { unsafe { VirtualFree(p, 0, 0x8000) }; }
    }
    let _ = gran;
}
