#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFOSIZE` reader - Depth of RX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words"]
pub type RXFIFOSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFIFOSIZE` reader - Depth of TX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words"]
pub type TXFIFOSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUALSPI` reader - Support for Dual I/O SPI"]
pub type DUALSPI_R = crate::BitReader<bool>;
#[doc = "Field `QUADSPI` reader - Support for Quad I/O SPI"]
pub type QUADSPI_R = crate::BitReader<bool>;
#[doc = "Field `DIRECTIO` reader - Support for Direct SPI IO"]
pub type DIRECTIO_R = crate::BitReader<bool>;
#[doc = "Field `SLAVE` reader - Support for SPI Slave mode"]
pub type SLAVE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Depth of RX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Depth of TX FIFO 0x0: 2 words 0x1: 4 words 0x2: 8 words 0x3: 16 words 0x4: 32 words 0x5: 64 words 0x6: 128 words"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Support for Dual I/O SPI"]
    #[inline(always)]
    pub fn dualspi(&self) -> DUALSPI_R {
        DUALSPI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Support for Quad I/O SPI"]
    #[inline(always)]
    pub fn quadspi(&self) -> QUADSPI_R {
        QUADSPI_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Support for Direct SPI IO"]
    #[inline(always)]
    pub fn directio(&self) -> DIRECTIO_R {
        DIRECTIO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Support for SPI Slave mode"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONFIG to value 0x4311"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x4311;
}
