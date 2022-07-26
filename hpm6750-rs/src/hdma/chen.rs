#[doc = "Register `CHEN` reader"]
pub struct R(crate::R<CHEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHEN` reader - Alias of the Enable field of all ChnCtrl registers"]
pub type CHEN_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alias of the Enable field of all ChnCtrl registers"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(self.bits)
    }
}
#[doc = "Channel Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chen](index.html) module"]
pub struct CHEN_SPEC;
impl crate::RegisterSpec for CHEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chen::R](R) reader structure"]
impl crate::Readable for CHEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHEN to value 0"]
impl crate::Resettable for CHEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
