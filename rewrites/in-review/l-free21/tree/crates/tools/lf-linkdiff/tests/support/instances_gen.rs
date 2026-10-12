// Generated from the verified rewrites: one entry per instance of this batch.
// WORDS are the head and save words the batch touches; NODES are the static nodes.
#![allow(dead_code)]

use lf_linkdiff::rewrites;

/// One verified rewrite and the three addresses it names.
pub struct Instance {
    /// The batch name of the rewrite (its file name stem).
    pub name: &'static str,
    /// The original's address of the function.
    pub address: &'static str,
    /// The verified rewrite export, called in the test runtime.
    pub call: extern "cdecl" fn() -> u32,
    /// The head word the rewrite reads and overwrites with the node.
    pub head: u32,
    /// The word the rewrite saves the old head into.
    pub save: u32,
    /// The static node the rewrite pushes.
    pub node: u32,
}

/// Every word the batch touches.
pub const WORDS: &[u32] = &[0x017AD16C, 0x0110DBDC, 0x0110DBC4, 0x0110DBAC, 0x017AD1B8, 0x0110DC00, 0x0110DC20, 0x0110DC10, 0x0110DBF0, 0x0110DD84, 0x0110DD94, 0x0110DD40, 0x0110DDC0, 0x0110DC64, 0x0110DC34, 0x0110DD28, 0x0110DCF4, 0x0110DC7C, 0x0110DD58, 0x0110DC4C, 0x0110DC94, 0x0110DD0C, 0x0110DD70, 0x0110DDA8, 0x0110DCC4, 0x0110DCAC, 0x0110DCDC, 0x0110DDF0, 0x0110DE50, 0x0110DE20, 0x0110DE68, 0x0110DE08, 0x0110DE80, 0x0110DE98, 0x0110DDD8, 0x0110DE38, 0x0110DEDC, 0x0110DEFC, 0x0110DEBC, 0x0110DECC, 0x0110DEEC, 0x0110DEAC, 0x017ACD24, 0x0110E138, 0x0110E178, 0x0110E170, 0x0110E6A8, 0x0110E6C4, 0x0110E6F4, 0x0110EA1C, 0x0110E9E4, 0x0110E9D4, 0x0110E9C4, 0x0110E9A4, 0x0110E974, 0x0110EA2C, 0x0110E9F4, 0x0110EA3C, 0x0110EA14, 0x0110EA4C, 0x0110E9B4];

/// Every static node the batch pushes.
pub const NODES: &[u32] = &[0x0110DBCC, 0x0110DBB4, 0x0110DB9C, 0x0110DBF4, 0x0110DC14, 0x0110DC04, 0x0110DBE4, 0x0110DD78, 0x0110DD88, 0x0110DD30, 0x0110DDB0, 0x0110DC54, 0x0110DC24, 0x0110DD18, 0x0110DCE4, 0x0110DC6C, 0x0110DD48, 0x0110DC3C, 0x0110DC84, 0x0110DCFC, 0x0110DD60, 0x0110DD98, 0x0110DCB4, 0x0110DC9C, 0x0110DCCC, 0x0110DDE0, 0x0110DE40, 0x0110DE10, 0x0110DE58, 0x0110DDF8, 0x0110DE70, 0x0110DE88, 0x0110DDC8, 0x0110DE28, 0x0110DED0, 0x0110DEF0, 0x0110DEB0, 0x0110DEC0, 0x0110DEE0, 0x0110DEA0, 0x0110E134, 0x0110E174, 0x0110E16C, 0x0110E698, 0x0110E6B4, 0x0110E6E4, 0x0110EA18, 0x0110E9D8, 0x0110E9C8, 0x0110E9B8, 0x0110E998, 0x0110E968, 0x0110EA20, 0x0110E9E8, 0x0110EA30, 0x0110EA08, 0x0110EA40, 0x0110E9A8];

