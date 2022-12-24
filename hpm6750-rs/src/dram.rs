#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Bus (AXI) Weight Control Register 0"]
    pub bmw0: BMW0,
    #[doc = "0x0c - Bus (AXI) Weight Control Register 1"]
    pub bmw1: BMW1,
    #[doc = "0x10 - Base Register 0 (for SDRAM CS0 device)"]
    pub br_base0: BR_BASE0,
    #[doc = "0x14 - Base Register 1 (for SDRAM CS1 device)"]
    pub br_base1: BR_BASE1,
    _reserved5: [u8; 0x20],
    #[doc = "0x38 - Interrupt Enable Register"]
    pub inten: INTEN,
    #[doc = "0x3c - Interrupt Status Register"]
    pub intr: INTR,
    #[doc = "0x40 - SDRAM Control Register 0"]
    pub sdrctrl0: SDRCTRL0,
    #[doc = "0x44 - SDRAM Control Register 1"]
    pub sdrctrl1: SDRCTRL1,
    #[doc = "0x48 - SDRAM Control Register 2"]
    pub sdrctrl2: SDRCTRL2,
    #[doc = "0x4c - SDRAM Control Register 3"]
    pub sdrctrl3: SDRCTRL3,
    _reserved11: [u8; 0x40],
    #[doc = "0x90 - IP Command Control Register 0"]
    pub saddr: SADDR,
    #[doc = "0x94 - IP Command Control Register 1"]
    pub datsz: DATSZ,
    #[doc = "0x98 - IP Command Control Register 2"]
    pub bytemsk: BYTEMSK,
    #[doc = "0x9c - IP Command Register"]
    pub ipcmd: IPCMD,
    #[doc = "0xa0 - TX DATA Register"]
    pub iptx: IPTX,
    _reserved16: [u8; 0x0c],
    #[doc = "0xb0 - RX DATA Register"]
    pub iprx: IPRX,
    _reserved17: [u8; 0x0c],
    #[doc = "0xc0 - Status Register 0"]
    pub stat0: STAT0,
    _reserved18: [u8; 0x8c],
    #[doc = "0x150 - Delay Line Config Register"]
    pub dlycfg: DLYCFG,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "BMW0 (rw) register accessor: an alias for `Reg<BMW0_SPEC>`"]
pub type BMW0 = crate::Reg<bmw0::BMW0_SPEC>;
#[doc = "Bus (AXI) Weight Control Register 0"]
pub mod bmw0;
#[doc = "BMW1 (rw) register accessor: an alias for `Reg<BMW1_SPEC>`"]
pub type BMW1 = crate::Reg<bmw1::BMW1_SPEC>;
#[doc = "Bus (AXI) Weight Control Register 1"]
pub mod bmw1;
#[doc = "BR_BASE0 (rw) register accessor: an alias for `Reg<BR_BASE0_SPEC>`"]
pub type BR_BASE0 = crate::Reg<br_base0::BR_BASE0_SPEC>;
#[doc = "Base Register 0 (for SDRAM CS0 device)"]
pub mod br_base0;
#[doc = "BR_BASE1 (rw) register accessor: an alias for `Reg<BR_BASE1_SPEC>`"]
pub type BR_BASE1 = crate::Reg<br_base1::BR_BASE1_SPEC>;
#[doc = "Base Register 1 (for SDRAM CS1 device)"]
pub mod br_base1;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "INTR (rw) register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod intr;
#[doc = "SDRCTRL0 (rw) register accessor: an alias for `Reg<SDRCTRL0_SPEC>`"]
pub type SDRCTRL0 = crate::Reg<sdrctrl0::SDRCTRL0_SPEC>;
#[doc = "SDRAM Control Register 0"]
pub mod sdrctrl0;
#[doc = "SDRCTRL1 (rw) register accessor: an alias for `Reg<SDRCTRL1_SPEC>`"]
pub type SDRCTRL1 = crate::Reg<sdrctrl1::SDRCTRL1_SPEC>;
#[doc = "SDRAM Control Register 1"]
pub mod sdrctrl1;
#[doc = "SDRCTRL2 (rw) register accessor: an alias for `Reg<SDRCTRL2_SPEC>`"]
pub type SDRCTRL2 = crate::Reg<sdrctrl2::SDRCTRL2_SPEC>;
#[doc = "SDRAM Control Register 2"]
pub mod sdrctrl2;
#[doc = "SDRCTRL3 (rw) register accessor: an alias for `Reg<SDRCTRL3_SPEC>`"]
pub type SDRCTRL3 = crate::Reg<sdrctrl3::SDRCTRL3_SPEC>;
#[doc = "SDRAM Control Register 3"]
pub mod sdrctrl3;
#[doc = "SADDR (rw) register accessor: an alias for `Reg<SADDR_SPEC>`"]
pub type SADDR = crate::Reg<saddr::SADDR_SPEC>;
#[doc = "IP Command Control Register 0"]
pub mod saddr;
#[doc = "DATSZ (rw) register accessor: an alias for `Reg<DATSZ_SPEC>`"]
pub type DATSZ = crate::Reg<datsz::DATSZ_SPEC>;
#[doc = "IP Command Control Register 1"]
pub mod datsz;
#[doc = "BYTEMSK (rw) register accessor: an alias for `Reg<BYTEMSK_SPEC>`"]
pub type BYTEMSK = crate::Reg<bytemsk::BYTEMSK_SPEC>;
#[doc = "IP Command Control Register 2"]
pub mod bytemsk;
#[doc = "IPCMD (rw) register accessor: an alias for `Reg<IPCMD_SPEC>`"]
pub type IPCMD = crate::Reg<ipcmd::IPCMD_SPEC>;
#[doc = "IP Command Register"]
pub mod ipcmd;
#[doc = "IPTX (rw) register accessor: an alias for `Reg<IPTX_SPEC>`"]
pub type IPTX = crate::Reg<iptx::IPTX_SPEC>;
#[doc = "TX DATA Register"]
pub mod iptx;
#[doc = "IPRX (rw) register accessor: an alias for `Reg<IPRX_SPEC>`"]
pub type IPRX = crate::Reg<iprx::IPRX_SPEC>;
#[doc = "RX DATA Register"]
pub mod iprx;
#[doc = "STAT0 (r) register accessor: an alias for `Reg<STAT0_SPEC>`"]
pub type STAT0 = crate::Reg<stat0::STAT0_SPEC>;
#[doc = "Status Register 0"]
pub mod stat0;
#[doc = "DLYCFG (rw) register accessor: an alias for `Reg<DLYCFG_SPEC>`"]
pub type DLYCFG = crate::Reg<dlycfg::DLYCFG_SPEC>;
#[doc = "Delay Line Config Register"]
pub mod dlycfg;
