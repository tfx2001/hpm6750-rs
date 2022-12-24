#[doc = "Register `INTRST` reader"]
pub struct R(crate::R<INTRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTRST` writer"]
pub struct W(crate::W<INTRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTRST_SPEC>;
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
impl From<crate::W<INTRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFOORINT` reader - RX FIFO Overrun interrupt. This bit is set when RX FIFO Overrun interrupts occur. (Slave mode only)"]
pub type RXFIFOORINT_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOORINT` writer - RX FIFO Overrun interrupt. This bit is set when RX FIFO Overrun interrupts occur. (Slave mode only)"]
pub type RXFIFOORINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRST_SPEC, bool, O>;
#[doc = "Field `TXFIFOURINT` reader - TX FIFO Underrun interrupt. This bit is set when TX FIFO Underrun interrupts occur. (Slave mode only)"]
pub type TXFIFOURINT_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOURINT` writer - TX FIFO Underrun interrupt. This bit is set when TX FIFO Underrun interrupts occur. (Slave mode only)"]
pub type TXFIFOURINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRST_SPEC, bool, O>;
#[doc = "Field `RXFIFOINT` reader - RX FIFO Threshold interrupt. This bit is set when RX FIFO Threshold interrupts occur."]
pub type RXFIFOINT_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOINT` writer - RX FIFO Threshold interrupt. This bit is set when RX FIFO Threshold interrupts occur."]
pub type RXFIFOINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRST_SPEC, bool, O>;
#[doc = "Field `TXFIFOINT` reader - TX FIFO Threshold interrupt. This bit is set when TX FIFO Threshold interrupts occur."]
pub type TXFIFOINT_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOINT` writer - TX FIFO Threshold interrupt. This bit is set when TX FIFO Threshold interrupts occur."]
pub type TXFIFOINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRST_SPEC, bool, O>;
#[doc = "Field `ENDINT` reader - End of SPI Transfer interrupt. This bit is set when End of SPI Transfer interrupts occur."]
pub type ENDINT_R = crate::BitReader<bool>;
#[doc = "Field `ENDINT` writer - End of SPI Transfer interrupt. This bit is set when End of SPI Transfer interrupts occur."]
pub type ENDINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRST_SPEC, bool, O>;
#[doc = "Field `SLVCMDINT` reader - Slave Command Interrupt. This bit is set when Slave Command interrupts occur. (Slave mode only)"]
pub type SLVCMDINT_R = crate::BitReader<bool>;
#[doc = "Field `SLVCMDINT` writer - Slave Command Interrupt. This bit is set when Slave Command interrupts occur. (Slave mode only)"]
pub type SLVCMDINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTRST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RX FIFO Overrun interrupt. This bit is set when RX FIFO Overrun interrupts occur. (Slave mode only)"]
    #[inline(always)]
    pub fn rxfifoorint(&self) -> RXFIFOORINT_R {
        RXFIFOORINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Underrun interrupt. This bit is set when TX FIFO Underrun interrupts occur. (Slave mode only)"]
    #[inline(always)]
    pub fn txfifourint(&self) -> TXFIFOURINT_R {
        TXFIFOURINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Threshold interrupt. This bit is set when RX FIFO Threshold interrupts occur."]
    #[inline(always)]
    pub fn rxfifoint(&self) -> RXFIFOINT_R {
        RXFIFOINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FIFO Threshold interrupt. This bit is set when TX FIFO Threshold interrupts occur."]
    #[inline(always)]
    pub fn txfifoint(&self) -> TXFIFOINT_R {
        TXFIFOINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of SPI Transfer interrupt. This bit is set when End of SPI Transfer interrupts occur."]
    #[inline(always)]
    pub fn endint(&self) -> ENDINT_R {
        ENDINT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave Command Interrupt. This bit is set when Slave Command interrupts occur. (Slave mode only)"]
    #[inline(always)]
    pub fn slvcmdint(&self) -> SLVCMDINT_R {
        SLVCMDINT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX FIFO Overrun interrupt. This bit is set when RX FIFO Overrun interrupts occur. (Slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifoorint(&mut self) -> RXFIFOORINT_W<0> {
        RXFIFOORINT_W::new(self)
    }
    #[doc = "Bit 1 - TX FIFO Underrun interrupt. This bit is set when TX FIFO Underrun interrupts occur. (Slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn txfifourint(&mut self) -> TXFIFOURINT_W<1> {
        TXFIFOURINT_W::new(self)
    }
    #[doc = "Bit 2 - RX FIFO Threshold interrupt. This bit is set when RX FIFO Threshold interrupts occur."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifoint(&mut self) -> RXFIFOINT_W<2> {
        RXFIFOINT_W::new(self)
    }
    #[doc = "Bit 3 - TX FIFO Threshold interrupt. This bit is set when TX FIFO Threshold interrupts occur."]
    #[inline(always)]
    #[must_use]
    pub fn txfifoint(&mut self) -> TXFIFOINT_W<3> {
        TXFIFOINT_W::new(self)
    }
    #[doc = "Bit 4 - End of SPI Transfer interrupt. This bit is set when End of SPI Transfer interrupts occur."]
    #[inline(always)]
    #[must_use]
    pub fn endint(&mut self) -> ENDINT_W<4> {
        ENDINT_W::new(self)
    }
    #[doc = "Bit 5 - Slave Command Interrupt. This bit is set when Slave Command interrupts occur. (Slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn slvcmdint(&mut self) -> SLVCMDINT_W<5> {
        SLVCMDINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intrst](index.html) module"]
pub struct INTRST_SPEC;
impl crate::RegisterSpec for INTRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intrst::R](R) reader structure"]
impl crate::Readable for INTRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intrst::W](W) writer structure"]
impl crate::Writable for INTRST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTRST to value 0"]
impl crate::Resettable for INTRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
