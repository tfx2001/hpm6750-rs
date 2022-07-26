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
#[doc = "Field `PHASE_START` reader - Enable this bit to send a START condition at the beginning of transaction. Master mode only."]
pub type PHASE_START_R = crate::BitReader<bool>;
#[doc = "Field `PHASE_START` writer - Enable this bit to send a START condition at the beginning of transaction. Master mode only."]
pub type PHASE_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PHASE_ADDR` reader - Enable this bit to send the address after START condition. Master mode only."]
pub type PHASE_ADDR_R = crate::BitReader<bool>;
#[doc = "Field `PHASE_ADDR` writer - Enable this bit to send the address after START condition. Master mode only."]
pub type PHASE_ADDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PHASE_DATA` reader - Enable this bit to send the data after Address phase. Master mode only."]
pub type PHASE_DATA_R = crate::BitReader<bool>;
#[doc = "Field `PHASE_DATA` writer - Enable this bit to send the data after Address phase. Master mode only."]
pub type PHASE_DATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PHASE_STOP` reader - Enable this bit to send a STOP condition at the end of a transaction. Master mode only."]
pub type PHASE_STOP_R = crate::BitReader<bool>;
#[doc = "Field `PHASE_STOP` writer - Enable this bit to send a STOP condition at the end of a transaction. Master mode only."]
pub type PHASE_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DIR` reader - Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DATACNT` reader - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means 256 bytes. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received."]
pub type DATACNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATACNT` writer - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means 256 bytes. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received."]
pub type DATACNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 12 - Enable this bit to send a START condition at the beginning of transaction. Master mode only."]
    #[inline(always)]
    pub fn phase_start(&self) -> PHASE_START_R {
        PHASE_START_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable this bit to send the address after START condition. Master mode only."]
    #[inline(always)]
    pub fn phase_addr(&self) -> PHASE_ADDR_R {
        PHASE_ADDR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable this bit to send the data after Address phase. Master mode only."]
    #[inline(always)]
    pub fn phase_data(&self) -> PHASE_DATA_R {
        PHASE_DATA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable this bit to send a STOP condition at the end of a transaction. Master mode only."]
    #[inline(always)]
    pub fn phase_stop(&self) -> PHASE_STOP_R {
        PHASE_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:7 - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means 256 bytes. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received."]
    #[inline(always)]
    pub fn datacnt(&self) -> DATACNT_R {
        DATACNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - Enable this bit to send a START condition at the beginning of transaction. Master mode only."]
    #[inline(always)]
    pub fn phase_start(&mut self) -> PHASE_START_W<12> {
        PHASE_START_W::new(self)
    }
    #[doc = "Bit 11 - Enable this bit to send the address after START condition. Master mode only."]
    #[inline(always)]
    pub fn phase_addr(&mut self) -> PHASE_ADDR_W<11> {
        PHASE_ADDR_W::new(self)
    }
    #[doc = "Bit 10 - Enable this bit to send the data after Address phase. Master mode only."]
    #[inline(always)]
    pub fn phase_data(&mut self) -> PHASE_DATA_W<10> {
        PHASE_DATA_W::new(self)
    }
    #[doc = "Bit 9 - Enable this bit to send a STOP condition at the end of a transaction. Master mode only."]
    #[inline(always)]
    pub fn phase_stop(&mut self) -> PHASE_STOP_W<9> {
        PHASE_STOP_W::new(self)
    }
    #[doc = "Bit 8 - Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<8> {
        DIR_W::new(self)
    }
    #[doc = "Bits 0:7 - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means 256 bytes. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received."]
    #[inline(always)]
    pub fn datacnt(&mut self) -> DATACNT_W<0> {
        DATACNT_W::new(self)
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
#[doc = "`reset()` method sets CTRL to value 0x1e00"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1e00
    }
}
