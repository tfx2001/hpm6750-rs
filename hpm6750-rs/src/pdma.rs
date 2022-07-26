#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Status Register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x08 - Out Layer Control Register"]
    pub out_ctrl: crate::Reg<out_ctrl::OUT_CTRL_SPEC>,
    #[doc = "0x0c - Output buffer address"]
    pub out_buf: crate::Reg<out_buf::OUT_BUF_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - Outlayer Pitch Register"]
    pub out_pitch: crate::Reg<out_pitch::OUT_PITCH_SPEC>,
    #[doc = "0x18 - Output Lower Right Corner Register"]
    pub out_lrc: crate::Reg<out_lrc::OUT_LRC_SPEC>,
    #[doc = "0x1c - Layer Upper Left Corner Register"]
    pub out_ps_0_ulc: crate::Reg<out_ps_0_ulc::OUT_PS_0_ULC_SPEC>,
    #[doc = "0x20 - Layer Lower Right Corner Register"]
    pub out_ps_0_lrc: crate::Reg<out_ps_0_lrc::OUT_PS_0_LRC_SPEC>,
    #[doc = "0x24 - Layer Upper Left Corner Register"]
    pub out_ps_1_ulc: crate::Reg<out_ps_1_ulc::OUT_PS_1_ULC_SPEC>,
    #[doc = "0x28 - Layer Lower Right Corner Register"]
    pub out_ps_1_lrc: crate::Reg<out_ps_1_lrc::OUT_PS_1_LRC_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Layer Control Register"]
    pub ps_0_ctrl: crate::Reg<ps_0_ctrl::PS_0_CTRL_SPEC>,
    #[doc = "0x34 - Layer data buffer address"]
    pub ps_0_buf: crate::Reg<ps_0_buf::PS_0_BUF_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x40 - Layer data pitch register"]
    pub ps_0_pitch: crate::Reg<ps_0_pitch::PS_0_PITCH_SPEC>,
    #[doc = "0x44 - Layer background color register"]
    pub ps_0_bkgd: crate::Reg<ps_0_bkgd::PS_0_BKGD_SPEC>,
    #[doc = "0x48 - Layer scale register"]
    pub ps_0_scale: crate::Reg<ps_0_scale::PS_0_SCALE_SPEC>,
    #[doc = "0x4c - Layer offset register"]
    pub ps_0_offset: crate::Reg<ps_0_offset::PS_0_OFFSET_SPEC>,
    #[doc = "0x50 - Layer low color key register"]
    pub ps_0_clrkey_low: crate::Reg<ps_0_clrkey_low::PS_0_CLRKEY_LOW_SPEC>,
    #[doc = "0x54 - Layer high color key register"]
    pub ps_0_clrkey_high: crate::Reg<ps_0_clrkey_high::PS_0_CLRKEY_HIGH_SPEC>,
    #[doc = "0x58 - Layer original size register"]
    pub ps_0_org: crate::Reg<ps_0_org::PS_0_ORG_SPEC>,
    _reserved19: [u8; 0x04],
    #[doc = "0x60 - layer control register"]
    pub ps_1_ctrl: crate::Reg<ps_1_ctrl::PS_1_CTRL_SPEC>,
    #[doc = "0x64 - Layer data buffer address"]
    pub ps_1_buf: crate::Reg<ps_1_buf::PS_1_BUF_SPEC>,
    _reserved21: [u8; 0x08],
    #[doc = "0x70 - Layer data pitch register"]
    pub ps_1_pitch: crate::Reg<ps_1_pitch::PS_1_PITCH_SPEC>,
    #[doc = "0x74 - Layer background color register"]
    pub ps_1_bkgd: crate::Reg<ps_1_bkgd::PS_1_BKGD_SPEC>,
    #[doc = "0x78 - Layer scale register"]
    pub ps_1_scale: crate::Reg<ps_1_scale::PS_1_SCALE_SPEC>,
    #[doc = "0x7c - Layer offset register"]
    pub ps_1_offset: crate::Reg<ps_1_offset::PS_1_OFFSET_SPEC>,
    #[doc = "0x80 - Layer low color key register"]
    pub ps_1_clrkey_low: crate::Reg<ps_1_clrkey_low::PS_1_CLRKEY_LOW_SPEC>,
    #[doc = "0x84 - Layer high color key register"]
    pub ps_1_clrkey_high: crate::Reg<ps_1_clrkey_high::PS_1_CLRKEY_HIGH_SPEC>,
    #[doc = "0x88 - Layer original size register"]
    pub ps_1_org: crate::Reg<ps_1_org::PS_1_ORG_SPEC>,
    _reserved28: [u8; 0x14],
    #[doc = "0xa0 - YUV2RGB coefficients register 0"]
    pub yuv2rgb_coef0: crate::Reg<yuv2rgb_coef0::YUV2RGB_COEF0_SPEC>,
    #[doc = "0xa4 - YUV2RGB coefficients register 1"]
    pub yuv2rgb_coef1: crate::Reg<yuv2rgb_coef1::YUV2RGB_COEF1_SPEC>,
    #[doc = "0xa8 - YUV2RGB coefficients register 2"]
    pub yuv2rgb_coef2: crate::Reg<yuv2rgb_coef2::YUV2RGB_COEF2_SPEC>,
    #[doc = "0xac - RGB2YUV coefficients register 0"]
    pub rgb2yuv_coef0: crate::Reg<rgb2yuv_coef0::RGB2YUV_COEF0_SPEC>,
    #[doc = "0xb0 - RGB2YUV coefficients register 1"]
    pub rgb2yuv_coef1: crate::Reg<rgb2yuv_coef1::RGB2YUV_COEF1_SPEC>,
    #[doc = "0xb4 - RGB2YUV coefficients register 2"]
    pub rgb2yuv_coef2: crate::Reg<rgb2yuv_coef2::RGB2YUV_COEF2_SPEC>,
    #[doc = "0xb8 - RGB2YUV coefficients register 3"]
    pub rgb2yuv_coef3: crate::Reg<rgb2yuv_coef3::RGB2YUV_COEF3_SPEC>,
    #[doc = "0xbc - RGB2YUV coefficients register 4"]
    pub rgb2yuv_coef4: crate::Reg<rgb2yuv_coef4::RGB2YUV_COEF4_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "OUT_CTRL register accessor: an alias for `Reg<OUT_CTRL_SPEC>`"]
