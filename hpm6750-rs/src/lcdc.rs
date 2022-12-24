#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Background Color Register"]
    pub bgnd_cl: BGND_CL,
    #[doc = "0x08 - Display Window Size Register"]
    pub disp_wn_size: DISP_WN_SIZE,
    #[doc = "0x0c - HSYNC Config Register"]
    pub hsync_para: HSYNC_PARA,
    #[doc = "0x10 - VSYNC Config Register"]
    pub vsync_para: VSYNC_PARA,
    #[doc = "0x14 - DMA Status Register"]
    pub dma_st: DMA_ST,
    #[doc = "0x18 - Status Register"]
    pub st: ST,
    #[doc = "0x1c - Interrupt Enable Register"]
    pub int_en: INT_EN,
    #[doc = "0x20 - TX FIFO Register"]
    pub txfifo: TXFIFO,
    _reserved9: [u8; 0x01dc],
    #[doc = "0x200 - Layer Control Register"]
    pub layer_0_layctrl: LAYER_0_LAYCTRL,
    #[doc = "0x204 - Layer Alpha Register"]
    pub layer_0_alphas: LAYER_0_ALPHAS,
    #[doc = "0x208 - Layer Size Register"]
    pub layer_0_laysize: LAYER_0_LAYSIZE,
    #[doc = "0x20c - Layer Position Register"]
    pub layer_0_laypos: LAYER_0_LAYPOS,
    #[doc = "0x210 - Layer Buffer Pointer Register"]
    pub layer_0_start0: LAYER_0_START0,
    _reserved14: [u8; 0x04],
    #[doc = "0x218 - Layer Bus Config Register"]
    pub layer_0_linecfg: LAYER_0_LINECFG,
    #[doc = "0x21c - Layer Background Color Register"]
    pub layer_0_bg_cl: LAYER_0_BG_CL,
    #[doc = "0x220 - Layer Color Space Conversion Config Register 0"]
    pub layer_0_csc_coef0: LAYER_0_CSC_COEF0,
    #[doc = "0x224 - Layer Color Space Conversion Config Register 1"]
    pub layer_0_csc_coef1: LAYER_0_CSC_COEF1,
    #[doc = "0x228 - Layer Color Space Conversion Config Register 2"]
    pub layer_0_csc_coef2: LAYER_0_CSC_COEF2,
    _reserved19: [u8; 0x14],
    #[doc = "0x240 - Layer Control Register"]
    pub layer_1_layctrl: LAYER_1_LAYCTRL,
    #[doc = "0x244 - Layer Alpha Register"]
    pub layer_1_alphas: LAYER_1_ALPHAS,
    #[doc = "0x248 - Layer Size Register"]
    pub layer_1_laysize: LAYER_1_LAYSIZE,
    #[doc = "0x24c - Layer Position Register"]
    pub layer_1_laypos: LAYER_1_LAYPOS,
    #[doc = "0x250 - Layer Buffer Pointer Register"]
    pub layer_1_start0: LAYER_1_START0,
    _reserved24: [u8; 0x04],
    #[doc = "0x258 - Layer Bus Config Register"]
    pub layer_1_linecfg: LAYER_1_LINECFG,
    #[doc = "0x25c - Layer Background Color Register"]
    pub layer_1_bg_cl: LAYER_1_BG_CL,
    #[doc = "0x260 - Layer Color Space Conversion Config Register 0"]
    pub layer_1_csc_coef0: LAYER_1_CSC_COEF0,
    #[doc = "0x264 - Layer Color Space Conversion Config Register 1"]
    pub layer_1_csc_coef1: LAYER_1_CSC_COEF1,
    #[doc = "0x268 - Layer Color Space Conversion Config Register 2"]
    pub layer_1_csc_coef2: LAYER_1_CSC_COEF2,
    _reserved29: [u8; 0x14],
    #[doc = "0x280 - Layer Control Register"]
    pub layer_2_layctrl: LAYER_2_LAYCTRL,
    #[doc = "0x284 - Layer Alpha Register"]
    pub layer_2_alphas: LAYER_2_ALPHAS,
    #[doc = "0x288 - Layer Size Register"]
    pub layer_2_laysize: LAYER_2_LAYSIZE,
    #[doc = "0x28c - Layer Position Register"]
    pub layer_2_laypos: LAYER_2_LAYPOS,
    #[doc = "0x290 - Layer Buffer Pointer Register"]
    pub layer_2_start0: LAYER_2_START0,
    _reserved34: [u8; 0x04],
    #[doc = "0x298 - Layer Bus Config Register"]
    pub layer_2_linecfg: LAYER_2_LINECFG,
    #[doc = "0x29c - Layer Background Color Register"]
    pub layer_2_bg_cl: LAYER_2_BG_CL,
    #[doc = "0x2a0 - Layer Color Space Conversion Config Register 0"]
    pub layer_2_csc_coef0: LAYER_2_CSC_COEF0,
    #[doc = "0x2a4 - Layer Color Space Conversion Config Register 1"]
    pub layer_2_csc_coef1: LAYER_2_CSC_COEF1,
    #[doc = "0x2a8 - Layer Color Space Conversion Config Register 2"]
    pub layer_2_csc_coef2: LAYER_2_CSC_COEF2,
    _reserved39: [u8; 0x14],
    #[doc = "0x2c0 - Layer Control Register"]
    pub layer_3_layctrl: LAYER_3_LAYCTRL,
    #[doc = "0x2c4 - Layer Alpha Register"]
    pub layer_3_alphas: LAYER_3_ALPHAS,
    #[doc = "0x2c8 - Layer Size Register"]
    pub layer_3_laysize: LAYER_3_LAYSIZE,
    #[doc = "0x2cc - Layer Position Register"]
    pub layer_3_laypos: LAYER_3_LAYPOS,
    #[doc = "0x2d0 - Layer Buffer Pointer Register"]
    pub layer_3_start0: LAYER_3_START0,
    _reserved44: [u8; 0x04],
    #[doc = "0x2d8 - Layer Bus Config Register"]
    pub layer_3_linecfg: LAYER_3_LINECFG,
    #[doc = "0x2dc - Layer Background Color Register"]
    pub layer_3_bg_cl: LAYER_3_BG_CL,
    #[doc = "0x2e0 - Layer Color Space Conversion Config Register 0"]
    pub layer_3_csc_coef0: LAYER_3_CSC_COEF0,
    #[doc = "0x2e4 - Layer Color Space Conversion Config Register 1"]
    pub layer_3_csc_coef1: LAYER_3_CSC_COEF1,
    #[doc = "0x2e8 - Layer Color Space Conversion Config Register 2"]
    pub layer_3_csc_coef2: LAYER_3_CSC_COEF2,
    _reserved49: [u8; 0x14],
    #[doc = "0x300 - Layer Control Register"]
    pub layer_4_layctrl: LAYER_4_LAYCTRL,
    #[doc = "0x304 - Layer Alpha Register"]
    pub layer_4_alphas: LAYER_4_ALPHAS,
    #[doc = "0x308 - Layer Size Register"]
    pub layer_4_laysize: LAYER_4_LAYSIZE,
    #[doc = "0x30c - Layer Position Register"]
    pub layer_4_laypos: LAYER_4_LAYPOS,
    #[doc = "0x310 - Layer Buffer Pointer Register"]
    pub layer_4_start0: LAYER_4_START0,
    _reserved54: [u8; 0x04],
    #[doc = "0x318 - Layer Bus Config Register"]
    pub layer_4_linecfg: LAYER_4_LINECFG,
    #[doc = "0x31c - Layer Background Color Register"]
    pub layer_4_bg_cl: LAYER_4_BG_CL,
    #[doc = "0x320 - Layer Color Space Conversion Config Register 0"]
    pub layer_4_csc_coef0: LAYER_4_CSC_COEF0,
    #[doc = "0x324 - Layer Color Space Conversion Config Register 1"]
    pub layer_4_csc_coef1: LAYER_4_CSC_COEF1,
    #[doc = "0x328 - Layer Color Space Conversion Config Register 2"]
    pub layer_4_csc_coef2: LAYER_4_CSC_COEF2,
    _reserved59: [u8; 0x14],
    #[doc = "0x340 - Layer Control Register"]
    pub layer_5_layctrl: LAYER_5_LAYCTRL,
    #[doc = "0x344 - Layer Alpha Register"]
    pub layer_5_alphas: LAYER_5_ALPHAS,
    #[doc = "0x348 - Layer Size Register"]
    pub layer_5_laysize: LAYER_5_LAYSIZE,
    #[doc = "0x34c - Layer Position Register"]
    pub layer_5_laypos: LAYER_5_LAYPOS,
    #[doc = "0x350 - Layer Buffer Pointer Register"]
    pub layer_5_start0: LAYER_5_START0,
    _reserved64: [u8; 0x04],
    #[doc = "0x358 - Layer Bus Config Register"]
    pub layer_5_linecfg: LAYER_5_LINECFG,
    #[doc = "0x35c - Layer Background Color Register"]
    pub layer_5_bg_cl: LAYER_5_BG_CL,
    #[doc = "0x360 - Layer Color Space Conversion Config Register 0"]
    pub layer_5_csc_coef0: LAYER_5_CSC_COEF0,
    #[doc = "0x364 - Layer Color Space Conversion Config Register 1"]
    pub layer_5_csc_coef1: LAYER_5_CSC_COEF1,
    #[doc = "0x368 - Layer Color Space Conversion Config Register 2"]
    pub layer_5_csc_coef2: LAYER_5_CSC_COEF2,
    _reserved69: [u8; 0x14],
    #[doc = "0x380 - Layer Control Register"]
    pub layer_6_layctrl: LAYER_6_LAYCTRL,
    #[doc = "0x384 - Layer Alpha Register"]
    pub layer_6_alphas: LAYER_6_ALPHAS,
    #[doc = "0x388 - Layer Size Register"]
    pub layer_6_laysize: LAYER_6_LAYSIZE,
    #[doc = "0x38c - Layer Position Register"]
    pub layer_6_laypos: LAYER_6_LAYPOS,
    #[doc = "0x390 - Layer Buffer Pointer Register"]
    pub layer_6_start0: LAYER_6_START0,
    _reserved74: [u8; 0x04],
    #[doc = "0x398 - Layer Bus Config Register"]
    pub layer_6_linecfg: LAYER_6_LINECFG,
    #[doc = "0x39c - Layer Background Color Register"]
    pub layer_6_bg_cl: LAYER_6_BG_CL,
    #[doc = "0x3a0 - Layer Color Space Conversion Config Register 0"]
    pub layer_6_csc_coef0: LAYER_6_CSC_COEF0,
    #[doc = "0x3a4 - Layer Color Space Conversion Config Register 1"]
    pub layer_6_csc_coef1: LAYER_6_CSC_COEF1,
    #[doc = "0x3a8 - Layer Color Space Conversion Config Register 2"]
    pub layer_6_csc_coef2: LAYER_6_CSC_COEF2,
    _reserved79: [u8; 0x14],
    #[doc = "0x3c0 - Layer Control Register"]
    pub layer_7_layctrl: LAYER_7_LAYCTRL,
    #[doc = "0x3c4 - Layer Alpha Register"]
    pub layer_7_alphas: LAYER_7_ALPHAS,
    #[doc = "0x3c8 - Layer Size Register"]
    pub layer_7_laysize: LAYER_7_LAYSIZE,
    #[doc = "0x3cc - Layer Position Register"]
    pub layer_7_laypos: LAYER_7_LAYPOS,
    #[doc = "0x3d0 - Layer Buffer Pointer Register"]
    pub layer_7_start0: LAYER_7_START0,
    _reserved84: [u8; 0x04],
    #[doc = "0x3d8 - Layer Bus Config Register"]
    pub layer_7_linecfg: LAYER_7_LINECFG,
    #[doc = "0x3dc - Layer Background Color Register"]
    pub layer_7_bg_cl: LAYER_7_BG_CL,
    #[doc = "0x3e0 - Layer Color Space Conversion Config Register 0"]
    pub layer_7_csc_coef0: LAYER_7_CSC_COEF0,
    #[doc = "0x3e4 - Layer Color Space Conversion Config Register 1"]
    pub layer_7_csc_coef1: LAYER_7_CSC_COEF1,
    #[doc = "0x3e8 - Layer Color Space Conversion Config Register 2"]
    pub layer_7_csc_coef2: LAYER_7_CSC_COEF2,
    _reserved89: [u8; 0x14],
    #[doc = "0x400 - Clut Load Control Register"]
    pub clut_load: CLUT_LOAD,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "BGND_CL (rw) register accessor: an alias for `Reg<BGND_CL_SPEC>`"]
