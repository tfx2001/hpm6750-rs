#[doc = "Register `COEF_STE_ACT` reader"]
pub struct R(crate::R<COEF_STE_ACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COEF_STE_ACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COEF_STE_ACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COEF_STE_ACT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VAL` reader - The current detected short time energy"]
pub type VAL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The current detected short time energy"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[doc = "Short Time Energy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coef_ste_act](index.html) module"]
pub struct COEF_STE_ACT_SPEC;
impl crate::RegisterSpec for COEF_STE_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coef_ste_act::R](R) reader structure"]
impl crate::Readable for COEF_STE_ACT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COEF_STE_ACT to value 0"]
impl crate::Resettable for COEF_STE_ACT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
