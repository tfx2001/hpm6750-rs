#[doc = "Register `MONITOR_SLICE1_CURRENT` reader"]
pub struct R(crate::R<MONITOR_SLICE1_CURRENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MONITOR_SLICE1_CURRENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MONITOR_SLICE1_CURRENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MONITOR_SLICE1_CURRENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FREQUENCY` reader - self updating measure result"]
pub type FREQUENCY_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - self updating measure result"]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new(self.bits)
    }
}
#[doc = "Clock measure result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monitor_slice1_current](index.html) module"]
pub struct MONITOR_SLICE1_CURRENT_SPEC;
impl crate::RegisterSpec for MONITOR_SLICE1_CURRENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [monitor_slice1_current::R](R) reader structure"]
impl crate::Readable for MONITOR_SLICE1_CURRENT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MONITOR_SLICE1_CURRENT to value 0"]
impl crate::Resettable for MONITOR_SLICE1_CURRENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
