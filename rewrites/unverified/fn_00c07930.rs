// original: 0x00c07930 stream_get_or_create_root
/// Return the shared streaming root, creating it on first use.
///
/// When the root global is already set, returns it without calling anything.
/// Otherwise allocates 12 bytes (cdecl/1); a null answer clears the global
/// and returns null, while a live answer is initialised through the pair
/// helper (thiscall/0) and the helper's answer is stored to the global and
/// returned. Cdecl with no arguments.
lf_checker_rt::export!(cdecl, rw_00c07930() -> u32 {
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
        const INIT: u32 = 2;
        const ROOT_GLOBAL: u32 = 0x01683288;
        const BLOCK: u32 = 12;
        let root = rd32(lf_checker_rt::relocated(ROOT_GLOBAL));
        if root != 0 {
            return root;
        }
        let obj: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, BLOCK);
        if obj == 0 {
            wr32(lf_checker_rt::relocated(ROOT_GLOBAL), 0);
            return 0;
        }
        let init: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, obj);
        wr32(lf_checker_rt::relocated(ROOT_GLOBAL), init);
        init
    }
});
