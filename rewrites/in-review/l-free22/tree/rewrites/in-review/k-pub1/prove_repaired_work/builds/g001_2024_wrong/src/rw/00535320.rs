lf_checker_rt::export!(thiscall, rw_00535320(_this: u32, want: u32) -> u32 {
    unsafe {
        const BOARD: u32 = 0x83;
        const NONE: u32 = 0xFFFF_FFFF;
        const ST_COUNT: usize = 3;
        const ST_LIST: usize = 4;
        let mut st = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD, st.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NONE;
        }
        let count = st[ST_COUNT] as i32;
        if count <= 0 {
            return NONE;
        }
        let list = st[ST_LIST];
        let mut i = 0i32;
        while i < count {
            if rd(list, i as u32) == want {
                return (i as u32).wrapping_add(1);
            }
            i += 1;
        }
        NONE
    }});

lf_checker_rt::export!(thiscall, lane_base_00535320(_this: u32, want: u32) -> u32 {
    unsafe {
        const BOARD: u32 = 0x83;
        const NONE: u32 = 0xFFFF_FFFF;
        const ST_COUNT: usize = 3;
        const ST_LIST: usize = 4;
        let mut st = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(1, u32, BOARD, st.as_mut_ptr() as u32);
        if ok & 0xFF == 0 {
            return NONE;
        }
        let count = st[ST_COUNT] as i32;
        if count <= 0 {
            return NONE;
        }
        let list = st[ST_LIST];
        let mut i = 0i32;
        while i < count {
            if rd(list, i as u32) == want {
                return i as u32;
            }
            i += 1;
        }
        NONE
    }});

unsafe fn rd(base: u32, idx: u32) -> u32 {
    unsafe { (base.wrapping_add(idx.wrapping_mul(4)) as *const u32).read_unaligned() }
}
