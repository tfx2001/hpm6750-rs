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
#[doc = "Field `I2S_EN` reader - enable for the module"]
pub type I2S_EN_R = crate::BitReader<bool>;
#[doc = "Field `I2S_EN` writer - enable for the module"]
pub type I2S_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RX_EN` reader - enable for each RX data pad"]
pub type RX_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_EN` writer - enable for each RX data pad"]
pub type RX_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `TX_EN` reader - enable for each TX data pad"]
pub type TX_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_EN` writer - enable for each TX data pad"]
pub type TX_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RXFIFOCLR` reader - Self-clear"]
pub type RXFIFOCLR_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOCLR` writer - Self-clear"]
pub type RXFIFOCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TXFIFOCLR` reader - Self-clear"]
pub type TXFIFOCLR_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOCLR` writer - Self-clear"]
pub type TXFIFOCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RX_DMA_EN` reader - Asserted to use DMA, else to use interrupt"]
pub type RX_DMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_DMA_EN` writer - Asserted to use DMA, else to use interrupt"]
pub type RX_DMA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TX_DMA_EN` reader - Asserted to use DMA, else to use interrupt"]
pub type TX_DMA_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_DMA_EN` writer - Asserted to use DMA, else to use interrupt"]
pub type TX_DMA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable This bit controls the generation of an interrupt when an error condition (UD, OV) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
pub type ERRIE_R = crate::BitReader<bool>;
#[doc = "Field `ERRIE` writer - Error interrupt enable This bit controls the generation of an interrupt when an error condition (UD, OV) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RXDAIE` reader - RX buffer data available interrupt enable 0: RXNE interrupt masked 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
pub type RXDAIE_R = crate::BitReader<bool>;
#[doc = "Field `RXDAIE` writer - RX buffer data available interrupt enable 0: RXNE interrupt masked 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
pub type RXDAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `TXDNIE` reader - TX buffer data needed interrupt enable 0: TXE interrupt masked 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
pub type TXDNIE_R = crate::BitReader<bool>;
#[doc = "Field `TXDNIE` writer - TX buffer data needed interrupt enable 0: TXE interrupt masked 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
pub type TXDNIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SFTRST_CLKGEN` reader - software reset the CLK GEN module if asserted to be 1'b1. Self-clear."]
pub type SFTRST_CLKGEN_R = crate::BitReader<bool>;
#[doc = "Field `SFTRST_CLKGEN` writer - software reset the CLK GEN module if asserted to be 1'b1. Self-clear."]
pub type SFTRST_CLKGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SFTRST_TX` reader - software reset the TX module if asserted to be 1'b1. Self-clear."]
pub type SFTRST_TX_R = crate::BitReader<bool>;
#[doc = "Field `SFTRST_TX` writer - software reset the TX module if asserted to be 1'b1. Self-clear."]
pub type SFTRST_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SFTRST_RX` reader - software reset the RX module if asserted to be 1'b1. Self-clear."]
pub type SFTRST_RX_R = crate::BitReader<bool>;
#[doc = "Field `SFTRST_RX` writer - software reset the RX module if asserted to be 1'b1. Self-clear."]
pub type SFTRST_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - enable for the module"]
    #[inline(always)]
    pub fn i2s_en(&self) -> I2S_EN_R {
        I2S_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - enable for each RX data pad"]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - enable for each TX data pad"]
    #[inline(always)]
    pub fn tx_en(&self) -> TX_EN_R {
        TX_EN_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Self-clear"]
    #[inline(always)]
    pub fn rxfifoclr(&self) -> RXFIFOCLR_R {
        RXFIFOCLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Self-clear"]
    #[inline(always)]
    pub fn txfifoclr(&self) -> TXFIFOCLR_R {
        TXFIFOCLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Asserted to use DMA, else to use interrupt"]
    #[inline(always)]
    pub fn rx_dma_en(&self) -> RX_DMA_EN_R {
        RX_DMA_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Asserted to use DMA, else to use interrupt"]
    #[inline(always)]
    pub fn tx_dma_en(&self) -> TX_DMA_EN_R {
        TX_DMA_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Error interrupt enable This bit controls the generation of an interrupt when an error condition (UD, OV) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RX buffer data available interrupt enable 0: RXNE interrupt masked 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    #[inline(always)]
    pub fn rxdaie(&self) -> RXDAIE_R {
        RXDAIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TX buffer data needed interrupt enable 0: TXE interrupt masked 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    #[inline(always)]
    pub fn txdnie(&self) -> TXDNIE_R {
        TXDNIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - software reset the CLK GEN module if asserted to be 1'b1. Self-clear."]
    #[inline(always)]
    pub fn sftrst_clkgen(&self) -> SFTRST_CLKGEN_R {
        SFTRST_CLKGEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - software reset the TX module if asserted to be 1'b1. Self-clear."]
    #[inline(always)]
    pub fn sftrst_tx(&self) -> SFTRST_TX_R {
        SFTRST_TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - software reset the RX module if asserted to be 1'b1. Self-clear."]
    #[inline(always)]
    pub fn sftrst_rx(&self) -> SFTRST_RX_R {
        SFTRST_RX_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable for the module"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_en(&mut self) -> I2S_EN_W<0> {
        I2S_EN_W::new(self)
    }
    #[doc = "Bits 1:4 - enable for each RX data pad"]
    #[inline(always)]
    #[must_use]
    pub fn rx_en(&mut self) -> RX_EN_W<1> {
        RX_EN_W::new(self)
    }
    #[doc = "Bits 5:8 - enable for each TX data pad"]
    #[inline(always)]
    #[must_use]
    pub fn tx_en(&mut self) -> TX_EN_W<5> {
        TX_EN_W::new(self)
    }
    #[doc = "Bit 9 - Self-clear"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifoclr(&mut self) -> RXFIFOCLR_W<9> {
        RXFIFOCLR_W::new(self)
    }
    #[doc = "Bit 10 - Self-clear"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoclr(&mut self) -> TXFIFOCLR_W<10> {
        TXFIFOCLR_W::new(self)
    }
    #[doc = "Bit 11 - Asserted to use DMA, else to use interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dma_en(&mut self) -> RX_DMA_EN_W<11> {
        RX_DMA_EN_W::new(self)
    }
    #[doc = "Bit 12 - Asserted to use DMA, else to use interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tx_dma_en(&mut self) -> TX_DMA_EN_W<12> {
        TX_DMA_EN_W::new(self)
    }
    #[doc = "Bit 13 - Error interrupt enable This bit controls the generation of an interrupt when an error condition (UD, OV) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<13> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 14 - RX buffer data available interrupt enable 0: RXNE interrupt masked 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn rxdaie(&mut self) -> RXDAIE_W<14> {
        RXDAIE_W::new(self)
    }
    #[doc = "Bit 15 - TX buffer data needed interrupt enable 0: TXE interrupt masked 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    #[inline(always)]
    #[must_use]
    pub fn txdnie(&mut self) -> TXDNIE_W<15> {
        TXDNIE_W::new(self)
    }
    #[doc = "Bit 16 - software reset the CLK GEN module if asserted to be 1'b1. Self-clear."]
    #[inline(always)]
    #[must_use]
    pub fn sftrst_clkgen(&mut self) -> SFTRST_CLKGEN_W<16> {
        SFTRST_CLKGEN_W::new(self)
    }
    #[doc = "Bit 17 - software reset the TX module if asserted to be 1'b1. Self-clear."]
    #[inline(always)]
    #[must_use]
    pub fn sftrst_tx(&mut self) -> SFTRST_TX_W<17> {
        SFTRST_TX_W::new(self)
    }
    #[doc = "Bit 18 - software reset the RX module if asserted to be 1'b1. Self-clear."]
    #[inline(always)]
    #[must_use]
    pub fn sftrst_rx(&mut self) -> SFTRST_RX_W<18> {
        SFTRST_RX_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
