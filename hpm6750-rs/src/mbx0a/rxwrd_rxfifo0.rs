#[doc = "Register `RXWRD_RXFIFO0` reader"]
pub struct R(crate::R<RXWRD_RXFIFO0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXWRD_RXFIFO0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXWRD_RXFIFO0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXWRD_RXFIFO0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFO` reader - RXFIFO for receiving message from other core, FIFO size, 4x32 can read one of the word address to pop data to the FIFO; can also use 4x32 burst read from 0x020 to read 4 words from the FIFO."]
pub type RXFIFO_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RXFIFO for receiving message from other core, FIFO size, 4x32 can read one of the word address to pop data to the FIFO; can also use 4x32 burst read from 0x020 to read 4 words from the FIFO."]
    #[inline(always)]
    pub fn rxfifo(&self) -> RXFIFO_R {
        RXFIFO_R::new(self.bits)
    }
}
#[doc = "RXFIFO for receiving message from other core\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxwrd_rxfifo0](index.html) module"]
pub struct RXWRD_RXFIFO0_SPEC;
impl crate::RegisterSpec for RXWRD_RXFIFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxwrd_rxfifo0::R](R) reader structure"]
impl crate::Readable for RXWRD_RXFIFO0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXWRD_RXFIFO0 to value 0"]
impl crate::Resettable for RXWRD_RXFIFO0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
