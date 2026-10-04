//! Vector-register call arguments: all eight XMM registers.
//!
//! Version 4 logged XMM0 and XMM1 at a call (`logxmm`, `logxmm1`) and could
//! load them from a stack argument on the rewrite side (`xmm0_from_stack`,
//! `xmm1_from_stack`), because a Rust rewrite cannot place a value in a
//! vector register for a call. Version 5 generalises both to XMM0-XMM7
//! with `logxmm_regs: [n, ...]` and `xmm_from_stack: {"n": idx}`. The legacy
//! keys keep their exact meaning and slots (XMM0 at byte 224 and XMM1 at
//! byte 240 of the base log entry); XMM2-XMM7 are logged into a parallel
//! extension log at the base entry's stride.
//!
//! The new keys fail closed: a register transported from the stack must
//! also be logged (the stack arguments of a transport callee are left out
//! of the call key, so an unlogged transported register would be compared
//! nowhere), and the transported argument must be one the callee declares.

/// Number of XMM registers on a 32-bit x86 target.
pub const XMM_REGS: usize = 8;
/// Byte offset of the XMM0 slot in a base log entry.
pub const LOG_XMM0_OFF: usize = 224;
/// Byte offset of the XMM1 slot in a base log entry.
pub const LOG_XMM1_OFF: usize = 240;
/// Bytes of the extension entry used by XMM2-XMM7.
pub const EXT_XMM_BYTES: usize = (XMM_REGS - 2) * 16;

// XMM2-XMM7 must fit in one extension entry of the shared stride.
const _: () = assert!(EXT_XMM_BYTES <= crate::snap::LOG_ENTRY);

/// Which XMM registers a callee's stub logs and which it loads from a
/// stack argument on the rewrite side.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct XmmCallCfg {
    /// `log[n]`: the stub copies all 16 bytes of `XMMn` into the call log.
    pub log: [bool; XMM_REGS],
    /// `from_stack[n]`: on the rewrite side only, the stub loads `XMMn`
    /// (`movss`, 4 bytes, upper lanes zeroed) from this stack argument.
    pub from_stack: [Option<usize>; XMM_REGS],
}

/// The contract's vector-register options for one callee, as parsed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct XmmCallKeys {
    /// Legacy `logxmm`.
    pub logxmm: bool,
    /// Legacy `logxmm1`.
    pub logxmm1: bool,
    /// Legacy `xmm0_from_stack`.
    pub xmm0_from_stack: Option<usize>,
    /// Legacy `xmm1_from_stack`.
    pub xmm1_from_stack: Option<usize>,
    /// Version 5 `logxmm_regs`.
    pub logxmm_regs: Vec<usize>,
    /// Version 5 `xmm_from_stack` as (register, stack argument) pairs.
    pub xmm_from_stack: Vec<(usize, usize)>,
}

impl XmmCallCfg {
    /// Merge the legacy and version 5 keys into one configuration.
    ///
    /// # Errors
    /// Returns a message for a register above 7, a version 5 transport
    /// that disagrees with the legacy key for the same register, a version 5
    /// transport of an argument the callee does not declare, or a version 5
    /// transported register that is not logged.
    pub fn merge(id: u32, k: &XmmCallKeys, nargs: usize) -> Result<XmmCallCfg, String> {
        let mut c = XmmCallCfg::default();
        c.log[0] = k.logxmm;
        c.log[1] = k.logxmm1;
        c.from_stack[0] = k.xmm0_from_stack;
        c.from_stack[1] = k.xmm1_from_stack;
        for &r in &k.logxmm_regs {
            if r >= XMM_REGS {
                return Err(format!("callee {id} logxmm_regs names xmm{r} (0-7 exist)"));
            }
            c.log[r] = true;
        }
        for &(r, idx) in &k.xmm_from_stack {
            if r >= XMM_REGS {
                return Err(format!(
                    "callee {id} xmm_from_stack names xmm{r} (0-7 exist)"
                ));
            }
            if let Some(prev) = c.from_stack[r]
                && prev != idx
            {
                return Err(format!(
                    "callee {id} transports xmm{r} from stack arg {prev} and {idx}"
                ));
            }
            if idx >= nargs {
                return Err(format!(
                    "callee {id} xmm_from_stack loads xmm{r} from stack arg {idx}, but nargs is {nargs}"
                ));
            }
            if !c.log[r] {
                return Err(format!(
                    "callee {id} transports xmm{r} without logging it (add {r} to logxmm_regs): the argument would be compared nowhere"
                ));
            }
            c.from_stack[r] = Some(idx);
        }
        Ok(c)
    }

