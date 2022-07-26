#[doc = "Register `OUTDMACNT` reader"]
pub struct R(crate::R<OUTDMACNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTDMACNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTDMACNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTDMACNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VAL` reader - The out DMA counter"]
pub type VAL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The out DMA counter"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[doc = "Out DMA Bytes Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outdmacnt](index.html) module"]
pub struct OUTDMACNT_SPEC;
impl crate::RegisterSpec for OUTDMACNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outdmacnt::R](R) reader structure"]
impl crate::Readable for OUTDMACNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUTDMACNT to value 0"]
impl crate::Resettable for OUTDMACNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
