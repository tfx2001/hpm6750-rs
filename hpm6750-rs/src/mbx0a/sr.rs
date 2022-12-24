#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWMV` reader - RX word message valid, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written word message in the RXREG. 0, no valid word message yet in the RXREG."]
pub type RWMV_R = crate::BitReader<bool>;
#[doc = "Field `TWME` reader - TX word message empty, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, means this core had write word message to TXREG. 0, means no valid word message in the TXREG yet."]
pub type TWME_R = crate::BitReader<bool>;
#[doc = "Field `RFMF` reader - RX FIFO Message Full, message from other core; will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written 4x32 message in the RXFIFO. 0, no 4x32 RX FIFO message from other core yet."]
pub type RFMF_R = crate::BitReader<bool>;
#[doc = "Field `RFMA` reader - RX FIFO Message Available, available data in the 4x32 TX FIFO message buffer to the other core, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, no any data in the 4x32 TXFIFO message buffer. 0, there are some data in the the 4x32 TXFIFO message buffer already."]
pub type RFMA_R = crate::BitReader<bool>;
#[doc = "Field `TFME` reader - TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet."]
pub type TFME_R = crate::BitReader<bool>;
#[doc = "Field `TFME` writer - TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet."]
pub type TFME_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TFMA` reader - TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)"]
pub type TFMA_R = crate::BitReader<bool>;
#[doc = "Field `TFMA` writer - TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)"]
pub type TFMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `EW2RO` reader - bus Error for Write to Read Only address; this bit is W1C bit. 1, write to read only address happened in the bus of this block. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type EW2RO_R = crate::BitReader<bool>;
#[doc = "Field `EW2RO` writer - bus Error for Write to Read Only address; this bit is W1C bit. 1, write to read only address happened in the bus of this block. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type EW2RO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `EAIVA` reader - bus Error for Accessing Invalid Address; this bit is W1C bit. 1, read and write to invalid address in the bus of this block, will set this bit. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type EAIVA_R = crate::BitReader<bool>;
#[doc = "Field `EAIVA` writer - bus Error for Accessing Invalid Address; this bit is W1C bit. 1, read and write to invalid address in the bus of this block, will set this bit. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type EAIVA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `EWTFF` reader - bus Error for write when tx fifo full, this bit is W1C bit. 1, write to a fulled tx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type EWTFF_R = crate::BitReader<bool>;
#[doc = "Field `EWTFF` writer - bus Error for write when tx fifo full, this bit is W1C bit. 1, write to a fulled tx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type EWTFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `ERRFE` reader - bus Error for read when rx fifo empty, this bit is W1C bit. 1, read from a empty rx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type ERRFE_R = crate::BitReader<bool>;
#[doc = "Field `ERRFE` writer - bus Error for read when rx fifo empty, this bit is W1C bit. 1, read from a empty rx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type ERRFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `EWTRF` reader - bus Error for write when tx word message are still valid, this bit is W1C bit. 1, write to word message when the word message are still valid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type EWTRF_R = crate::BitReader<bool>;
#[doc = "Field `EWTRF` writer - bus Error for write when tx word message are still valid, this bit is W1C bit. 1, write to word message when the word message are still valid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type EWTRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `ERRRE` reader - bus Error for read when rx word message are still invalid, this bit is W1C bit. 1, read from word message when the word message are still invalid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type ERRRE_R = crate::BitReader<bool>;
#[doc = "Field `ERRRE` writer - bus Error for read when rx word message are still invalid, this bit is W1C bit. 1, read from word message when the word message are still invalid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type ERRRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `TFEC` reader - TX FIFO empty message word count"]
pub type TFEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFVC` reader - RX FIFO valid message count"]
pub type RFVC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - RX word message valid, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written word message in the RXREG. 0, no valid word message yet in the RXREG."]
    #[inline(always)]
    pub fn rwmv(&self) -> RWMV_R {
        RWMV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX word message empty, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, means this core had write word message to TXREG. 0, means no valid word message in the TXREG yet."]
    #[inline(always)]
    pub fn twme(&self) -> TWME_R {
        TWME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Message Full, message from other core; will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written 4x32 message in the RXFIFO. 0, no 4x32 RX FIFO message from other core yet."]
    #[inline(always)]
    pub fn rfmf(&self) -> RFMF_R {
        RFMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO Message Available, available data in the 4x32 TX FIFO message buffer to the other core, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, no any data in the 4x32 TXFIFO message buffer. 0, there are some data in the the 4x32 TXFIFO message buffer already."]
    #[inline(always)]
    pub fn rfma(&self) -> RFMA_R {
        RFMA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet."]
    #[inline(always)]
    pub fn tfme(&self) -> TFME_R {
        TFME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)"]
    #[inline(always)]
    pub fn tfma(&self) -> TFMA_R {
        TFMA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - bus Error for Write to Read Only address; this bit is W1C bit. 1, write to read only address happened in the bus of this block. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    pub fn ew2ro(&self) -> EW2RO_R {
        EW2RO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - bus Error for Accessing Invalid Address; this bit is W1C bit. 1, read and write to invalid address in the bus of this block, will set this bit. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    pub fn eaiva(&self) -> EAIVA_R {
        EAIVA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - bus Error for write when tx fifo full, this bit is W1C bit. 1, write to a fulled tx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    pub fn ewtff(&self) -> EWTFF_R {
        EWTFF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - bus Error for read when rx fifo empty, this bit is W1C bit. 1, read from a empty rx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    pub fn errfe(&self) -> ERRFE_R {
        ERRFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - bus Error for write when tx word message are still valid, this bit is W1C bit. 1, write to word message when the word message are still valid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    pub fn ewtrf(&self) -> EWTRF_R {
        EWTRF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - bus Error for read when rx word message are still invalid, this bit is W1C bit. 1, read from word message when the word message are still invalid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    pub fn errre(&self) -> ERRRE_R {
        ERRRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - TX FIFO empty message word count"]
    #[inline(always)]
    pub fn tfec(&self) -> TFEC_R {
        TFEC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - RX FIFO valid message count"]
    #[inline(always)]
    pub fn rfvc(&self) -> RFVC_R {
        RFVC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet."]
    #[inline(always)]
    #[must_use]
    pub fn tfme(&mut self) -> TFME_W<6> {
        TFME_W::new(self)
    }
    #[doc = "Bit 7 - TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)"]
    #[inline(always)]
    #[must_use]
    pub fn tfma(&mut self) -> TFMA_W<7> {
        TFMA_W::new(self)
    }
    #[doc = "Bit 8 - bus Error for Write to Read Only address; this bit is W1C bit. 1, write to read only address happened in the bus of this block. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    #[must_use]
    pub fn ew2ro(&mut self) -> EW2RO_W<8> {
        EW2RO_W::new(self)
    }
    #[doc = "Bit 9 - bus Error for Accessing Invalid Address; this bit is W1C bit. 1, read and write to invalid address in the bus of this block, will set this bit. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    #[must_use]
    pub fn eaiva(&mut self) -> EAIVA_W<9> {
        EAIVA_W::new(self)
    }
    #[doc = "Bit 10 - bus Error for write when tx fifo full, this bit is W1C bit. 1, write to a fulled tx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    #[must_use]
    pub fn ewtff(&mut self) -> EWTFF_W<10> {
        EWTFF_W::new(self)
    }
    #[doc = "Bit 11 - bus Error for read when rx fifo empty, this bit is W1C bit. 1, read from a empty rx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    #[must_use]
    pub fn errfe(&mut self) -> ERRFE_W<11> {
        ERRFE_W::new(self)
    }
    #[doc = "Bit 12 - bus Error for write when tx word message are still valid, this bit is W1C bit. 1, write to word message when the word message are still valid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    #[must_use]
    pub fn ewtrf(&mut self) -> EWTRF_W<12> {
        EWTRF_W::new(self)
    }
    #[doc = "Bit 13 - bus Error for read when rx word message are still invalid, this bit is W1C bit. 1, read from word message when the word message are still invalid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    #[must_use]
    pub fn errre(&mut self) -> ERRRE_W<13> {
        ERRRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0xe2"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0xe2;
}