    /// Whether any register is transported (the callee's stack arguments
    /// are then left out of the call key on both sides).
    #[must_use]
    pub fn any_transport(&self) -> bool {
        self.from_stack.iter().any(Option::is_some)
    }

    /// Logged registers above XMM1, in register order (the extension log).
    #[must_use]
    pub fn ext_logged(&self) -> Vec<usize> {
        (2..XMM_REGS).filter(|&r| self.log[r]).collect()
    }
}

/// Where the 16 logged bytes of XMM`reg` live.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XmmSlot {
    /// Byte offset inside the base log entry (XMM0, XMM1).
    Base(usize),
    /// Byte offset inside the extension log entry (XMM2-XMM7).
    Ext(usize),
}

/// Placement of XMM`reg` in the call log.
///
/// # Panics
/// Never for `reg < 8`; larger registers are rejected by [`XmmCallCfg::merge`].
#[must_use]
pub fn slot(reg: usize) -> XmmSlot {
    match reg {
        0 => XmmSlot::Base(LOG_XMM0_OFF),
        1 => XmmSlot::Base(LOG_XMM1_OFF),
        r => {
            assert!(r < XMM_REGS, "xmm{r} does not exist");
            XmmSlot::Ext((r - 2) * 16)
        }
    }
}

/// Register field of a `ModRM` byte for register number `reg` (`reg` < 8).
fn reg_field(reg: usize) -> u8 {
    // Masked to three bits, so the conversion cannot fail.
    u8::try_from(reg & 7).unwrap_or(0) << 3
}

/// `movss xmm<reg>, [esp + disp]` with a SIB byte and a 32-bit
/// displacement (9 bytes). For XMM0 and XMM1 this is exactly the version 4
/// transport encoding.
#[must_use]
pub fn movss_from_esp(reg: usize, disp: u32) -> Vec<u8> {
    let mut v = vec![0xF3, 0x0F, 0x10, 0x84 | reg_field(reg), 0x24];
    v.extend_from_slice(&disp.to_le_bytes());
    v
}

