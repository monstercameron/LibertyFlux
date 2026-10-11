//! Banked voice slots: slot bytes resolved through a row-pointer table.
//!
//! Lifted from the five verified rewrites that share this shape: the
//! object carries a bank byte at `+0x40`, slot bytes at `+0x48`, a
//! parameter word at `+0x54` and an indexed-slot byte at `+0xB0`. Two
//! globals hold the row-table base (`TABLE`) and the node stride
//! (`STRIDE`). Bank `b` selects the row word at
//! `TABLE + b * ROW_STRIDE + ROW_SLOT`, and slot `s` selects the node at
//! `row + STRIDE * s` (all wrapping). A slot byte of [`NO_SLOT`] means
//! empty. The voice nodes themselves are owned elsewhere; routines that
//! act on them take a [`VoiceNode`] key and reach the node through a
//! trait, so the proof rebuilds each 32-bit address from the key per
//! case.
//!
//! Row bases and the table base travel as opaque `u32` values: the lift
//! does wrapping arithmetic on them exactly like the original but never
//! dereferences them, so the code is pointer-width independent.

/// Bank byte: selects the row of the row-pointer table.
pub const BANK_OFF: u32 = 0x40;
/// First slot byte.
pub const SLOT_OFF: u32 = 0x48;
/// Parameter word read by the retrigger.
pub const PARAM_OFF: u32 = 0x54;
/// Indexed-slot byte read by the probe.
pub const INDEXED_OFF: u32 = 0xB0;
/// Slot byte meaning "no slot".
pub const NO_SLOT: u8 = 0xFF;
/// Bytes from one bank's row word to the next.
pub const ROW_STRIDE: u32 = 0x6F40;
/// Offset of the row word inside its bank block.
pub const ROW_SLOT: u32 = 0x6F10;
/// Voice slots scanned by the probe.
pub const PROBED_SLOTS: u32 = 2;

/// A voice node selected by bank and slot: the key the traits below take
/// where the 32-bit code passes the node's address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VoiceNode {
    /// Bank byte (`+0x40`).
    pub bank: u8,
    /// Slot byte (from `+0x48`, `+0xB0`, or a chain link).
    pub slot: u8,
}

/// The subsystem state behind the `TABLE`/`STRIDE` globals: the node
/// stride, the opaque table base, and one opaque row base per bank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceBankFile {
    /// Node stride (`STRIDE` global).
    stride: u32,
    /// Row-table base (`TABLE` global), kept for the retrigger's
    /// null-target answer; never dereferenced.
    table_base: u32,
    /// Row base per bank, indexed by the bank byte.
    rows: Vec<u32>,
}

impl VoiceBankFile {
    /// A bank file with `stride`, `table_base` and the per-bank `rows`.
    #[must_use]
    pub const fn from_parts(stride: u32, table_base: u32, rows: Vec<u32>) -> Self {
        Self {
            stride,
            table_base,
            rows,
        }
    }

    /// The row base of `bank`.
    ///
    /// # Panics
    ///
    /// When no row was recorded for `bank` (the original reads the row
    /// word at `TABLE + bank * ROW_STRIDE + ROW_SLOT` regardless).
    #[must_use]
    pub fn row_of(&self, bank: u8) -> u32 {
        *self.rows.get(usize::from(bank)).unwrap_or_else(|| {
            panic!(
                "bank {bank:#x} past the {} recorded rows: the original reads the row word regardless",
                self.rows.len()
            )
        })
    }

    /// The node's 32-bit address as an opaque value:
    /// `row + STRIDE * slot`, wrapping. Used for the null-target checks
    /// and by the proof to rebuild addresses; never dereferenced here.
    #[must_use]
    pub fn node_addr(&self, node: VoiceNode) -> u32 {
        self.row_of(node.bank)
            .wrapping_add(self.stride.wrapping_mul(u32::from(node.slot)))
    }
}

/// Runs one operation on a voice node: the op-forward's callee.
pub trait Operate {
    /// Runs the operation on `node` with `a1`, answering as the 32-bit callee.
    fn operate(&mut self, node: VoiceNode, a1: u32) -> u32;
}

impl<F: FnMut(VoiceNode, u32) -> u32> Operate for F {
    fn operate(&mut self, node: VoiceNode, a1: u32) -> u32 {
        self(node, a1)
    }
}

/// The retrigger's three callees: setup, retrigger and chain.
pub trait Retrigger {
    /// The setup call on `node` with the object's parameter word (the
    /// 32-bit call's trailing zero is a placeholder and is not passed).
    fn setup(&mut self, node: VoiceNode, param: u32);
    /// The retrigger call on `node` with `a1`.
    fn retrigger(&mut self, node: VoiceNode, a1: u32);
    /// The chain call on the object itself, whose answer is returned.
    fn chain(&mut self) -> u32;
}

/// The probe's reads and callees: node liveness and tags plus the setup
/// pair and the offer.
pub trait Probe {
    /// Whether the indexed node is live (the byte at node `+0x72`).
    fn entry_live(&mut self, node: VoiceNode) -> bool;
    /// The first setup call, on the object with the indexed node.
    fn setup_this(&mut self, node: VoiceNode);
    /// The second setup call, on the indexed node.
    fn setup_entry(&mut self, node: VoiceNode);
    /// The entry's tag (the half-word at node `+6`).
    fn entry_tag(&mut self, node: VoiceNode) -> u16;
    /// The offer call on `node` with the stimulus; the low byte decides.
    fn offer(&mut self, node: VoiceNode, arg0: u32) -> u32;
}

/// One banked-slot object: bank, slot bytes, parameter and indexed byte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BankedSlots {
    /// Bank byte (`+0x40`).
    bank: u8,
    /// Slot bytes (`+0x48` onwards).
    slots: Vec<u8>,
    /// Parameter word (`+0x54`).
    param: u32,
    /// Indexed-slot byte (`+0xB0`).
    indexed: u8,
}

