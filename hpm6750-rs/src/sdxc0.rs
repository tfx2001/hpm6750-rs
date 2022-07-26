#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No description avaiable"]
    pub sdmasa: crate::Reg<sdmasa::SDMASA_SPEC>,
    #[doc = "0x04 - No description avaiable"]
    pub blk_attr: crate::Reg<blk_attr::BLK_ATTR_SPEC>,
    #[doc = "0x08 - No description avaiable"]
    pub cmd_arg: crate::Reg<cmd_arg::CMD_ARG_SPEC>,
    #[doc = "0x0c - No description avaiable"]
    pub cmd_xfer: crate::Reg<cmd_xfer::CMD_XFER_SPEC>,
    #[doc = "0x10 - No description avaiable"]
    pub resp_resp01: crate::Reg<resp_resp01::RESP_RESP01_SPEC>,
    #[doc = "0x14 - No description avaiable"]
    pub resp_resp23: crate::Reg<resp_resp23::RESP_RESP23_SPEC>,
    #[doc = "0x18 - No description avaiable"]
    pub resp_resp45: crate::Reg<resp_resp45::RESP_RESP45_SPEC>,
    #[doc = "0x1c - No description avaiable"]
    pub resp_resp67: crate::Reg<resp_resp67::RESP_RESP67_SPEC>,
    #[doc = "0x20 - No description avaiable"]
    pub buf_data: crate::Reg<buf_data::BUF_DATA_SPEC>,
    #[doc = "0x24 - No description avaiable"]
    pub pstate: crate::Reg<pstate::PSTATE_SPEC>,
    #[doc = "0x28 - No description avaiable"]
    pub prot_ctrl: crate::Reg<prot_ctrl::PROT_CTRL_SPEC>,
    #[doc = "0x2c - No description avaiable"]
    pub sys_ctrl: crate::Reg<sys_ctrl::SYS_CTRL_SPEC>,
    #[doc = "0x30 - No description avaiable"]
    pub int_stat: crate::Reg<int_stat::INT_STAT_SPEC>,
    #[doc = "0x34 - No description avaiable"]
    pub int_stat_en: crate::Reg<int_stat_en::INT_STAT_EN_SPEC>,
    #[doc = "0x38 - No description avaiable"]
    pub int_signal_en: crate::Reg<int_signal_en::INT_SIGNAL_EN_SPEC>,
    #[doc = "0x3c - No description avaiable"]
    pub ac_host_ctrl: crate::Reg<ac_host_ctrl::AC_HOST_CTRL_SPEC>,
    #[doc = "0x40 - No description avaiable"]
    pub capabilities1: crate::Reg<capabilities1::CAPABILITIES1_SPEC>,
    #[doc = "0x44 - No description avaiable"]
    pub capabilities2: crate::Reg<capabilities2::CAPABILITIES2_SPEC>,
    #[doc = "0x48 - No description avaiable"]
    pub curr_capabilities1: crate::Reg<curr_capabilities1::CURR_CAPABILITIES1_SPEC>,
    #[doc = "0x4c - No description avaiable"]
    pub curr_capabilities2: crate::Reg<curr_capabilities2::CURR_CAPABILITIES2_SPEC>,
    #[doc = "0x50 - No description avaiable"]
    pub force_event: crate::Reg<force_event::FORCE_EVENT_SPEC>,
    #[doc = "0x54 - No description avaiable"]
    pub adma_err_stat: crate::Reg<adma_err_stat::ADMA_ERR_STAT_SPEC>,
    #[doc = "0x58 - No description avaiable"]
    pub adma_sys_addr: crate::Reg<adma_sys_addr::ADMA_SYS_ADDR_SPEC>,
    _reserved23: [u8; 0x04],
    #[doc = "0x60 - No description avaiable"]
    pub preset_init: crate::Reg<preset_init::PRESET_INIT_SPEC>,
    #[doc = "0x62 - No description avaiable"]
    pub preset_ds: crate::Reg<preset_ds::PRESET_DS_SPEC>,
    #[doc = "0x64 - No description avaiable"]
    pub preset_hs: crate::Reg<preset_hs::PRESET_HS_SPEC>,
    #[doc = "0x66 - No description avaiable"]
    pub preset_sdr12: crate::Reg<preset_sdr12::PRESET_SDR12_SPEC>,
    #[doc = "0x68 - No description avaiable"]
    pub preset_sdr25: crate::Reg<preset_sdr25::PRESET_SDR25_SPEC>,
    #[doc = "0x6a - No description avaiable"]
    pub preset_sdr50: crate::Reg<preset_sdr50::PRESET_SDR50_SPEC>,
    #[doc = "0x6c - No description avaiable"]
    pub preset_sdr104: crate::Reg<preset_sdr104::PRESET_SDR104_SPEC>,
    #[doc = "0x6e - No description avaiable"]
    pub preset_ddr50: crate::Reg<preset_ddr50::PRESET_DDR50_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0x74 - No description avaiable"]
    pub preset_uhs2: crate::Reg<preset_uhs2::PRESET_UHS2_SPEC>,
    _reserved32: [u8; 0x02],
    #[doc = "0x78 - No description avaiable"]
    pub adma_id_addr: crate::Reg<adma_id_addr::ADMA_ID_ADDR_SPEC>,
    _reserved33: [u8; 0x6a],
    #[doc = "0xe6 - No description avaiable"]
    pub p_embedded_cntrl: crate::Reg<p_embedded_cntrl::P_EMBEDDED_CNTRL_SPEC>,
    #[doc = "0xe8 - No description avaiable"]
    pub p_vendor_specific_area: crate::Reg<p_vendor_specific_area::P_VENDOR_SPECIFIC_AREA_SPEC>,
    #[doc = "0xea - No description avaiable"]
    pub p_vendor2_specific_area: crate::Reg<p_vendor2_specific_area::P_VENDOR2_SPECIFIC_AREA_SPEC>,
    _reserved36: [u8; 0x10],
    #[doc = "0xfc - No description avaiable"]
    pub slot_intr_status: crate::Reg<slot_intr_status::SLOT_INTR_STATUS_SPEC>,
    _reserved37: [u8; 0x82],
    #[doc = "0x180 - No description avaiable"]
    pub cqver: crate::Reg<cqver::CQVER_SPEC>,
    #[doc = "0x184 - No description avaiable"]
    pub cqcap: crate::Reg<cqcap::CQCAP_SPEC>,
    #[doc = "0x188 - No description avaiable"]
    pub cqcfg: crate::Reg<cqcfg::CQCFG_SPEC>,
    #[doc = "0x18c - No description avaiable"]
    pub cqctl: crate::Reg<cqctl::CQCTL_SPEC>,
    #[doc = "0x190 - No description avaiable"]
    pub cqis: crate::Reg<cqis::CQIS_SPEC>,
    #[doc = "0x194 - No description avaiable"]
    pub cqise: crate::Reg<cqise::CQISE_SPEC>,
    #[doc = "0x198 - No description avaiable"]
    pub cqisge: crate::Reg<cqisge::CQISGE_SPEC>,
    #[doc = "0x19c - No description avaiable"]
    pub cqic: crate::Reg<cqic::CQIC_SPEC>,
    #[doc = "0x1a0 - No description avaiable"]
    pub cqtdlba: crate::Reg<cqtdlba::CQTDLBA_SPEC>,
    _reserved46: [u8; 0x04],
    #[doc = "0x1a8 - No description avaiable"]
    pub cqtdbr: crate::Reg<cqtdbr::CQTDBR_SPEC>,
    #[doc = "0x1ac - No description avaiable"]
    pub cqtcn: crate::Reg<cqtcn::CQTCN_SPEC>,
    #[doc = "0x1b0 - No description avaiable"]
    pub cqdqs: crate::Reg<cqdqs::CQDQS_SPEC>,
    #[doc = "0x1b4 - No description avaiable"]
    pub cqdpt: crate::Reg<cqdpt::CQDPT_SPEC>,
    #[doc = "0x1b8 - No description avaiable"]
    pub cqtclr: crate::Reg<cqtclr::CQTCLR_SPEC>,
    _reserved51: [u8; 0x04],
    #[doc = "0x1c0 - No description avaiable"]
    pub cqssc1: crate::Reg<cqssc1::CQSSC1_SPEC>,
    #[doc = "0x1c4 - No description avaiable"]
    pub cqssc2: crate::Reg<cqssc2::CQSSC2_SPEC>,
    #[doc = "0x1c8 - No description avaiable"]
    pub cqcrdct: crate::Reg<cqcrdct::CQCRDCT_SPEC>,
    _reserved54: [u8; 0x04],
    #[doc = "0x1d0 - No description avaiable"]
    pub cqrmem: crate::Reg<cqrmem::CQRMEM_SPEC>,
    #[doc = "0x1d4 - No description avaiable"]
    pub cqterri: crate::Reg<cqterri::CQTERRI_SPEC>,
    #[doc = "0x1d8 - No description avaiable"]
    pub cqcri: crate::Reg<cqcri::CQCRI_SPEC>,
    #[doc = "0x1dc - No description avaiable"]
    pub cqcra: crate::Reg<cqcra::CQCRA_SPEC>,
    _reserved58: [u8; 0x0320],
    #[doc = "0x500 - No description avaiable"]
    pub mshc_ver_id: crate::Reg<mshc_ver_id::MSHC_VER_ID_SPEC>,
    #[doc = "0x504 - No description avaiable"]
    pub mshc_ver_type: crate::Reg<mshc_ver_type::MSHC_VER_TYPE_SPEC>,
    _reserved60: [u8; 0x08],
    #[doc = "0x510 - Y"]
    pub mbiu_ctrl: crate::Reg<mbiu_ctrl::MBIU_CTRL_SPEC>,
    _reserved61: [u8; 0x18],
    #[doc = "0x52c - No description avaiable"]
    pub emmc_boot_ctrl: crate::Reg<emmc_boot_ctrl::EMMC_BOOT_CTRL_SPEC>,
    _reserved62: [u8; 0x10],
    #[doc = "0x540 - No description avaiable"]
    pub auto_tuning_ctrl: crate::Reg<auto_tuning_ctrl::AUTO_TUNING_CTRL_SPEC>,
    #[doc = "0x544 - No description avaiable"]
    pub auto_tuning_stat: crate::Reg<auto_tuning_stat::AUTO_TUNING_STAT_SPEC>,
}
#[doc = "SDMASA register accessor: an alias for `Reg<SDMASA_SPEC>`"]
pub type SDMASA = crate::Reg<sdmasa::SDMASA_SPEC>;
#[doc = "No description avaiable"]
pub mod sdmasa;
#[doc = "BLK_ATTR register accessor: an alias for `Reg<BLK_ATTR_SPEC>`"]
pub type BLK_ATTR = crate::Reg<blk_attr::BLK_ATTR_SPEC>;
#[doc = "No description avaiable"]
pub mod blk_attr;
#[doc = "CMD_ARG register accessor: an alias for `Reg<CMD_ARG_SPEC>`"]
pub type CMD_ARG = crate::Reg<cmd_arg::CMD_ARG_SPEC>;
#[doc = "No description avaiable"]
pub mod cmd_arg;
#[doc = "CMD_XFER register accessor: an alias for `Reg<CMD_XFER_SPEC>`"]
pub type CMD_XFER = crate::Reg<cmd_xfer::CMD_XFER_SPEC>;
#[doc = "No description avaiable"]
pub mod cmd_xfer;
#[doc = "RESP_RESP01 register accessor: an alias for `Reg<RESP_RESP01_SPEC>`"]
pub type RESP_RESP01 = crate::Reg<resp_resp01::RESP_RESP01_SPEC>;
#[doc = "No description avaiable"]
pub mod resp_resp01;
#[doc = "RESP_RESP23 register accessor: an alias for `Reg<RESP_RESP23_SPEC>`"]
pub type RESP_RESP23 = crate::Reg<resp_resp23::RESP_RESP23_SPEC>;
#[doc = "No description avaiable"]
pub mod resp_resp23;
#[doc = "RESP_RESP45 register accessor: an alias for `Reg<RESP_RESP45_SPEC>`"]
pub type RESP_RESP45 = crate::Reg<resp_resp45::RESP_RESP45_SPEC>;
#[doc = "No description avaiable"]
pub mod resp_resp45;
#[doc = "RESP_RESP67 register accessor: an alias for `Reg<RESP_RESP67_SPEC>`"]
pub type RESP_RESP67 = crate::Reg<resp_resp67::RESP_RESP67_SPEC>;
#[doc = "No description avaiable"]
pub mod resp_resp67;
#[doc = "BUF_DATA register accessor: an alias for `Reg<BUF_DATA_SPEC>`"]
pub type BUF_DATA = crate::Reg<buf_data::BUF_DATA_SPEC>;
#[doc = "No description avaiable"]
pub mod buf_data;
#[doc = "PSTATE register accessor: an alias for `Reg<PSTATE_SPEC>`"]
pub type PSTATE = crate::Reg<pstate::PSTATE_SPEC>;
#[doc = "No description avaiable"]
pub mod pstate;
#[doc = "PROT_CTRL register accessor: an alias for `Reg<PROT_CTRL_SPEC>`"]
pub type PROT_CTRL = crate::Reg<prot_ctrl::PROT_CTRL_SPEC>;
#[doc = "No description avaiable"]
pub mod prot_ctrl;
#[doc = "SYS_CTRL register accessor: an alias for `Reg<SYS_CTRL_SPEC>`"]
pub type SYS_CTRL = crate::Reg<sys_ctrl::SYS_CTRL_SPEC>;
#[doc = "No description avaiable"]
pub mod sys_ctrl;
#[doc = "INT_STAT register accessor: an alias for `Reg<INT_STAT_SPEC>`"]
pub type INT_STAT = crate::Reg<int_stat::INT_STAT_SPEC>;
#[doc = "No description avaiable"]
pub mod int_stat;
#[doc = "INT_STAT_EN register accessor: an alias for `Reg<INT_STAT_EN_SPEC>`"]
pub type INT_STAT_EN = crate::Reg<int_stat_en::INT_STAT_EN_SPEC>;
#[doc = "No description avaiable"]
pub mod int_stat_en;
#[doc = "INT_SIGNAL_EN register accessor: an alias for `Reg<INT_SIGNAL_EN_SPEC>`"]
pub type INT_SIGNAL_EN = crate::Reg<int_signal_en::INT_SIGNAL_EN_SPEC>;
#[doc = "No description avaiable"]
pub mod int_signal_en;
#[doc = "AC_HOST_CTRL register accessor: an alias for `Reg<AC_HOST_CTRL_SPEC>`"]
pub type AC_HOST_CTRL = crate::Reg<ac_host_ctrl::AC_HOST_CTRL_SPEC>;
#[doc = "No description avaiable"]
pub mod ac_host_ctrl;
#[doc = "CAPABILITIES1 register accessor: an alias for `Reg<CAPABILITIES1_SPEC>`"]
pub type CAPABILITIES1 = crate::Reg<capabilities1::CAPABILITIES1_SPEC>;
#[doc = "No description avaiable"]
pub mod capabilities1;
#[doc = "CAPABILITIES2 register accessor: an alias for `Reg<CAPABILITIES2_SPEC>`"]
pub type CAPABILITIES2 = crate::Reg<capabilities2::CAPABILITIES2_SPEC>;
#[doc = "No description avaiable"]
pub mod capabilities2;
#[doc = "CURR_CAPABILITIES1 register accessor: an alias for `Reg<CURR_CAPABILITIES1_SPEC>`"]
pub type CURR_CAPABILITIES1 = crate::Reg<curr_capabilities1::CURR_CAPABILITIES1_SPEC>;
#[doc = "No description avaiable"]
pub mod curr_capabilities1;
#[doc = "CURR_CAPABILITIES2 register accessor: an alias for `Reg<CURR_CAPABILITIES2_SPEC>`"]
pub type CURR_CAPABILITIES2 = crate::Reg<curr_capabilities2::CURR_CAPABILITIES2_SPEC>;
#[doc = "No description avaiable"]
pub mod curr_capabilities2;
#[doc = "FORCE_EVENT register accessor: an alias for `Reg<FORCE_EVENT_SPEC>`"]
pub type FORCE_EVENT = crate::Reg<force_event::FORCE_EVENT_SPEC>;
#[doc = "No description avaiable"]
pub mod force_event;
#[doc = "ADMA_ERR_STAT register accessor: an alias for `Reg<ADMA_ERR_STAT_SPEC>`"]
pub type ADMA_ERR_STAT = crate::Reg<adma_err_stat::ADMA_ERR_STAT_SPEC>;
#[doc = "No description avaiable"]
pub mod adma_err_stat;
#[doc = "ADMA_SYS_ADDR register accessor: an alias for `Reg<ADMA_SYS_ADDR_SPEC>`"]
pub type ADMA_SYS_ADDR = crate::Reg<adma_sys_addr::ADMA_SYS_ADDR_SPEC>;
#[doc = "No description avaiable"]
pub mod adma_sys_addr;
#[doc = "PRESET_INIT register accessor: an alias for `Reg<PRESET_INIT_SPEC>`"]
pub type PRESET_INIT = crate::Reg<preset_init::PRESET_INIT_SPEC>;
#[doc = "No description avaiable"]
pub mod preset_init;
#[doc = "PRESET_DS register accessor: an alias for `Reg<PRESET_DS_SPEC>`"]
pub type PRESET_DS = crate::Reg<preset_ds::PRESET_DS_SPEC>;
#[doc = "No description avaiable"]
pub mod preset_ds;
#[doc = "PRESET_HS register accessor: an alias for `Reg<PRESET_HS_SPEC>`"]
pub type PRESET_HS = crate::Reg<preset_hs::PRESET_HS_SPEC>;
#[doc = "No description avaiable"]
pub mod preset_hs;
#[doc = "PRESET_SDR12 register accessor: an alias for `Reg<PRESET_SDR12_SPEC>`"]
pub type PRESET_SDR12 = crate::Reg<preset_sdr12::PRESET_SDR12_SPEC>;
#[doc = "No description avaiable"]
pub mod preset_sdr12;
#[doc = "PRESET_SDR25 register accessor: an alias for `Reg<PRESET_SDR25_SPEC>`"]
pub type PRESET_SDR25 = crate::Reg<preset_sdr25::PRESET_SDR25_SPEC>;
#[doc = "No description avaiable"]
pub mod preset_sdr25;
#[doc = "PRESET_SDR50 register accessor: an alias for `Reg<PRESET_SDR50_SPEC>`"]
pub type PRESET_SDR50 = crate::Reg<preset_sdr50::PRESET_SDR50_SPEC>;
#[doc = "No description avaiable"]
pub mod preset_sdr50;
#[doc = "PRESET_SDR104 register accessor: an alias for `Reg<PRESET_SDR104_SPEC>`"]
pub type PRESET_SDR104 = crate::Reg<preset_sdr104::PRESET_SDR104_SPEC>;
#[doc = "No description avaiable"]
pub mod preset_sdr104;
#[doc = "PRESET_DDR50 register accessor: an alias for `Reg<PRESET_DDR50_SPEC>`"]
pub type PRESET_DDR50 = crate::Reg<preset_ddr50::PRESET_DDR50_SPEC>;
#[doc = "No description avaiable"]
pub mod preset_ddr50;
#[doc = "PRESET_UHS2 register accessor: an alias for `Reg<PRESET_UHS2_SPEC>`"]
pub type PRESET_UHS2 = crate::Reg<preset_uhs2::PRESET_UHS2_SPEC>;
#[doc = "No description avaiable"]
pub mod preset_uhs2;
#[doc = "ADMA_ID_ADDR register accessor: an alias for `Reg<ADMA_ID_ADDR_SPEC>`"]
pub type ADMA_ID_ADDR = crate::Reg<adma_id_addr::ADMA_ID_ADDR_SPEC>;
#[doc = "No description avaiable"]
pub mod adma_id_addr;
#[doc = "P_EMBEDDED_CNTRL register accessor: an alias for `Reg<P_EMBEDDED_CNTRL_SPEC>`"]
pub type P_EMBEDDED_CNTRL = crate::Reg<p_embedded_cntrl::P_EMBEDDED_CNTRL_SPEC>;
#[doc = "No description avaiable"]
pub mod p_embedded_cntrl;
#[doc = "P_VENDOR_SPECIFIC_AREA register accessor: an alias for `Reg<P_VENDOR_SPECIFIC_AREA_SPEC>`"]
pub type P_VENDOR_SPECIFIC_AREA = crate::Reg<p_vendor_specific_area::P_VENDOR_SPECIFIC_AREA_SPEC>;
#[doc = "No description avaiable"]
pub mod p_vendor_specific_area;
#[doc = "P_VENDOR2_SPECIFIC_AREA register accessor: an alias for `Reg<P_VENDOR2_SPECIFIC_AREA_SPEC>`"]
pub type P_VENDOR2_SPECIFIC_AREA =
    crate::Reg<p_vendor2_specific_area::P_VENDOR2_SPECIFIC_AREA_SPEC>;
