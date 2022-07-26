#[doc = "Register `DMA_AXI_MODE` reader"]
pub struct R(crate::R<DMA_AXI_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_AXI_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_AXI_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_AXI_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_AXI_MODE` writer"]
pub struct W(crate::W<DMA_AXI_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_AXI_MODE_SPEC>;
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
impl From<crate::W<DMA_AXI_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_AXI_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_LPI` reader - Enable Low Power Interface (LPI) When set to 1, this bit enables the LPI mode supported by the GMAC-AXI configuration and accepts the LPI request from the AXI System Clock controller. When set to 0, this bit disables the LPI mode and always denies the LPI request from the AXI System Clock controller."]
pub type EN_LPI_R = crate::BitReader<bool>;
#[doc = "Field `EN_LPI` writer - Enable Low Power Interface (LPI) When set to 1, this bit enables the LPI mode supported by the GMAC-AXI configuration and accepts the LPI request from the AXI System Clock controller. When set to 0, this bit disables the LPI mode and always denies the LPI request from the AXI System Clock controller."]
pub type EN_LPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_AXI_MODE_SPEC, bool, O>;
#[doc = "Field `LPI_XIT_FRM` reader - Unlock on Magic Packet or Remote Wake-Up Frame When set to 1, this bit enables the GMAC-AXI to come out of the LPI mode only when the magic packet or remote wake-up frame is received. When set to 0, this bit enables the GMAC-AXI to come out of LPI mode when any frame is received."]
pub type LPI_XIT_FRM_R = crate::BitReader<bool>;
#[doc = "Field `LPI_XIT_FRM` writer - Unlock on Magic Packet or Remote Wake-Up Frame When set to 1, this bit enables the GMAC-AXI to come out of the LPI mode only when the magic packet or remote wake-up frame is received. When set to 0, this bit enables the GMAC-AXI to come out of LPI mode when any frame is received."]
pub type LPI_XIT_FRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_AXI_MODE_SPEC, bool, O>;
#[doc = "Field `WR_OSR_LMT` reader - AXI Maximum Write Outstanding Request Limit This value limits the maximum outstanding request on the AXI write interface. Maximum outstanding requests = WR_OSR_LMT+1 Note: - Bit 22 is reserved if AXI_GM_MAX_WR_REQUESTS = 4. - Bit 23 bit is reserved if AXI_GM_MAX_WR_REQUESTS != 16."]
pub type WR_OSR_LMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_OSR_LMT` writer - AXI Maximum Write Outstanding Request Limit This value limits the maximum outstanding request on the AXI write interface. Maximum outstanding requests = WR_OSR_LMT+1 Note: - Bit 22 is reserved if AXI_GM_MAX_WR_REQUESTS = 4. - Bit 23 bit is reserved if AXI_GM_MAX_WR_REQUESTS != 16."]
pub type WR_OSR_LMT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_AXI_MODE_SPEC, u8, u8, 4, O>;
#[doc = "Field `RD_OSR_LMT` reader - AXI Maximum Read Outstanding Request Limit This value limits the maximum outstanding request on the AXI read interface. Maximum outstanding requests = RD_OSR_LMT+1 Note: - Bit 18 is reserved if AXI_GM_MAX_RD_REQUESTS = 4. - Bit 19 is reserved if AXI_GM_MAX_RD_REQUESTS != 16."]
pub type RD_OSR_LMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_OSR_LMT` writer - AXI Maximum Read Outstanding Request Limit This value limits the maximum outstanding request on the AXI read interface. Maximum outstanding requests = RD_OSR_LMT+1 Note: - Bit 18 is reserved if AXI_GM_MAX_RD_REQUESTS = 4. - Bit 19 is reserved if AXI_GM_MAX_RD_REQUESTS != 16."]
pub type RD_OSR_LMT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_AXI_MODE_SPEC, u8, u8, 4, O>;
#[doc = "Field `ONEKBBE` reader - 1 KB Boundary Crossing Enable for the GMAC-AXI Master When set, the GMAC-AXI master performs burst transfers that do not cross 1 KB boundary. When reset, the GMAC-AXI master performs burst transfers that do not cross 4 KB boundary."]
pub type ONEKBBE_R = crate::BitReader<bool>;
#[doc = "Field `ONEKBBE` writer - 1 KB Boundary Crossing Enable for the GMAC-AXI Master When set, the GMAC-AXI master performs burst transfers that do not cross 1 KB boundary. When reset, the GMAC-AXI master performs burst transfers that do not cross 4 KB boundary."]
pub type ONEKBBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_AXI_MODE_SPEC, bool, O>;
#[doc = "Field `AXI_AAL` reader - Address-Aligned Beats This bit is read-only bit and reflects the Bit 25 (AAL) of Register 0 (Bus Mode Register). When this bit is set to 1, the GMAC-AXI performs address-aligned burst transfers on both read and write channels."]
pub type AXI_AAL_R = crate::BitReader<bool>;
#[doc = "Field `AXI_AAL` writer - Address-Aligned Beats This bit is read-only bit and reflects the Bit 25 (AAL) of Register 0 (Bus Mode Register). When this bit is set to 1, the GMAC-AXI performs address-aligned burst transfers on both read and write channels."]
pub type AXI_AAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_AXI_MODE_SPEC, bool, O>;
#[doc = "Field `BLEN256` reader - AXI Burst Length 256 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 256 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 256. Otherwise, this bit is reserved and is read-only (RO)."]
pub type BLEN256_R = crate::BitReader<bool>;
#[doc = "Field `BLEN256` writer - AXI Burst Length 256 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 256 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 256. Otherwise, this bit is reserved and is read-only (RO)."]
pub type BLEN256_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_AXI_MODE_SPEC, bool, O>;
#[doc = "Field `BLEN128` reader - AXI Burst Length 128 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 128 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 128 or more. Otherwise, this bit is reserved and is read-only (RO)."]
pub type BLEN128_R = crate::BitReader<bool>;
#[doc = "Field `BLEN128` writer - AXI Burst Length 128 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 128 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 128 or more. Otherwise, this bit is reserved and is read-only (RO)."]
pub type BLEN128_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_AXI_MODE_SPEC, bool, O>;
#[doc = "Field `BLEN64` reader - AXI Burst Length 64 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 64 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 64 or more. Otherwise, this bit is reserved and is read-only (RO)."]
pub type BLEN64_R = crate::BitReader<bool>;
#[doc = "Field `BLEN64` writer - AXI Burst Length 64 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 64 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 64 or more. Otherwise, this bit is reserved and is read-only (RO)."]
pub type BLEN64_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_AXI_MODE_SPEC, bool, O>;
#[doc = "Field `BLEN32` reader - AXI Burst Length 32 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 32 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 32 or more. Otherwise, this bit is reserved and is read-only (RO)."]
pub type BLEN32_R = crate::BitReader<bool>;
#[doc = "Field `BLEN32` writer - AXI Burst Length 32 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 32 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 32 or more. Otherwise, this bit is reserved and is read-only (RO)."]
pub type BLEN32_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_AXI_MODE_SPEC, bool, O>;
#[doc = "Field `BLEN16` reader - AXI Burst Length 16 When this bit is set to 1 or UNDEF is set to 1, the GMAC-AXI is allowed to select a burst length of 16 on the AXI master interface."]
pub type BLEN16_R = crate::BitReader<bool>;
#[doc = "Field `BLEN16` writer - AXI Burst Length 16 When this bit is set to 1 or UNDEF is set to 1, the GMAC-AXI is allowed to select a burst length of 16 on the AXI master interface."]
pub type BLEN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_AXI_MODE_SPEC, bool, O>;
#[doc = "Field `BLEN8` reader - AXI Burst Length 8 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 8 on the AXI master interface. Setting this bit has no effect when UNDEF is set to 1."]
pub type BLEN8_R = crate::BitReader<bool>;
#[doc = "Field `BLEN8` writer - AXI Burst Length 8 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 8 on the AXI master interface. Setting this bit has no effect when UNDEF is set to 1."]
pub type BLEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_AXI_MODE_SPEC, bool, O>;
#[doc = "Field `BLEN4` reader - AXI Burst Length 4 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 4 on the AXI master interface. Setting this bit has no effect when UNDEF is set to 1."]
pub type BLEN4_R = crate::BitReader<bool>;
#[doc = "Field `BLEN4` writer - AXI Burst Length 4 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 4 on the AXI master interface. Setting this bit has no effect when UNDEF is set to 1."]
pub type BLEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_AXI_MODE_SPEC, bool, O>;
#[doc = "Field `UNDEF` reader - AXI Undefined Burst Length This bit is read-only bit and indicates the complement (invert) value of Bit 16 (FB) in Register 0 (Bus Mode Register). - When this bit is set to 1, the GMAC-AXI is allowed to perform any burst length equal to or below the maximum allowed burst length programmed in Bits\\[7:3\\]. - When this bit is set to 0, the GMAC-AXI is allowed to perform only fixed burst lengths as indicated by BLEN256, BLEN128, BLEN64, BLEN32, BLEN16, BLEN8, or BLEN4, or a burst length of 1. If UNDEF is set and none of the BLEN bits is set, then GMAC-AXI is allowed to perform a burst length of 16."]
pub type UNDEF_R = crate::BitReader<bool>;
#[doc = "Field `UNDEF` writer - AXI Undefined Burst Length This bit is read-only bit and indicates the complement (invert) value of Bit 16 (FB) in Register 0 (Bus Mode Register). - When this bit is set to 1, the GMAC-AXI is allowed to perform any burst length equal to or below the maximum allowed burst length programmed in Bits\\[7:3\\]. - When this bit is set to 0, the GMAC-AXI is allowed to perform only fixed burst lengths as indicated by BLEN256, BLEN128, BLEN64, BLEN32, BLEN16, BLEN8, or BLEN4, or a burst length of 1. If UNDEF is set and none of the BLEN bits is set, then GMAC-AXI is allowed to perform a burst length of 16."]
pub type UNDEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_AXI_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Enable Low Power Interface (LPI) When set to 1, this bit enables the LPI mode supported by the GMAC-AXI configuration and accepts the LPI request from the AXI System Clock controller. When set to 0, this bit disables the LPI mode and always denies the LPI request from the AXI System Clock controller."]
    #[inline(always)]
    pub fn en_lpi(&self) -> EN_LPI_R {
        EN_LPI_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Unlock on Magic Packet or Remote Wake-Up Frame When set to 1, this bit enables the GMAC-AXI to come out of the LPI mode only when the magic packet or remote wake-up frame is received. When set to 0, this bit enables the GMAC-AXI to come out of LPI mode when any frame is received."]
    #[inline(always)]
    pub fn lpi_xit_frm(&self) -> LPI_XIT_FRM_R {
        LPI_XIT_FRM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 20:23 - AXI Maximum Write Outstanding Request Limit This value limits the maximum outstanding request on the AXI write interface. Maximum outstanding requests = WR_OSR_LMT+1 Note: - Bit 22 is reserved if AXI_GM_MAX_WR_REQUESTS = 4. - Bit 23 bit is reserved if AXI_GM_MAX_WR_REQUESTS != 16."]
    #[inline(always)]
    pub fn wr_osr_lmt(&self) -> WR_OSR_LMT_R {
        WR_OSR_LMT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AXI Maximum Read Outstanding Request Limit This value limits the maximum outstanding request on the AXI read interface. Maximum outstanding requests = RD_OSR_LMT+1 Note: - Bit 18 is reserved if AXI_GM_MAX_RD_REQUESTS = 4. - Bit 19 is reserved if AXI_GM_MAX_RD_REQUESTS != 16."]
    #[inline(always)]
    pub fn rd_osr_lmt(&self) -> RD_OSR_LMT_R {
        RD_OSR_LMT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - 1 KB Boundary Crossing Enable for the GMAC-AXI Master When set, the GMAC-AXI master performs burst transfers that do not cross 1 KB boundary. When reset, the GMAC-AXI master performs burst transfers that do not cross 4 KB boundary."]
    #[inline(always)]
    pub fn onekbbe(&self) -> ONEKBBE_R {
        ONEKBBE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Address-Aligned Beats This bit is read-only bit and reflects the Bit 25 (AAL) of Register 0 (Bus Mode Register). When this bit is set to 1, the GMAC-AXI performs address-aligned burst transfers on both read and write channels."]
    #[inline(always)]
    pub fn axi_aal(&self) -> AXI_AAL_R {
        AXI_AAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 7 - AXI Burst Length 256 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 256 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 256. Otherwise, this bit is reserved and is read-only (RO)."]
    #[inline(always)]
    pub fn blen256(&self) -> BLEN256_R {
        BLEN256_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - AXI Burst Length 128 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 128 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 128 or more. Otherwise, this bit is reserved and is read-only (RO)."]
    #[inline(always)]
    pub fn blen128(&self) -> BLEN128_R {
        BLEN128_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - AXI Burst Length 64 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 64 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 64 or more. Otherwise, this bit is reserved and is read-only (RO)."]
    #[inline(always)]
    pub fn blen64(&self) -> BLEN64_R {
        BLEN64_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - AXI Burst Length 32 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 32 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 32 or more. Otherwise, this bit is reserved and is read-only (RO)."]
    #[inline(always)]
    pub fn blen32(&self) -> BLEN32_R {
        BLEN32_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - AXI Burst Length 16 When this bit is set to 1 or UNDEF is set to 1, the GMAC-AXI is allowed to select a burst length of 16 on the AXI master interface."]
    #[inline(always)]
    pub fn blen16(&self) -> BLEN16_R {
        BLEN16_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - AXI Burst Length 8 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 8 on the AXI master interface. Setting this bit has no effect when UNDEF is set to 1."]
    #[inline(always)]
    pub fn blen8(&self) -> BLEN8_R {
        BLEN8_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - AXI Burst Length 4 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 4 on the AXI master interface. Setting this bit has no effect when UNDEF is set to 1."]
    #[inline(always)]
    pub fn blen4(&self) -> BLEN4_R {
        BLEN4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - AXI Undefined Burst Length This bit is read-only bit and indicates the complement (invert) value of Bit 16 (FB) in Register 0 (Bus Mode Register). - When this bit is set to 1, the GMAC-AXI is allowed to perform any burst length equal to or below the maximum allowed burst length programmed in Bits\\[7:3\\]. - When this bit is set to 0, the GMAC-AXI is allowed to perform only fixed burst lengths as indicated by BLEN256, BLEN128, BLEN64, BLEN32, BLEN16, BLEN8, or BLEN4, or a burst length of 1. If UNDEF is set and none of the BLEN bits is set, then GMAC-AXI is allowed to perform a burst length of 16."]
    #[inline(always)]
    pub fn undef(&self) -> UNDEF_R {
        UNDEF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Enable Low Power Interface (LPI) When set to 1, this bit enables the LPI mode supported by the GMAC-AXI configuration and accepts the LPI request from the AXI System Clock controller. When set to 0, this bit disables the LPI mode and always denies the LPI request from the AXI System Clock controller."]
    #[inline(always)]
    pub fn en_lpi(&mut self) -> EN_LPI_W<31> {
        EN_LPI_W::new(self)
    }
    #[doc = "Bit 30 - Unlock on Magic Packet or Remote Wake-Up Frame When set to 1, this bit enables the GMAC-AXI to come out of the LPI mode only when the magic packet or remote wake-up frame is received. When set to 0, this bit enables the GMAC-AXI to come out of LPI mode when any frame is received."]
    #[inline(always)]
    pub fn lpi_xit_frm(&mut self) -> LPI_XIT_FRM_W<30> {
        LPI_XIT_FRM_W::new(self)
    }
    #[doc = "Bits 20:23 - AXI Maximum Write Outstanding Request Limit This value limits the maximum outstanding request on the AXI write interface. Maximum outstanding requests = WR_OSR_LMT+1 Note: - Bit 22 is reserved if AXI_GM_MAX_WR_REQUESTS = 4. - Bit 23 bit is reserved if AXI_GM_MAX_WR_REQUESTS != 16."]
    #[inline(always)]
    pub fn wr_osr_lmt(&mut self) -> WR_OSR_LMT_W<20> {
        WR_OSR_LMT_W::new(self)
    }
    #[doc = "Bits 16:19 - AXI Maximum Read Outstanding Request Limit This value limits the maximum outstanding request on the AXI read interface. Maximum outstanding requests = RD_OSR_LMT+1 Note: - Bit 18 is reserved if AXI_GM_MAX_RD_REQUESTS = 4. - Bit 19 is reserved if AXI_GM_MAX_RD_REQUESTS != 16."]
    #[inline(always)]
    pub fn rd_osr_lmt(&mut self) -> RD_OSR_LMT_W<16> {
        RD_OSR_LMT_W::new(self)
    }
    #[doc = "Bit 13 - 1 KB Boundary Crossing Enable for the GMAC-AXI Master When set, the GMAC-AXI master performs burst transfers that do not cross 1 KB boundary. When reset, the GMAC-AXI master performs burst transfers that do not cross 4 KB boundary."]
    #[inline(always)]
    pub fn onekbbe(&mut self) -> ONEKBBE_W<13> {
        ONEKBBE_W::new(self)
    }
    #[doc = "Bit 12 - Address-Aligned Beats This bit is read-only bit and reflects the Bit 25 (AAL) of Register 0 (Bus Mode Register). When this bit is set to 1, the GMAC-AXI performs address-aligned burst transfers on both read and write channels."]
    #[inline(always)]
    pub fn axi_aal(&mut self) -> AXI_AAL_W<12> {
        AXI_AAL_W::new(self)
    }
    #[doc = "Bit 7 - AXI Burst Length 256 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 256 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 256. Otherwise, this bit is reserved and is read-only (RO)."]
    #[inline(always)]
    pub fn blen256(&mut self) -> BLEN256_W<7> {
        BLEN256_W::new(self)
    }
    #[doc = "Bit 6 - AXI Burst Length 128 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 128 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 128 or more. Otherwise, this bit is reserved and is read-only (RO)."]
    #[inline(always)]
    pub fn blen128(&mut self) -> BLEN128_W<6> {
        BLEN128_W::new(self)
    }
    #[doc = "Bit 5 - AXI Burst Length 64 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 64 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 64 or more. Otherwise, this bit is reserved and is read-only (RO)."]
    #[inline(always)]
    pub fn blen64(&mut self) -> BLEN64_W<5> {
        BLEN64_W::new(self)
    }
    #[doc = "Bit 4 - AXI Burst Length 32 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 32 on the AXI master interface. This bit is present only when the configuration parameter AXI_BL is set to 32 or more. Otherwise, this bit is reserved and is read-only (RO)."]
    #[inline(always)]
    pub fn blen32(&mut self) -> BLEN32_W<4> {
        BLEN32_W::new(self)
    }
    #[doc = "Bit 3 - AXI Burst Length 16 When this bit is set to 1 or UNDEF is set to 1, the GMAC-AXI is allowed to select a burst length of 16 on the AXI master interface."]
    #[inline(always)]
    pub fn blen16(&mut self) -> BLEN16_W<3> {
        BLEN16_W::new(self)
    }
    #[doc = "Bit 2 - AXI Burst Length 8 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 8 on the AXI master interface. Setting this bit has no effect when UNDEF is set to 1."]
    #[inline(always)]
    pub fn blen8(&mut self) -> BLEN8_W<2> {
        BLEN8_W::new(self)
    }
    #[doc = "Bit 1 - AXI Burst Length 4 When this bit is set to 1, the GMAC-AXI is allowed to select a burst length of 4 on the AXI master interface. Setting this bit has no effect when UNDEF is set to 1."]
    #[inline(always)]
    pub fn blen4(&mut self) -> BLEN4_W<1> {
        BLEN4_W::new(self)
    }
    #[doc = "Bit 0 - AXI Undefined Burst Length This bit is read-only bit and indicates the complement (invert) value of Bit 16 (FB) in Register 0 (Bus Mode Register). - When this bit is set to 1, the GMAC-AXI is allowed to perform any burst length equal to or below the maximum allowed burst length programmed in Bits\\[7:3\\]. - When this bit is set to 0, the GMAC-AXI is allowed to perform only fixed burst lengths as indicated by BLEN256, BLEN128, BLEN64, BLEN32, BLEN16, BLEN8, or BLEN4, or a burst length of 1. If UNDEF is set and none of the BLEN bits is set, then GMAC-AXI is allowed to perform a burst length of 16."]
    #[inline(always)]
    pub fn undef(&mut self) -> UNDEF_W<0> {
        UNDEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI Bus Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_axi_mode](index.html) module"]
pub struct DMA_AXI_MODE_SPEC;
impl crate::RegisterSpec for DMA_AXI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_axi_mode::R](R) reader structure"]
impl crate::Readable for DMA_AXI_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_axi_mode::W](W) writer structure"]
impl crate::Writable for DMA_AXI_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_AXI_MODE to value 0"]
impl crate::Resettable for DMA_AXI_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