pub type BGND_CL = crate::Reg<bgnd_cl::BGND_CL_SPEC>;
#[doc = "Background Color Register"]
pub mod bgnd_cl;
#[doc = "DISP_WN_SIZE (rw) register accessor: an alias for `Reg<DISP_WN_SIZE_SPEC>`"]
pub type DISP_WN_SIZE = crate::Reg<disp_wn_size::DISP_WN_SIZE_SPEC>;
#[doc = "Display Window Size Register"]
pub mod disp_wn_size;
#[doc = "HSYNC_PARA (rw) register accessor: an alias for `Reg<HSYNC_PARA_SPEC>`"]
pub type HSYNC_PARA = crate::Reg<hsync_para::HSYNC_PARA_SPEC>;
#[doc = "HSYNC Config Register"]
pub mod hsync_para;
#[doc = "VSYNC_PARA (rw) register accessor: an alias for `Reg<VSYNC_PARA_SPEC>`"]
pub type VSYNC_PARA = crate::Reg<vsync_para::VSYNC_PARA_SPEC>;
#[doc = "VSYNC Config Register"]
pub mod vsync_para;
#[doc = "DMA_ST (rw) register accessor: an alias for `Reg<DMA_ST_SPEC>`"]
pub type DMA_ST = crate::Reg<dma_st::DMA_ST_SPEC>;
#[doc = "DMA Status Register"]
pub mod dma_st;
#[doc = "ST (rw) register accessor: an alias for `Reg<ST_SPEC>`"]
pub type ST = crate::Reg<st::ST_SPEC>;
#[doc = "Status Register"]
pub mod st;
#[doc = "INT_EN (rw) register accessor: an alias for `Reg<INT_EN_SPEC>`"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod int_en;
#[doc = "TXFIFO (rw) register accessor: an alias for `Reg<TXFIFO_SPEC>`"]
pub type TXFIFO = crate::Reg<txfifo::TXFIFO_SPEC>;
#[doc = "TX FIFO Register"]
pub mod txfifo;
#[doc = "LAYER_0_LAYCTRL (rw) register accessor: an alias for `Reg<LAYER_0_LAYCTRL_SPEC>`"]
pub type LAYER_0_LAYCTRL = crate::Reg<layer_0_layctrl::LAYER_0_LAYCTRL_SPEC>;
#[doc = "Layer Control Register"]
pub mod layer_0_layctrl;
#[doc = "LAYER_0_ALPHAS (rw) register accessor: an alias for `Reg<LAYER_0_ALPHAS_SPEC>`"]
pub type LAYER_0_ALPHAS = crate::Reg<layer_0_alphas::LAYER_0_ALPHAS_SPEC>;
#[doc = "Layer Alpha Register"]
pub mod layer_0_alphas;
#[doc = "LAYER_0_LAYSIZE (rw) register accessor: an alias for `Reg<LAYER_0_LAYSIZE_SPEC>`"]
pub type LAYER_0_LAYSIZE = crate::Reg<layer_0_laysize::LAYER_0_LAYSIZE_SPEC>;
#[doc = "Layer Size Register"]
pub mod layer_0_laysize;
#[doc = "LAYER_0_LAYPOS (rw) register accessor: an alias for `Reg<LAYER_0_LAYPOS_SPEC>`"]
pub type LAYER_0_LAYPOS = crate::Reg<layer_0_laypos::LAYER_0_LAYPOS_SPEC>;
#[doc = "Layer Position Register"]
pub mod layer_0_laypos;
#[doc = "LAYER_0_START0 (rw) register accessor: an alias for `Reg<LAYER_0_START0_SPEC>`"]
pub type LAYER_0_START0 = crate::Reg<layer_0_start0::LAYER_0_START0_SPEC>;
#[doc = "Layer Buffer Pointer Register"]
pub mod layer_0_start0;
#[doc = "LAYER_0_LINECFG (rw) register accessor: an alias for `Reg<LAYER_0_LINECFG_SPEC>`"]
pub type LAYER_0_LINECFG = crate::Reg<layer_0_linecfg::LAYER_0_LINECFG_SPEC>;
#[doc = "Layer Bus Config Register"]
pub mod layer_0_linecfg;
#[doc = "LAYER_0_BG_CL (rw) register accessor: an alias for `Reg<LAYER_0_BG_CL_SPEC>`"]
pub type LAYER_0_BG_CL = crate::Reg<layer_0_bg_cl::LAYER_0_BG_CL_SPEC>;
#[doc = "Layer Background Color Register"]
pub mod layer_0_bg_cl;
#[doc = "LAYER_0_CSC_COEF0 (rw) register accessor: an alias for `Reg<LAYER_0_CSC_COEF0_SPEC>`"]
pub type LAYER_0_CSC_COEF0 = crate::Reg<layer_0_csc_coef0::LAYER_0_CSC_COEF0_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod layer_0_csc_coef0;
#[doc = "LAYER_0_CSC_COEF1 (rw) register accessor: an alias for `Reg<LAYER_0_CSC_COEF1_SPEC>`"]
pub type LAYER_0_CSC_COEF1 = crate::Reg<layer_0_csc_coef1::LAYER_0_CSC_COEF1_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod layer_0_csc_coef1;
#[doc = "LAYER_0_CSC_COEF2 (rw) register accessor: an alias for `Reg<LAYER_0_CSC_COEF2_SPEC>`"]
pub type LAYER_0_CSC_COEF2 = crate::Reg<layer_0_csc_coef2::LAYER_0_CSC_COEF2_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod layer_0_csc_coef2;
#[doc = "LAYER_1_LAYCTRL (rw) register accessor: an alias for `Reg<LAYER_1_LAYCTRL_SPEC>`"]
pub type LAYER_1_LAYCTRL = crate::Reg<layer_1_layctrl::LAYER_1_LAYCTRL_SPEC>;
#[doc = "Layer Control Register"]
pub mod layer_1_layctrl;
#[doc = "LAYER_1_ALPHAS (rw) register accessor: an alias for `Reg<LAYER_1_ALPHAS_SPEC>`"]
pub type LAYER_1_ALPHAS = crate::Reg<layer_1_alphas::LAYER_1_ALPHAS_SPEC>;
#[doc = "Layer Alpha Register"]
pub mod layer_1_alphas;
#[doc = "LAYER_1_LAYSIZE (rw) register accessor: an alias for `Reg<LAYER_1_LAYSIZE_SPEC>`"]
pub type LAYER_1_LAYSIZE = crate::Reg<layer_1_laysize::LAYER_1_LAYSIZE_SPEC>;
#[doc = "Layer Size Register"]
pub mod layer_1_laysize;
#[doc = "LAYER_1_LAYPOS (rw) register accessor: an alias for `Reg<LAYER_1_LAYPOS_SPEC>`"]
pub type LAYER_1_LAYPOS = crate::Reg<layer_1_laypos::LAYER_1_LAYPOS_SPEC>;
#[doc = "Layer Position Register"]
pub mod layer_1_laypos;
#[doc = "LAYER_1_START0 (rw) register accessor: an alias for `Reg<LAYER_1_START0_SPEC>`"]
pub type LAYER_1_START0 = crate::Reg<layer_1_start0::LAYER_1_START0_SPEC>;
#[doc = "Layer Buffer Pointer Register"]
pub mod layer_1_start0;
#[doc = "LAYER_1_LINECFG (rw) register accessor: an alias for `Reg<LAYER_1_LINECFG_SPEC>`"]
pub type LAYER_1_LINECFG = crate::Reg<layer_1_linecfg::LAYER_1_LINECFG_SPEC>;
#[doc = "Layer Bus Config Register"]
pub mod layer_1_linecfg;
#[doc = "LAYER_1_BG_CL (rw) register accessor: an alias for `Reg<LAYER_1_BG_CL_SPEC>`"]
pub type LAYER_1_BG_CL = crate::Reg<layer_1_bg_cl::LAYER_1_BG_CL_SPEC>;
#[doc = "Layer Background Color Register"]
pub mod layer_1_bg_cl;
#[doc = "LAYER_1_CSC_COEF0 (rw) register accessor: an alias for `Reg<LAYER_1_CSC_COEF0_SPEC>`"]
pub type LAYER_1_CSC_COEF0 = crate::Reg<layer_1_csc_coef0::LAYER_1_CSC_COEF0_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod layer_1_csc_coef0;
#[doc = "LAYER_1_CSC_COEF1 (rw) register accessor: an alias for `Reg<LAYER_1_CSC_COEF1_SPEC>`"]
pub type LAYER_1_CSC_COEF1 = crate::Reg<layer_1_csc_coef1::LAYER_1_CSC_COEF1_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod layer_1_csc_coef1;
#[doc = "LAYER_1_CSC_COEF2 (rw) register accessor: an alias for `Reg<LAYER_1_CSC_COEF2_SPEC>`"]
pub type LAYER_1_CSC_COEF2 = crate::Reg<layer_1_csc_coef2::LAYER_1_CSC_COEF2_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod layer_1_csc_coef2;
#[doc = "LAYER_2_LAYCTRL (rw) register accessor: an alias for `Reg<LAYER_2_LAYCTRL_SPEC>`"]
pub type LAYER_2_LAYCTRL = crate::Reg<layer_2_layctrl::LAYER_2_LAYCTRL_SPEC>;
#[doc = "Layer Control Register"]
pub mod layer_2_layctrl;
#[doc = "LAYER_2_ALPHAS (rw) register accessor: an alias for `Reg<LAYER_2_ALPHAS_SPEC>`"]
pub type LAYER_2_ALPHAS = crate::Reg<layer_2_alphas::LAYER_2_ALPHAS_SPEC>;
#[doc = "Layer Alpha Register"]
pub mod layer_2_alphas;
#[doc = "LAYER_2_LAYSIZE (rw) register accessor: an alias for `Reg<LAYER_2_LAYSIZE_SPEC>`"]
pub type LAYER_2_LAYSIZE = crate::Reg<layer_2_laysize::LAYER_2_LAYSIZE_SPEC>;
#[doc = "Layer Size Register"]
pub mod layer_2_laysize;
#[doc = "LAYER_2_LAYPOS (rw) register accessor: an alias for `Reg<LAYER_2_LAYPOS_SPEC>`"]
pub type LAYER_2_LAYPOS = crate::Reg<layer_2_laypos::LAYER_2_LAYPOS_SPEC>;
#[doc = "Layer Position Register"]
pub mod layer_2_laypos;
#[doc = "LAYER_2_START0 (rw) register accessor: an alias for `Reg<LAYER_2_START0_SPEC>`"]
pub type LAYER_2_START0 = crate::Reg<layer_2_start0::LAYER_2_START0_SPEC>;
#[doc = "Layer Buffer Pointer Register"]
pub mod layer_2_start0;
#[doc = "LAYER_2_LINECFG (rw) register accessor: an alias for `Reg<LAYER_2_LINECFG_SPEC>`"]
pub type LAYER_2_LINECFG = crate::Reg<layer_2_linecfg::LAYER_2_LINECFG_SPEC>;
#[doc = "Layer Bus Config Register"]
pub mod layer_2_linecfg;
#[doc = "LAYER_2_BG_CL (rw) register accessor: an alias for `Reg<LAYER_2_BG_CL_SPEC>`"]
pub type LAYER_2_BG_CL = crate::Reg<layer_2_bg_cl::LAYER_2_BG_CL_SPEC>;
#[doc = "Layer Background Color Register"]
pub mod layer_2_bg_cl;
#[doc = "LAYER_2_CSC_COEF0 (rw) register accessor: an alias for `Reg<LAYER_2_CSC_COEF0_SPEC>`"]
pub type LAYER_2_CSC_COEF0 = crate::Reg<layer_2_csc_coef0::LAYER_2_CSC_COEF0_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod layer_2_csc_coef0;
#[doc = "LAYER_2_CSC_COEF1 (rw) register accessor: an alias for `Reg<LAYER_2_CSC_COEF1_SPEC>`"]
pub type LAYER_2_CSC_COEF1 = crate::Reg<layer_2_csc_coef1::LAYER_2_CSC_COEF1_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod layer_2_csc_coef1;
#[doc = "LAYER_2_CSC_COEF2 (rw) register accessor: an alias for `Reg<LAYER_2_CSC_COEF2_SPEC>`"]
pub type LAYER_2_CSC_COEF2 = crate::Reg<layer_2_csc_coef2::LAYER_2_CSC_COEF2_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod layer_2_csc_coef2;
#[doc = "LAYER_3_LAYCTRL (rw) register accessor: an alias for `Reg<LAYER_3_LAYCTRL_SPEC>`"]
pub type LAYER_3_LAYCTRL = crate::Reg<layer_3_layctrl::LAYER_3_LAYCTRL_SPEC>;
#[doc = "Layer Control Register"]
pub mod layer_3_layctrl;
#[doc = "LAYER_3_ALPHAS (rw) register accessor: an alias for `Reg<LAYER_3_ALPHAS_SPEC>`"]
pub type LAYER_3_ALPHAS = crate::Reg<layer_3_alphas::LAYER_3_ALPHAS_SPEC>;
#[doc = "Layer Alpha Register"]
pub mod layer_3_alphas;
#[doc = "LAYER_3_LAYSIZE (rw) register accessor: an alias for `Reg<LAYER_3_LAYSIZE_SPEC>`"]
pub type LAYER_3_LAYSIZE = crate::Reg<layer_3_laysize::LAYER_3_LAYSIZE_SPEC>;
#[doc = "Layer Size Register"]
pub mod layer_3_laysize;
#[doc = "LAYER_3_LAYPOS (rw) register accessor: an alias for `Reg<LAYER_3_LAYPOS_SPEC>`"]
pub type LAYER_3_LAYPOS = crate::Reg<layer_3_laypos::LAYER_3_LAYPOS_SPEC>;
#[doc = "Layer Position Register"]
pub mod layer_3_laypos;
#[doc = "LAYER_3_START0 (rw) register accessor: an alias for `Reg<LAYER_3_START0_SPEC>`"]
pub type LAYER_3_START0 = crate::Reg<layer_3_start0::LAYER_3_START0_SPEC>;
#[doc = "Layer Buffer Pointer Register"]
pub mod layer_3_start0;
#[doc = "LAYER_3_LINECFG (rw) register accessor: an alias for `Reg<LAYER_3_LINECFG_SPEC>`"]
pub type LAYER_3_LINECFG = crate::Reg<layer_3_linecfg::LAYER_3_LINECFG_SPEC>;
#[doc = "Layer Bus Config Register"]
pub mod layer_3_linecfg;
#[doc = "LAYER_3_BG_CL (rw) register accessor: an alias for `Reg<LAYER_3_BG_CL_SPEC>`"]
pub type LAYER_3_BG_CL = crate::Reg<layer_3_bg_cl::LAYER_3_BG_CL_SPEC>;
#[doc = "Layer Background Color Register"]
pub mod layer_3_bg_cl;
#[doc = "LAYER_3_CSC_COEF0 (rw) register accessor: an alias for `Reg<LAYER_3_CSC_COEF0_SPEC>`"]
pub type LAYER_3_CSC_COEF0 = crate::Reg<layer_3_csc_coef0::LAYER_3_CSC_COEF0_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod layer_3_csc_coef0;
#[doc = "LAYER_3_CSC_COEF1 (rw) register accessor: an alias for `Reg<LAYER_3_CSC_COEF1_SPEC>`"]
pub type LAYER_3_CSC_COEF1 = crate::Reg<layer_3_csc_coef1::LAYER_3_CSC_COEF1_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod layer_3_csc_coef1;
#[doc = "LAYER_3_CSC_COEF2 (rw) register accessor: an alias for `Reg<LAYER_3_CSC_COEF2_SPEC>`"]
pub type LAYER_3_CSC_COEF2 = crate::Reg<layer_3_csc_coef2::LAYER_3_CSC_COEF2_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod layer_3_csc_coef2;
#[doc = "LAYER_4_LAYCTRL (rw) register accessor: an alias for `Reg<LAYER_4_LAYCTRL_SPEC>`"]
pub type LAYER_4_LAYCTRL = crate::Reg<layer_4_layctrl::LAYER_4_LAYCTRL_SPEC>;
#[doc = "Layer Control Register"]
pub mod layer_4_layctrl;
#[doc = "LAYER_4_ALPHAS (rw) register accessor: an alias for `Reg<LAYER_4_ALPHAS_SPEC>`"]
pub type LAYER_4_ALPHAS = crate::Reg<layer_4_alphas::LAYER_4_ALPHAS_SPEC>;
#[doc = "Layer Alpha Register"]
pub mod layer_4_alphas;
#[doc = "LAYER_4_LAYSIZE (rw) register accessor: an alias for `Reg<LAYER_4_LAYSIZE_SPEC>`"]
pub type LAYER_4_LAYSIZE = crate::Reg<layer_4_laysize::LAYER_4_LAYSIZE_SPEC>;
#[doc = "Layer Size Register"]
pub mod layer_4_laysize;
#[doc = "LAYER_4_LAYPOS (rw) register accessor: an alias for `Reg<LAYER_4_LAYPOS_SPEC>`"]
pub type LAYER_4_LAYPOS = crate::Reg<layer_4_laypos::LAYER_4_LAYPOS_SPEC>;
#[doc = "Layer Position Register"]
pub mod layer_4_laypos;
#[doc = "LAYER_4_START0 (rw) register accessor: an alias for `Reg<LAYER_4_START0_SPEC>`"]
pub type LAYER_4_START0 = crate::Reg<layer_4_start0::LAYER_4_START0_SPEC>;
#[doc = "Layer Buffer Pointer Register"]
pub mod layer_4_start0;
#[doc = "LAYER_4_LINECFG (rw) register accessor: an alias for `Reg<LAYER_4_LINECFG_SPEC>`"]
pub type LAYER_4_LINECFG = crate::Reg<layer_4_linecfg::LAYER_4_LINECFG_SPEC>;
#[doc = "Layer Bus Config Register"]
pub mod layer_4_linecfg;
#[doc = "LAYER_4_BG_CL (rw) register accessor: an alias for `Reg<LAYER_4_BG_CL_SPEC>`"]
pub type LAYER_4_BG_CL = crate::Reg<layer_4_bg_cl::LAYER_4_BG_CL_SPEC>;
#[doc = "Layer Background Color Register"]
pub mod layer_4_bg_cl;
#[doc = "LAYER_4_CSC_COEF0 (rw) register accessor: an alias for `Reg<LAYER_4_CSC_COEF0_SPEC>`"]
pub type LAYER_4_CSC_COEF0 = crate::Reg<layer_4_csc_coef0::LAYER_4_CSC_COEF0_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod layer_4_csc_coef0;
#[doc = "LAYER_4_CSC_COEF1 (rw) register accessor: an alias for `Reg<LAYER_4_CSC_COEF1_SPEC>`"]
pub type LAYER_4_CSC_COEF1 = crate::Reg<layer_4_csc_coef1::LAYER_4_CSC_COEF1_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod layer_4_csc_coef1;
#[doc = "LAYER_4_CSC_COEF2 (rw) register accessor: an alias for `Reg<LAYER_4_CSC_COEF2_SPEC>`"]
pub type LAYER_4_CSC_COEF2 = crate::Reg<layer_4_csc_coef2::LAYER_4_CSC_COEF2_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod layer_4_csc_coef2;
#[doc = "LAYER_5_LAYCTRL (rw) register accessor: an alias for `Reg<LAYER_5_LAYCTRL_SPEC>`"]
pub type LAYER_5_LAYCTRL = crate::Reg<layer_5_layctrl::LAYER_5_LAYCTRL_SPEC>;
#[doc = "Layer Control Register"]
pub mod layer_5_layctrl;
#[doc = "LAYER_5_ALPHAS (rw) register accessor: an alias for `Reg<LAYER_5_ALPHAS_SPEC>`"]
pub type LAYER_5_ALPHAS = crate::Reg<layer_5_alphas::LAYER_5_ALPHAS_SPEC>;
#[doc = "Layer Alpha Register"]
pub mod layer_5_alphas;
#[doc = "LAYER_5_LAYSIZE (rw) register accessor: an alias for `Reg<LAYER_5_LAYSIZE_SPEC>`"]
pub type LAYER_5_LAYSIZE = crate::Reg<layer_5_laysize::LAYER_5_LAYSIZE_SPEC>;
#[doc = "Layer Size Register"]
pub mod layer_5_laysize;
#[doc = "LAYER_5_LAYPOS (rw) register accessor: an alias for `Reg<LAYER_5_LAYPOS_SPEC>`"]
pub type LAYER_5_LAYPOS = crate::Reg<layer_5_laypos::LAYER_5_LAYPOS_SPEC>;
#[doc = "Layer Position Register"]
pub mod layer_5_laypos;
#[doc = "LAYER_5_START0 (rw) register accessor: an alias for `Reg<LAYER_5_START0_SPEC>`"]
pub type LAYER_5_START0 = crate::Reg<layer_5_start0::LAYER_5_START0_SPEC>;
#[doc = "Layer Buffer Pointer Register"]
pub mod layer_5_start0;
#[doc = "LAYER_5_LINECFG (rw) register accessor: an alias for `Reg<LAYER_5_LINECFG_SPEC>`"]
pub type LAYER_5_LINECFG = crate::Reg<layer_5_linecfg::LAYER_5_LINECFG_SPEC>;
#[doc = "Layer Bus Config Register"]
pub mod layer_5_linecfg;
#[doc = "LAYER_5_BG_CL (rw) register accessor: an alias for `Reg<LAYER_5_BG_CL_SPEC>`"]
pub type LAYER_5_BG_CL = crate::Reg<layer_5_bg_cl::LAYER_5_BG_CL_SPEC>;
#[doc = "Layer Background Color Register"]
pub mod layer_5_bg_cl;
#[doc = "LAYER_5_CSC_COEF0 (rw) register accessor: an alias for `Reg<LAYER_5_CSC_COEF0_SPEC>`"]
pub type LAYER_5_CSC_COEF0 = crate::Reg<layer_5_csc_coef0::LAYER_5_CSC_COEF0_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod layer_5_csc_coef0;
#[doc = "LAYER_5_CSC_COEF1 (rw) register accessor: an alias for `Reg<LAYER_5_CSC_COEF1_SPEC>`"]
pub type LAYER_5_CSC_COEF1 = crate::Reg<layer_5_csc_coef1::LAYER_5_CSC_COEF1_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod layer_5_csc_coef1;
#[doc = "LAYER_5_CSC_COEF2 (rw) register accessor: an alias for `Reg<LAYER_5_CSC_COEF2_SPEC>`"]
pub type LAYER_5_CSC_COEF2 = crate::Reg<layer_5_csc_coef2::LAYER_5_CSC_COEF2_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod layer_5_csc_coef2;
#[doc = "LAYER_6_LAYCTRL (rw) register accessor: an alias for `Reg<LAYER_6_LAYCTRL_SPEC>`"]
pub type LAYER_6_LAYCTRL = crate::Reg<layer_6_layctrl::LAYER_6_LAYCTRL_SPEC>;
#[doc = "Layer Control Register"]
pub mod layer_6_layctrl;
#[doc = "LAYER_6_ALPHAS (rw) register accessor: an alias for `Reg<LAYER_6_ALPHAS_SPEC>`"]
pub type LAYER_6_ALPHAS = crate::Reg<layer_6_alphas::LAYER_6_ALPHAS_SPEC>;
#[doc = "Layer Alpha Register"]
pub mod layer_6_alphas;
#[doc = "LAYER_6_LAYSIZE (rw) register accessor: an alias for `Reg<LAYER_6_LAYSIZE_SPEC>`"]
pub type LAYER_6_LAYSIZE = crate::Reg<layer_6_laysize::LAYER_6_LAYSIZE_SPEC>;
#[doc = "Layer Size Register"]
pub mod layer_6_laysize;
#[doc = "LAYER_6_LAYPOS (rw) register accessor: an alias for `Reg<LAYER_6_LAYPOS_SPEC>`"]
pub type LAYER_6_LAYPOS = crate::Reg<layer_6_laypos::LAYER_6_LAYPOS_SPEC>;
#[doc = "Layer Position Register"]
pub mod layer_6_laypos;
#[doc = "LAYER_6_START0 (rw) register accessor: an alias for `Reg<LAYER_6_START0_SPEC>`"]
pub type LAYER_6_START0 = crate::Reg<layer_6_start0::LAYER_6_START0_SPEC>;
#[doc = "Layer Buffer Pointer Register"]
pub mod layer_6_start0;
#[doc = "LAYER_6_LINECFG (rw) register accessor: an alias for `Reg<LAYER_6_LINECFG_SPEC>`"]
pub type LAYER_6_LINECFG = crate::Reg<layer_6_linecfg::LAYER_6_LINECFG_SPEC>;
#[doc = "Layer Bus Config Register"]
pub mod layer_6_linecfg;
#[doc = "LAYER_6_BG_CL (rw) register accessor: an alias for `Reg<LAYER_6_BG_CL_SPEC>`"]
pub type LAYER_6_BG_CL = crate::Reg<layer_6_bg_cl::LAYER_6_BG_CL_SPEC>;
#[doc = "Layer Background Color Register"]
pub mod layer_6_bg_cl;
#[doc = "LAYER_6_CSC_COEF0 (rw) register accessor: an alias for `Reg<LAYER_6_CSC_COEF0_SPEC>`"]
pub type LAYER_6_CSC_COEF0 = crate::Reg<layer_6_csc_coef0::LAYER_6_CSC_COEF0_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod layer_6_csc_coef0;
#[doc = "LAYER_6_CSC_COEF1 (rw) register accessor: an alias for `Reg<LAYER_6_CSC_COEF1_SPEC>`"]
pub type LAYER_6_CSC_COEF1 = crate::Reg<layer_6_csc_coef1::LAYER_6_CSC_COEF1_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod layer_6_csc_coef1;
#[doc = "LAYER_6_CSC_COEF2 (rw) register accessor: an alias for `Reg<LAYER_6_CSC_COEF2_SPEC>`"]
pub type LAYER_6_CSC_COEF2 = crate::Reg<layer_6_csc_coef2::LAYER_6_CSC_COEF2_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod layer_6_csc_coef2;
#[doc = "LAYER_7_LAYCTRL (rw) register accessor: an alias for `Reg<LAYER_7_LAYCTRL_SPEC>`"]
pub type LAYER_7_LAYCTRL = crate::Reg<layer_7_layctrl::LAYER_7_LAYCTRL_SPEC>;
#[doc = "Layer Control Register"]
pub mod layer_7_layctrl;
#[doc = "LAYER_7_ALPHAS (rw) register accessor: an alias for `Reg<LAYER_7_ALPHAS_SPEC>`"]
pub type LAYER_7_ALPHAS = crate::Reg<layer_7_alphas::LAYER_7_ALPHAS_SPEC>;
#[doc = "Layer Alpha Register"]
pub mod layer_7_alphas;
#[doc = "LAYER_7_LAYSIZE (rw) register accessor: an alias for `Reg<LAYER_7_LAYSIZE_SPEC>`"]
pub type LAYER_7_LAYSIZE = crate::Reg<layer_7_laysize::LAYER_7_LAYSIZE_SPEC>;
#[doc = "Layer Size Register"]
pub mod layer_7_laysize;
#[doc = "LAYER_7_LAYPOS (rw) register accessor: an alias for `Reg<LAYER_7_LAYPOS_SPEC>`"]
pub type LAYER_7_LAYPOS = crate::Reg<layer_7_laypos::LAYER_7_LAYPOS_SPEC>;
#[doc = "Layer Position Register"]
pub mod layer_7_laypos;
#[doc = "LAYER_7_START0 (rw) register accessor: an alias for `Reg<LAYER_7_START0_SPEC>`"]
pub type LAYER_7_START0 = crate::Reg<layer_7_start0::LAYER_7_START0_SPEC>;
#[doc = "Layer Buffer Pointer Register"]
pub mod layer_7_start0;
#[doc = "LAYER_7_LINECFG (rw) register accessor: an alias for `Reg<LAYER_7_LINECFG_SPEC>`"]
pub type LAYER_7_LINECFG = crate::Reg<layer_7_linecfg::LAYER_7_LINECFG_SPEC>;
#[doc = "Layer Bus Config Register"]
pub mod layer_7_linecfg;
#[doc = "LAYER_7_BG_CL (rw) register accessor: an alias for `Reg<LAYER_7_BG_CL_SPEC>`"]
pub type LAYER_7_BG_CL = crate::Reg<layer_7_bg_cl::LAYER_7_BG_CL_SPEC>;
#[doc = "Layer Background Color Register"]
pub mod layer_7_bg_cl;
#[doc = "LAYER_7_CSC_COEF0 (rw) register accessor: an alias for `Reg<LAYER_7_CSC_COEF0_SPEC>`"]
pub type LAYER_7_CSC_COEF0 = crate::Reg<layer_7_csc_coef0::LAYER_7_CSC_COEF0_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod layer_7_csc_coef0;
#[doc = "LAYER_7_CSC_COEF1 (rw) register accessor: an alias for `Reg<LAYER_7_CSC_COEF1_SPEC>`"]
pub type LAYER_7_CSC_COEF1 = crate::Reg<layer_7_csc_coef1::LAYER_7_CSC_COEF1_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod layer_7_csc_coef1;
#[doc = "LAYER_7_CSC_COEF2 (rw) register accessor: an alias for `Reg<LAYER_7_CSC_COEF2_SPEC>`"]
pub type LAYER_7_CSC_COEF2 = crate::Reg<layer_7_csc_coef2::LAYER_7_CSC_COEF2_SPEC>;
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod layer_7_csc_coef2;
#[doc = "CLUT_LOAD (rw) register accessor: an alias for `Reg<CLUT_LOAD_SPEC>`"]
pub type CLUT_LOAD = crate::Reg<clut_load::CLUT_LOAD_SPEC>;
#[doc = "Clut Load Control Register"]
pub mod clut_load;
