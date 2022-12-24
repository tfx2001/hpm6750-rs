#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOEMPTY` reader - Indicates that the FIFO is empty."]
pub type FIFOEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFOFULL` reader - Indicates that the FIFO is full."]
pub type FIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `FIFOHALF` reader - Transmitter: Indicates that the FIFO is half-empty."]
pub type FIFOHALF_R = crate::BitReader<bool>;
#[doc = "Field `ADDRHIT` reader - Master: indicates that a slave has responded to the transaction. Slave: indicates that a transaction is targeting the controller (including the General Call)."]
pub type ADDRHIT_R = crate::BitReader<bool>;
#[doc = "Field `ADDRHIT` writer - Master: indicates that a slave has responded to the transaction. Slave: indicates that a transaction is targeting the controller (including the General Call)."]
pub type ADDRHIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `ARBLOSE` reader - Indicates that the controller has lost the bus arbitration."]
pub type ARBLOSE_R = crate::BitReader<bool>;
#[doc = "Field `ARBLOSE` writer - Indicates that the controller has lost the bus arbitration."]
pub type ARBLOSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Indicates that a STOP Condition has been transmitted/received."]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Indicates that a STOP Condition has been transmitted/received."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `START` reader - Indicates that a START Condition or a repeated START condition has been transmitted/received."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Indicates that a START Condition or a repeated START condition has been transmitted/received."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `BYTETRANS` reader - Indicates that a byte of data has been transmitted."]
pub type BYTETRANS_R = crate::BitReader<bool>;
#[doc = "Field `BYTETRANS` writer - Indicates that a byte of data has been transmitted."]
pub type BYTETRANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `BYTERECV` reader - Indicates that a byte of data has been received."]
pub type BYTERECV_R = crate::BitReader<bool>;
#[doc = "Field `BYTERECV` writer - Indicates that a byte of data has been received."]
pub type BYTERECV_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `CMPL` reader - Transaction Completion Master: Indicates that a transaction has been issued from this master and completed without losing the bus arbitration Slave: Indicates that a transaction addressing the controller has been completed. This status bit must be cleared to receive the next transaction; otherwise, the next incoming transaction will be blocked."]
pub type CMPL_R = crate::BitReader<bool>;
#[doc = "Field `CMPL` writer - Transaction Completion Master: Indicates that a transaction has been issued from this master and completed without losing the bus arbitration Slave: Indicates that a transaction addressing the controller has been completed. This status bit must be cleared to receive the next transaction; otherwise, the next incoming transaction will be blocked."]
pub type CMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `ACK` reader - Indicates the type of the last received/transmitted acknowledgement bit: 1: ACK 0: NACK"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `BUSBUSY` reader - Indicates that the bus is busy The bus is busy when a START condition is on bus and it ends when a STOP condition is seen on bus 1: Busy 0: Not busy"]
pub type BUSBUSY_R = crate::BitReader<bool>;
#[doc = "Field `GENCALL` reader - Indicates that the address of the current transaction is a general call address: 1: General call 0: Not general call"]
pub type GENCALL_R = crate::BitReader<bool>;
#[doc = "Field `LINESCL` reader - Indicates the current status of the SCL line on the bus 1: high 0: low"]
pub type LINESCL_R = crate::BitReader<bool>;
#[doc = "Field `LINESDA` reader - Indicates the current status of the SDA line on the bus 1: high 0: low"]
pub type LINESDA_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates that the FIFO is empty."]
    #[inline(always)]
    pub fn fifoempty(&self) -> FIFOEMPTY_R {
        FIFOEMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that the FIFO is full."]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter: Indicates that the FIFO is half-empty."]
    #[inline(always)]
    pub fn fifohalf(&self) -> FIFOHALF_R {
        FIFOHALF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master: indicates that a slave has responded to the transaction. Slave: indicates that a transaction is targeting the controller (including the General Call)."]
    #[inline(always)]
    pub fn addrhit(&self) -> ADDRHIT_R {
        ADDRHIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that the controller has lost the bus arbitration."]
    #[inline(always)]
    pub fn arblose(&self) -> ARBLOSE_R {
        ARBLOSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that a STOP Condition has been transmitted/received."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that a START Condition or a repeated START condition has been transmitted/received."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that a byte of data has been transmitted."]
    #[inline(always)]
    pub fn bytetrans(&self) -> BYTETRANS_R {
        BYTETRANS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates that a byte of data has been received."]
    #[inline(always)]
    pub fn byterecv(&self) -> BYTERECV_R {
        BYTERECV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transaction Completion Master: Indicates that a transaction has been issued from this master and completed without losing the bus arbitration Slave: Indicates that a transaction addressing the controller has been completed. This status bit must be cleared to receive the next transaction; otherwise, the next incoming transaction will be blocked."]
    #[inline(always)]
    pub fn cmpl(&self) -> CMPL_R {
        CMPL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates the type of the last received/transmitted acknowledgement bit: 1: ACK 0: NACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates that the bus is busy The bus is busy when a START condition is on bus and it ends when a STOP condition is seen on bus 1: Busy 0: Not busy"]
    #[inline(always)]
    pub fn busbusy(&self) -> BUSBUSY_R {
        BUSBUSY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates that the address of the current transaction is a general call address: 1: General call 0: Not general call"]
    #[inline(always)]
    pub fn gencall(&self) -> GENCALL_R {
        GENCALL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates the current status of the SCL line on the bus 1: high 0: low"]
    #[inline(always)]
    pub fn linescl(&self) -> LINESCL_R {
        LINESCL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates the current status of the SDA line on the bus 1: high 0: low"]
    #[inline(always)]
    pub fn linesda(&self) -> LINESDA_R {
        LINESDA_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Master: indicates that a slave has responded to the transaction. Slave: indicates that a transaction is targeting the controller (including the General Call)."]
    #[inline(always)]
    #[must_use]
    pub fn addrhit(&mut self) -> ADDRHIT_W<3> {
        ADDRHIT_W::new(self)
    }
    #[doc = "Bit 4 - Indicates that the controller has lost the bus arbitration."]
    #[inline(always)]
    #[must_use]
    pub fn arblose(&mut self) -> ARBLOSE_W<4> {
        ARBLOSE_W::new(self)
    }
    #[doc = "Bit 5 - Indicates that a STOP Condition has been transmitted/received."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<5> {
        STOP_W::new(self)
    }
    #[doc = "Bit 6 - Indicates that a START Condition or a repeated START condition has been transmitted/received."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<6> {
        START_W::new(self)
    }
    #[doc = "Bit 7 - Indicates that a byte of data has been transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn bytetrans(&mut self) -> BYTETRANS_W<7> {
        BYTETRANS_W::new(self)
    }
    #[doc = "Bit 8 - Indicates that a byte of data has been received."]
    #[inline(always)]
    #[must_use]
    pub fn byterecv(&mut self) -> BYTERECV_W<8> {
        BYTERECV_W::new(self)
    }
    #[doc = "Bit 9 - Transaction Completion Master: Indicates that a transaction has been issued from this master and completed without losing the bus arbitration Slave: Indicates that a transaction addressing the controller has been completed. This status bit must be cleared to receive the next transaction; otherwise, the next incoming transaction will be blocked."]
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
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x01"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
