#[doc = "Register `PRESET_DS` reader"]
pub struct R(crate::R<PRESET_DS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESET_DS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESET_DS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESET_DS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FREQ_SEL_VAL` reader - SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
pub type FREQ_SEL_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLK_GEN_SEL_VAL` reader - Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
pub type CLK_GEN_SEL_VAL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:9 - SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
    #[inline(always)]
    pub fn freq_sel_val(&self) -> FREQ_SEL_VAL_R {
        FREQ_SEL_VAL_R::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10 - Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
    #[inline(always)]
    pub fn clk_gen_sel_val(&self) -> CLK_GEN_SEL_VAL_R {
        CLK_GEN_SEL_VAL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preset_ds](index.html) module"]
pub struct PRESET_DS_SPEC;
impl crate::RegisterSpec for PRESET_DS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [preset_ds::R](R) reader structure"]
impl crate::Readable for PRESET_DS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRESET_DS to value 0"]
impl crate::Resettable for PRESET_DS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
