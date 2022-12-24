#[doc = "Register `CHANNEL_CH0_CAPPOS` reader"]
pub struct R(crate::R<CHANNEL_CH0_CAPPOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CH0_CAPPOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CH0_CAPPOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CH0_CAPPOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPPOS` reader - This register contains the counter value captured at input signal rising edge"]
pub type CAPPOS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains the counter value captured at input signal rising edge"]
    #[inline(always)]
    pub fn cappos(&self) -> CAPPOS_R {
        CAPPOS_R::new(self.bits)
    }
}
#[doc = "Capture rising edge register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_ch0_cappos](index.html) module"]
pub struct CHANNEL_CH0_CAPPOS_SPEC;
impl crate::RegisterSpec for CHANNEL_CH0_CAPPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_ch0_cappos::R](R) reader structure"]
impl crate::Readable for CHANNEL_CH0_CAPPOS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHANNEL_CH0_CAPPOS to value 0"]
impl crate::Resettable for CHANNEL_CH0_CAPPOS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
