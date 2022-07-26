#[doc = "Register `MMC_INTR_RX` reader"]
pub struct R(crate::R<MMC_INTR_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_INTR_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_INTR_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_INTR_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_INTR_RX` writer"]
pub struct W(crate::W<MMC_INTR_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_INTR_RX_SPEC>;
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
impl From<crate::W<MMC_INTR_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_INTR_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXCTRLFIS` reader - MMC Receive Control Frame Counter Interrupt Status This bit is set when the rxctrlframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXCTRLFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXCTRLFIS` writer - MMC Receive Control Frame Counter Interrupt Status This bit is set when the rxctrlframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXCTRLFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXRCVERRFIS` reader - MMC Receive Error Frame Counter Interrupt Status This bit is set when the rxrcverror counter reaches half of the maximum value or the maximum value."]
pub type RXRCVERRFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXRCVERRFIS` writer - MMC Receive Error Frame Counter Interrupt Status This bit is set when the rxrcverror counter reaches half of the maximum value or the maximum value."]
pub type RXRCVERRFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXWDOGFIS` reader - MMC Receive Watchdog Error Frame Counter Interrupt Status This bit is set when the rxwatchdog error counter reaches half of the maximum value or the maximum value."]
pub type RXWDOGFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXWDOGFIS` writer - MMC Receive Watchdog Error Frame Counter Interrupt Status This bit is set when the rxwatchdog error counter reaches half of the maximum value or the maximum value."]
pub type RXWDOGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXVLANGBFIS` reader - MMC Receive VLAN Good Bad Frame Counter Interrupt Status This bit is set when the rxvlanframes_gb counter reaches half of the maximum value or the maximum value."]
pub type RXVLANGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXVLANGBFIS` writer - MMC Receive VLAN Good Bad Frame Counter Interrupt Status This bit is set when the rxvlanframes_gb counter reaches half of the maximum value or the maximum value."]
pub type RXVLANGBFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXFOVFIS` reader - MMC Receive FIFO Overflow Frame Counter Interrupt Status This bit is set when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
pub type RXFOVFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXFOVFIS` writer - MMC Receive FIFO Overflow Frame Counter Interrupt Status This bit is set when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
pub type RXFOVFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXPAUSFIS` reader - MMC Receive Pause Frame Counter Interrupt Status This bit is set when the rxpauseframes counter reaches half of the maximum value or the maximum value."]
pub type RXPAUSFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXPAUSFIS` writer - MMC Receive Pause Frame Counter Interrupt Status This bit is set when the rxpauseframes counter reaches half of the maximum value or the maximum value."]
pub type RXPAUSFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXORANGEFIS` reader - MMC Receive Out Of Range Error Frame Counter Interrupt Status. This bit is set when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
pub type RXORANGEFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXORANGEFIS` writer - MMC Receive Out Of Range Error Frame Counter Interrupt Status. This bit is set when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
pub type RXORANGEFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXLENERFIS` reader - MMC Receive Length Error Frame Counter Interrupt Status This bit is set when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
pub type RXLENERFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXLENERFIS` writer - MMC Receive Length Error Frame Counter Interrupt Status This bit is set when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
pub type RXLENERFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXUCGFIS` reader - MMC Receive Unicast Good Frame Counter Interrupt Status This bit is set when the rxunicastframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXUCGFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXUCGFIS` writer - MMC Receive Unicast Good Frame Counter Interrupt Status This bit is set when the rxunicastframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXUCGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RX1024TMAXOCTGBFIS` reader - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status. This bit is set when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX1024TMAXOCTGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `RX1024TMAXOCTGBFIS` writer - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status. This bit is set when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX1024TMAXOCTGBFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RX512T1023OCTGBFIS` reader - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX512T1023OCTGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `RX512T1023OCTGBFIS` writer - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX512T1023OCTGBFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RX256T511OCTGBFIS` reader - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX256T511OCTGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `RX256T511OCTGBFIS` writer - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX256T511OCTGBFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RX128T255OCTGBFIS` reader - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX128T255OCTGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `RX128T255OCTGBFIS` writer - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX128T255OCTGBFIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RX65T127OCTGBFIS` reader - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX65T127OCTGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `RX65T127OCTGBFIS` writer - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX65T127OCTGBFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RX64OCTGBFIS` reader - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX64OCTGBFIS_R = crate::BitReader<bool>;
#[doc = "Field `RX64OCTGBFIS` writer - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX64OCTGBFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXOSIZEGFIS` reader - MMC Receive Oversize Good Frame Counter Interrupt Status This bit is set when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
pub type RXOSIZEGFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXOSIZEGFIS` writer - MMC Receive Oversize Good Frame Counter Interrupt Status This bit is set when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
pub type RXOSIZEGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXUSIZEGFIS` reader - MMC Receive Undersize Good Frame Counter Interrupt Status This bit is set when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
pub type RXUSIZEGFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXUSIZEGFIS` writer - MMC Receive Undersize Good Frame Counter Interrupt Status This bit is set when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
pub type RXUSIZEGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXJABERFIS` reader - MMC Receive Jabber Error Frame Counter Interrupt Status This bit is set when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
pub type RXJABERFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXJABERFIS` writer - MMC Receive Jabber Error Frame Counter Interrupt Status This bit is set when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
pub type RXJABERFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXRUNTFIS` reader - MMC Receive Runt Frame Counter Interrupt Status This bit is set when the rxrunterror counter reaches half of the maximum value or the maximum value."]
pub type RXRUNTFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXRUNTFIS` writer - MMC Receive Runt Frame Counter Interrupt Status This bit is set when the rxrunterror counter reaches half of the maximum value or the maximum value."]
pub type RXRUNTFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXALGNERFIS` reader - MMC Receive Alignment Error Frame Counter Interrupt Status This bit is set when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
pub type RXALGNERFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXALGNERFIS` writer - MMC Receive Alignment Error Frame Counter Interrupt Status This bit is set when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
pub type RXALGNERFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXCRCERFIS` reader - MMC Receive CRC Error Frame Counter Interrupt Status This bit is set when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
pub type RXCRCERFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXCRCERFIS` writer - MMC Receive CRC Error Frame Counter Interrupt Status This bit is set when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
pub type RXCRCERFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXMCGFIS` reader - MMC Receive Multicast Good Frame Counter Interrupt Status This bit is set when the rxmulticastframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXMCGFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXMCGFIS` writer - MMC Receive Multicast Good Frame Counter Interrupt Status This bit is set when the rxmulticastframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXMCGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXBCGFIS` reader - MMC Receive Broadcast Good Frame Counter Interrupt Status This bit is set when the rxbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXBCGFIS_R = crate::BitReader<bool>;
#[doc = "Field `RXBCGFIS` writer - MMC Receive Broadcast Good Frame Counter Interrupt Status This bit is set when the rxbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXBCGFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXGOCTIS` reader - MMC Receive Good Octet Counter Interrupt Status This bit is set when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
pub type RXGOCTIS_R = crate::BitReader<bool>;
#[doc = "Field `RXGOCTIS` writer - MMC Receive Good Octet Counter Interrupt Status This bit is set when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
pub type RXGOCTIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXGBOCTIS` reader - MMC Receive Good Bad Octet Counter Interrupt Status This bit is set when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
pub type RXGBOCTIS_R = crate::BitReader<bool>;
#[doc = "Field `RXGBOCTIS` writer - MMC Receive Good Bad Octet Counter Interrupt Status This bit is set when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
pub type RXGBOCTIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
#[doc = "Field `RXGBFRMIS` reader - MMC Receive Good Bad Frame Counter Interrupt Status This bit is set when the rxframecount_gb counter reaches half of the maximum value or the maximum value."]
pub type RXGBFRMIS_R = crate::BitReader<bool>;
#[doc = "Field `RXGBFRMIS` writer - MMC Receive Good Bad Frame Counter Interrupt Status This bit is set when the rxframecount_gb counter reaches half of the maximum value or the maximum value."]
pub type RXGBFRMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_RX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Status This bit is set when the rxctrlframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxctrlfis(&self) -> RXCTRLFIS_R {
        RXCTRLFIS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Status This bit is set when the rxrcverror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxrcverrfis(&self) -> RXRCVERRFIS_R {
        RXRCVERRFIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Status This bit is set when the rxwatchdog error counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxwdogfis(&self) -> RXWDOGFIS_R {
        RXWDOGFIS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Status This bit is set when the rxvlanframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxvlangbfis(&self) -> RXVLANGBFIS_R {
        RXVLANGBFIS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Status This bit is set when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxfovfis(&self) -> RXFOVFIS_R {
        RXFOVFIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Status This bit is set when the rxpauseframes counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxpausfis(&self) -> RXPAUSFIS_R {
        RXPAUSFIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Status. This bit is set when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxorangefis(&self) -> RXORANGEFIS_R {
        RXORANGEFIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Status This bit is set when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxlenerfis(&self) -> RXLENERFIS_R {
        RXLENERFIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Status This bit is set when the rxunicastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxucgfis(&self) -> RXUCGFIS_R {
        RXUCGFIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status. This bit is set when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfis(&self) -> RX1024TMAXOCTGBFIS_R {
        RX1024TMAXOCTGBFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx512t1023octgbfis(&self) -> RX512T1023OCTGBFIS_R {
        RX512T1023OCTGBFIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx256t511octgbfis(&self) -> RX256T511OCTGBFIS_R {
        RX256T511OCTGBFIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx128t255octgbfis(&self) -> RX128T255OCTGBFIS_R {
        RX128T255OCTGBFIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx65t127octgbfis(&self) -> RX65T127OCTGBFIS_R {
        RX65T127OCTGBFIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx64octgbfis(&self) -> RX64OCTGBFIS_R {
        RX64OCTGBFIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Status This bit is set when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxosizegfis(&self) -> RXOSIZEGFIS_R {
        RXOSIZEGFIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Status This bit is set when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxusizegfis(&self) -> RXUSIZEGFIS_R {
        RXUSIZEGFIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Status This bit is set when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxjaberfis(&self) -> RXJABERFIS_R {
        RXJABERFIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Status This bit is set when the rxrunterror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxruntfis(&self) -> RXRUNTFIS_R {
        RXRUNTFIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Status This bit is set when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxalgnerfis(&self) -> RXALGNERFIS_R {
        RXALGNERFIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Status This bit is set when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxcrcerfis(&self) -> RXCRCERFIS_R {
        RXCRCERFIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Status This bit is set when the rxmulticastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxmcgfis(&self) -> RXMCGFIS_R {
        RXMCGFIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Status This bit is set when the rxbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxbcgfis(&self) -> RXBCGFIS_R {
        RXBCGFIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Status This bit is set when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxgoctis(&self) -> RXGOCTIS_R {
        RXGOCTIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Status This bit is set when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxgboctis(&self) -> RXGBOCTIS_R {
        RXGBOCTIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Status This bit is set when the rxframecount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxgbfrmis(&self) -> RXGBFRMIS_R {
        RXGBFRMIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Status This bit is set when the rxctrlframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxctrlfis(&mut self) -> RXCTRLFIS_W<25> {
        RXCTRLFIS_W::new(self)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Status This bit is set when the rxrcverror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxrcverrfis(&mut self) -> RXRCVERRFIS_W<24> {
        RXRCVERRFIS_W::new(self)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Status This bit is set when the rxwatchdog error counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxwdogfis(&mut self) -> RXWDOGFIS_W<23> {
        RXWDOGFIS_W::new(self)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Status This bit is set when the rxvlanframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxvlangbfis(&mut self) -> RXVLANGBFIS_W<22> {
        RXVLANGBFIS_W::new(self)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Status This bit is set when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxfovfis(&mut self) -> RXFOVFIS_W<21> {
        RXFOVFIS_W::new(self)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Status This bit is set when the rxpauseframes counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxpausfis(&mut self) -> RXPAUSFIS_W<20> {
        RXPAUSFIS_W::new(self)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Status. This bit is set when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxorangefis(&mut self) -> RXORANGEFIS_W<19> {
        RXORANGEFIS_W::new(self)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Status This bit is set when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxlenerfis(&mut self) -> RXLENERFIS_W<18> {
        RXLENERFIS_W::new(self)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Status This bit is set when the rxunicastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxucgfis(&mut self) -> RXUCGFIS_W<17> {
        RXUCGFIS_W::new(self)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Status. This bit is set when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfis(&mut self) -> RX1024TMAXOCTGBFIS_W<16> {
        RX1024TMAXOCTGBFIS_W::new(self)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx512t1023octgbfis(&mut self) -> RX512T1023OCTGBFIS_W<15> {
        RX512T1023OCTGBFIS_W::new(self)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx256t511octgbfis(&mut self) -> RX256T511OCTGBFIS_W<14> {
        RX256T511OCTGBFIS_W::new(self)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx128t255octgbfis(&mut self) -> RX128T255OCTGBFIS_W<13> {
        RX128T255OCTGBFIS_W::new(self)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx65t127octgbfis(&mut self) -> RX65T127OCTGBFIS_W<12> {
        RX65T127OCTGBFIS_W::new(self)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Status This bit is set when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx64octgbfis(&mut self) -> RX64OCTGBFIS_W<11> {
        RX64OCTGBFIS_W::new(self)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Status This bit is set when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxosizegfis(&mut self) -> RXOSIZEGFIS_W<10> {
        RXOSIZEGFIS_W::new(self)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Status This bit is set when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxusizegfis(&mut self) -> RXUSIZEGFIS_W<9> {
        RXUSIZEGFIS_W::new(self)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Status This bit is set when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxjaberfis(&mut self) -> RXJABERFIS_W<8> {
        RXJABERFIS_W::new(self)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Status This bit is set when the rxrunterror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxruntfis(&mut self) -> RXRUNTFIS_W<7> {
        RXRUNTFIS_W::new(self)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Status This bit is set when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxalgnerfis(&mut self) -> RXALGNERFIS_W<6> {
        RXALGNERFIS_W::new(self)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Status This bit is set when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxcrcerfis(&mut self) -> RXCRCERFIS_W<5> {
        RXCRCERFIS_W::new(self)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Status This bit is set when the rxmulticastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxmcgfis(&mut self) -> RXMCGFIS_W<4> {
        RXMCGFIS_W::new(self)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Status This bit is set when the rxbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxbcgfis(&mut self) -> RXBCGFIS_W<3> {
        RXBCGFIS_W::new(self)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Status This bit is set when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxgoctis(&mut self) -> RXGOCTIS_W<2> {
        RXGOCTIS_W::new(self)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Status This bit is set when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxgboctis(&mut self) -> RXGBOCTIS_W<1> {
        RXGBOCTIS_W::new(self)
    }
    #[doc = "Bit 0 - MMC Receive Good Bad Frame Counter Interrupt Status This bit is set when the rxframecount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxgbfrmis(&mut self) -> RXGBFRMIS_W<0> {
        RXGBFRMIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Receive Interrupt maintains the interrupt generated from all of the receive statistic counters.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_intr_rx](index.html) module"]
pub struct MMC_INTR_RX_SPEC;
impl crate::RegisterSpec for MMC_INTR_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_intr_rx::R](R) reader structure"]
impl crate::Readable for MMC_INTR_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_intr_rx::W](W) writer structure"]
impl crate::Writable for MMC_INTR_RX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMC_INTR_RX to value 0"]
impl crate::Resettable for MMC_INTR_RX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
