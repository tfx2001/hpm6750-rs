#[doc = "Register `RECNT` reader"]
pub struct R(crate::R<RECNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECNT` reader - Receive Error CouNT (number of errors during reception) RECNT is incremented and decremented as defined in the CAN specification. RECNT does not overflow. If TXB=1, then the error counters are frozen."]
pub type RECNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive Error CouNT (number of errors during reception) RECNT is incremented and decremented as defined in the CAN specification. RECNT does not overflow. If TXB=1, then the error counters are frozen."]
    #[inline(always)]
    pub fn recnt(&self) -> RECNT_R {
        RECNT_R::new(self.bits)
    }
}
#[doc = "Error Counter Registers RECNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [recnt](index.html) module"]
pub struct RECNT_SPEC;
impl crate::RegisterSpec for RECNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [recnt::R](R) reader structure"]
impl crate::Readable for RECNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RECNT to value 0"]
impl crate::Resettable for RECNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
