#[doc = "Register `TECNT` reader"]
pub struct R(crate::R<TECNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TECNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TECNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TECNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TECNT` reader - Transmit Error CouNT (number of errors during transmission) TECNT is incremented and decremented as defined in the CAN specification. In case of the “bus off state” TECNT may overflow. If TXB=1, then the error counters are frozen."]
pub type TECNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error CouNT (number of errors during transmission) TECNT is incremented and decremented as defined in the CAN specification. In case of the “bus off state” TECNT may overflow. If TXB=1, then the error counters are frozen."]
    #[inline(always)]
    pub fn tecnt(&self) -> TECNT_R {
        TECNT_R::new(self.bits)
    }
}
#[doc = "Error Counter Registers TECNT\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tecnt](index.html) module"]
pub struct TECNT_SPEC;
impl crate::RegisterSpec for TECNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tecnt::R](R) reader structure"]
impl crate::Readable for TECNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TECNT to value 0"]
impl crate::Resettable for TECNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
