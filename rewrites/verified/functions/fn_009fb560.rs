// original: 0x009fb560 CPlayStatIntStr::CPlayStatIntStr

/// Build a playstats int/str node for `key_index`, optionally resolving a
/// display name through `cfg_obj`.
///
/// `key_index` (cdecl arg 0) selects a name pointer from `NAME_TABLE`
/// (`NAME_TABLE[key_index]`); a null entry falls back to `DEFAULT_NAME`.
/// `cfg_obj` (cdecl arg 1) is a nullable object: a null pointer, or a zero
/// pick below, skips name resolution and goes straight to message formatting.
/// The working structs live in the original's aligned frame and are never
/// observed outside the call log: only argument values, the eight snapped
/// message words, and the return value are compared.
///
/// Sequence: callee 0 initialises the node (thiscall, 5 and 2); the node is
/// tagged with `NODE_TAG` and callee 1 links two temporaries (cdecl, one
/// pointer argument — its second push belongs to callee 2, which reads both
/// stack slots per its prologue); callee 2 converts through the link result
/// (cdecl, link and one pointer). Callee 3 registers the name (cdecl, name,
/// 0, name) and callee 4 attaches the result (thiscall, result and name).
/// When `cfg_obj` is non-null, flag byte `CFG_FLAG` selects the pick: nonzero
/// reads the inner object at `CFG_INNER` and calls callee 5 (thiscall), zero
/// reads the sub-object at `CFG_SUB` and calls callee 6 (cdecl) with the word
/// at `SUB_VALUE`. A nonzero pick stamps `CONV_MARK` when the flag is zero
/// and runs callee 7 (cdecl, two pointers and `CONV_LEN`). Callee 8 then
/// formats `MSG_LEN` bytes into the message buffer (cdecl, buffer and length;
/// the stub's words stand in for the format result) and callee 9 consumes it
/// (thiscall), whose answer is returned. Callee 10 is the stack-cookie check
/// (no arguments, registers preserved).
lf_checker_rt::export!(cdecl, rw_009fb560(key_index: u32, cfg_obj: u32) -> u32 {
    unsafe {
        const NAME_TABLE: u32 = 0x0103_FC88;
        const DEFAULT_NAME: u32 = 0x00E9_96E0;
        const NODE_TAG: u32 = 0x00E9_98D8;
        const CFG_FLAG: u32 = 0x219;
        const CFG_SUB: u32 = 0x21C;
        const CFG_INNER: u32 = 0x228;
        const SUB_VALUE: u32 = 0x12C;
        const CONV_MARK: u16 = 0x2A;
        const CONV_LEN: u32 = 0x20;
        const MSG_LEN: u32 = 0x58;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        // Shadow frame: the original's aligned working structs. Only the
        // message buffer is ever observed (through callee 9's snapshot).
        let mut node = [0u32; 4];
        let mut link_tmp = [0u32; 4];
        let mut conv_tmp = [0u32; 2];
        let mut attach = [0u32; 4];
        let mut conv_src = [0u32; 8];
        let mut conv_dst = [0u32; 8];
        let mut mark: u16 = 0;
        let mut msg = [0u32; 22];

        let table = lf_checker_rt::relocated(NAME_TABLE);
        let default_name = lf_checker_rt::relocated(DEFAULT_NAME);
        let _: u32 = lf_checker_rt::callee_thiscall!(0, u32, node.as_mut_ptr() as u32, 5u32, 2u32);
        link_tmp[0] = lf_checker_rt::relocated(NODE_TAG);
        let link: u32 = lf_checker_rt::callee_cdecl!(1, u32, link_tmp.as_mut_ptr() as u32);
        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, link, conv_tmp.as_mut_ptr() as u32);
        let entry = rd32(table.wrapping_add(key_index.wrapping_mul(4)));
        let name = if entry != 0 { entry } else { default_name };
        let registered: u32 = lf_checker_rt::callee_cdecl!(3, u32, name, 0u32, name);
        let _: u32 =
            lf_checker_rt::callee_thiscall!(4, u32, attach.as_mut_ptr() as u32, registered, name);
        let mut picked = 0u32;
        if cfg_obj != 0 {
            let flag = (cfg_obj.wrapping_add(CFG_FLAG) as *const u8).read();
            if flag != 0 {
                let inner = rd32(cfg_obj.wrapping_add(CFG_INNER));
                picked = lf_checker_rt::callee_thiscall!(5, u32, inner);
            } else {
                let sub = rd32(cfg_obj.wrapping_add(CFG_SUB));
                let val = rd32(sub.wrapping_add(SUB_VALUE));
                picked = lf_checker_rt::callee_cdecl!(6, u32, val);
            }
        }
        if picked != 0 {
            let flag = (cfg_obj.wrapping_add(CFG_FLAG) as *const u8).read();
            if flag == 0 {
                mark = CONV_MARK;
                core::hint::black_box(&mark);
            }
            let _: u32 = lf_checker_rt::callee_cdecl!(
                7,
                u32,
                conv_src.as_mut_ptr() as u32,
                conv_dst.as_mut_ptr() as u32,
                CONV_LEN
            );
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(8, u32, msg.as_mut_ptr() as u32, MSG_LEN);
        let result: u32 = lf_checker_rt::callee_thiscall!(9, u32, msg.as_mut_ptr() as u32);
        let _: u32 = lf_checker_rt::callee_cdecl!(10, u32,);
        result
    }
});
