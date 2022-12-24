#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Filter Control Register"]
    pub filtctrl: FILTCTRL,
    #[doc = "0x08 - Decision Control Register 0"]
    pub dec_ctrl0: DEC_CTRL0,
    #[doc = "0x0c - Decision Control Register 1"]
    pub dec_ctrl1: DEC_CTRL1,
    #[doc = "0x10 - Decision Control Register 2"]
    pub dec_ctrl2: DEC_CTRL2,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Status"]
    pub st: ST,
    #[doc = "0x1c - Out FIFO"]
    pub ofifo: OFIFO,
    #[doc = "0x20 - Run Command Register"]
    pub run: RUN,
    #[doc = "0x24 - Out FIFO Control Register"]
    pub ofifo_ctrl: OFIFO_CTRL,
    #[doc = "0x28 - CIC Configuration Register"]
    pub cic_cfg: CIC_CFG,
    _reserved10: [u8; 0x74],
    #[doc = "0xa0 - Short Time Energy Register"]
    pub coef_ste_act: COEF_STE_ACT,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "FILTCTRL (rw) register accessor: an alias for `Reg<FILTCTRL_SPEC>`"]
pub type FILTCTRL = crate::Reg<filtctrl::FILTCTRL_SPEC>;
#[doc = "Filter Control Register"]
pub mod filtctrl;
#[doc = "DEC_CTRL0 (rw) register accessor: an alias for `Reg<DEC_CTRL0_SPEC>`"]
pub type DEC_CTRL0 = crate::Reg<dec_ctrl0::DEC_CTRL0_SPEC>;
#[doc = "Decision Control Register 0"]
pub mod dec_ctrl0;
#[doc = "DEC_CTRL1 (rw) register accessor: an alias for `Reg<DEC_CTRL1_SPEC>`"]
pub type DEC_CTRL1 = crate::Reg<dec_ctrl1::DEC_CTRL1_SPEC>;
#[doc = "Decision Control Register 1"]
pub mod dec_ctrl1;
#[doc = "DEC_CTRL2 (rw) register accessor: an alias for `Reg<DEC_CTRL2_SPEC>`"]
pub type DEC_CTRL2 = crate::Reg<dec_ctrl2::DEC_CTRL2_SPEC>;
#[doc = "Decision Control Register 2"]
pub mod dec_ctrl2;
#[doc = "ST (rw) register accessor: an alias for `Reg<ST_SPEC>`"]
pub type ST = crate::Reg<st::ST_SPEC>;
#[doc = "Status"]
pub mod st;
#[doc = "OFIFO (rw) register accessor: an alias for `Reg<OFIFO_SPEC>`"]
pub type OFIFO = crate::Reg<ofifo::OFIFO_SPEC>;
#[doc = "Out FIFO"]
pub mod ofifo;
#[doc = "RUN (rw) register accessor: an alias for `Reg<RUN_SPEC>`"]
pub type RUN = crate::Reg<run::RUN_SPEC>;
#[doc = "Run Command Register"]
pub mod run;
#[doc = "OFIFO_CTRL (rw) register accessor: an alias for `Reg<OFIFO_CTRL_SPEC>`"]
pub type OFIFO_CTRL = crate::Reg<ofifo_ctrl::OFIFO_CTRL_SPEC>;
#[doc = "Out FIFO Control Register"]
pub mod ofifo_ctrl;
#[doc = "CIC_CFG (rw) register accessor: an alias for `Reg<CIC_CFG_SPEC>`"]
pub type CIC_CFG = crate::Reg<cic_cfg::CIC_CFG_SPEC>;
#[doc = "CIC Configuration Register"]
pub mod cic_cfg;
#[doc = "COEF_STE_ACT (r) register accessor: an alias for `Reg<COEF_STE_ACT_SPEC>`"]
pub type COEF_STE_ACT = crate::Reg<coef_ste_act::COEF_STE_ACT_SPEC>;
#[doc = "Short Time Energy Register"]
pub mod coef_ste_act;
