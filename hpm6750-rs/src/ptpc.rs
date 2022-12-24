#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 0"]
    pub ptpc_0_ctrl0: PTPC_0_CTRL0,
    #[doc = "0x04 - Control Register 1"]
    pub ptpc_0_ctrl1: PTPC_0_CTRL1,
    #[doc = "0x08 - timestamp high"]
    pub ptpc_0_timeh: PTPC_0_TIMEH,
    #[doc = "0x0c - timestamp low"]
    pub ptpc_0_timel: PTPC_0_TIMEL,
    #[doc = "0x10 - timestamp update high"]
    pub ptpc_0_ts_updth: PTPC_0_TS_UPDTH,
    #[doc = "0x14 - timestamp update low"]
    pub ptpc_0_ts_updtl: PTPC_0_TS_UPDTL,
    #[doc = "0x18 - No description avaiable"]
    pub ptpc_0_addend: PTPC_0_ADDEND,
    #[doc = "0x1c - No description avaiable"]
    pub ptpc_0_tarh: PTPC_0_TARH,
    #[doc = "0x20 - No description avaiable"]
    pub ptpc_0_tarl: PTPC_0_TARL,
    _reserved9: [u8; 0x08],
    #[doc = "0x2c - No description avaiable"]
    pub ptpc_0_pps_ctrl: PTPC_0_PPS_CTRL,
    #[doc = "0x30 - No description avaiable"]
    pub ptpc_0_capt_snaph: PTPC_0_CAPT_SNAPH,
    #[doc = "0x34 - No description avaiable"]
    pub ptpc_0_capt_snapl: PTPC_0_CAPT_SNAPL,
    _reserved12: [u8; 0x0fc8],
    #[doc = "0x1000 - Control Register 0"]
    pub ptpc_1_ctrl0: PTPC_1_CTRL0,
    #[doc = "0x1004 - Control Register 1"]
    pub ptpc_1_ctrl1: PTPC_1_CTRL1,
    #[doc = "0x1008 - timestamp high"]
    pub ptpc_1_timeh: PTPC_1_TIMEH,
    #[doc = "0x100c - timestamp low"]
    pub ptpc_1_timel: PTPC_1_TIMEL,
    #[doc = "0x1010 - timestamp update high"]
    pub ptpc_1_ts_updth: PTPC_1_TS_UPDTH,
    #[doc = "0x1014 - timestamp update low"]
    pub ptpc_1_ts_updtl: PTPC_1_TS_UPDTL,
    #[doc = "0x1018 - No description avaiable"]
    pub ptpc_1_addend: PTPC_1_ADDEND,
    #[doc = "0x101c - No description avaiable"]
    pub ptpc_1_tarh: PTPC_1_TARH,
    #[doc = "0x1020 - No description avaiable"]
    pub ptpc_1_tarl: PTPC_1_TARL,
    _reserved21: [u8; 0x08],
    #[doc = "0x102c - No description avaiable"]
    pub ptpc_1_pps_ctrl: PTPC_1_PPS_CTRL,
    #[doc = "0x1030 - No description avaiable"]
    pub ptpc_1_capt_snaph: PTPC_1_CAPT_SNAPH,
    #[doc = "0x1034 - No description avaiable"]
    pub ptpc_1_capt_snapl: PTPC_1_CAPT_SNAPL,
    _reserved24: [u8; 0x0fc8],
    #[doc = "0x2000 - No description avaiable"]
    pub time_sel: TIME_SEL,
    #[doc = "0x2004 - No description avaiable"]
    pub int_sts: INT_STS,
    #[doc = "0x2008 - No description avaiable"]
    pub int_en: INT_EN,
}
#[doc = "PTPC_0_CTRL0 (rw) register accessor: an alias for `Reg<PTPC_0_CTRL0_SPEC>`"]
pub type PTPC_0_CTRL0 = crate::Reg<ptpc_0_ctrl0::PTPC_0_CTRL0_SPEC>;
#[doc = "Control Register 0"]
pub mod ptpc_0_ctrl0;
#[doc = "PTPC_0_CTRL1 (rw) register accessor: an alias for `Reg<PTPC_0_CTRL1_SPEC>`"]
pub type PTPC_0_CTRL1 = crate::Reg<ptpc_0_ctrl1::PTPC_0_CTRL1_SPEC>;
#[doc = "Control Register 1"]
pub mod ptpc_0_ctrl1;
#[doc = "PTPC_0_TIMEH (r) register accessor: an alias for `Reg<PTPC_0_TIMEH_SPEC>`"]
pub type PTPC_0_TIMEH = crate::Reg<ptpc_0_timeh::PTPC_0_TIMEH_SPEC>;
#[doc = "timestamp high"]
pub mod ptpc_0_timeh;
#[doc = "PTPC_0_TIMEL (r) register accessor: an alias for `Reg<PTPC_0_TIMEL_SPEC>`"]
pub type PTPC_0_TIMEL = crate::Reg<ptpc_0_timel::PTPC_0_TIMEL_SPEC>;
#[doc = "timestamp low"]
pub mod ptpc_0_timel;
#[doc = "PTPC_0_TS_UPDTH (rw) register accessor: an alias for `Reg<PTPC_0_TS_UPDTH_SPEC>`"]
pub type PTPC_0_TS_UPDTH = crate::Reg<ptpc_0_ts_updth::PTPC_0_TS_UPDTH_SPEC>;
#[doc = "timestamp update high"]
pub mod ptpc_0_ts_updth;
#[doc = "PTPC_0_TS_UPDTL (rw) register accessor: an alias for `Reg<PTPC_0_TS_UPDTL_SPEC>`"]
pub type PTPC_0_TS_UPDTL = crate::Reg<ptpc_0_ts_updtl::PTPC_0_TS_UPDTL_SPEC>;
#[doc = "timestamp update low"]
pub mod ptpc_0_ts_updtl;
#[doc = "PTPC_0_ADDEND (rw) register accessor: an alias for `Reg<PTPC_0_ADDEND_SPEC>`"]
pub type PTPC_0_ADDEND = crate::Reg<ptpc_0_addend::PTPC_0_ADDEND_SPEC>;
#[doc = "No description avaiable"]
pub mod ptpc_0_addend;
#[doc = "PTPC_0_TARH (rw) register accessor: an alias for `Reg<PTPC_0_TARH_SPEC>`"]
pub type PTPC_0_TARH = crate::Reg<ptpc_0_tarh::PTPC_0_TARH_SPEC>;
#[doc = "No description avaiable"]
pub mod ptpc_0_tarh;
#[doc = "PTPC_0_TARL (rw) register accessor: an alias for `Reg<PTPC_0_TARL_SPEC>`"]
pub type PTPC_0_TARL = crate::Reg<ptpc_0_tarl::PTPC_0_TARL_SPEC>;
#[doc = "No description avaiable"]
pub mod ptpc_0_tarl;
#[doc = "PTPC_0_PPS_CTRL (rw) register accessor: an alias for `Reg<PTPC_0_PPS_CTRL_SPEC>`"]
pub type PTPC_0_PPS_CTRL = crate::Reg<ptpc_0_pps_ctrl::PTPC_0_PPS_CTRL_SPEC>;
#[doc = "No description avaiable"]
pub mod ptpc_0_pps_ctrl;
#[doc = "PTPC_0_CAPT_SNAPH (r) register accessor: an alias for `Reg<PTPC_0_CAPT_SNAPH_SPEC>`"]
pub type PTPC_0_CAPT_SNAPH = crate::Reg<ptpc_0_capt_snaph::PTPC_0_CAPT_SNAPH_SPEC>;
#[doc = "No description avaiable"]
pub mod ptpc_0_capt_snaph;
#[doc = "PTPC_0_CAPT_SNAPL (rw) register accessor: an alias for `Reg<PTPC_0_CAPT_SNAPL_SPEC>`"]
pub type PTPC_0_CAPT_SNAPL = crate::Reg<ptpc_0_capt_snapl::PTPC_0_CAPT_SNAPL_SPEC>;
#[doc = "No description avaiable"]
pub mod ptpc_0_capt_snapl;
#[doc = "PTPC_1_CTRL0 (rw) register accessor: an alias for `Reg<PTPC_1_CTRL0_SPEC>`"]
pub type PTPC_1_CTRL0 = crate::Reg<ptpc_1_ctrl0::PTPC_1_CTRL0_SPEC>;
#[doc = "Control Register 0"]
pub mod ptpc_1_ctrl0;
#[doc = "PTPC_1_CTRL1 (rw) register accessor: an alias for `Reg<PTPC_1_CTRL1_SPEC>`"]
pub type PTPC_1_CTRL1 = crate::Reg<ptpc_1_ctrl1::PTPC_1_CTRL1_SPEC>;
#[doc = "Control Register 1"]
pub mod ptpc_1_ctrl1;
#[doc = "PTPC_1_TIMEH (r) register accessor: an alias for `Reg<PTPC_1_TIMEH_SPEC>`"]
pub type PTPC_1_TIMEH = crate::Reg<ptpc_1_timeh::PTPC_1_TIMEH_SPEC>;
#[doc = "timestamp high"]
pub mod ptpc_1_timeh;
#[doc = "PTPC_1_TIMEL (r) register accessor: an alias for `Reg<PTPC_1_TIMEL_SPEC>`"]
pub type PTPC_1_TIMEL = crate::Reg<ptpc_1_timel::PTPC_1_TIMEL_SPEC>;
#[doc = "timestamp low"]
pub mod ptpc_1_timel;
#[doc = "PTPC_1_TS_UPDTH (rw) register accessor: an alias for `Reg<PTPC_1_TS_UPDTH_SPEC>`"]
pub type PTPC_1_TS_UPDTH = crate::Reg<ptpc_1_ts_updth::PTPC_1_TS_UPDTH_SPEC>;
#[doc = "timestamp update high"]
pub mod ptpc_1_ts_updth;
#[doc = "PTPC_1_TS_UPDTL (rw) register accessor: an alias for `Reg<PTPC_1_TS_UPDTL_SPEC>`"]
pub type PTPC_1_TS_UPDTL = crate::Reg<ptpc_1_ts_updtl::PTPC_1_TS_UPDTL_SPEC>;
#[doc = "timestamp update low"]
pub mod ptpc_1_ts_updtl;
#[doc = "PTPC_1_ADDEND (rw) register accessor: an alias for `Reg<PTPC_1_ADDEND_SPEC>`"]
pub type PTPC_1_ADDEND = crate::Reg<ptpc_1_addend::PTPC_1_ADDEND_SPEC>;
#[doc = "No description avaiable"]
pub mod ptpc_1_addend;
#[doc = "PTPC_1_TARH (rw) register accessor: an alias for `Reg<PTPC_1_TARH_SPEC>`"]
pub type PTPC_1_TARH = crate::Reg<ptpc_1_tarh::PTPC_1_TARH_SPEC>;
#[doc = "No description avaiable"]
pub mod ptpc_1_tarh;
#[doc = "PTPC_1_TARL (rw) register accessor: an alias for `Reg<PTPC_1_TARL_SPEC>`"]
pub type PTPC_1_TARL = crate::Reg<ptpc_1_tarl::PTPC_1_TARL_SPEC>;
#[doc = "No description avaiable"]
pub mod ptpc_1_tarl;
#[doc = "PTPC_1_PPS_CTRL (rw) register accessor: an alias for `Reg<PTPC_1_PPS_CTRL_SPEC>`"]
pub type PTPC_1_PPS_CTRL = crate::Reg<ptpc_1_pps_ctrl::PTPC_1_PPS_CTRL_SPEC>;
#[doc = "No description avaiable"]
pub mod ptpc_1_pps_ctrl;
#[doc = "PTPC_1_CAPT_SNAPH (r) register accessor: an alias for `Reg<PTPC_1_CAPT_SNAPH_SPEC>`"]
pub type PTPC_1_CAPT_SNAPH = crate::Reg<ptpc_1_capt_snaph::PTPC_1_CAPT_SNAPH_SPEC>;
#[doc = "No description avaiable"]
pub mod ptpc_1_capt_snaph;
#[doc = "PTPC_1_CAPT_SNAPL (rw) register accessor: an alias for `Reg<PTPC_1_CAPT_SNAPL_SPEC>`"]
pub type PTPC_1_CAPT_SNAPL = crate::Reg<ptpc_1_capt_snapl::PTPC_1_CAPT_SNAPL_SPEC>;
#[doc = "No description avaiable"]
pub mod ptpc_1_capt_snapl;
#[doc = "TIME_SEL (rw) register accessor: an alias for `Reg<TIME_SEL_SPEC>`"]
pub type TIME_SEL = crate::Reg<time_sel::TIME_SEL_SPEC>;
#[doc = "No description avaiable"]
pub mod time_sel;
#[doc = "INT_STS (rw) register accessor: an alias for `Reg<INT_STS_SPEC>`"]
pub type INT_STS = crate::Reg<int_sts::INT_STS_SPEC>;
#[doc = "No description avaiable"]
pub mod int_sts;
#[doc = "INT_EN (rw) register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "No description avaiable"]
pub mod int_en;
