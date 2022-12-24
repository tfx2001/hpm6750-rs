#[doc = "Register `SPDHIS_SPDHIS1` reader"]
pub struct R(crate::R<SPDHIS_SPDHIS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPDHIS_SPDHIS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPDHIS_SPDHIS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPDHIS_SPDHIS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPDHIS0` reader - copy of spdcnt, load from spdcnt after any transition from a = low, b = low"]
pub type SPDHIS0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - copy of spdcnt, load from spdcnt after any transition from a = low, b = low"]
    #[inline(always)]
    pub fn spdhis0(&self) -> SPDHIS0_R {
        SPDHIS0_R::new(self.bits)
    }
}
#[doc = "Speed history 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdhis_spdhis1](index.html) module"]
pub struct SPDHIS_SPDHIS1_SPEC;
impl crate::RegisterSpec for SPDHIS_SPDHIS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spdhis_spdhis1::R](R) reader structure"]
impl crate::Readable for SPDHIS_SPDHIS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPDHIS_SPDHIS1 to value 0"]
impl crate::Resettable for SPDHIS_SPDHIS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
