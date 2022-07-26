#[doc = "Register `TRANSFMT` reader"]
pub struct R(crate::R<TRANSFMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSFMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSFMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSFMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSFMT` writer"]
pub struct W(crate::W<TRANSFMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSFMT_SPEC>;
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
impl From<crate::W<TRANSFMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSFMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRLEN` reader - Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes"]
pub type ADDRLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRLEN` writer - Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes"]
pub type ADDRLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRANSFMT_SPEC, u8, u8, 2, O>;
#[doc = "Field `DATALEN` reader - The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)"]
pub type DATALEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATALEN` writer - The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)"]
pub type DATALEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRANSFMT_SPEC, u8, u8, 5, O>;
#[doc = "Field `DATAMERGE` reader - Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed."]
pub type DATAMERGE_R = crate::BitReader<bool>;
#[doc = "Field `DATAMERGE` writer - Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed."]
pub type DATAMERGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSFMT_SPEC, bool, O>;
#[doc = "Field `MOSIBIDIR` reader - Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two"]
pub type MOSIBIDIR_R = crate::BitReader<bool>;
#[doc = "Field `MOSIBIDIR` writer - Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two"]
pub type MOSIBIDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSFMT_SPEC, bool, O>;
#[doc = "Field `LSB` reader - Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first"]
pub type LSB_R = crate::BitReader<bool>;
#[doc = "Field `LSB` writer - Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first"]
pub type LSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSFMT_SPEC, bool, O>;
#[doc = "Field `SLVMODE` reader - SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode"]
pub type SLVMODE_R = crate::BitReader<bool>;
#[doc = "Field `SLVMODE` writer - SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode"]
pub type SLVMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSFMT_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSFMT_SPEC, bool, O>;
#[doc = "Field `CPHA` reader - SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges"]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRANSFMT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 16:17 - Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes"]
    #[inline(always)]
    pub fn addrlen(&self) -> ADDRLEN_R {
        ADDRLEN_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 8:12 - The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)"]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed."]
    #[inline(always)]
    pub fn datamerge(&self) -> DATAMERGE_R {
        DATAMERGE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 4 - Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two"]
    #[inline(always)]
    pub fn mosibidir(&self) -> MOSIBIDIR_R {
        MOSIBIDIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first"]
    #[inline(always)]
    pub fn lsb(&self) -> LSB_R {
        LSB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode"]
    #[inline(always)]
    pub fn slvmode(&self) -> SLVMODE_R {
        SLVMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes"]
    #[inline(always)]
    pub fn addrlen(&mut self) -> ADDRLEN_W<16> {
        ADDRLEN_W::new(self)
    }
    #[doc = "Bits 8:12 - The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)"]
    #[inline(always)]
    pub fn datalen(&mut self) -> DATALEN_W<8> {
        DATALEN_W::new(self)
    }
    #[doc = "Bit 7 - Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed."]
    #[inline(always)]
    pub fn datamerge(&mut self) -> DATAMERGE_W<7> {
        DATAMERGE_W::new(self)
    }
    #[doc = "Bit 4 - Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two"]
    #[inline(always)]
    pub fn mosibidir(&mut self) -> MOSIBIDIR_W<4> {
        MOSIBIDIR_W::new(self)
    }
    #[doc = "Bit 3 - Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first"]
    #[inline(always)]
    pub fn lsb(&mut self) -> LSB_W<3> {
        LSB_W::new(self)
    }
    #[doc = "Bit 2 - SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode"]
    #[inline(always)]
    pub fn slvmode(&mut self) -> SLVMODE_W<2> {
        SLVMODE_W::new(self)
    }
    #[doc = "Bit 1 - SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<1> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 0 - SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<0> {
        CPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Format Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transfmt](index.html) module"]
pub struct TRANSFMT_SPEC;
impl crate::RegisterSpec for TRANSFMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transfmt::R](R) reader structure"]
impl crate::Readable for TRANSFMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transfmt::W](W) writer structure"]
impl crate::Writable for TRANSFMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRANSFMT to value 0x0002_0780"]
impl crate::Resettable for TRANSFMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0780
    }
}
