#[doc = "Register `TXWRD_TXFIFO0` writer"]
pub struct W(crate::W<TXWRD_TXFIFO0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXWRD_TXFIFO0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TXWRD_TXFIFO0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXWRD_TXFIFO0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFIFO` writer - TXFIFO for sending message to other core, FIFO size, 4x32 can write one of the word address to push data to the FIFO; can also use 4x32 burst write from 0x010 to push 4 words to the FIFO."]
pub type TXFIFO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TXWRD_TXFIFO0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - TXFIFO for sending message to other core, FIFO size, 4x32 can write one of the word address to push data to the FIFO; can also use 4x32 burst write from 0x010 to push 4 words to the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo(&mut self) -> TXFIFO_W<0> {
        TXFIFO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXFIFO for sending message to other core\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txwrd_txfifo0](index.html) module"]
pub struct TXWRD_TXFIFO0_SPEC;
impl crate::RegisterSpec for TXWRD_TXFIFO0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txwrd_txfifo0::W](W) writer structure"]
impl crate::Writable for TXWRD_TXFIFO0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXWRD_TXFIFO0 to value 0"]
impl crate::Resettable for TXWRD_TXFIFO0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
