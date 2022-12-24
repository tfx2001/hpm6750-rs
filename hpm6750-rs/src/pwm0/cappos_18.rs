#[doc = "Register `CAPPOS_18` reader"]
pub struct R(crate::R<CAPPOS_18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPPOS_18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPPOS_18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPPOS_18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPPOS` reader - counter value captured at input posedge"]
pub type CAPPOS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 4:31 - counter value captured at input posedge"]
    #[inline(always)]
    pub fn cappos(&self) -> CAPPOS_R {
        CAPPOS_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[doc = "Capture rising edge register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cappos_18](index.html) module"]
pub struct CAPPOS_18_SPEC;
impl crate::RegisterSpec for CAPPOS_18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cappos_18::R](R) reader structure"]
impl crate::Readable for CAPPOS_18_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPPOS_18 to value 0"]
impl crate::Resettable for CAPPOS_18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
