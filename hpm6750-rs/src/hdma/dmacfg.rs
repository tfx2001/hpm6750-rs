#[doc = "Register `DMACFG` reader"]
pub struct R(crate::R<DMACFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHANNELNUM` reader - Channel number 0x1: 1 channel 0x2: 2 channels ... 0x8: 8 channels Others: Invalid"]
pub type CHANNELNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIFODEPTH` reader - FIFO depth 0x4: 4 entries 0x8: 8 entries 0x10: 16 entries 0x20: 32 entries Others: Invalid"]
pub type FIFODEPTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REQNUM` reader - Request/acknowledge pair number 0x0: 0 pair 0x1: 1 pair 0x2: 2 pairs ... 0x10: 16 pairs"]
pub type REQNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSNUM` reader - AXI bus interface number 0x0: 1 AXI bus 0x1: 2 AXI busses"]
pub type BUSNUM_R = crate::BitReader<bool>;
#[doc = "Field `CORENUM` reader - DMA core number 0x0: 1 core 0x1: 2 cores"]
pub type CORENUM_R = crate::BitReader<bool>;
#[doc = "Field `ADDRWIDTH` reader - AXI bus address width 0x18: 24 bits 0x19: 25 bits ... 0x40: 64 bits Others: Invalid"]
pub type ADDRWIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAWIDTH` reader - AXI bus data width 0x0: 32 bits 0x1: 64 bits 0x2: 128 bits 0x3: 256 bits"]
pub type DATAWIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REQSYNC` reader - DMA request synchronization. The DMA request synchronization should be configured to avoid signal integrity problems when the request signal is not clocked by the system bus clock, which the DMA control logic operates in. If the request synchronization is not configured, the request signal is sampled directly without synchronization. 0x0: Request synchronization is not configured 0x1: Request synchronization is configured"]
pub type REQSYNC_R = crate::BitReader<bool>;
#[doc = "Field `CHAINXFR` reader - Chain transfer 0x0: Chain transfer is not configured 0x1: Chain transfer is configured"]
pub type CHAINXFR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Channel number 0x1: 1 channel 0x2: 2 channels ... 0x8: 8 channels Others: Invalid"]
    #[inline(always)]
    pub fn channelnum(&self) -> CHANNELNUM_R {
        CHANNELNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - FIFO depth 0x4: 4 entries 0x8: 8 entries 0x10: 16 entries 0x20: 32 entries Others: Invalid"]
    #[inline(always)]
    pub fn fifodepth(&self) -> FIFODEPTH_R {
        FIFODEPTH_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:14 - Request/acknowledge pair number 0x0: 0 pair 0x1: 1 pair 0x2: 2 pairs ... 0x10: 16 pairs"]
    #[inline(always)]
    pub fn reqnum(&self) -> REQNUM_R {
        REQNUM_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - AXI bus interface number 0x0: 1 AXI bus 0x1: 2 AXI busses"]
    #[inline(always)]
    pub fn busnum(&self) -> BUSNUM_R {
        BUSNUM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA core number 0x0: 1 core 0x1: 2 cores"]
    #[inline(always)]
    pub fn corenum(&self) -> CORENUM_R {
        CORENUM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - AXI bus address width 0x18: 24 bits 0x19: 25 bits ... 0x40: 64 bits Others: Invalid"]
    #[inline(always)]
    pub fn addrwidth(&self) -> ADDRWIDTH_R {
        ADDRWIDTH_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:25 - AXI bus data width 0x0: 32 bits 0x1: 64 bits 0x2: 128 bits 0x3: 256 bits"]
    #[inline(always)]
    pub fn datawidth(&self) -> DATAWIDTH_R {
        DATAWIDTH_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - DMA request synchronization. The DMA request synchronization should be configured to avoid signal integrity problems when the request signal is not clocked by the system bus clock, which the DMA control logic operates in. If the request synchronization is not configured, the request signal is sampled directly without synchronization. 0x0: Request synchronization is not configured 0x1: Request synchronization is configured"]
    #[inline(always)]
    pub fn reqsync(&self) -> REQSYNC_R {
        REQSYNC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Chain transfer 0x0: Chain transfer is not configured 0x1: Chain transfer is configured"]
    #[inline(always)]
    pub fn chainxfr(&self) -> CHAINXFR_R {
        CHAINXFR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "DMAC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacfg](index.html) module"]
pub struct DMACFG_SPEC;
impl crate::RegisterSpec for DMACFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmacfg::R](R) reader structure"]
impl crate::Readable for DMACFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACFG to value 0"]
impl crate::Resettable for DMACFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
