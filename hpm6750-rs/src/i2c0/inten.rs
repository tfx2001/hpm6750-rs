#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOEMPTY` reader - Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty."]
pub type FIFOEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFOEMPTY` writer - Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty."]
pub type FIFOEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `FIFOFULL` reader - Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full."]
pub type FIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `FIFOFULL` writer - Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full."]
pub type FIFOFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `FIFOHALF` reader - Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is <= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered."]
pub type FIFOHALF_R = crate::BitReader<bool>;
#[doc = "Field `FIFOHALF` writer - Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is <= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered."]
pub type FIFOHALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `ADDRHIT` reader - Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed."]
pub type ADDRHIT_R = crate::BitReader<bool>;
#[doc = "Field `ADDRHIT` writer - Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed."]
pub type ADDRHIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `ARBLOSE` reader - Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode."]
pub type ARBLOSE_R = crate::BitReader<bool>;
#[doc = "Field `ARBLOSE` writer - Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode."]
pub type ARBLOSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected."]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `START` reader - Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `BYTETRANS` reader - Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted."]
pub type BYTETRANS_R = crate::BitReader<bool>;
#[doc = "Field `BYTETRANS` writer - Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted."]
pub type BYTETRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `BYTERECV` reader - Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually."]
pub type BYTERECV_R = crate::BitReader<bool>;
#[doc = "Field `BYTERECV` writer - Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually."]
pub type BYTERECV_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
#[doc = "Field `CMPL` reader - Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed."]
pub type CMPL_R = crate::BitReader<bool>;
#[doc = "Field `CMPL` writer - Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed."]
pub type CMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty."]
    #[inline(always)]
    pub fn fifoempty(&self) -> FIFOEMPTY_R {
        FIFOEMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full."]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is <= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered."]
    #[inline(always)]
    pub fn fifohalf(&self) -> FIFOHALF_R {
        FIFOHALF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed."]
    #[inline(always)]
    pub fn addrhit(&self) -> ADDRHIT_R {
        ADDRHIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode."]
    #[inline(always)]
    pub fn arblose(&self) -> ARBLOSE_R {
        ARBLOSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted."]
    #[inline(always)]
    pub fn bytetrans(&self) -> BYTETRANS_R {
        BYTETRANS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually."]
    #[inline(always)]
    pub fn byterecv(&self) -> BYTERECV_R {
        BYTERECV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed."]
    #[inline(always)]
    pub fn cmpl(&self) -> CMPL_R {
        CMPL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty."]
    #[inline(always)]
    #[must_use]
    pub fn fifoempty(&mut self) -> FIFOEMPTY_W<0> {
        FIFOEMPTY_W::new(self)
    }
    #[doc = "Bit 1 - Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full."]
    #[inline(always)]
    #[must_use]
    pub fn fifofull(&mut self) -> FIFOFULL_W<1> {
        FIFOFULL_W::new(self)
    }
    #[doc = "Bit 2 - Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is <= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered."]
    #[inline(always)]
    #[must_use]
    pub fn fifohalf(&mut self) -> FIFOHALF_W<2> {
        FIFOHALF_W::new(self)
    }
    #[doc = "Bit 3 - Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed."]
    #[inline(always)]
    #[must_use]
    pub fn addrhit(&mut self) -> ADDRHIT_W<3> {
        ADDRHIT_W::new(self)
    }
    #[doc = "Bit 4 - Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode."]
    #[inline(always)]
    #[must_use]
    pub fn arblose(&mut self) -> ARBLOSE_W<4> {
        ARBLOSE_W::new(self)
    }
    #[doc = "Bit 5 - Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<5> {
        STOP_W::new(self)
    }
    #[doc = "Bit 6 - Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<6> {
        START_W::new(self)
    }
    #[doc = "Bit 7 - Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn bytetrans(&mut self) -> BYTETRANS_W<7> {
        BYTETRANS_W::new(self)
    }
    #[doc = "Bit 8 - Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually."]
    #[inline(always)]
    #[must_use]
    pub fn byterecv(&mut self) -> BYTERECV_W<8> {
        BYTERECV_W::new(self)
    }
    #[doc = "Bit 9 - Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed."]
    #[inline(always)]
    #[must_use]
    pub fn cmpl(&mut self) -> CMPL_W<9> {
        CMPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
