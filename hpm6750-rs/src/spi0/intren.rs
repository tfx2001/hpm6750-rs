#[doc = "Register `INTREN` reader"]
pub struct R(crate::R<INTREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTREN` writer"]
pub struct W(crate::W<INTREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTREN_SPEC>;
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
impl From<crate::W<INTREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFOORINTEN` reader - Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)"]
pub type RXFIFOORINTEN_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOORINTEN` writer - Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)"]
pub type RXFIFOORINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTREN_SPEC, bool, O>;
#[doc = "Field `TXFIFOURINTEN` reader - Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)"]
pub type TXFIFOURINTEN_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOURINTEN` writer - Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)"]
pub type TXFIFOURINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTREN_SPEC, bool, O>;
#[doc = "Field `RXFIFOINTEN` reader - Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold."]
pub type RXFIFOINTEN_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOINTEN` writer - Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold."]
pub type RXFIFOINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTREN_SPEC, bool, O>;
#[doc = "Field `TXFIFOINTEN` reader - Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold."]
pub type TXFIFOINTEN_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOINTEN` writer - Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold."]
pub type TXFIFOINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTREN_SPEC, bool, O>;
#[doc = "Field `ENDINTEN` reader - Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)"]
pub type ENDINTEN_R = crate::BitReader<bool>;
#[doc = "Field `ENDINTEN` writer - Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)"]
pub type ENDINTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTREN_SPEC, bool, O>;
#[doc = "Field `SLVCMDEN` reader - Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)"]
pub type SLVCMDEN_R = crate::BitReader<bool>;
#[doc = "Field `SLVCMDEN` writer - Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)"]
pub type SLVCMDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTREN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)"]
    #[inline(always)]
    pub fn rxfifoorinten(&self) -> RXFIFOORINTEN_R {
        RXFIFOORINTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)"]
    #[inline(always)]
    pub fn txfifourinten(&self) -> TXFIFOURINTEN_R {
        TXFIFOURINTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold."]
    #[inline(always)]
    pub fn rxfifointen(&self) -> RXFIFOINTEN_R {
        RXFIFOINTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold."]
    #[inline(always)]
    pub fn txfifointen(&self) -> TXFIFOINTEN_R {
        TXFIFOINTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)"]
    #[inline(always)]
    pub fn endinten(&self) -> ENDINTEN_R {
        ENDINTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)"]
    #[inline(always)]
    pub fn slvcmden(&self) -> SLVCMDEN_R {
        SLVCMDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifoorinten(&mut self) -> RXFIFOORINTEN_W<0> {
        RXFIFOORINTEN_W::new(self)
    }
    #[doc = "Bit 1 - Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn txfifourinten(&mut self) -> TXFIFOURINTEN_W<1> {
        TXFIFOURINTEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifointen(&mut self) -> RXFIFOINTEN_W<2> {
        RXFIFOINTEN_W::new(self)
    }
    #[doc = "Bit 3 - Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold."]
    #[inline(always)]
    #[must_use]
    pub fn txfifointen(&mut self) -> TXFIFOINTEN_W<3> {
        TXFIFOINTEN_W::new(self)
    }
    #[doc = "Bit 4 - Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)"]
    #[inline(always)]
    #[must_use]
    pub fn endinten(&mut self) -> ENDINTEN_W<4> {
        ENDINTEN_W::new(self)
    }
    #[doc = "Bit 5 - Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn slvcmden(&mut self) -> SLVCMDEN_W<5> {
        SLVCMDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intren](index.html) module"]
pub struct INTREN_SPEC;
impl crate::RegisterSpec for INTREN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intren::R](R) reader structure"]
impl crate::Readable for INTREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intren::W](W) writer structure"]
impl crate::Writable for INTREN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTREN to value 0"]
impl crate::Resettable for INTREN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
