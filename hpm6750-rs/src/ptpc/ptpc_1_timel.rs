#[doc = "Register `PTPC_1_TIMEL` reader"]
pub struct R(crate::R<PTPC_1_TIMEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPC_1_TIMEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPC_1_TIMEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPC_1_TIMEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMESTAMP_LOW` reader - No description avaiable"]
pub type TIMESTAMP_LOW_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    pub fn timestamp_low(&self) -> TIMESTAMP_LOW_R {
        TIMESTAMP_LOW_R::new(self.bits)
    }
}
#[doc = "timestamp low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpc_1_timel](index.html) module"]
pub struct PTPC_1_TIMEL_SPEC;
impl crate::RegisterSpec for PTPC_1_TIMEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptpc_1_timel::R](R) reader structure"]
impl crate::Readable for PTPC_1_TIMEL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTPC_1_TIMEL to value 0"]
impl crate::Resettable for PTPC_1_TIMEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
