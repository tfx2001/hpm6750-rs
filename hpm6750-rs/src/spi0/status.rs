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
#[doc = "Field `SPIACTIVE` reader - SPI register programming is in progress. In master mode, SPIActive becomes 1 after the SPI command register is written and becomes 0 after the transfer is finished. In slave mode, SPIActive becomes 1 after the SPI CS signal is asserted and becomes 0 after the SPI CS signal is deasserted. Note that due to clock synchronization, it may take at most two spi_clock cycles for SPIActive to change when the corresponding condition happens. Note this bit stays 0 when Direct IO Control or the memory-mapped interface is used."]
pub type SPIACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `RXNUM_5_0` reader - Number of valid entries in the Receive FIFO"]
pub type RXNUM_5_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXEMPTY` reader - Receive FIFO Empty flag"]
pub type RXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` reader - Receive FIFO Full flag"]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `TXNUM_5_0` reader - Number of valid entries in the Transmit FIFO"]
pub type TXNUM_5_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXEMPTY` reader - Transmit FIFO Empty flag"]
pub type TXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `TXFULL` reader - Transmit FIFO Full flag"]
pub type TXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXNUM_7_6` reader - Number of valid entries in the Receive FIFO"]
pub type RXNUM_7_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXNUM_7_6` reader - Number of valid entries in the Transmit FIFO"]
pub type TXNUM_7_6_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - SPI register programming is in progress. In master mode, SPIActive becomes 1 after the SPI command register is written and becomes 0 after the transfer is finished. In slave mode, SPIActive becomes 1 after the SPI CS signal is asserted and becomes 0 after the SPI CS signal is deasserted. Note that due to clock synchronization, it may take at most two spi_clock cycles for SPIActive to change when the corresponding condition happens. Note this bit stays 0 when Direct IO Control or the memory-mapped interface is used."]
    #[inline(always)]
    pub fn spiactive(&self) -> SPIACTIVE_R {
        SPIACTIVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - Number of valid entries in the Receive FIFO"]
    #[inline(always)]
    pub fn rxnum_5_0(&self) -> RXNUM_5_0_R {
        RXNUM_5_0_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Receive FIFO Empty flag"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO Full flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Number of valid entries in the Transmit FIFO"]
    #[inline(always)]
    pub fn txnum_5_0(&self) -> TXNUM_5_0_R {
        TXNUM_5_0_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Transmit FIFO Empty flag"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit FIFO Full flag"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Number of valid entries in the Receive FIFO"]
    #[inline(always)]
    pub fn rxnum_7_6(&self) -> RXNUM_7_6_R {
        RXNUM_7_6_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Number of valid entries in the Transmit FIFO"]
    #[inline(always)]
    pub fn txnum_7_6(&self) -> TXNUM_7_6_R {
        TXNUM_7_6_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
