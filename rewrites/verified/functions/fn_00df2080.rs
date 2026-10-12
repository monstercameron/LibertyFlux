// original: 0x00DF2080 UIMontageText::enable_text_controls

/// Enable this montage view's text controls when the control-state field at
/// `+0x220` is zero. The global tree pointer supplies a receiver for two
/// name lookups: the first always runs, and the second runs only when the
/// view's virtual check at slot `0x1c` returns zero. Each found tree node
/// receives its virtual hook at slot `0x1b0`; the second path also invokes
/// this view's slot `0x1ac`. Every path finishes by forwarding `this` to the
/// base frame method. String and global data are read from the mapped image.
lf_checker_rt::export!(thiscall, rw_00df2080(this: u32) -> u32 {
    unsafe {
        const CONTROL_STATE: u32 = 0x220;
        const TREE_GLOBAL: u32 = 0x0198_1A4C;
        const EDITOR_NAME: u32 = 0x00F0_11DC;
        const CONTROLLER_NAME: u32 = 0x00F0_11F0;
        const NODE_HOOK_SLOT: u32 = 0x1b0;
        const ENABLE_CHECK_SLOT: u32 = 0x1c;
        const ENABLE_HOOK_SLOT: u32 = 0x1ac;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn invoke_node_hook(node: u32, slot: u32) -> u32 {
            unsafe {
                let table = read_u32(node);
                let address = read_u32(table.wrapping_add(slot));
                let method: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(address as usize);
                method(node)
            }
        }

        #[inline(always)]
        unsafe fn invoke_view_hook(view: u32, table: u32, slot: u32) -> u8 {
            unsafe {
                let address = read_u32(table.wrapping_add(slot));
                let method: extern "thiscall" fn(u32) -> u8 =
                    core::mem::transmute(address as usize);
                method(view)
            }
        }

        if read_u32(this.wrapping_add(CONTROL_STATE)) == 0 {
            let tree = lf_checker_rt::relocated(TREE_GLOBAL);
            let editor = lf_checker_rt::callee_thiscall!(
                2,
                u32,
                tree,
                lf_checker_rt::relocated(EDITOR_NAME),
            );
            let _ = invoke_node_hook(editor, NODE_HOOK_SLOT);
        }

        let view_table = read_u32(this);
        if invoke_view_hook(this, view_table, ENABLE_CHECK_SLOT) == 0 {
            let tree = lf_checker_rt::relocated(TREE_GLOBAL);
            let controller = lf_checker_rt::callee_thiscall!(
                2,
                u32,
                tree,
                lf_checker_rt::relocated(CONTROLLER_NAME),
            );
            let _ = invoke_node_hook(controller, NODE_HOOK_SLOT);
            let _ = invoke_view_hook(this, view_table, ENABLE_HOOK_SLOT);
        }

        lf_checker_rt::callee_thiscall!(5, u32, this)
    }
});
