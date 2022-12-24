#[doc = "Register `MMC_INTR_MASK_TX` reader"]
pub struct R(crate::R<MMC_INTR_MASK_TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_INTR_MASK_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_INTR_MASK_TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_INTR_MASK_TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_INTR_MASK_TX` writer"]
pub struct W(crate::W<MMC_INTR_MASK_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_INTR_MASK_TX_SPEC>;
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
impl From<crate::W<MMC_INTR_MASK_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_INTR_MASK_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXGBOCTIM` reader - MMC Transmit Good Bad Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
pub type TXGBOCTIM_R = crate::BitReader<bool>;
#[doc = "Field `TXGBOCTIM` writer - MMC Transmit Good Bad Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
pub type TXGBOCTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXGBFRMIM` reader - MMC Transmit Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txframecount_gb counter reaches half of the maximum value or the maximum value."]
pub type TXGBFRMIM_R = crate::BitReader<bool>;
#[doc = "Field `TXGBFRMIM` writer - MMC Transmit Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txframecount_gb counter reaches half of the maximum value or the maximum value."]
pub type TXGBFRMIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXBCGFIM` reader - MMC Transmit Broadcast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
pub type TXBCGFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXBCGFIM` writer - MMC Transmit Broadcast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
pub type TXBCGFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXMCGFIM` reader - MMC Transmit Multicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastframes_g counter reaches half of the maximum value or the maximum value."]
pub type TXMCGFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXMCGFIM` writer - MMC Transmit Multicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastframes_g counter reaches half of the maximum value or the maximum value."]
pub type TXMCGFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TX64OCTGBFIM` reader - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX64OCTGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `TX64OCTGBFIM` writer - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX64OCTGBFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TX65T127OCTGBFIM` reader - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX65T127OCTGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `TX65T127OCTGBFIM` writer - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX65T127OCTGBFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TX128T255OCTGBFIM` reader - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX128T255OCTGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `TX128T255OCTGBFIM` writer - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX128T255OCTGBFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TX256T511OCTGBFIM` reader - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX256T511OCTGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `TX256T511OCTGBFIM` writer - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX256T511OCTGBFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TX512T1023OCTGBFIM` reader - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX512T1023OCTGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `TX512T1023OCTGBFIM` writer - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX512T1023OCTGBFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TX1024TMAXOCTGBFIM` reader - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX1024TMAXOCTGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `TX1024TMAXOCTGBFIM` writer - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX1024TMAXOCTGBFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXUCGBFIM` reader - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txunicastframes_gb counter reaches half of the maximum value or the maximum value."]
pub type TXUCGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXUCGBFIM` writer - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txunicastframes_gb counter reaches half of the maximum value or the maximum value."]
pub type TXUCGBFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXMCGBFIM` reader - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastframes_gb counter reaches half of the maximum value or the maximum value."]
pub type TXMCGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXMCGBFIM` writer - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastframes_gb counter reaches half of the maximum value or the maximum value."]
pub type TXMCGBFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXBCGBFIM` reader - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastframes_gb counter reaches half of the maximum value or the maximum value."]
pub type TXBCGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXBCGBFIM` writer - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastframes_gb counter reaches half of the maximum value or the maximum value."]
pub type TXBCGBFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXUFLOWERFIM` reader - MMC Transmit Underflow Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
pub type TXUFLOWERFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXUFLOWERFIM` writer - MMC Transmit Underflow Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
pub type TXUFLOWERFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXSCOLGFIM` reader - MMC Transmit Single Collision Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
pub type TXSCOLGFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXSCOLGFIM` writer - MMC Transmit Single Collision Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
pub type TXSCOLGFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXMCOLGFIM` reader - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
pub type TXMCOLGFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXMCOLGFIM` writer - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
pub type TXMCOLGFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXDEFFIM` reader - MMC Transmit Deferred Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txdeferred counter reaches half of the maximum value or the maximum value."]
pub type TXDEFFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXDEFFIM` writer - MMC Transmit Deferred Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txdeferred counter reaches half of the maximum value or the maximum value."]
pub type TXDEFFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXLATCOLFIM` reader - MMC Transmit Late Collision Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txlatecol counter reaches half of the maximum value or the maximum value."]
pub type TXLATCOLFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXLATCOLFIM` writer - MMC Transmit Late Collision Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txlatecol counter reaches half of the maximum value or the maximum value."]
pub type TXLATCOLFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXEXCOLFIM` reader - MMC Transmit Excessive Collision Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txexcesscol counter reaches half of the maximum value or the maximum value."]
pub type TXEXCOLFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXEXCOLFIM` writer - MMC Transmit Excessive Collision Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txexcesscol counter reaches half of the maximum value or the maximum value."]
pub type TXEXCOLFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXCARERFIM` reader - MMC Transmit Carrier Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txcarriererror counter reaches half of the maximum value or the maximum value."]
pub type TXCARERFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXCARERFIM` writer - MMC Transmit Carrier Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txcarriererror counter reaches half of the maximum value or the maximum value."]
pub type TXCARERFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXGOCTIM` reader - MMC Transmit Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
pub type TXGOCTIM_R = crate::BitReader<bool>;
#[doc = "Field `TXGOCTIM` writer - MMC Transmit Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
pub type TXGOCTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXGFRMIM` reader - MMC Transmit Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txframecount_g counter reaches half of the maximum value or the maximum value."]
pub type TXGFRMIM_R = crate::BitReader<bool>;
#[doc = "Field `TXGFRMIM` writer - MMC Transmit Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txframecount_g counter reaches half of the maximum value or the maximum value."]
pub type TXGFRMIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXEXDEFFIM` reader - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txexcessdef counter reaches half of the maximum value or the maximum value."]
pub type TXEXDEFFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXEXDEFFIM` writer - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txexcessdef counter reaches half of the maximum value or the maximum value."]
pub type TXEXDEFFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXPAUSFIM` reader - MMC Transmit Pause Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txpauseframes counter reaches half of the maximum value or the maximum value."]
pub type TXPAUSFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXPAUSFIM` writer - MMC Transmit Pause Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txpauseframes counter reaches half of the maximum value or the maximum value."]
pub type TXPAUSFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXVLANGFIM` reader - MMC Transmit VLAN Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txvlanframes_g counter reaches half of the maximum value or the maximum value."]
pub type TXVLANGFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXVLANGFIM` writer - MMC Transmit VLAN Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txvlanframes_g counter reaches half of the maximum value or the maximum value."]
pub type TXVLANGFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
#[doc = "Field `TXOSIZEGFIM` reader - MMC Transmit Oversize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txoversize_g counter reaches half of the maximum value or the maximum value."]
pub type TXOSIZEGFIM_R = crate::BitReader<bool>;
#[doc = "Field `TXOSIZEGFIM` writer - MMC Transmit Oversize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txoversize_g counter reaches half of the maximum value or the maximum value."]
pub type TXOSIZEGFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_TX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txgboctim(&self) -> TXGBOCTIM_R {
        TXGBOCTIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txframecount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txgbfrmim(&self) -> TXGBFRMIM_R {
        TXGBFRMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txbcgfim(&self) -> TXBCGFIM_R {
        TXBCGFIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txmcgfim(&self) -> TXMCGFIM_R {
        TXMCGFIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn tx64octgbfim(&self) -> TX64OCTGBFIM_R {
        TX64OCTGBFIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn tx65t127octgbfim(&self) -> TX65T127OCTGBFIM_R {
        TX65T127OCTGBFIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn tx128t255octgbfim(&self) -> TX128T255OCTGBFIM_R {
        TX128T255OCTGBFIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn tx256t511octgbfim(&self) -> TX256T511OCTGBFIM_R {
        TX256T511OCTGBFIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn tx512t1023octgbfim(&self) -> TX512T1023OCTGBFIM_R {
        TX512T1023OCTGBFIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfim(&self) -> TX1024TMAXOCTGBFIM_R {
        TX1024TMAXOCTGBFIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txunicastframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txucgbfim(&self) -> TXUCGBFIM_R {
        TXUCGBFIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txmcgbfim(&self) -> TXMCGBFIM_R {
        TXMCGBFIM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txbcgbfim(&self) -> TXBCGBFIM_R {
        TXBCGBFIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txuflowerfim(&self) -> TXUFLOWERFIM_R {
        TXUFLOWERFIM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txscolgfim(&self) -> TXSCOLGFIM_R {
        TXSCOLGFIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txmcolgfim(&self) -> TXMCOLGFIM_R {
        TXMCOLGFIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txdeferred counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txdeffim(&self) -> TXDEFFIM_R {
        TXDEFFIM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txlatecol counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txlatcolfim(&self) -> TXLATCOLFIM_R {
        TXLATCOLFIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txexcesscol counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txexcolfim(&self) -> TXEXCOLFIM_R {
        TXEXCOLFIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txcarriererror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txcarerfim(&self) -> TXCARERFIM_R {
        TXCARERFIM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txgoctim(&self) -> TXGOCTIM_R {
        TXGOCTIM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txframecount_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txgfrmim(&self) -> TXGFRMIM_R {
        TXGFRMIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txexcessdef counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txexdeffim(&self) -> TXEXDEFFIM_R {
        TXEXDEFFIM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txpauseframes counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txpausfim(&self) -> TXPAUSFIM_R {
        TXPAUSFIM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txvlanframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txvlangfim(&self) -> TXVLANGFIM_R {
        TXVLANGFIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txoversize_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txosizegfim(&self) -> TXOSIZEGFIM_R {
        TXOSIZEGFIM_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txgboctim(&mut self) -> TXGBOCTIM_W<0> {
        TXGBOCTIM_W::new(self)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txframecount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txgbfrmim(&mut self) -> TXGBFRMIM_W<1> {
        TXGBFRMIM_W::new(self)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txbcgfim(&mut self) -> TXBCGFIM_W<2> {
        TXBCGFIM_W::new(self)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txmcgfim(&mut self) -> TXMCGFIM_W<3> {
        TXMCGFIM_W::new(self)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn tx64octgbfim(&mut self) -> TX64OCTGBFIM_W<4> {
        TX64OCTGBFIM_W::new(self)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn tx65t127octgbfim(&mut self) -> TX65T127OCTGBFIM_W<5> {
        TX65T127OCTGBFIM_W::new(self)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn tx128t255octgbfim(&mut self) -> TX128T255OCTGBFIM_W<6> {
        TX128T255OCTGBFIM_W::new(self)
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn tx256t511octgbfim(&mut self) -> TX256T511OCTGBFIM_W<7> {
        TX256T511OCTGBFIM_W::new(self)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn tx512t1023octgbfim(&mut self) -> TX512T1023OCTGBFIM_W<8> {
        TX512T1023OCTGBFIM_W::new(self)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn tx1024tmaxoctgbfim(&mut self) -> TX1024TMAXOCTGBFIM_W<9> {
        TX1024TMAXOCTGBFIM_W::new(self)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txunicastframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txucgbfim(&mut self) -> TXUCGBFIM_W<10> {
        TXUCGBFIM_W::new(self)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticastframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txmcgbfim(&mut self) -> TXMCGBFIM_W<11> {
        TXMCGBFIM_W::new(self)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txbroadcastframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txbcgbfim(&mut self) -> TXBCGBFIM_W<12> {
        TXBCGBFIM_W::new(self)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txuflowerfim(&mut self) -> TXUFLOWERFIM_W<13> {
        TXUFLOWERFIM_W::new(self)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txscolgfim(&mut self) -> TXSCOLGFIM_W<14> {
        TXSCOLGFIM_W::new(self)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txmcolgfim(&mut self) -> TXMCOLGFIM_W<15> {
        TXMCOLGFIM_W::new(self)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txdeferred counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txdeffim(&mut self) -> TXDEFFIM_W<16> {
        TXDEFFIM_W::new(self)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txlatecol counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txlatcolfim(&mut self) -> TXLATCOLFIM_W<17> {
        TXLATCOLFIM_W::new(self)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txexcesscol counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txexcolfim(&mut self) -> TXEXCOLFIM_W<18> {
        TXEXCOLFIM_W::new(self)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txcarriererror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txcarerfim(&mut self) -> TXCARERFIM_W<19> {
        TXCARERFIM_W::new(self)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txgoctim(&mut self) -> TXGOCTIM_W<20> {
        TXGOCTIM_W::new(self)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txframecount_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txgfrmim(&mut self) -> TXGFRMIM_W<21> {
        TXGFRMIM_W::new(self)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txexcessdef counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txexdeffim(&mut self) -> TXEXDEFFIM_W<22> {
        TXEXDEFFIM_W::new(self)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txpauseframes counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txpausfim(&mut self) -> TXPAUSFIM_W<23> {
        TXPAUSFIM_W::new(self)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txvlanframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txvlangfim(&mut self) -> TXVLANGFIM_W<24> {
        TXVLANGFIM_W::new(self)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the txoversize_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txosizegfim(&mut self) -> TXOSIZEGFIM_W<25> {
        TXOSIZEGFIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Transmit Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_intr_mask_tx](index.html) module"]
pub struct MMC_INTR_MASK_TX_SPEC;
impl crate::RegisterSpec for MMC_INTR_MASK_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_intr_mask_tx::R](R) reader structure"]
impl crate::Readable for MMC_INTR_MASK_TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_intr_mask_tx::W](W) writer structure"]
impl crate::Writable for MMC_INTR_MASK_TX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMC_INTR_MASK_TX to value 0"]
impl crate::Resettable for MMC_INTR_MASK_TX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