impl BankedSlots {
    /// An object with `bank`, `slots`, `param` and `indexed`.
    #[must_use]
    pub const fn from_parts(bank: u8, slots: Vec<u8>, param: u32, indexed: u8) -> Self {
        Self {
            bank,
            slots,
            param,
            indexed,
        }
    }

    /// Reads slot byte `index`.
    ///
    /// # Panics
    ///
    /// When `index` is past the modelled slots (the original reads the
    /// neighbouring bytes regardless).
    fn slot(&self, index: u32) -> u8 {
        *self.slots.get(index as usize).unwrap_or_else(|| {
            panic!(
                "slot index {index:#x} past the {} modelled slots: the original reads on regardless",
                self.slots.len()
            )
        })
    }

    /// Resolves `value` to a slot byte and stores it at `index`.
    ///
    /// A null `value` stores [`NO_SLOT`]. Otherwise the row base of the
    /// bank is subtracted from `value` (wrapping), the difference divided
    /// by the stride, and the low byte of the quotient stored. Answers
    /// `index`.
    ///
    /// # Panics
    ///
    /// When `index` is past the modelled slots, or the stride is zero
    /// with a nonzero value (the original divides regardless and faults).
    pub fn lookup_store(&mut self, file: &VoiceBankFile, index: u32, value: u32) -> u32 {
        if value == 0 {
            let len = self.slots.len();
            let slot = self.slots.get_mut(index as usize).unwrap_or_else(|| {
                panic!(
                    "slot index {index:#x} past the {len} modelled slots: the original writes on regardless"
                )
            });
            *slot = NO_SLOT;
            return index;
        }
        let row = file.row_of(self.bank);
        let quot = value.wrapping_sub(row) / file.stride;
        let len = self.slots.len();
        let slot = self.slots.get_mut(index as usize).unwrap_or_else(|| {
            panic!(
                "slot index {index:#x} past the {len} modelled slots: the original writes on regardless"
            )
        });
        // The original stores the quotient's low byte.
        #[allow(clippy::cast_possible_truncation)]
        let byte = quot as u8;
        *slot = byte;
        index
    }

    /// Looks up the node for slot `idx`: `None` for an empty slot.
    ///
    /// # Panics
    ///
    /// When `idx` is past the modelled slots.
    #[must_use]
    pub fn node_lookup(&self, file: &VoiceBankFile, idx: u32) -> Option<VoiceNode> {
        let slot = self.slot(idx);
        if slot == NO_SLOT {
            return None;
        }
        // The row read is observable (it can fault past the table), so it
        // happens here even though only the key is returned; the proof
        // rebuilds the 32-bit address from the key per case.
        let _ = file.row_of(self.bank);
        Some(VoiceNode {
            bank: self.bank,
            slot,
        })
    }

    /// Forwards one operation to the selected voice node.
    ///
    /// An empty slot, or a null target address, answers 0 without calling.
    /// Otherwise the operation runs on the node with `a1` and its answer
    /// is returned.
    ///
    /// # Panics
    ///
    /// When the object models no slots.
    pub fn op_forward(&self, file: &VoiceBankFile, a1: u32, op: &mut impl Operate) -> u32 {
        let slot = self.slot(0);
        if slot == NO_SLOT {
            return 0;
        }
        let node = VoiceNode {
            bank: self.bank,
            slot,
        };
        if file.node_addr(node) == 0 {
            return 0;
        }
        op.operate(node, a1)
    }

    /// Retriggers the selected voice node, then chains on.
    ///
    /// An empty slot answers `0xFF` at once. A null target answers the
    /// table base. Otherwise the setup call runs on the node with the
    /// parameter word, the retrigger call with `a1`, and the chain
    /// answer is returned.
    ///
    /// # Panics
    ///
    /// When the object models no slots.
    pub fn retrigger(&self, file: &VoiceBankFile, a1: u32, ops: &mut impl Retrigger) -> u32 {
        let slot = self.slot(0);
        if slot == NO_SLOT {
            return u32::from(NO_SLOT);
        }
        let node = VoiceNode {
            bank: self.bank,
            slot,
        };
        if file.node_addr(node) == 0 {
            return file.table_base;
        }
        ops.setup(node, self.param);
        ops.retrigger(node, a1);
        ops.chain()
    }

    /// Probes the voice slots for a live entry.
    ///
    /// When the indexed node is live, both setup steps run. Then each of
    /// the two slot bytes is scanned: an empty slot, or a null entry, is
    /// skipped; tag 2 is offered the stimulus and latches when the
    /// offer's low byte is nonzero; tag 1 latches unconditionally.
    /// Answers whether any slot latched.
    ///
    /// # Panics
    ///
    /// When the object models fewer than two slots.
    pub fn probe(&self, file: &VoiceBankFile, arg0: u32, probe: &mut impl Probe) -> bool {
        let indexed = VoiceNode {
            bank: self.bank,
            slot: self.indexed,
        };
        if probe.entry_live(indexed) {
            probe.setup_this(indexed);
            probe.setup_entry(indexed);
        }
        let mut latched = false;
        for i in 0..PROBED_SLOTS {
            let slot = self.slot(i);
            if slot == NO_SLOT {
                continue;
            }
            let node = VoiceNode {
                bank: self.bank,
                slot,
            };
            if file.node_addr(node) == 0 {
                continue;
            }
            let tag = probe.entry_tag(node);
            if tag == 2 {
                if probe.offer(node, arg0) & 0xFF != 0 {
                    latched = true;
                }
            } else if tag == 1 {
                latched = true;
            }
        }
        latched
    }
}
