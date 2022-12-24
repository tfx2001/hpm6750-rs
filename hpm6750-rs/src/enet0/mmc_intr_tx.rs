#[doc = "Register `MMC_INTR_TX` reader"]
pub struct R(crate::R<MMC_INTR_TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_INTR_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_INTR_TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_INTR_TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_INTR_TX` writer"]
pub struct W(crate::W<MMC_INTR_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_INTR_TX_SPEC>;
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
impl From<crate::W<MMC_INTR_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_INTR_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXGBOCTIS` reader - MMC Transmit Good Bad Octet Counter Interrupt Status This bit is set when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
pub type TXGBOCTIS_R = crate::BitReader<bool>;
#[doc = "Field `TXGBOCTIS` writer - MMC Transmit Good Bad Octet Counter Interrupt Status This bit is set when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
pub type TXGBOCTIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXGBFRMIS` reader - MMC Transmit Good Bad Frame Counter Interrupt Status This bit is set when the txframecount_gb counter reaches half of the maximum value or the maximum value."]
pub type TXGBFRMIS_R = crate::BitReader<bool>;
#[doc = "Field `TXGBFRMIS` writer - MMC Transmit Good Bad Frame Counter Interrupt Status This bit is set when the txframecount_gb counter reaches half of the maximum value or the maximum value."]
pub type TXGBFRMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXBCGFIS` reader - MMC Transmit Broadcast Good Frame Counter Interrupt Status This bit is set when the txbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
pub type TXBCGFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXBCGFIS` writer - MMC Transmit Broadcast Good Frame Counter Interrupt Status This bit is set when the txbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
pub type TXBCGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXMCGFIS` reader - MMC Transmit Multicast Good Frame Counter Interrupt Status This bit is set when the txmulticastframes_g counter reaches half of the maximum value or the maximum value."]
pub type TXMCGFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXMCGFIS` writer - MMC Transmit Multicast Good Frame Counter Interrupt Status This bit is set when the txmulticastframes_g counter reaches half of the maximum value or the maximum value."]
pub type TXMCGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TX64OCTGBFIS` reader - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX64OCTGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `TX64OCTGBFIS` writer - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX64OCTGBFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TX65T127OCTGBFIS` reader - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx65to127octets_gb counter reaches half the maximum value, and also when it reaches the maximum value."]
pub type TX65T127OCTGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `TX65T127OCTGBFIS` writer - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx65to127octets_gb counter reaches half the maximum value, and also when it reaches the maximum value."]
pub type TX65T127OCTGBFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TX128T255OCTGBFIS` reader - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX128T255OCTGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `TX128T255OCTGBFIS` writer - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX128T255OCTGBFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TX256T511OCTGBFIS` reader - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX256T511OCTGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `TX256T511OCTGBFIS` writer - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX256T511OCTGBFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TX512T1023OCTGBFIS` reader - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX512T1023OCTGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `TX512T1023OCTGBFIS` writer - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX512T1023OCTGBFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TX1024TMAXOCTGBFIS` reader - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX1024TMAXOCTGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `TX1024TMAXOCTGBFIS` writer - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
pub type TX1024TMAXOCTGBFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXUCGBFIS` reader - MMC Transmit Unicast Good Bad Frame Counter Interrupt Status This bit is set when the txunicastframes_gb counter reaches half of the maximum value or the maximum value."]
pub type TXUCGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXUCGBFIS` writer - MMC Transmit Unicast Good Bad Frame Counter Interrupt Status This bit is set when the txunicastframes_gb counter reaches half of the maximum value or the maximum value."]
pub type TXUCGBFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXMCGBFIS` reader - MMC Transmit Multicast Good Bad Frame Counter Interrupt Status The bit is set when the txmulticastframes_gb counter reaches half of the maximum value or the maximum value."]
pub type TXMCGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXMCGBFIS` writer - MMC Transmit Multicast Good Bad Frame Counter Interrupt Status The bit is set when the txmulticastframes_gb counter reaches half of the maximum value or the maximum value."]
pub type TXMCGBFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXBCGBFIS` reader - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status This bit is set when the txbroadcastframes_gb counter reaches half of the maximum value or the maximum value."]
pub type TXBCGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXBCGBFIS` writer - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status This bit is set when the txbroadcastframes_gb counter reaches half of the maximum value or the maximum value."]
pub type TXBCGBFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXUFLOWERFIS` reader - MMC Transmit Underflow Error Frame Counter Interrupt Status This bit is set when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
pub type TXUFLOWERFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXUFLOWERFIS` writer - MMC Transmit Underflow Error Frame Counter Interrupt Status This bit is set when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
pub type TXUFLOWERFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXSCOLGFIS` reader - MMC Transmit Single Collision Good Frame Counter Interrupt Status This bit is set when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
pub type TXSCOLGFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXSCOLGFIS` writer - MMC Transmit Single Collision Good Frame Counter Interrupt Status This bit is set when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
pub type TXSCOLGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXMCOLGFIS` reader - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status This bit is set when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
pub type TXMCOLGFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXMCOLGFIS` writer - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status This bit is set when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
pub type TXMCOLGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXDEFFIS` reader - MMC Transmit Deferred Frame Counter Interrupt Status This bit is set when the txdeferred counter reaches half of the maximum value or the maximum value."]
pub type TXDEFFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXDEFFIS` writer - MMC Transmit Deferred Frame Counter Interrupt Status This bit is set when the txdeferred counter reaches half of the maximum value or the maximum value."]
pub type TXDEFFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXLATCOLFIS` reader - MMC Transmit Late Collision Frame Counter Interrupt Status This bit is set when the txlatecol counter reaches half of the maximum value or the maximum value."]
pub type TXLATCOLFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXLATCOLFIS` writer - MMC Transmit Late Collision Frame Counter Interrupt Status This bit is set when the txlatecol counter reaches half of the maximum value or the maximum value."]
pub type TXLATCOLFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXEXCOLFIS` reader - MMC Transmit Excessive Collision Frame Counter Interrupt Status This bit is set when the txexesscol counter reaches half of the maximum value or the maximum value."]
pub type TXEXCOLFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXEXCOLFIS` writer - MMC Transmit Excessive Collision Frame Counter Interrupt Status This bit is set when the txexesscol counter reaches half of the maximum value or the maximum value."]
pub type TXEXCOLFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXCARERFIS` reader - MMC Transmit Carrier Error Frame Counter Interrupt Status This bit is set when the txcarriererror counter reaches half of the maximum value or the maximum value."]
pub type TXCARERFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXCARERFIS` writer - MMC Transmit Carrier Error Frame Counter Interrupt Status This bit is set when the txcarriererror counter reaches half of the maximum value or the maximum value."]
pub type TXCARERFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXGOCTIS` reader - MMC Transmit Good Octet Counter Interrupt Status This bit is set when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
pub type TXGOCTIS_R = crate::BitReader<bool>;
#[doc = "Field `TXGOCTIS` writer - MMC Transmit Good Octet Counter Interrupt Status This bit is set when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
pub type TXGOCTIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXGFRMIS` reader - MMC Transmit Good Frame Counter Interrupt Status This bit is set when the txframecount_g counter reaches half of the maximum value or the maximum value."]
pub type TXGFRMIS_R = crate::BitReader<bool>;
#[doc = "Field `TXGFRMIS` writer - MMC Transmit Good Frame Counter Interrupt Status This bit is set when the txframecount_g counter reaches half of the maximum value or the maximum value."]
pub type TXGFRMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXEXDEFFIS` reader - MMC Transmit Excessive Deferral Frame Counter Interrupt Status This bit is set when the txexcessdef counter reaches half of the maximum value or the maximum value."]
pub type TXEXDEFFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXEXDEFFIS` writer - MMC Transmit Excessive Deferral Frame Counter Interrupt Status This bit is set when the txexcessdef counter reaches half of the maximum value or the maximum value."]
pub type TXEXDEFFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXPAUSFIS` reader - MMC Transmit Pause Frame Counter Interrupt Status This bit is set when the txpauseframeserror counter reaches half of the maximum value or the maximum value."]
pub type TXPAUSFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXPAUSFIS` writer - MMC Transmit Pause Frame Counter Interrupt Status This bit is set when the txpauseframeserror counter reaches half of the maximum value or the maximum value."]
pub type TXPAUSFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXVLANGFIS` reader - MMC Transmit VLAN Good Frame Counter Interrupt Status This bit is set when the txvlanframes_g counter reaches half of the maximum value or the maximum value."]
pub type TXVLANGFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXVLANGFIS` writer - MMC Transmit VLAN Good Frame Counter Interrupt Status This bit is set when the txvlanframes_g counter reaches half of the maximum value or the maximum value."]
pub type TXVLANGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
#[doc = "Field `TXOSIZEGFIS` reader - MMC Transmit Oversize Good Frame Counter Interrupt Status This bit is set when the txoversize_g counter reaches half of the maximum value or the maximum value."]
pub type TXOSIZEGFIS_R = crate::BitReader<bool>;
#[doc = "Field `TXOSIZEGFIS` writer - MMC Transmit Oversize Good Frame Counter Interrupt Status This bit is set when the txoversize_g counter reaches half of the maximum value or the maximum value."]
pub type TXOSIZEGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_TX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Status This bit is set when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txgboctis(&self) -> TXGBOCTIS_R {
        TXGBOCTIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Status This bit is set when the txframecount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txgbfrmis(&self) -> TXGBFRMIS_R {
        TXGBFRMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Status This bit is set when the txbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txbcgfis(&self) -> TXBCGFIS_R {
        TXBCGFIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Status This bit is set when the txmulticastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txmcgfis(&self) -> TXMCGFIS_R {
        TXMCGFIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn tx64octgbfis(&self) -> TX64OCTGBFIS_R {
        TX64OCTGBFIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx65to127octets_gb counter reaches half the maximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    pub fn tx65t127octgbfis(&self) -> TX65T127OCTGBFIS_R {
        TX65T127OCTGBFIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn tx128t255octgbfis(&self) -> TX128T255OCTGBFIS_R {
        TX128T255OCTGBFIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn tx256t511octgbfis(&self) -> TX256T511OCTGBFIS_R {
        TX256T511OCTGBFIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn tx512t1023octgbfis(&self) -> TX512T1023OCTGBFIS_R {
        TX512T1023OCTGBFIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn tx1024tmaxoctgbfis(&self) -> TX1024TMAXOCTGBFIS_R {
        TX1024TMAXOCTGBFIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Status This bit is set when the txunicastframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txucgbfis(&self) -> TXUCGBFIS_R {
        TXUCGBFIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Status The bit is set when the txmulticastframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txmcgbfis(&self) -> TXMCGBFIS_R {
        TXMCGBFIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status This bit is set when the txbroadcastframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txbcgbfis(&self) -> TXBCGBFIS_R {
        TXBCGBFIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Status This bit is set when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txuflowerfis(&self) -> TXUFLOWERFIS_R {
        TXUFLOWERFIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Status This bit is set when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txscolgfis(&self) -> TXSCOLGFIS_R {
        TXSCOLGFIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status This bit is set when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txmcolgfis(&self) -> TXMCOLGFIS_R {
        TXMCOLGFIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Status This bit is set when the txdeferred counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txdeffis(&self) -> TXDEFFIS_R {
        TXDEFFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Status This bit is set when the txlatecol counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txlatcolfis(&self) -> TXLATCOLFIS_R {
        TXLATCOLFIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Status This bit is set when the txexesscol counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txexcolfis(&self) -> TXEXCOLFIS_R {
        TXEXCOLFIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Status This bit is set when the txcarriererror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txcarerfis(&self) -> TXCARERFIS_R {
        TXCARERFIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Status This bit is set when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txgoctis(&self) -> TXGOCTIS_R {
        TXGOCTIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Status This bit is set when the txframecount_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txgfrmis(&self) -> TXGFRMIS_R {
        TXGFRMIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Status This bit is set when the txexcessdef counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txexdeffis(&self) -> TXEXDEFFIS_R {
        TXEXDEFFIS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Status This bit is set when the txpauseframeserror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txpausfis(&self) -> TXPAUSFIS_R {
        TXPAUSFIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Status This bit is set when the txvlanframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txvlangfis(&self) -> TXVLANGFIS_R {
        TXVLANGFIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Status This bit is set when the txoversize_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn txosizegfis(&self) -> TXOSIZEGFIS_R {
        TXOSIZEGFIS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Transmit Good Bad Octet Counter Interrupt Status This bit is set when the txoctetcount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txgboctis(&mut self) -> TXGBOCTIS_W<0> {
        TXGBOCTIS_W::new(self)
    }
    #[doc = "Bit 1 - MMC Transmit Good Bad Frame Counter Interrupt Status This bit is set when the txframecount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txgbfrmis(&mut self) -> TXGBFRMIS_W<1> {
        TXGBFRMIS_W::new(self)
    }
    #[doc = "Bit 2 - MMC Transmit Broadcast Good Frame Counter Interrupt Status This bit is set when the txbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txbcgfis(&mut self) -> TXBCGFIS_W<2> {
        TXBCGFIS_W::new(self)
    }
    #[doc = "Bit 3 - MMC Transmit Multicast Good Frame Counter Interrupt Status This bit is set when the txmulticastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txmcgfis(&mut self) -> TXMCGFIS_W<3> {
        TXMCGFIS_W::new(self)
    }
    #[doc = "Bit 4 - MMC Transmit 64 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx64octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn tx64octgbfis(&mut self) -> TX64OCTGBFIS_W<4> {
        TX64OCTGBFIS_W::new(self)
    }
    #[doc = "Bit 5 - MMC Transmit 65 to 127 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx65to127octets_gb counter reaches half the maximum value, and also when it reaches the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn tx65t127octgbfis(&mut self) -> TX65T127OCTGBFIS_W<5> {
        TX65T127OCTGBFIS_W::new(self)
    }
    #[doc = "Bit 6 - MMC Transmit 128 to 255 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn tx128t255octgbfis(&mut self) -> TX128T255OCTGBFIS_W<6> {
        TX128T255OCTGBFIS_W::new(self)
    }
    #[doc = "Bit 7 - MMC Transmit 256 to 511 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn tx256t511octgbfis(&mut self) -> TX256T511OCTGBFIS_W<7> {
        TX256T511OCTGBFIS_W::new(self)
    }
    #[doc = "Bit 8 - MMC Transmit 512 to 1023 Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn tx512t1023octgbfis(&mut self) -> TX512T1023OCTGBFIS_W<8> {
        TX512T1023OCTGBFIS_W::new(self)
    }
    #[doc = "Bit 9 - MMC Transmit 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status This bit is set when the tx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn tx1024tmaxoctgbfis(&mut self) -> TX1024TMAXOCTGBFIS_W<9> {
        TX1024TMAXOCTGBFIS_W::new(self)
    }
    #[doc = "Bit 10 - MMC Transmit Unicast Good Bad Frame Counter Interrupt Status This bit is set when the txunicastframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txucgbfis(&mut self) -> TXUCGBFIS_W<10> {
        TXUCGBFIS_W::new(self)
    }
    #[doc = "Bit 11 - MMC Transmit Multicast Good Bad Frame Counter Interrupt Status The bit is set when the txmulticastframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txmcgbfis(&mut self) -> TXMCGBFIS_W<11> {
        TXMCGBFIS_W::new(self)
    }
    #[doc = "Bit 12 - MMC Transmit Broadcast Good Bad Frame Counter Interrupt Status This bit is set when the txbroadcastframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txbcgbfis(&mut self) -> TXBCGBFIS_W<12> {
        TXBCGBFIS_W::new(self)
    }
    #[doc = "Bit 13 - MMC Transmit Underflow Error Frame Counter Interrupt Status This bit is set when the txunderflowerror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txuflowerfis(&mut self) -> TXUFLOWERFIS_W<13> {
        TXUFLOWERFIS_W::new(self)
    }
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Frame Counter Interrupt Status This bit is set when the txsinglecol_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txscolgfis(&mut self) -> TXSCOLGFIS_W<14> {
        TXSCOLGFIS_W::new(self)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Frame Counter Interrupt Status This bit is set when the txmulticol_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txmcolgfis(&mut self) -> TXMCOLGFIS_W<15> {
        TXMCOLGFIS_W::new(self)
    }
    #[doc = "Bit 16 - MMC Transmit Deferred Frame Counter Interrupt Status This bit is set when the txdeferred counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txdeffis(&mut self) -> TXDEFFIS_W<16> {
        TXDEFFIS_W::new(self)
    }
    #[doc = "Bit 17 - MMC Transmit Late Collision Frame Counter Interrupt Status This bit is set when the txlatecol counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txlatcolfis(&mut self) -> TXLATCOLFIS_W<17> {
        TXLATCOLFIS_W::new(self)
    }
    #[doc = "Bit 18 - MMC Transmit Excessive Collision Frame Counter Interrupt Status This bit is set when the txexesscol counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txexcolfis(&mut self) -> TXEXCOLFIS_W<18> {
        TXEXCOLFIS_W::new(self)
    }
    #[doc = "Bit 19 - MMC Transmit Carrier Error Frame Counter Interrupt Status This bit is set when the txcarriererror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txcarerfis(&mut self) -> TXCARERFIS_W<19> {
        TXCARERFIS_W::new(self)
    }
    #[doc = "Bit 20 - MMC Transmit Good Octet Counter Interrupt Status This bit is set when the txoctetcount_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txgoctis(&mut self) -> TXGOCTIS_W<20> {
        TXGOCTIS_W::new(self)
    }
    #[doc = "Bit 21 - MMC Transmit Good Frame Counter Interrupt Status This bit is set when the txframecount_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txgfrmis(&mut self) -> TXGFRMIS_W<21> {
        TXGFRMIS_W::new(self)
    }
    #[doc = "Bit 22 - MMC Transmit Excessive Deferral Frame Counter Interrupt Status This bit is set when the txexcessdef counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txexdeffis(&mut self) -> TXEXDEFFIS_W<22> {
        TXEXDEFFIS_W::new(self)
    }
    #[doc = "Bit 23 - MMC Transmit Pause Frame Counter Interrupt Status This bit is set when the txpauseframeserror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txpausfis(&mut self) -> TXPAUSFIS_W<23> {
        TXPAUSFIS_W::new(self)
    }
    #[doc = "Bit 24 - MMC Transmit VLAN Good Frame Counter Interrupt Status This bit is set when the txvlanframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txvlangfis(&mut self) -> TXVLANGFIS_W<24> {
        TXVLANGFIS_W::new(self)
    }
    #[doc = "Bit 25 - MMC Transmit Oversize Good Frame Counter Interrupt Status This bit is set when the txoversize_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    #[must_use]
    pub fn txosizegfis(&mut self) -> TXOSIZEGFIS_W<25> {
        TXOSIZEGFIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Transmit Interrupt maintains the interrupt generated from all of the transmit statistic counters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_intr_tx](index.html) module"]
pub struct MMC_INTR_TX_SPEC;
impl crate::RegisterSpec for MMC_INTR_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_intr_tx::R](R) reader structure"]
impl crate::Readable for MMC_INTR_TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_intr_tx::W](W) writer structure"]
impl crate::Writable for MMC_INTR_TX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMC_INTR_TX to value 0"]
impl crate::Resettable for MMC_INTR_TX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
