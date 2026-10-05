// original: 0x00c06bf0 stream_dup_name_register
/// Duplicate a name string and register it in two tables.
///
/// Allocates 0x100 bytes (cdecl/1), copies `name` into them through the
/// string helper (cdecl/3) and terminates the buffer, then registers the
/// pointer through the table helper (thiscall/1) for `this`+0x10 and stores
/// it into the helper's answer, then registers `this`+0x18 and zeroes that
/// answer. Returns the second answer. Thiscall: one stack word, callee
/// cleans 4. Only the first 64 bytes of the copy are observed.
lf_checker_rt::export!(thiscall, rw_00c06bf0(this: u32, name: u32) -> u32 {
    unsafe {
#[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const ALLOC: u32 = 1;
        const COPY: u32 = 2;
        const REG: u32 = 3;
        const BUF: u32 = 0x100;
        let buf: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, BUF);
        let _: u32 = lf_checker_rt::callee_cdecl!(COPY, u32, buf, name, BUF);
        wr8(buf + 0xFF, 0);
        let q1: u32 = lf_checker_rt::callee_thiscall!(REG, u32, this + 0x10, 0x10);
        wr32(q1, buf);
        let q2: u32 = lf_checker_rt::callee_thiscall!(REG, u32, this + 0x18, 0x10);
        wr32(q2, 0);
        q2
    }
});
