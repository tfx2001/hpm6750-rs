#[doc = "Register `PTPC_0_TIMEH` reader"]
pub struct R(crate::R<PTPC_0_TIMEH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPC_0_TIMEH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPC_0_TIMEH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPC_0_TIMEH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMESTAMP_HIGH` reader - No description avaiable"]
pub type TIMESTAMP_HIGH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    pub fn timestamp_high(&self) -> TIMESTAMP_HIGH_R {
        TIMESTAMP_HIGH_R::new(self.bits)
    }
}
#[doc = "timestamp high\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpc_0_timeh](index.html) module"]
pub struct PTPC_0_TIMEH_SPEC;
impl crate::RegisterSpec for PTPC_0_TIMEH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptpc_0_timeh::R](R) reader structure"]
impl crate::Readable for PTPC_0_TIMEH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTPC_0_TIMEH to value 0"]
impl crate::Resettable for PTPC_0_TIMEH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