/// One entry per instance of this batch.
pub const INSTANCES: &[Instance] = &[
    Instance { name: "fn_00E5C050", address: "0x00E5C050", call: rewrites::fn_00E5C050::rw_00e5c050, head: 0x017AD16C, save: 0x0110DBDC, node: 0x0110DBCC },
    Instance { name: "fn_00E5C070", address: "0x00E5C070", call: rewrites::fn_00E5C070::rw_00e5c070, head: 0x017AD16C, save: 0x0110DBC4, node: 0x0110DBB4 },
    Instance { name: "fn_00E5C090", address: "0x00E5C090", call: rewrites::fn_00E5C090::rw_00e5c090, head: 0x017AD16C, save: 0x0110DBAC, node: 0x0110DB9C },
    Instance { name: "fn_00E5C0B0", address: "0x00E5C0B0", call: rewrites::fn_00E5C0B0::rw_00e5c0b0, head: 0x017AD1B8, save: 0x0110DC00, node: 0x0110DBF4 },
    Instance { name: "fn_00E5C0D0", address: "0x00E5C0D0", call: rewrites::fn_00E5C0D0::rw_00e5c0d0, head: 0x017AD1B8, save: 0x0110DC20, node: 0x0110DC14 },
    Instance { name: "fn_00E5C0F0", address: "0x00E5C0F0", call: rewrites::fn_00E5C0F0::rw_00e5c0f0, head: 0x017AD1B8, save: 0x0110DC10, node: 0x0110DC04 },
    Instance { name: "fn_00E5C110", address: "0x00E5C110", call: rewrites::fn_00E5C110::rw_00e5c110, head: 0x017AD1B8, save: 0x0110DBF0, node: 0x0110DBE4 },
    Instance { name: "fn_00E5C130", address: "0x00E5C130", call: rewrites::fn_00E5C130::rw_00e5c130, head: 0x017AD1B8, save: 0x0110DD84, node: 0x0110DD78 },
    Instance { name: "fn_00E5C150", address: "0x00E5C150", call: rewrites::fn_00E5C150::rw_00e5c150, head: 0x017AD1B8, save: 0x0110DD94, node: 0x0110DD88 },
    Instance { name: "fn_00E5C170", address: "0x00E5C170", call: rewrites::fn_00E5C170::rw_00e5c170, head: 0x017AD16C, save: 0x0110DD40, node: 0x0110DD30 },
    Instance { name: "fn_00E5C190", address: "0x00E5C190", call: rewrites::fn_00E5C190::rw_00e5c190, head: 0x017AD16C, save: 0x0110DDC0, node: 0x0110DDB0 },
    Instance { name: "fn_00E5C260", address: "0x00E5C260", call: rewrites::fn_00E5C260::rw_00e5c260, head: 0x017AD16C, save: 0x0110DC64, node: 0x0110DC54 },
    Instance { name: "fn_00E5C280", address: "0x00E5C280", call: rewrites::fn_00E5C280::rw_00e5c280, head: 0x017AD16C, save: 0x0110DC34, node: 0x0110DC24 },
    Instance { name: "fn_00E5C2A0", address: "0x00E5C2A0", call: rewrites::fn_00E5C2A0::rw_00e5c2a0, head: 0x017AD16C, save: 0x0110DD28, node: 0x0110DD18 },
    Instance { name: "fn_00E5C2C0", address: "0x00E5C2C0", call: rewrites::fn_00E5C2C0::rw_00e5c2c0, head: 0x017AD16C, save: 0x0110DCF4, node: 0x0110DCE4 },
    Instance { name: "fn_00E5C2E0", address: "0x00E5C2E0", call: rewrites::fn_00E5C2E0::rw_00e5c2e0, head: 0x017AD16C, save: 0x0110DC7C, node: 0x0110DC6C },
    Instance { name: "fn_00E5C300", address: "0x00E5C300", call: rewrites::fn_00E5C300::rw_00e5c300, head: 0x017AD16C, save: 0x0110DD58, node: 0x0110DD48 },
    Instance { name: "fn_00E5C320", address: "0x00E5C320", call: rewrites::fn_00E5C320::rw_00e5c320, head: 0x017AD16C, save: 0x0110DC4C, node: 0x0110DC3C },
    Instance { name: "fn_00E5C340", address: "0x00E5C340", call: rewrites::fn_00E5C340::rw_00e5c340, head: 0x017AD16C, save: 0x0110DC94, node: 0x0110DC84 },
    Instance { name: "fn_00E5C360", address: "0x00E5C360", call: rewrites::fn_00E5C360::rw_00e5c360, head: 0x017AD16C, save: 0x0110DD0C, node: 0x0110DCFC },
    Instance { name: "fn_00E5C380", address: "0x00E5C380", call: rewrites::fn_00E5C380::rw_00e5c380, head: 0x017AD16C, save: 0x0110DD70, node: 0x0110DD60 },
    Instance { name: "fn_00E5C3A0", address: "0x00E5C3A0", call: rewrites::fn_00E5C3A0::rw_00e5c3a0, head: 0x017AD16C, save: 0x0110DDA8, node: 0x0110DD98 },
    Instance { name: "fn_00E5C3C0", address: "0x00E5C3C0", call: rewrites::fn_00E5C3C0::rw_00e5c3c0, head: 0x017AD16C, save: 0x0110DCC4, node: 0x0110DCB4 },
    Instance { name: "fn_00E5C3E0", address: "0x00E5C3E0", call: rewrites::fn_00E5C3E0::rw_00e5c3e0, head: 0x017AD16C, save: 0x0110DCAC, node: 0x0110DC9C },
    Instance { name: "fn_00E5C400", address: "0x00E5C400", call: rewrites::fn_00E5C400::rw_00e5c400, head: 0x017AD16C, save: 0x0110DCDC, node: 0x0110DCCC },
    Instance { name: "fn_00E5C460", address: "0x00E5C460", call: rewrites::fn_00E5C460::rw_00e5c460, head: 0x017AD16C, save: 0x0110DDF0, node: 0x0110DDE0 },
    Instance { name: "fn_00E5C480", address: "0x00E5C480", call: rewrites::fn_00E5C480::rw_00e5c480, head: 0x017AD16C, save: 0x0110DE50, node: 0x0110DE40 },
    Instance { name: "fn_00E5C4A0", address: "0x00E5C4A0", call: rewrites::fn_00E5C4A0::rw_00e5c4a0, head: 0x017AD16C, save: 0x0110DE20, node: 0x0110DE10 },
    Instance { name: "fn_00E5C4C0", address: "0x00E5C4C0", call: rewrites::fn_00E5C4C0::rw_00e5c4c0, head: 0x017AD16C, save: 0x0110DE68, node: 0x0110DE58 },
    Instance { name: "fn_00E5C4E0", address: "0x00E5C4E0", call: rewrites::fn_00E5C4E0::rw_00e5c4e0, head: 0x017AD16C, save: 0x0110DE08, node: 0x0110DDF8 },
    Instance { name: "fn_00E5C500", address: "0x00E5C500", call: rewrites::fn_00E5C500::rw_00e5c500, head: 0x017AD16C, save: 0x0110DE80, node: 0x0110DE70 },
    Instance { name: "fn_00E5C520", address: "0x00E5C520", call: rewrites::fn_00E5C520::rw_00e5c520, head: 0x017AD16C, save: 0x0110DE98, node: 0x0110DE88 },
    Instance { name: "fn_00E5C540", address: "0x00E5C540", call: rewrites::fn_00E5C540::rw_00e5c540, head: 0x017AD16C, save: 0x0110DDD8, node: 0x0110DDC8 },
    Instance { name: "fn_00E5C560", address: "0x00E5C560", call: rewrites::fn_00E5C560::rw_00e5c560, head: 0x017AD16C, save: 0x0110DE38, node: 0x0110DE28 },
    Instance { name: "fn_00E5C580", address: "0x00E5C580", call: rewrites::fn_00E5C580::rw_00e5c580, head: 0x017AD1B8, save: 0x0110DEDC, node: 0x0110DED0 },
    Instance { name: "fn_00E5C5A0", address: "0x00E5C5A0", call: rewrites::fn_00E5C5A0::rw_00e5c5a0, head: 0x017AD1B8, save: 0x0110DEFC, node: 0x0110DEF0 },
    Instance { name: "fn_00E5C5C0", address: "0x00E5C5C0", call: rewrites::fn_00E5C5C0::rw_00e5c5c0, head: 0x017AD1B8, save: 0x0110DEBC, node: 0x0110DEB0 },
    Instance { name: "fn_00E5C5E0", address: "0x00E5C5E0", call: rewrites::fn_00E5C5E0::rw_00e5c5e0, head: 0x017AD1B8, save: 0x0110DECC, node: 0x0110DEC0 },
    Instance { name: "fn_00E5C600", address: "0x00E5C600", call: rewrites::fn_00E5C600::rw_00e5c600, head: 0x017AD1B8, save: 0x0110DEEC, node: 0x0110DEE0 },
    Instance { name: "fn_00E5C620", address: "0x00E5C620", call: rewrites::fn_00E5C620::rw_00e5c620, head: 0x017AD1B8, save: 0x0110DEAC, node: 0x0110DEA0 },
    Instance { name: "fn_00E5CB10", address: "0x00E5CB10", call: rewrites::fn_00E5CB10::rw_00e5cb10, head: 0x017ACD24, save: 0x0110E138, node: 0x0110E134 },
    Instance { name: "fn_00E5CB30", address: "0x00E5CB30", call: rewrites::fn_00E5CB30::rw_00e5cb30, head: 0x017ACD24, save: 0x0110E178, node: 0x0110E174 },
    Instance { name: "fn_00E5CB50", address: "0x00E5CB50", call: rewrites::fn_00E5CB50::rw_00e5cb50, head: 0x017ACD24, save: 0x0110E170, node: 0x0110E16C },
    Instance { name: "fn_00E5DB10", address: "0x00E5DB10", call: rewrites::fn_00E5DB10::rw_00e5db10, head: 0x017AD16C, save: 0x0110E6A8, node: 0x0110E698 },
    Instance { name: "fn_00E5DB30", address: "0x00E5DB30", call: rewrites::fn_00E5DB30::rw_00e5db30, head: 0x017AD16C, save: 0x0110E6C4, node: 0x0110E6B4 },
    Instance { name: "fn_00E5DB50", address: "0x00E5DB50", call: rewrites::fn_00E5DB50::rw_00e5db50, head: 0x017AD16C, save: 0x0110E6F4, node: 0x0110E6E4 },
    Instance { name: "fn_00E5E600", address: "0x00E5E600", call: rewrites::fn_00E5E600::rw_00e5e600, head: 0x017ACD24, save: 0x0110EA1C, node: 0x0110EA18 },
    Instance { name: "fn_00E5E620", address: "0x00E5E620", call: rewrites::fn_00E5E620::rw_00e5e620, head: 0x017AD1B8, save: 0x0110E9E4, node: 0x0110E9D8 },
    Instance { name: "fn_00E5E640", address: "0x00E5E640", call: rewrites::fn_00E5E640::rw_00e5e640, head: 0x017AD1B8, save: 0x0110E9D4, node: 0x0110E9C8 },
    Instance { name: "fn_00E5E660", address: "0x00E5E660", call: rewrites::fn_00E5E660::rw_00e5e660, head: 0x017AD1B8, save: 0x0110E9C4, node: 0x0110E9B8 },
    Instance { name: "fn_00E5E680", address: "0x00E5E680", call: rewrites::fn_00E5E680::rw_00e5e680, head: 0x017AD1B8, save: 0x0110E9A4, node: 0x0110E998 },
    Instance { name: "fn_00E5E6A0", address: "0x00E5E6A0", call: rewrites::fn_00E5E6A0::rw_00e5e6a0, head: 0x017AD1B8, save: 0x0110E974, node: 0x0110E968 },
    Instance { name: "fn_00E5E6C0", address: "0x00E5E6C0", call: rewrites::fn_00E5E6C0::rw_00e5e6c0, head: 0x017AD1B8, save: 0x0110EA2C, node: 0x0110EA20 },
    Instance { name: "fn_00E5E6E0", address: "0x00E5E6E0", call: rewrites::fn_00E5E6E0::rw_00e5e6e0, head: 0x017AD1B8, save: 0x0110E9F4, node: 0x0110E9E8 },
    Instance { name: "fn_00E5E700", address: "0x00E5E700", call: rewrites::fn_00E5E700::rw_00e5e700, head: 0x017AD1B8, save: 0x0110EA3C, node: 0x0110EA30 },
    Instance { name: "fn_00E5E720", address: "0x00E5E720", call: rewrites::fn_00E5E720::rw_00e5e720, head: 0x017AD1B8, save: 0x0110EA14, node: 0x0110EA08 },
    Instance { name: "fn_00E5E740", address: "0x00E5E740", call: rewrites::fn_00E5E740::rw_00e5e740, head: 0x017AD1B8, save: 0x0110EA4C, node: 0x0110EA40 },
    Instance { name: "fn_00E5E760", address: "0x00E5E760", call: rewrites::fn_00E5E760::rw_00e5e760, head: 0x017AD1B8, save: 0x0110E9B4, node: 0x0110E9A8 },
];
