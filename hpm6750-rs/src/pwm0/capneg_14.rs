#[doc = "Register `CAPNEG_14` reader"]
pub struct R(crate::R<CAPNEG_14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPNEG_14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPNEG_14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPNEG_14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPNEG` reader - counter value captured at input signal falling edge"]
pub type CAPNEG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - counter value captured at input signal falling edge"]
    #[inline(always)]
    pub fn capneg(&self) -> CAPNEG_R {
        CAPNEG_R::new(self.bits)
    }
}
#[doc = "Capture falling edge register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capneg_14](index.html) module"]
pub struct CAPNEG_14_SPEC;
impl crate::RegisterSpec for CAPNEG_14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capneg_14::R](R) reader structure"]
impl crate::Readable for CAPNEG_14_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPNEG_14 to value 0"]
impl crate::Resettable for CAPNEG_14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
