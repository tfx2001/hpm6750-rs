#[doc = "Register `CHANNEL_CH2_CAPNEG` reader"]
pub struct R(crate::R<CHANNEL_CH2_CAPNEG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNEL_CH2_CAPNEG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNEL_CH2_CAPNEG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNEL_CH2_CAPNEG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPNEG` reader - This register contains the counter value captured at input signal falling edge"]
pub type CAPNEG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains the counter value captured at input signal falling edge"]
    #[inline(always)]
    pub fn capneg(&self) -> CAPNEG_R {
        CAPNEG_R::new(self.bits)
    }
}
#[doc = "Capture falling edge register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channel_ch2_capneg](index.html) module"]
pub struct CHANNEL_CH2_CAPNEG_SPEC;
impl crate::RegisterSpec for CHANNEL_CH2_CAPNEG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channel_ch2_capneg::R](R) reader structure"]
impl crate::Readable for CHANNEL_CH2_CAPNEG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHANNEL_CH2_CAPNEG to value 0"]
impl crate::Resettable for CHANNEL_CH2_CAPNEG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
