#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x18 - Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x1c - Address Register"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x20 - Data Register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x24 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x28 - Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x2c - Setup Register"]
    pub setup: crate::Reg<setup::SETUP_SPEC>,
    #[doc = "0x30 - I2C Timing Paramater Multiplier"]
    pub tpm: crate::Reg<tpm::TPM_SPEC>,
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod inten;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address Register"]
pub mod addr;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data Register"]
pub mod data;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "SETUP register accessor: an alias for `Reg<SETUP_SPEC>`"]
pub type SETUP = crate::Reg<setup::SETUP_SPEC>;
#[doc = "Setup Register"]
pub mod setup;
#[doc = "TPM register accessor: an alias for `Reg<TPM_SPEC>`"]
pub type TPM = crate::Reg<tpm::TPM_SPEC>;
#[doc = "I2C Timing Paramater Multiplier"]
pub mod tpm;
