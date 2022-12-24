#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x04 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x08 - Status Register"]
    pub sta: STA,
    #[doc = "0x0c - Error Registers"]
    pub err: ERR,
    #[doc = "0x10 - FIFO out to bus/cpu"]
    pub fo2b: FO2B,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - FIFO out to SDP as AES engine key"]
    pub r2sk_fo2s0: R2SK_FO2S0,
    #[doc = "0x24 - FIFO out to SDP as AES engine key"]
    pub r2sk_fo2s1: R2SK_FO2S1,
    #[doc = "0x28 - FIFO out to SDP as AES engine key"]
    pub r2sk_fo2s2: R2SK_FO2S2,
    #[doc = "0x2c - FIFO out to SDP as AES engine key"]
    pub r2sk_fo2s3: R2SK_FO2S3,
    #[doc = "0x30 - FIFO out to SDP as AES engine key"]
    pub r2sk_fo2s4: R2SK_FO2S4,
    #[doc = "0x34 - FIFO out to SDP as AES engine key"]
    pub r2sk_fo2s5: R2SK_FO2S5,
    #[doc = "0x38 - FIFO out to SDP as AES engine key"]
    pub r2sk_fo2s6: R2SK_FO2S6,
    #[doc = "0x3c - FIFO out to SDP as AES engine key"]
    pub r2sk_fo2s7: R2SK_FO2S7,
}
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STA (r) register accessor: an alias for `Reg<STA_SPEC>`"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "Status Register"]
pub mod sta;
#[doc = "ERR (r) register accessor: an alias for `Reg<ERR_SPEC>`"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "Error Registers"]
pub mod err;
#[doc = "FO2B (r) register accessor: an alias for `Reg<FO2B_SPEC>`"]
pub type FO2B = crate::Reg<fo2b::FO2B_SPEC>;
#[doc = "FIFO out to bus/cpu"]
pub mod fo2b;
#[doc = "R2SK_FO2S0 (r) register accessor: an alias for `Reg<R2SK_FO2S0_SPEC>`"]
pub type R2SK_FO2S0 = crate::Reg<r2sk_fo2s0::R2SK_FO2S0_SPEC>;
#[doc = "FIFO out to SDP as AES engine key"]
pub mod r2sk_fo2s0;
#[doc = "R2SK_FO2S1 (r) register accessor: an alias for `Reg<R2SK_FO2S1_SPEC>`"]
pub type R2SK_FO2S1 = crate::Reg<r2sk_fo2s1::R2SK_FO2S1_SPEC>;
#[doc = "FIFO out to SDP as AES engine key"]
pub mod r2sk_fo2s1;
#[doc = "R2SK_FO2S2 (r) register accessor: an alias for `Reg<R2SK_FO2S2_SPEC>`"]
pub type R2SK_FO2S2 = crate::Reg<r2sk_fo2s2::R2SK_FO2S2_SPEC>;
#[doc = "FIFO out to SDP as AES engine key"]
pub mod r2sk_fo2s2;
#[doc = "R2SK_FO2S3 (r) register accessor: an alias for `Reg<R2SK_FO2S3_SPEC>`"]
pub type R2SK_FO2S3 = crate::Reg<r2sk_fo2s3::R2SK_FO2S3_SPEC>;
#[doc = "FIFO out to SDP as AES engine key"]
pub mod r2sk_fo2s3;
#[doc = "R2SK_FO2S4 (r) register accessor: an alias for `Reg<R2SK_FO2S4_SPEC>`"]
pub type R2SK_FO2S4 = crate::Reg<r2sk_fo2s4::R2SK_FO2S4_SPEC>;
#[doc = "FIFO out to SDP as AES engine key"]
pub mod r2sk_fo2s4;
#[doc = "R2SK_FO2S5 (r) register accessor: an alias for `Reg<R2SK_FO2S5_SPEC>`"]
pub type R2SK_FO2S5 = crate::Reg<r2sk_fo2s5::R2SK_FO2S5_SPEC>;
#[doc = "FIFO out to SDP as AES engine key"]
pub mod r2sk_fo2s5;
#[doc = "R2SK_FO2S6 (r) register accessor: an alias for `Reg<R2SK_FO2S6_SPEC>`"]
pub type R2SK_FO2S6 = crate::Reg<r2sk_fo2s6::R2SK_FO2S6_SPEC>;
#[doc = "FIFO out to SDP as AES engine key"]
pub mod r2sk_fo2s6;
#[doc = "R2SK_FO2S7 (r) register accessor: an alias for `Reg<R2SK_FO2S7_SPEC>`"]
pub type R2SK_FO2S7 = crate::Reg<r2sk_fo2s7::R2SK_FO2S7_SPEC>;
#[doc = "FIFO out to SDP as AES engine key"]
pub mod r2sk_fo2s7;
