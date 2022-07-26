#[doc = "Register `PRESET_SDR12` reader"]
pub struct R(crate::R<PRESET_SDR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESET_SDR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESET_SDR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESET_SDR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLK_GEN_SEL_VAL` reader - Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
pub type CLK_GEN_SEL_VAL_R = crate::BitReader<bool>;
#[doc = "Field `FREQ_SEL_VAL` reader - SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
pub type FREQ_SEL_VAL_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 10 - Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
    #[inline(always)]
    pub fn clk_gen_sel_val(&self) -> CLK_GEN_SEL_VAL_R {
        CLK_GEN_SEL_VAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 0:9 - SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
    #[inline(always)]
    pub fn freq_sel_val(&self) -> FREQ_SEL_VAL_R {
        FREQ_SEL_VAL_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preset_sdr12](index.html) module"]
pub struct PRESET_SDR12_SPEC;
impl crate::RegisterSpec for PRESET_SDR12_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [preset_sdr12::R](R) reader structure"]
impl crate::Readable for PRESET_SDR12_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRESET_SDR12 to value 0"]
impl crate::Resettable for PRESET_SDR12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
