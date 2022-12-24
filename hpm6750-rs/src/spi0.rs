#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Transfer Format Register"]
    pub transfmt: TRANSFMT,
    #[doc = "0x14 - Direct IO Control Register"]
    pub directio: DIRECTIO,
    _reserved2: [u8; 0x08],
    #[doc = "0x20 - Transfer Control Register"]
    pub transctrl: TRANSCTRL,
    #[doc = "0x24 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x28 - Address Register"]
    pub addr: ADDR,
    #[doc = "0x2c - Data Register"]
    pub data: DATA,
    #[doc = "0x30 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x34 - Status Register"]
    pub status: STATUS,
    #[doc = "0x38 - Interrupt Enable Register"]
    pub intren: INTREN,
    #[doc = "0x3c - Interrupt Status Register"]
    pub intrst: INTRST,
    #[doc = "0x40 - Interface Timing Register"]
    pub timing: TIMING,
    _reserved11: [u8; 0x1c],
    #[doc = "0x60 - Slave Status Register"]
    pub slvst: SLVST,
    #[doc = "0x64 - Slave Data Count Register"]
    pub slvdatacnt: SLVDATACNT,
    _reserved13: [u8; 0x14],
    #[doc = "0x7c - Configuration Register"]
    pub config: CONFIG,
}
#[doc = "TRANSFMT (rw) register accessor: an alias for `Reg<TRANSFMT_SPEC>`"]
pub type TRANSFMT = crate::Reg<transfmt::TRANSFMT_SPEC>;
#[doc = "Transfer Format Register"]
pub mod transfmt;
#[doc = "DIRECTIO (rw) register accessor: an alias for `Reg<DIRECTIO_SPEC>`"]
pub type DIRECTIO = crate::Reg<directio::DIRECTIO_SPEC>;
#[doc = "Direct IO Control Register"]
pub mod directio;
#[doc = "TRANSCTRL (rw) register accessor: an alias for `Reg<TRANSCTRL_SPEC>`"]
pub type TRANSCTRL = crate::Reg<transctrl::TRANSCTRL_SPEC>;
#[doc = "Transfer Control Register"]
pub mod transctrl;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "ADDR (rw) register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address Register"]
pub mod addr;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data Register"]
pub mod data;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "INTREN (rw) register accessor: an alias for `Reg<INTREN_SPEC>`"]
pub type INTREN = crate::Reg<intren::INTREN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod intren;
#[doc = "INTRST (rw) register accessor: an alias for `Reg<INTRST_SPEC>`"]
pub type INTRST = crate::Reg<intrst::INTRST_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod intrst;
#[doc = "TIMING (rw) register accessor: an alias for `Reg<TIMING_SPEC>`"]
pub type TIMING = crate::Reg<timing::TIMING_SPEC>;
#[doc = "Interface Timing Register"]
pub mod timing;
#[doc = "SLVST (rw) register accessor: an alias for `Reg<SLVST_SPEC>`"]
pub type SLVST = crate::Reg<slvst::SLVST_SPEC>;
#[doc = "Slave Status Register"]
pub mod slvst;
#[doc = "SLVDATACNT (r) register accessor: an alias for `Reg<SLVDATACNT_SPEC>`"]
pub type SLVDATACNT = crate::Reg<slvdatacnt::SLVDATACNT_SPEC>;
#[doc = "Slave Data Count Register"]
pub mod slvdatacnt;
#[doc = "CONFIG (r) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration Register"]
pub mod config;