pub type OUT_CTRL = crate::Reg<out_ctrl::OUT_CTRL_SPEC>;
#[doc = "Out Layer Control Register"]
pub mod out_ctrl;
#[doc = "OUT_BUF register accessor: an alias for `Reg<OUT_BUF_SPEC>`"]
pub type OUT_BUF = crate::Reg<out_buf::OUT_BUF_SPEC>;
#[doc = "Output buffer address"]
pub mod out_buf;
#[doc = "OUT_PITCH register accessor: an alias for `Reg<OUT_PITCH_SPEC>`"]
pub type OUT_PITCH = crate::Reg<out_pitch::OUT_PITCH_SPEC>;
#[doc = "Outlayer Pitch Register"]
pub mod out_pitch;
#[doc = "OUT_LRC register accessor: an alias for `Reg<OUT_LRC_SPEC>`"]
pub type OUT_LRC = crate::Reg<out_lrc::OUT_LRC_SPEC>;
#[doc = "Output Lower Right Corner Register"]
pub mod out_lrc;
#[doc = "OUT_PS_0_ULC register accessor: an alias for `Reg<OUT_PS_0_ULC_SPEC>`"]
pub type OUT_PS_0_ULC = crate::Reg<out_ps_0_ulc::OUT_PS_0_ULC_SPEC>;
#[doc = "Layer Upper Left Corner Register"]
pub mod out_ps_0_ulc;
#[doc = "OUT_PS_0_LRC register accessor: an alias for `Reg<OUT_PS_0_LRC_SPEC>`"]
pub type OUT_PS_0_LRC = crate::Reg<out_ps_0_lrc::OUT_PS_0_LRC_SPEC>;
#[doc = "Layer Lower Right Corner Register"]
pub mod out_ps_0_lrc;
#[doc = "OUT_PS_1_ULC register accessor: an alias for `Reg<OUT_PS_1_ULC_SPEC>`"]
pub type OUT_PS_1_ULC = crate::Reg<out_ps_1_ulc::OUT_PS_1_ULC_SPEC>;
#[doc = "Layer Upper Left Corner Register"]
pub mod out_ps_1_ulc;
#[doc = "OUT_PS_1_LRC register accessor: an alias for `Reg<OUT_PS_1_LRC_SPEC>`"]
pub type OUT_PS_1_LRC = crate::Reg<out_ps_1_lrc::OUT_PS_1_LRC_SPEC>;
#[doc = "Layer Lower Right Corner Register"]
pub mod out_ps_1_lrc;
#[doc = "PS_0_CTRL register accessor: an alias for `Reg<PS_0_CTRL_SPEC>`"]
pub type PS_0_CTRL = crate::Reg<ps_0_ctrl::PS_0_CTRL_SPEC>;
#[doc = "Layer Control Register"]
pub mod ps_0_ctrl;
#[doc = "PS_0_BUF register accessor: an alias for `Reg<PS_0_BUF_SPEC>`"]
pub type PS_0_BUF = crate::Reg<ps_0_buf::PS_0_BUF_SPEC>;
#[doc = "Layer data buffer address"]
pub mod ps_0_buf;
#[doc = "PS_0_PITCH register accessor: an alias for `Reg<PS_0_PITCH_SPEC>`"]
pub type PS_0_PITCH = crate::Reg<ps_0_pitch::PS_0_PITCH_SPEC>;
#[doc = "Layer data pitch register"]
pub mod ps_0_pitch;
#[doc = "PS_0_BKGD register accessor: an alias for `Reg<PS_0_BKGD_SPEC>`"]
pub type PS_0_BKGD = crate::Reg<ps_0_bkgd::PS_0_BKGD_SPEC>;
#[doc = "Layer background color register"]
pub mod ps_0_bkgd;
#[doc = "PS_0_SCALE register accessor: an alias for `Reg<PS_0_SCALE_SPEC>`"]
pub type PS_0_SCALE = crate::Reg<ps_0_scale::PS_0_SCALE_SPEC>;
#[doc = "Layer scale register"]
pub mod ps_0_scale;
#[doc = "PS_0_OFFSET register accessor: an alias for `Reg<PS_0_OFFSET_SPEC>`"]
pub type PS_0_OFFSET = crate::Reg<ps_0_offset::PS_0_OFFSET_SPEC>;
#[doc = "Layer offset register"]
pub mod ps_0_offset;
#[doc = "PS_0_CLRKEY_LOW register accessor: an alias for `Reg<PS_0_CLRKEY_LOW_SPEC>`"]
pub type PS_0_CLRKEY_LOW = crate::Reg<ps_0_clrkey_low::PS_0_CLRKEY_LOW_SPEC>;
#[doc = "Layer low color key register"]
pub mod ps_0_clrkey_low;
#[doc = "PS_0_CLRKEY_HIGH register accessor: an alias for `Reg<PS_0_CLRKEY_HIGH_SPEC>`"]
pub type PS_0_CLRKEY_HIGH = crate::Reg<ps_0_clrkey_high::PS_0_CLRKEY_HIGH_SPEC>;
#[doc = "Layer high color key register"]
pub mod ps_0_clrkey_high;
#[doc = "PS_0_ORG register accessor: an alias for `Reg<PS_0_ORG_SPEC>`"]
pub type PS_0_ORG = crate::Reg<ps_0_org::PS_0_ORG_SPEC>;
#[doc = "Layer original size register"]
pub mod ps_0_org;
#[doc = "PS_1_CTRL register accessor: an alias for `Reg<PS_1_CTRL_SPEC>`"]
pub type PS_1_CTRL = crate::Reg<ps_1_ctrl::PS_1_CTRL_SPEC>;
#[doc = "layer control register"]
pub mod ps_1_ctrl;
#[doc = "PS_1_BUF register accessor: an alias for `Reg<PS_1_BUF_SPEC>`"]
pub type PS_1_BUF = crate::Reg<ps_1_buf::PS_1_BUF_SPEC>;
#[doc = "Layer data buffer address"]
pub mod ps_1_buf;
#[doc = "PS_1_PITCH register accessor: an alias for `Reg<PS_1_PITCH_SPEC>`"]
pub type PS_1_PITCH = crate::Reg<ps_1_pitch::PS_1_PITCH_SPEC>;
#[doc = "Layer data pitch register"]
pub mod ps_1_pitch;
#[doc = "PS_1_BKGD register accessor: an alias for `Reg<PS_1_BKGD_SPEC>`"]
pub type PS_1_BKGD = crate::Reg<ps_1_bkgd::PS_1_BKGD_SPEC>;
#[doc = "Layer background color register"]
pub mod ps_1_bkgd;
#[doc = "PS_1_SCALE register accessor: an alias for `Reg<PS_1_SCALE_SPEC>`"]
pub type PS_1_SCALE = crate::Reg<ps_1_scale::PS_1_SCALE_SPEC>;
#[doc = "Layer scale register"]
pub mod ps_1_scale;
#[doc = "PS_1_OFFSET register accessor: an alias for `Reg<PS_1_OFFSET_SPEC>`"]
pub type PS_1_OFFSET = crate::Reg<ps_1_offset::PS_1_OFFSET_SPEC>;
#[doc = "Layer offset register"]
pub mod ps_1_offset;
#[doc = "PS_1_CLRKEY_LOW register accessor: an alias for `Reg<PS_1_CLRKEY_LOW_SPEC>`"]
pub type PS_1_CLRKEY_LOW = crate::Reg<ps_1_clrkey_low::PS_1_CLRKEY_LOW_SPEC>;
#[doc = "Layer low color key register"]
pub mod ps_1_clrkey_low;
#[doc = "PS_1_CLRKEY_HIGH register accessor: an alias for `Reg<PS_1_CLRKEY_HIGH_SPEC>`"]
pub type PS_1_CLRKEY_HIGH = crate::Reg<ps_1_clrkey_high::PS_1_CLRKEY_HIGH_SPEC>;
#[doc = "Layer high color key register"]
pub mod ps_1_clrkey_high;
#[doc = "PS_1_ORG register accessor: an alias for `Reg<PS_1_ORG_SPEC>`"]
pub type PS_1_ORG = crate::Reg<ps_1_org::PS_1_ORG_SPEC>;
#[doc = "Layer original size register"]
pub mod ps_1_org;
#[doc = "YUV2RGB_COEF0 register accessor: an alias for `Reg<YUV2RGB_COEF0_SPEC>`"]
pub type YUV2RGB_COEF0 = crate::Reg<yuv2rgb_coef0::YUV2RGB_COEF0_SPEC>;
#[doc = "YUV2RGB coefficients register 0"]
pub mod yuv2rgb_coef0;
#[doc = "YUV2RGB_COEF1 register accessor: an alias for `Reg<YUV2RGB_COEF1_SPEC>`"]
pub type YUV2RGB_COEF1 = crate::Reg<yuv2rgb_coef1::YUV2RGB_COEF1_SPEC>;
#[doc = "YUV2RGB coefficients register 1"]
pub mod yuv2rgb_coef1;
#[doc = "YUV2RGB_COEF2 register accessor: an alias for `Reg<YUV2RGB_COEF2_SPEC>`"]
pub type YUV2RGB_COEF2 = crate::Reg<yuv2rgb_coef2::YUV2RGB_COEF2_SPEC>;
#[doc = "YUV2RGB coefficients register 2"]
pub mod yuv2rgb_coef2;
#[doc = "RGB2YUV_COEF0 register accessor: an alias for `Reg<RGB2YUV_COEF0_SPEC>`"]
pub type RGB2YUV_COEF0 = crate::Reg<rgb2yuv_coef0::RGB2YUV_COEF0_SPEC>;
#[doc = "RGB2YUV coefficients register 0"]
pub mod rgb2yuv_coef0;
#[doc = "RGB2YUV_COEF1 register accessor: an alias for `Reg<RGB2YUV_COEF1_SPEC>`"]
pub type RGB2YUV_COEF1 = crate::Reg<rgb2yuv_coef1::RGB2YUV_COEF1_SPEC>;
#[doc = "RGB2YUV coefficients register 1"]
pub mod rgb2yuv_coef1;
#[doc = "RGB2YUV_COEF2 register accessor: an alias for `Reg<RGB2YUV_COEF2_SPEC>`"]
pub type RGB2YUV_COEF2 = crate::Reg<rgb2yuv_coef2::RGB2YUV_COEF2_SPEC>;
#[doc = "RGB2YUV coefficients register 2"]
pub mod rgb2yuv_coef2;
#[doc = "RGB2YUV_COEF3 register accessor: an alias for `Reg<RGB2YUV_COEF3_SPEC>`"]
pub type RGB2YUV_COEF3 = crate::Reg<rgb2yuv_coef3::RGB2YUV_COEF3_SPEC>;
#[doc = "RGB2YUV coefficients register 3"]
pub mod rgb2yuv_coef3;
#[doc = "RGB2YUV_COEF4 register accessor: an alias for `Reg<RGB2YUV_COEF4_SPEC>`"]
pub type RGB2YUV_COEF4 = crate::Reg<rgb2yuv_coef4::RGB2YUV_COEF4_SPEC>;
#[doc = "RGB2YUV coefficients register 4"]
pub mod rgb2yuv_coef4;