/// `movlps [eax + disp], xmm<reg>` then `movhps [eax + disp + 8], xmm<reg>`
/// (both with 32-bit displacements, 14 bytes): the 16 bytes of the
/// register into the log entry `eax` points at. For XMM0 and XMM1 at their
/// base slots this is exactly the version 4 logging encoding.
#[must_use]
pub fn log_store(reg: usize, disp: u32) -> Vec<u8> {
    let modrm = 0x80 | reg_field(reg);
    let mut v = vec![0x0F, 0x13, modrm];
    v.extend_from_slice(&disp.to_le_bytes());
    v.extend_from_slice(&[0x0F, 0x17, modrm]);
    v.extend_from_slice(&disp.wrapping_add(8).to_le_bytes());
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keys() -> XmmCallKeys {
        XmmCallKeys::default()
    }

    #[test]
    fn legacy_keys_map_to_registers_zero_and_one() {
        let k = XmmCallKeys {
            logxmm: true,
            xmm0_from_stack: Some(0),
            logxmm1: true,
            xmm1_from_stack: Some(1),
            ..keys()
        };
        let c = XmmCallCfg::merge(1, &k, 2).unwrap();
        assert!(c.log[0] && c.log[1] && !c.log[2]);
        assert_eq!(c.from_stack[0], Some(0));
        assert_eq!(c.from_stack[1], Some(1));
        assert!(c.any_transport());
        assert!(c.ext_logged().is_empty());
        // Legacy transports keep their unchecked behaviour (no nargs bound,
        // no logging requirement): existing contracts must not change.
        let k = XmmCallKeys {
            xmm0_from_stack: Some(5),
            ..keys()
        };
        assert!(XmmCallCfg::merge(1, &k, 0).is_ok());
    }

    #[test]
    fn new_keys_cover_all_eight_registers() {
        let k = XmmCallKeys {
            logxmm_regs: vec![2, 5, 7],
            xmm_from_stack: vec![(2, 0), (5, 1)],
            ..keys()
        };
        let c = XmmCallCfg::merge(3, &k, 2).unwrap();
        assert_eq!(c.ext_logged(), vec![2, 5, 7]);
        assert_eq!(c.from_stack[2], Some(0));
        assert_eq!(c.from_stack[5], Some(1));
        assert_eq!(c.from_stack[7], None);
    }

    #[test]
    fn new_keys_fail_closed() {
        let bad_reg = XmmCallKeys {
            logxmm_regs: vec![8],
            ..keys()
        };
        assert!(XmmCallCfg::merge(1, &bad_reg, 1).is_err());
        let unlogged = XmmCallKeys {
            xmm_from_stack: vec![(3, 0)],
            ..keys()
        };
        let e = XmmCallCfg::merge(1, &unlogged, 1).unwrap_err();
        assert!(e.contains("compared nowhere"), "{e}");
        let past_nargs = XmmCallKeys {
            logxmm_regs: vec![3],
            xmm_from_stack: vec![(3, 2)],
            ..keys()
        };
        assert!(XmmCallCfg::merge(1, &past_nargs, 2).is_err());
        let conflict = XmmCallKeys {
            logxmm: true,
            xmm0_from_stack: Some(0),
            logxmm_regs: vec![0],
            xmm_from_stack: vec![(0, 1)],
            ..keys()
        };
        assert!(XmmCallCfg::merge(1, &conflict, 2).is_err());
        let agree = XmmCallKeys {
            logxmm: true,
            xmm0_from_stack: Some(1),
            xmm_from_stack: vec![(0, 1)],
            ..keys()
        };
        assert!(XmmCallCfg::merge(1, &agree, 2).is_ok());
    }

    #[test]
    fn slots_keep_legacy_offsets_and_extend_past_them() {
        assert_eq!(slot(0), XmmSlot::Base(224));
        assert_eq!(slot(1), XmmSlot::Base(240));
        assert_eq!(slot(2), XmmSlot::Ext(0));
        assert_eq!(slot(7), XmmSlot::Ext(80));
    }

    #[test]
    fn encodings_match_version_four_for_xmm0_and_xmm1() {
        // Version 4 emitted F3 0F 10 84 24 disp32 (xmm0) and modrm 8C (xmm1).
        assert_eq!(
            movss_from_esp(0, 4),
            vec![0xF3, 0x0F, 0x10, 0x84, 0x24, 4, 0, 0, 0]
        );
        assert_eq!(movss_from_esp(1, 8)[3], 0x8C);
        assert_eq!(movss_from_esp(5, 8)[3], 0xAC);
        // Version 4 logged xmm0 with 0F 13 80 / 0F 17 80 at 224 and 232.
        let v = log_store(0, 224);
        assert_eq!(&v[..3], &[0x0F, 0x13, 0x80]);
        assert_eq!(&v[3..7], &224u32.to_le_bytes());
        assert_eq!(&v[7..10], &[0x0F, 0x17, 0x80]);
        assert_eq!(&v[10..14], &232u32.to_le_bytes());
        assert_eq!(log_store(1, 240)[2], 0x88);
        assert_eq!(log_store(7, 0)[2], 0xB8);
    }
}