#[doc = "No description avaiable"]
pub mod p_vendor2_specific_area;
#[doc = "SLOT_INTR_STATUS register accessor: an alias for `Reg<SLOT_INTR_STATUS_SPEC>`"]
pub type SLOT_INTR_STATUS = crate::Reg<slot_intr_status::SLOT_INTR_STATUS_SPEC>;
#[doc = "No description avaiable"]
pub mod slot_intr_status;
#[doc = "CQVER register accessor: an alias for `Reg<CQVER_SPEC>`"]
pub type CQVER = crate::Reg<cqver::CQVER_SPEC>;
#[doc = "No description avaiable"]
pub mod cqver;
#[doc = "CQCAP register accessor: an alias for `Reg<CQCAP_SPEC>`"]
pub type CQCAP = crate::Reg<cqcap::CQCAP_SPEC>;
#[doc = "No description avaiable"]
pub mod cqcap;
#[doc = "CQCFG register accessor: an alias for `Reg<CQCFG_SPEC>`"]
pub type CQCFG = crate::Reg<cqcfg::CQCFG_SPEC>;
#[doc = "No description avaiable"]
pub mod cqcfg;
#[doc = "CQCTL register accessor: an alias for `Reg<CQCTL_SPEC>`"]
pub type CQCTL = crate::Reg<cqctl::CQCTL_SPEC>;
#[doc = "No description avaiable"]
pub mod cqctl;
#[doc = "CQIS register accessor: an alias for `Reg<CQIS_SPEC>`"]
pub type CQIS = crate::Reg<cqis::CQIS_SPEC>;
#[doc = "No description avaiable"]
pub mod cqis;
#[doc = "CQISE register accessor: an alias for `Reg<CQISE_SPEC>`"]
pub type CQISE = crate::Reg<cqise::CQISE_SPEC>;
#[doc = "No description avaiable"]
pub mod cqise;
#[doc = "CQISGE register accessor: an alias for `Reg<CQISGE_SPEC>`"]
pub type CQISGE = crate::Reg<cqisge::CQISGE_SPEC>;
#[doc = "No description avaiable"]
pub mod cqisge;
#[doc = "CQIC register accessor: an alias for `Reg<CQIC_SPEC>`"]
pub type CQIC = crate::Reg<cqic::CQIC_SPEC>;
#[doc = "No description avaiable"]
pub mod cqic;
#[doc = "CQTDLBA register accessor: an alias for `Reg<CQTDLBA_SPEC>`"]
pub type CQTDLBA = crate::Reg<cqtdlba::CQTDLBA_SPEC>;
#[doc = "No description avaiable"]
pub mod cqtdlba;
#[doc = "CQTDBR register accessor: an alias for `Reg<CQTDBR_SPEC>`"]
pub type CQTDBR = crate::Reg<cqtdbr::CQTDBR_SPEC>;
#[doc = "No description avaiable"]
pub mod cqtdbr;
#[doc = "CQTCN register accessor: an alias for `Reg<CQTCN_SPEC>`"]
pub type CQTCN = crate::Reg<cqtcn::CQTCN_SPEC>;
#[doc = "No description avaiable"]
pub mod cqtcn;
#[doc = "CQDQS register accessor: an alias for `Reg<CQDQS_SPEC>`"]
pub type CQDQS = crate::Reg<cqdqs::CQDQS_SPEC>;
#[doc = "No description avaiable"]
pub mod cqdqs;
#[doc = "CQDPT register accessor: an alias for `Reg<CQDPT_SPEC>`"]
pub type CQDPT = crate::Reg<cqdpt::CQDPT_SPEC>;
#[doc = "No description avaiable"]
pub mod cqdpt;
#[doc = "CQTCLR register accessor: an alias for `Reg<CQTCLR_SPEC>`"]
pub type CQTCLR = crate::Reg<cqtclr::CQTCLR_SPEC>;
#[doc = "No description avaiable"]
pub mod cqtclr;
#[doc = "CQSSC1 register accessor: an alias for `Reg<CQSSC1_SPEC>`"]
pub type CQSSC1 = crate::Reg<cqssc1::CQSSC1_SPEC>;
#[doc = "No description avaiable"]
pub mod cqssc1;
#[doc = "CQSSC2 register accessor: an alias for `Reg<CQSSC2_SPEC>`"]
pub type CQSSC2 = crate::Reg<cqssc2::CQSSC2_SPEC>;
#[doc = "No description avaiable"]
pub mod cqssc2;
#[doc = "CQCRDCT register accessor: an alias for `Reg<CQCRDCT_SPEC>`"]
pub type CQCRDCT = crate::Reg<cqcrdct::CQCRDCT_SPEC>;
#[doc = "No description avaiable"]
pub mod cqcrdct;
#[doc = "CQRMEM register accessor: an alias for `Reg<CQRMEM_SPEC>`"]
pub type CQRMEM = crate::Reg<cqrmem::CQRMEM_SPEC>;
#[doc = "No description avaiable"]
pub mod cqrmem;
#[doc = "CQTERRI register accessor: an alias for `Reg<CQTERRI_SPEC>`"]
pub type CQTERRI = crate::Reg<cqterri::CQTERRI_SPEC>;
#[doc = "No description avaiable"]
pub mod cqterri;
#[doc = "CQCRI register accessor: an alias for `Reg<CQCRI_SPEC>`"]
pub type CQCRI = crate::Reg<cqcri::CQCRI_SPEC>;
#[doc = "No description avaiable"]
pub mod cqcri;
#[doc = "CQCRA register accessor: an alias for `Reg<CQCRA_SPEC>`"]
pub type CQCRA = crate::Reg<cqcra::CQCRA_SPEC>;
#[doc = "No description avaiable"]
pub mod cqcra;
#[doc = "MSHC_VER_ID register accessor: an alias for `Reg<MSHC_VER_ID_SPEC>`"]
pub type MSHC_VER_ID = crate::Reg<mshc_ver_id::MSHC_VER_ID_SPEC>;
#[doc = "No description avaiable"]
pub mod mshc_ver_id;
#[doc = "MSHC_VER_TYPE register accessor: an alias for `Reg<MSHC_VER_TYPE_SPEC>`"]
pub type MSHC_VER_TYPE = crate::Reg<mshc_ver_type::MSHC_VER_TYPE_SPEC>;
#[doc = "No description avaiable"]
pub mod mshc_ver_type;
#[doc = "MBIU_CTRL register accessor: an alias for `Reg<MBIU_CTRL_SPEC>`"]
pub type MBIU_CTRL = crate::Reg<mbiu_ctrl::MBIU_CTRL_SPEC>;
#[doc = "Y"]
pub mod mbiu_ctrl;
#[doc = "EMMC_BOOT_CTRL register accessor: an alias for `Reg<EMMC_BOOT_CTRL_SPEC>`"]
pub type EMMC_BOOT_CTRL = crate::Reg<emmc_boot_ctrl::EMMC_BOOT_CTRL_SPEC>;
#[doc = "No description avaiable"]
pub mod emmc_boot_ctrl;
#[doc = "AUTO_TUNING_CTRL register accessor: an alias for `Reg<AUTO_TUNING_CTRL_SPEC>`"]
pub type AUTO_TUNING_CTRL = crate::Reg<auto_tuning_ctrl::AUTO_TUNING_CTRL_SPEC>;
#[doc = "No description avaiable"]
pub mod auto_tuning_ctrl;
#[doc = "AUTO_TUNING_STAT register accessor: an alias for `Reg<AUTO_TUNING_STAT_SPEC>`"]
pub type AUTO_TUNING_STAT = crate::Reg<auto_tuning_stat::AUTO_TUNING_STAT_SPEC>;
#[doc = "No description avaiable"]
pub mod auto_tuning_stat;
