#[doc = "Register `CHANNEL_CH3_CAPDTY` reader"]
pub struct R(crate::R<CHANNEL_CH3_CAPDTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CH3_CAPDTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CH3_CAPDTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CH3_CAPDTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MEAS_HIGH` reader - This register contains the input signal duty cycle when channel is configured to input capture measure mode."]
pub type MEAS_HIGH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains the input signal duty cycle when channel is configured to input capture measure mode."]
    #[inline(always)]
    pub fn meas_high(&self) -> MEAS_HIGH_R {
        MEAS_HIGH_R::new(self.bits)
    }
}
#[doc = "PWM duty cycle measure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_ch3_capdty](index.html) module"]
pub struct CHANNEL_CH3_CAPDTY_SPEC;
impl crate::RegisterSpec for CHANNEL_CH3_CAPDTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_ch3_capdty::R](R) reader structure"]
impl crate::Readable for CHANNEL_CH3_CAPDTY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHANNEL_CH3_CAPDTY to value 0"]
impl crate::Resettable for CHANNEL_CH3_CAPDTY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
