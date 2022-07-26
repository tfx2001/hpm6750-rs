#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXTHRES` reader - Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold."]
pub type TXTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXTHRES` writer - Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold."]
pub type TXTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `RXTHRES` reader - Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold."]
pub type RXTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXTHRES` writer - Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold."]
pub type RXTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXDMAEN` reader - TX DMA enable"]
pub type TXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `TXDMAEN` writer - TX DMA enable"]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RXDMAEN` reader - RX DMA enable"]
pub type RXDMAEN_R = crate::BitReader<bool>;
#[doc = "Field `RXDMAEN` writer - RX DMA enable"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TXFIFORST` reader - Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
pub type TXFIFORST_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFORST` writer - Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
pub type TXFIFORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RXFIFORST` reader - Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
pub type RXFIFORST_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFORST` writer - Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
pub type RXFIFORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SPIRST` reader - SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
pub type SPIRST_R = crate::BitReader<bool>;
#[doc = "Field `SPIRST` writer - SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
pub type SPIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 16:23 - Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold."]
    #[inline(always)]
    pub fn txthres(&self) -> TXTHRES_R {
        TXTHRES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold."]
    #[inline(always)]
    pub fn rxthres(&self) -> RXTHRES_R {
        RXTHRES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 4 - TX DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - RX DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
    #[inline(always)]
    pub fn txfiforst(&self) -> TXFIFORST_R {
        TXFIFORST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
    #[inline(always)]
    pub fn rxfiforst(&self) -> RXFIFORST_R {
        RXFIFORST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
    #[inline(always)]
    pub fn spirst(&self) -> SPIRST_R {
        SPIRST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold."]
    #[inline(always)]
    pub fn txthres(&mut self) -> TXTHRES_W<16> {
        TXTHRES_W::new(self)
    }
    #[doc = "Bits 8:15 - Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold."]
    #[inline(always)]
    pub fn rxthres(&mut self) -> RXTHRES_W<8> {
        RXTHRES_W::new(self)
    }
    #[doc = "Bit 4 - TX DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<4> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bit 3 - RX DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<3> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bit 2 - Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
    #[inline(always)]
    pub fn txfiforst(&mut self) -> TXFIFORST_W<2> {
        TXFIFORST_W::new(self)
    }
    #[doc = "Bit 1 - Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
    #[inline(always)]
    pub fn rxfiforst(&mut self) -> RXFIFORST_W<1> {
        RXFIFORST_W::new(self)
    }
    #[doc = "Bit 0 - SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
    #[inline(always)]
    pub fn spirst(&mut self) -> SPIRST_W<0> {
        SPIRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
