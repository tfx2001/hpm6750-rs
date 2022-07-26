#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - In DMA Misc Control Register"]
    pub indma_misc: crate::Reg<indma_misc::INDMA_MISC_SPEC>,
    #[doc = "0x04 - In DMA Buf Address"]
    pub indmabase: crate::Reg<indmabase::INDMABASE_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - In DMA Buf Control 0 Register"]
    pub indma_ctrl0: crate::Reg<indma_ctrl0::INDMA_CTRL0_SPEC>,
    #[doc = "0x10 - In DMA Buf Control 1 Register"]
    pub indma_ctrl1: crate::Reg<indma_ctrl1::INDMA_CTRL1_SPEC>,
    #[doc = "0x14 - In DMA Next Command Register"]
    pub inxt_cmd: crate::Reg<inxt_cmd::INXT_CMD_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x20 - Out DMA Misc Control Register"]
    pub outdma_misc: crate::Reg<outdma_misc::OUTDMA_MISC_SPEC>,
    #[doc = "0x24 - Out DMA Buf Address"]
    pub outdmabase: crate::Reg<outdmabase::OUTDMABASE_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - Out DMA Buf Control 0 Register"]
    pub outdma_ctrl0: crate::Reg<outdma_ctrl0::OUTDMA_CTRL0_SPEC>,
    #[doc = "0x30 - Out DMA Buf Control 1 Register"]
    pub outdma_ctrl1: crate::Reg<outdma_ctrl1::OUTDMA_CTRL1_SPEC>,
    #[doc = "0x34 - Out DMA Next Command Register"]
    pub onxt_cmd: crate::Reg<onxt_cmd::ONXT_CMD_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x40 - Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x44 - Status Register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x48 - Image width register"]
    pub width: crate::Reg<width::WIDTH_SPEC>,
    #[doc = "0x4c - Image height register"]
    pub height: crate::Reg<height::HEIGHT_SPEC>,
    #[doc = "0x50 - Buf Access Addr"]
    pub bufaddr: crate::Reg<bufaddr::BUFADDR_SPEC>,
    #[doc = "0x54 - Buf Access Data"]
    pub bufdata: crate::Reg<bufdata::BUFDATA_SPEC>,
    #[doc = "0x58 - Out DMA Bytes Counter"]
    pub outdmacnt: crate::Reg<outdmacnt::OUTDMACNT_SPEC>,
    #[doc = "0x5c - YUV2RGB coefficients Register 0"]
    pub csc_coef0: crate::Reg<csc_coef0::CSC_COEF0_SPEC>,
    #[doc = "0x60 - YUV2RGB coefficients Register 1"]
    pub csc_coef1: crate::Reg<csc_coef1::CSC_COEF1_SPEC>,
    #[doc = "0x64 - YUV2RGB coefficients Register 2"]
    pub csc_coef2: crate::Reg<csc_coef2::CSC_COEF2_SPEC>,
    #[doc = "0x68 - RGB2YUV coefficients Register 0"]
    pub rgb2yuv_coef0: crate::Reg<rgb2yuv_coef0::RGB2YUV_COEF0_SPEC>,
    #[doc = "0x6c - RGB2YUV coefficients Register 1"]
    pub rgb2yuv_coef1: crate::Reg<rgb2yuv_coef1::RGB2YUV_COEF1_SPEC>,
    #[doc = "0x70 - RGB2YUV coefficients Register 2"]
    pub rgb2yuv_coef2: crate::Reg<rgb2yuv_coef2::RGB2YUV_COEF2_SPEC>,
    #[doc = "0x74 - RGB2YUV coefficients Register 3"]
    pub rgb2yuv_coef3: crate::Reg<rgb2yuv_coef3::RGB2YUV_COEF3_SPEC>,
    #[doc = "0x78 - RGB2YUV coefficients Register 4"]
    pub rgb2yuv_coef4: crate::Reg<rgb2yuv_coef4::RGB2YUV_COEF4_SPEC>,
    _reserved25: [u8; 0x08],
    #[doc = "0x84 - Image Control Register 1"]
    pub imgreg1: crate::Reg<imgreg1::IMGREG1_SPEC>,
    #[doc = "0x88 - Image Control Register 2"]
    pub imgreg2: crate::Reg<imgreg2::IMGREG2_SPEC>,
    #[doc = "0x8c - Image Control Register 3"]
    pub imgreg3: crate::Reg<imgreg3::IMGREG3_SPEC>,
    #[doc = "0x90 - Image Control Register 40"]
    pub imgreg_reg40: crate::Reg<imgreg_reg40::IMGREG_REG40_SPEC>,
    #[doc = "0x94 - Image Control Register 41"]
    pub imgreg_reg41: crate::Reg<imgreg_reg41::IMGREG_REG41_SPEC>,
    #[doc = "0x98 - Image Control Register 42"]
    pub imgreg_reg42: crate::Reg<imgreg_reg42::IMGREG_REG42_SPEC>,
    #[doc = "0x9c - Image Control Register 43"]
    pub imgreg_reg43: crate::Reg<imgreg_reg43::IMGREG_REG43_SPEC>,
}
#[doc = "INDMA_MISC register accessor: an alias for `Reg<INDMA_MISC_SPEC>`"]
pub type INDMA_MISC = crate::Reg<indma_misc::INDMA_MISC_SPEC>;
#[doc = "In DMA Misc Control Register"]
pub mod indma_misc;
#[doc = "INDMABASE register accessor: an alias for `Reg<INDMABASE_SPEC>`"]
pub type INDMABASE = crate::Reg<indmabase::INDMABASE_SPEC>;
#[doc = "In DMA Buf Address"]
pub mod indmabase;
#[doc = "INDMA_CTRL0 register accessor: an alias for `Reg<INDMA_CTRL0_SPEC>`"]
pub type INDMA_CTRL0 = crate::Reg<indma_ctrl0::INDMA_CTRL0_SPEC>;
#[doc = "In DMA Buf Control 0 Register"]
pub mod indma_ctrl0;
#[doc = "INDMA_CTRL1 register accessor: an alias for `Reg<INDMA_CTRL1_SPEC>`"]
pub type INDMA_CTRL1 = crate::Reg<indma_ctrl1::INDMA_CTRL1_SPEC>;
#[doc = "In DMA Buf Control 1 Register"]
pub mod indma_ctrl1;
#[doc = "INXT_CMD register accessor: an alias for `Reg<INXT_CMD_SPEC>`"]
pub type INXT_CMD = crate::Reg<inxt_cmd::INXT_CMD_SPEC>;
#[doc = "In DMA Next Command Register"]
pub mod inxt_cmd;
#[doc = "OUTDMA_MISC register accessor: an alias for `Reg<OUTDMA_MISC_SPEC>`"]
pub type OUTDMA_MISC = crate::Reg<outdma_misc::OUTDMA_MISC_SPEC>;
#[doc = "Out DMA Misc Control Register"]
pub mod outdma_misc;
#[doc = "OUTDMABASE register accessor: an alias for `Reg<OUTDMABASE_SPEC>`"]
pub type OUTDMABASE = crate::Reg<outdmabase::OUTDMABASE_SPEC>;
#[doc = "Out DMA Buf Address"]
pub mod outdmabase;
#[doc = "OUTDMA_CTRL0 register accessor: an alias for `Reg<OUTDMA_CTRL0_SPEC>`"]
pub type OUTDMA_CTRL0 = crate::Reg<outdma_ctrl0::OUTDMA_CTRL0_SPEC>;
#[doc = "Out DMA Buf Control 0 Register"]
pub mod outdma_ctrl0;
#[doc = "OUTDMA_CTRL1 register accessor: an alias for `Reg<OUTDMA_CTRL1_SPEC>`"]
pub type OUTDMA_CTRL1 = crate::Reg<outdma_ctrl1::OUTDMA_CTRL1_SPEC>;
#[doc = "Out DMA Buf Control 1 Register"]
pub mod outdma_ctrl1;
#[doc = "ONXT_CMD register accessor: an alias for `Reg<ONXT_CMD_SPEC>`"]
pub type ONXT_CMD = crate::Reg<onxt_cmd::ONXT_CMD_SPEC>;
#[doc = "Out DMA Next Command Register"]
pub mod onxt_cmd;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "WIDTH register accessor: an alias for `Reg<WIDTH_SPEC>`"]
pub type WIDTH = crate::Reg<width::WIDTH_SPEC>;
#[doc = "Image width register"]
pub mod width;
#[doc = "HEIGHT register accessor: an alias for `Reg<HEIGHT_SPEC>`"]
pub type HEIGHT = crate::Reg<height::HEIGHT_SPEC>;
#[doc = "Image height register"]
pub mod height;
#[doc = "BUFADDR register accessor: an alias for `Reg<BUFADDR_SPEC>`"]
pub type BUFADDR = crate::Reg<bufaddr::BUFADDR_SPEC>;
#[doc = "Buf Access Addr"]
pub mod bufaddr;
#[doc = "BUFDATA register accessor: an alias for `Reg<BUFDATA_SPEC>`"]
pub type BUFDATA = crate::Reg<bufdata::BUFDATA_SPEC>;
#[doc = "Buf Access Data"]
pub mod bufdata;
#[doc = "OUTDMACNT register accessor: an alias for `Reg<OUTDMACNT_SPEC>`"]
pub type OUTDMACNT = crate::Reg<outdmacnt::OUTDMACNT_SPEC>;
#[doc = "Out DMA Bytes Counter"]
pub mod outdmacnt;
#[doc = "CSC_COEF0 register accessor: an alias for `Reg<CSC_COEF0_SPEC>`"]
pub type CSC_COEF0 = crate::Reg<csc_coef0::CSC_COEF0_SPEC>;
#[doc = "YUV2RGB coefficients Register 0"]
pub mod csc_coef0;
#[doc = "CSC_COEF1 register accessor: an alias for `Reg<CSC_COEF1_SPEC>`"]
pub type CSC_COEF1 = crate::Reg<csc_coef1::CSC_COEF1_SPEC>;
#[doc = "YUV2RGB coefficients Register 1"]
pub mod csc_coef1;
#[doc = "CSC_COEF2 register accessor: an alias for `Reg<CSC_COEF2_SPEC>`"]
pub type CSC_COEF2 = crate::Reg<csc_coef2::CSC_COEF2_SPEC>;
#[doc = "YUV2RGB coefficients Register 2"]
pub mod csc_coef2;
#[doc = "RGB2YUV_COEF0 register accessor: an alias for `Reg<RGB2YUV_COEF0_SPEC>`"]
pub type RGB2YUV_COEF0 = crate::Reg<rgb2yuv_coef0::RGB2YUV_COEF0_SPEC>;
#[doc = "RGB2YUV coefficients Register 0"]
pub mod rgb2yuv_coef0;
#[doc = "RGB2YUV_COEF1 register accessor: an alias for `Reg<RGB2YUV_COEF1_SPEC>`"]
pub type RGB2YUV_COEF1 = crate::Reg<rgb2yuv_coef1::RGB2YUV_COEF1_SPEC>;
#[doc = "RGB2YUV coefficients Register 1"]
pub mod rgb2yuv_coef1;
#[doc = "RGB2YUV_COEF2 register accessor: an alias for `Reg<RGB2YUV_COEF2_SPEC>`"]
pub type RGB2YUV_COEF2 = crate::Reg<rgb2yuv_coef2::RGB2YUV_COEF2_SPEC>;
#[doc = "RGB2YUV coefficients Register 2"]
pub mod rgb2yuv_coef2;
#[doc = "RGB2YUV_COEF3 register accessor: an alias for `Reg<RGB2YUV_COEF3_SPEC>`"]
pub type RGB2YUV_COEF3 = crate::Reg<rgb2yuv_coef3::RGB2YUV_COEF3_SPEC>;
#[doc = "RGB2YUV coefficients Register 3"]
pub mod rgb2yuv_coef3;
#[doc = "RGB2YUV_COEF4 register accessor: an alias for `Reg<RGB2YUV_COEF4_SPEC>`"]
pub type RGB2YUV_COEF4 = crate::Reg<rgb2yuv_coef4::RGB2YUV_COEF4_SPEC>;
#[doc = "RGB2YUV coefficients Register 4"]
pub mod rgb2yuv_coef4;
#[doc = "IMGREG1 register accessor: an alias for `Reg<IMGREG1_SPEC>`"]
pub type IMGREG1 = crate::Reg<imgreg1::IMGREG1_SPEC>;
#[doc = "Image Control Register 1"]
pub mod imgreg1;
#[doc = "IMGREG2 register accessor: an alias for `Reg<IMGREG2_SPEC>`"]
pub type IMGREG2 = crate::Reg<imgreg2::IMGREG2_SPEC>;
#[doc = "Image Control Register 2"]
pub mod imgreg2;
#[doc = "IMGREG3 register accessor: an alias for `Reg<IMGREG3_SPEC>`"]
pub type IMGREG3 = crate::Reg<imgreg3::IMGREG3_SPEC>;
#[doc = "Image Control Register 3"]
pub mod imgreg3;
#[doc = "IMGREG_REG40 register accessor: an alias for `Reg<IMGREG_REG40_SPEC>`"]
pub type IMGREG_REG40 = crate::Reg<imgreg_reg40::IMGREG_REG40_SPEC>;
#[doc = "Image Control Register 40"]
pub mod imgreg_reg40;
#[doc = "IMGREG_REG41 register accessor: an alias for `Reg<IMGREG_REG41_SPEC>`"]
pub type IMGREG_REG41 = crate::Reg<imgreg_reg41::IMGREG_REG41_SPEC>;
#[doc = "Image Control Register 41"]
pub mod imgreg_reg41;
#[doc = "IMGREG_REG42 register accessor: an alias for `Reg<IMGREG_REG42_SPEC>`"]
pub type IMGREG_REG42 = crate::Reg<imgreg_reg42::IMGREG_REG42_SPEC>;
#[doc = "Image Control Register 42"]
pub mod imgreg_reg42;
#[doc = "IMGREG_REG43 register accessor: an alias for `Reg<IMGREG_REG43_SPEC>`"]
pub type IMGREG_REG43 = crate::Reg<imgreg_reg43::IMGREG_REG43_SPEC>;
#[doc = "Image Control Register 43"]
pub mod imgreg_reg43;
