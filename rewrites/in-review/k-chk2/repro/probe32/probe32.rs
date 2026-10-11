use std::ffi::c_void;
#[repr(C)]
#[derive(Clone, Copy)]
struct Mbi { base: usize, alloc_base: usize, alloc_prot: u32, region_size: usize, state: u32, protect: u32, typ: u32 }
#[link(name = "kernel32")]
unsafe extern "system" {
    fn VirtualQuery(addr: *const c_void, buf: *mut Mbi, len: usize) -> usize;
    fn GetLastError() -> u32;
    fn GetModuleHandleW(name: *const u16) -> *mut c_void;
}
fn main() {
    let (lo, hi) = (0usize, 0x8000_0000usize);
    let mut a = lo;
    let mut n = 0;
    let mut rows = Vec::new();
    while a < hi {
        let mut m = Mbi { base: 0, alloc_base: 0, alloc_prot: 0, region_size: 0, state: 0, protect: 0, typ: 0 };
        let r = unsafe { VirtualQuery(a as *const c_void, &mut m, std::mem::size_of::<Mbi>()) };
        if r == 0 { rows.push(format!("VirtualQuery err {}", unsafe { GetLastError() })); break; }
        let st = match m.state { 0x1000 => "COMMIT", 0x2000 => "RESERVE", 0x10000 => "FREE", _ => "?" };
        let ty = match m.typ { 0 => "none", 0x20000 => "PRIVATE", 0x40000 => "MAPPED", 0x1000000 => "IMAGE", _ => "?" };
        if m.base <= 0x40_0000 && 0x40_0000 < m.base + m.region_size || m.base < 0x1_0000 || m.base >= 0x40_0000 && m.base < 0x41_0000 || m.typ == 0x1000000 {
            rows.push(format!("base={:#010x} alloc={:#010x} size={:#010x} state={} type={} prot={:#x}", m.base, m.alloc_base, m.region_size, st, ty, m.protect));
        }
        n += 1;
        let next = m.base.wrapping_add(m.region_size);
        if next <= a { break; }
        a = next;
    }
    let mut ok = 0; let mut bad = 0;
    let mut c = 0x40_0000usize;
    while c < 0x340_0000 {
        let p = unsafe { VirtualAlloc(c as *mut c_void, 0x1_0000, 0x2000, 0x01) };
        if p.is_null() { bad += 1; } else { ok += 1; unsafe { VirtualFree(p, 0, 0x8000) }; }
        c += 0x1_0000;
    }
    println!("probe32 pid={} regions={} image_base_of_exe={:?}", std::process::id(), n, unsafe { GetModuleHandleW(std::ptr::null()) });
    for r in &rows { println!("  {r}"); }
    println!("  chunks 0x400000..0x3400000 reservable: ok={ok} failed={bad}");
}
#[link(name = "kernel32")]
unsafe extern "system" {
    fn VirtualAlloc(addr: *mut c_void, size: usize, atype: u32, prot: u32) -> *mut c_void;
    fn VirtualFree(addr: *mut c_void, size: usize, ftype: u32) -> i32;
}
