#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFIFOT` writer - Receiver FIFO trigger level"]
pub type RFIFOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TFIFOT` writer - Transmitter FIFO trigger level"]
pub type TFIFOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DMAE` writer - DMA enable 0: Disable 1: Enable"]
pub type DMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `TFIFORST` writer - Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared."]
pub type TFIFORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `RFIFORST` writer - Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared."]
pub type RFIFORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
#[doc = "Field `FIFOE` writer - FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles."]
pub type FIFOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCR_SPEC, bool, O>;
impl W {
    #[doc = "Bits 6:7 - Receiver FIFO trigger level"]
    #[inline(always)]
    pub fn rfifot(&mut self) -> RFIFOT_W<6> {
        RFIFOT_W::new(self)
    }
    #[doc = "Bits 4:5 - Transmitter FIFO trigger level"]
    #[inline(always)]
    pub fn tfifot(&mut self) -> TFIFOT_W<4> {
        TFIFOT_W::new(self)
    }
    #[doc = "Bit 3 - DMA enable 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn dmae(&mut self) -> DMAE_W<3> {
        DMAE_W::new(self)
    }
    #[doc = "Bit 2 - Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared."]
    #[inline(always)]
    pub fn tfiforst(&mut self) -> TFIFORST_W<2> {
        TFIFORST_W::new(self)
    }
    #[doc = "Bit 1 - Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared."]
    #[inline(always)]
    pub fn rfiforst(&mut self) -> RFIFORST_W<1> {
        RFIFORST_W::new(self)
    }
    #[doc = "Bit 0 - FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles."]
    #[inline(always)]
    pub fn fifoe(&mut self) -> FIFOE_W<0> {
        FIFOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
