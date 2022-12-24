#[doc = "Register `BLK_ATTR` reader"]
pub struct R(crate::R<BLK_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK_ATTR` writer"]
pub struct W(crate::W<BLK_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK_ATTR_SPEC>;
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
impl From<crate::W<BLK_ATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFER_BLOCK_SIZE` reader - Transfer Block Size These bits specify the block size of data transfers. In case of memory, it is set to 512 bytes. It can be accessed only if no transaction is executing. Read operations during transfers may return an invalid value, and write operations are ignored. Following are the values for XFER_BLOCK_SIZE: - 0x1: 1 byte - 0x2: 2 bytes - 0x3: 3 bytes - . - 0x1FF: 511 byte - 0x200: 512 byt es - . - 0x800: 2048 bytes Note: This register must be programmed with a non-zero value for data transfer."]
pub type XFER_BLOCK_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XFER_BLOCK_SIZE` writer - Transfer Block Size These bits specify the block size of data transfers. In case of memory, it is set to 512 bytes. It can be accessed only if no transaction is executing. Read operations during transfers may return an invalid value, and write operations are ignored. Following are the values for XFER_BLOCK_SIZE: - 0x1: 1 byte - 0x2: 2 bytes - 0x3: 3 bytes - . - 0x1FF: 511 byte - 0x200: 512 byt es - . - 0x800: 2048 bytes Note: This register must be programmed with a non-zero value for data transfer."]
pub type XFER_BLOCK_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLK_ATTR_SPEC, u16, u16, 12, O>;
#[doc = "Field `SDMA_BUF_BDARY` reader - SDMA Buffer Boundary These bits specify the size of contiguous buffer in system memory. The SDMA transfer waits at every boundary specified by these fields and the Host Controller generates the DMA interrupt to request the Host Driver to update the SDMA System Address register. Values: - 0x0 (BYTES_4K): 4K bytes SDMA Buffer Boundary - 0x1 (BYTES_8K): 8K bytes SDMA Buffer Boundary - 0x2 (BYTES_16K): 16K bytes SDMA Buffer Boundary - 0x3 (BYTES_32K): 32K bytes SDMA Buffer Boundary - 0x4 (BYTES_64K): 64K bytes SDMA Buffer Boundary - 0x5 (BYTES_128K): 128K bytes SDMA Buffer Boundary - 0x6 (BYTES_256K): 256K bytes SDMA Buffer Boundary - 0x7 (BYTES_512K): 512K bytes SDMA Buffer Boundary"]
pub type SDMA_BUF_BDARY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDMA_BUF_BDARY` writer - SDMA Buffer Boundary These bits specify the size of contiguous buffer in system memory. The SDMA transfer waits at every boundary specified by these fields and the Host Controller generates the DMA interrupt to request the Host Driver to update the SDMA System Address register. Values: - 0x0 (BYTES_4K): 4K bytes SDMA Buffer Boundary - 0x1 (BYTES_8K): 8K bytes SDMA Buffer Boundary - 0x2 (BYTES_16K): 16K bytes SDMA Buffer Boundary - 0x3 (BYTES_32K): 32K bytes SDMA Buffer Boundary - 0x4 (BYTES_64K): 64K bytes SDMA Buffer Boundary - 0x5 (BYTES_128K): 128K bytes SDMA Buffer Boundary - 0x6 (BYTES_256K): 256K bytes SDMA Buffer Boundary - 0x7 (BYTES_512K): 512K bytes SDMA Buffer Boundary"]
pub type SDMA_BUF_BDARY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLK_ATTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `BLOCK_CNT` reader - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - . - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
pub type BLOCK_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLOCK_CNT` writer - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - . - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
pub type BLOCK_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BLK_ATTR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:11 - Transfer Block Size These bits specify the block size of data transfers. In case of memory, it is set to 512 bytes. It can be accessed only if no transaction is executing. Read operations during transfers may return an invalid value, and write operations are ignored. Following are the values for XFER_BLOCK_SIZE: - 0x1: 1 byte - 0x2: 2 bytes - 0x3: 3 bytes - . - 0x1FF: 511 byte - 0x200: 512 byt es - . - 0x800: 2048 bytes Note: This register must be programmed with a non-zero value for data transfer."]
    #[inline(always)]
    pub fn xfer_block_size(&self) -> XFER_BLOCK_SIZE_R {
        XFER_BLOCK_SIZE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary These bits specify the size of contiguous buffer in system memory. The SDMA transfer waits at every boundary specified by these fields and the Host Controller generates the DMA interrupt to request the Host Driver to update the SDMA System Address register. Values: - 0x0 (BYTES_4K): 4K bytes SDMA Buffer Boundary - 0x1 (BYTES_8K): 8K bytes SDMA Buffer Boundary - 0x2 (BYTES_16K): 16K bytes SDMA Buffer Boundary - 0x3 (BYTES_32K): 32K bytes SDMA Buffer Boundary - 0x4 (BYTES_64K): 64K bytes SDMA Buffer Boundary - 0x5 (BYTES_128K): 128K bytes SDMA Buffer Boundary - 0x6 (BYTES_256K): 256K bytes SDMA Buffer Boundary - 0x7 (BYTES_512K): 512K bytes SDMA Buffer Boundary"]
    #[inline(always)]
    pub fn sdma_buf_bdary(&self) -> SDMA_BUF_BDARY_R {
        SDMA_BUF_BDARY_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:31 - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - . - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
    #[inline(always)]
    pub fn block_cnt(&self) -> BLOCK_CNT_R {
        BLOCK_CNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer Block Size These bits specify the block size of data transfers. In case of memory, it is set to 512 bytes. It can be accessed only if no transaction is executing. Read operations during transfers may return an invalid value, and write operations are ignored. Following are the values for XFER_BLOCK_SIZE: - 0x1: 1 byte - 0x2: 2 bytes - 0x3: 3 bytes - . - 0x1FF: 511 byte - 0x200: 512 byt es - . - 0x800: 2048 bytes Note: This register must be programmed with a non-zero value for data transfer."]
    #[inline(always)]
    #[must_use]
    pub fn xfer_block_size(&mut self) -> XFER_BLOCK_SIZE_W<0> {
        XFER_BLOCK_SIZE_W::new(self)
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary These bits specify the size of contiguous buffer in system memory. The SDMA transfer waits at every boundary specified by these fields and the Host Controller generates the DMA interrupt to request the Host Driver to update the SDMA System Address register. Values: - 0x0 (BYTES_4K): 4K bytes SDMA Buffer Boundary - 0x1 (BYTES_8K): 8K bytes SDMA Buffer Boundary - 0x2 (BYTES_16K): 16K bytes SDMA Buffer Boundary - 0x3 (BYTES_32K): 32K bytes SDMA Buffer Boundary - 0x4 (BYTES_64K): 64K bytes SDMA Buffer Boundary - 0x5 (BYTES_128K): 128K bytes SDMA Buffer Boundary - 0x6 (BYTES_256K): 256K bytes SDMA Buffer Boundary - 0x7 (BYTES_512K): 512K bytes SDMA Buffer Boundary"]
    #[inline(always)]
    #[must_use]
    pub fn sdma_buf_bdary(&mut self) -> SDMA_BUF_BDARY_W<12> {
        SDMA_BUF_BDARY_W::new(self)
    }
    #[doc = "Bits 16:31 - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - . - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
    #[inline(always)]
    #[must_use]
    pub fn block_cnt(&mut self) -> BLOCK_CNT_W<16> {
        BLOCK_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk_attr](index.html) module"]
pub struct BLK_ATTR_SPEC;
impl crate::RegisterSpec for BLK_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk_attr::R](R) reader structure"]
impl crate::Readable for BLK_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk_attr::W](W) writer structure"]
impl crate::Writable for BLK_ATTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK_ATTR to value 0x0002_0210"]
impl crate::Resettable for BLK_ATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0210;
}
