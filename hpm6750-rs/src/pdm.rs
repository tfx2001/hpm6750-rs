#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Channel Control Register"]
    pub ch_ctrl: CH_CTRL,
    #[doc = "0x08 - Status Register"]
    pub st: ST,
    #[doc = "0x0c - Channel Configuration Register"]
    pub ch_cfg: CH_CFG,
    #[doc = "0x10 - CIC configuration register"]
    pub cic_cfg: CIC_CFG,
    #[doc = "0x14 - In Buf Control Register"]
    pub ctrl_inbuf: CTRL_INBUF,
    #[doc = "0x18 - Filter 0 Control Register"]
    pub ctrl_filt0: CTRL_FILT0,
    #[doc = "0x1c - Filter 1 Control Register"]
    pub ctrl_filt1: CTRL_FILT1,
    #[doc = "0x20 - Run Register"]
    pub run: RUN,
    #[doc = "0x24 - Memory Access Address"]
    pub memaddr: MEMADDR,
    #[doc = "0x28 - Memory Access Data"]
    pub memdata: MEMDATA,
    #[doc = "0x2c - HPF A Coef Register"]
    pub hpf_ma: HPF_MA,
    #[doc = "0x30 - HPF B Coef Register"]
    pub hpf_b: HPF_B,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "CH_CTRL (rw) register accessor: an alias for `Reg<CH_CTRL_SPEC>`"]
pub type CH_CTRL = crate::Reg<ch_ctrl::CH_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch_ctrl;
#[doc = "ST (rw) register accessor: an alias for `Reg<ST_SPEC>`"]
pub type ST = crate::Reg<st::ST_SPEC>;
#[doc = "Status Register"]
pub mod st;
#[doc = "CH_CFG (rw) register accessor: an alias for `Reg<CH_CFG_SPEC>`"]
pub type CH_CFG = crate::Reg<ch_cfg::CH_CFG_SPEC>;
#[doc = "Channel Configuration Register"]
pub mod ch_cfg;
#[doc = "CIC_CFG (rw) register accessor: an alias for `Reg<CIC_CFG_SPEC>`"]
pub type CIC_CFG = crate::Reg<cic_cfg::CIC_CFG_SPEC>;
#[doc = "CIC configuration register"]
pub mod cic_cfg;
#[doc = "CTRL_INBUF (rw) register accessor: an alias for `Reg<CTRL_INBUF_SPEC>`"]
pub type CTRL_INBUF = crate::Reg<ctrl_inbuf::CTRL_INBUF_SPEC>;
#[doc = "In Buf Control Register"]
pub mod ctrl_inbuf;
#[doc = "CTRL_FILT0 (rw) register accessor: an alias for `Reg<CTRL_FILT0_SPEC>`"]
pub type CTRL_FILT0 = crate::Reg<ctrl_filt0::CTRL_FILT0_SPEC>;
#[doc = "Filter 0 Control Register"]
pub mod ctrl_filt0;
#[doc = "CTRL_FILT1 (rw) register accessor: an alias for `Reg<CTRL_FILT1_SPEC>`"]
pub type CTRL_FILT1 = crate::Reg<ctrl_filt1::CTRL_FILT1_SPEC>;
#[doc = "Filter 1 Control Register"]
pub mod ctrl_filt1;
#[doc = "RUN (rw) register accessor: an alias for `Reg<RUN_SPEC>`"]
pub type RUN = crate::Reg<run::RUN_SPEC>;
#[doc = "Run Register"]
pub mod run;
#[doc = "MEMADDR (rw) register accessor: an alias for `Reg<MEMADDR_SPEC>`"]
pub type MEMADDR = crate::Reg<memaddr::MEMADDR_SPEC>;
#[doc = "Memory Access Address"]
pub mod memaddr;
#[doc = "MEMDATA (rw) register accessor: an alias for `Reg<MEMDATA_SPEC>`"]
pub type MEMDATA = crate::Reg<memdata::MEMDATA_SPEC>;
#[doc = "Memory Access Data"]
pub mod memdata;
#[doc = "HPF_MA (rw) register accessor: an alias for `Reg<HPF_MA_SPEC>`"]
pub type HPF_MA = crate::Reg<hpf_ma::HPF_MA_SPEC>;
#[doc = "HPF A Coef Register"]
pub mod hpf_ma;
#[doc = "HPF_B (rw) register accessor: an alias for `Reg<HPF_B_SPEC>`"]
pub type HPF_B = crate::Reg<hpf_b::HPF_B_SPEC>;
#[doc = "HPF B Coef Register"]
pub mod hpf_b;
