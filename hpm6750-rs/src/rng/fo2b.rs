#[doc = "Register `FO2B` reader"]
pub struct R(crate::R<FO2B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FO2B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FO2B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FO2B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FO2B` reader - SW read the FIFO output."]
pub type FO2B_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - SW read the FIFO output."]
    #[inline(always)]
    pub fn fo2b(&self) -> FO2B_R {
        FO2B_R::new(self.bits)
    }
}
#[doc = "FIFO out to bus/cpu\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fo2b](index.html) module"]
pub struct FO2B_SPEC;
impl crate::RegisterSpec for FO2B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fo2b::R](R) reader structure"]
impl crate::Readable for FO2B_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FO2B to value 0"]
impl crate::Resettable for FO2B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
