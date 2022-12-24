#[doc = "Register `RXREG` reader"]
pub struct R(crate::R<RXREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXREG` reader - Receive word message from other core."]
pub type RXREG_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive word message from other core."]
    #[inline(always)]
    pub fn rxreg(&self) -> RXREG_R {
        RXREG_R::new(self.bits)
    }
}
#[doc = "Receive word message from other core.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxreg](index.html) module"]
pub struct RXREG_SPEC;
impl crate::RegisterSpec for RXREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxreg::R](R) reader structure"]
impl crate::Readable for RXREG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXREG to value 0"]
impl crate::Resettable for RXREG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
