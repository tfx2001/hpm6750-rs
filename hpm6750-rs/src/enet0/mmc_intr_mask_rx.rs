#[doc = "Register `MMC_INTR_MASK_RX` reader"]
pub struct R(crate::R<MMC_INTR_MASK_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_INTR_MASK_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_INTR_MASK_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_INTR_MASK_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_INTR_MASK_RX` writer"]
pub struct W(crate::W<MMC_INTR_MASK_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_INTR_MASK_RX_SPEC>;
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
impl From<crate::W<MMC_INTR_MASK_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_INTR_MASK_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXCTRLFIM` reader - MMC Receive Control Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxctrlframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXCTRLFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXCTRLFIM` writer - MMC Receive Control Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxctrlframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXCTRLFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXRCVERRFIM` reader - MMC Receive Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxrcverror counter reaches half of the maximum value or the maximum value."]
pub type RXRCVERRFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXRCVERRFIM` writer - MMC Receive Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxrcverror counter reaches half of the maximum value or the maximum value."]
pub type RXRCVERRFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXWDOGFIM` reader - MMC Receive Watchdog Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxwatchdog counter reaches half of the maximum value or the maximum value."]
pub type RXWDOGFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXWDOGFIM` writer - MMC Receive Watchdog Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxwatchdog counter reaches half of the maximum value or the maximum value."]
pub type RXWDOGFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXVLANGBFIM` reader - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxvlanframes_gb counter reaches half of the maximum value or the maximum value."]
pub type RXVLANGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXVLANGBFIM` writer - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxvlanframes_gb counter reaches half of the maximum value or the maximum value."]
pub type RXVLANGBFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXFOVFIM` reader - MMC Receive FIFO Overflow Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
pub type RXFOVFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXFOVFIM` writer - MMC Receive FIFO Overflow Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
pub type RXFOVFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXPAUSFIM` reader - MMC Receive Pause Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxpauseframes counter reaches half of the maximum value or the maximum value."]
pub type RXPAUSFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXPAUSFIM` writer - MMC Receive Pause Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxpauseframes counter reaches half of the maximum value or the maximum value."]
pub type RXPAUSFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXORANGEFIM` reader - MMC Receive Out Of Range Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
pub type RXORANGEFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXORANGEFIM` writer - MMC Receive Out Of Range Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
pub type RXORANGEFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXLENERFIM` reader - MMC Receive Length Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
pub type RXLENERFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXLENERFIM` writer - MMC Receive Length Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
pub type RXLENERFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXUCGFIM` reader - MMC Receive Unicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxunicastframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXUCGFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXUCGFIM` writer - MMC Receive Unicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxunicastframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXUCGFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RX1024TMAXOCTGBFIM` reader - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask. Setting this bit masks the interrupt when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX1024TMAXOCTGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `RX1024TMAXOCTGBFIM` writer - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask. Setting this bit masks the interrupt when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX1024TMAXOCTGBFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RX512T1023OCTGBFIM` reader - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX512T1023OCTGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `RX512T1023OCTGBFIM` writer - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX512T1023OCTGBFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RX256T511OCTGBFIM` reader - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX256T511OCTGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `RX256T511OCTGBFIM` writer - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX256T511OCTGBFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RX128T255OCTGBFIM` reader - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX128T255OCTGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `RX128T255OCTGBFIM` writer - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX128T255OCTGBFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RX65T127OCTGBFIM` reader - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX65T127OCTGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `RX65T127OCTGBFIM` writer - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX65T127OCTGBFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RX64OCTGBFIM` reader - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX64OCTGBFIM_R = crate::BitReader<bool>;
#[doc = "Field `RX64OCTGBFIM` writer - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
pub type RX64OCTGBFIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXOSIZEGFIM` reader - MMC Receive Oversize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
pub type RXOSIZEGFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXOSIZEGFIM` writer - MMC Receive Oversize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
pub type RXOSIZEGFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXUSIZEGFIM` reader - MMC Receive Undersize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
pub type RXUSIZEGFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXUSIZEGFIM` writer - MMC Receive Undersize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
pub type RXUSIZEGFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXJABERFIM` reader - MMC Receive Jabber Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
pub type RXJABERFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXJABERFIM` writer - MMC Receive Jabber Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
pub type RXJABERFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXRUNTFIM` reader - MMC Receive Runt Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxrunterror counter reaches half of the maximum value or the maximum value."]
pub type RXRUNTFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXRUNTFIM` writer - MMC Receive Runt Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxrunterror counter reaches half of the maximum value or the maximum value."]
pub type RXRUNTFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXALGNERFIM` reader - MMC Receive Alignment Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
pub type RXALGNERFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXALGNERFIM` writer - MMC Receive Alignment Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
pub type RXALGNERFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXCRCERFIM` reader - MMC Receive CRC Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
pub type RXCRCERFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXCRCERFIM` writer - MMC Receive CRC Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
pub type RXCRCERFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXMCGFIM` reader - MMC Receive Multicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxmulticastframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXMCGFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXMCGFIM` writer - MMC Receive Multicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxmulticastframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXMCGFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXBCGFIM` reader - MMC Receive Broadcast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXBCGFIM_R = crate::BitReader<bool>;
#[doc = "Field `RXBCGFIM` writer - MMC Receive Broadcast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
pub type RXBCGFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXGOCTIM` reader - MMC Receive Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
pub type RXGOCTIM_R = crate::BitReader<bool>;
#[doc = "Field `RXGOCTIM` writer - MMC Receive Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
pub type RXGOCTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
#[doc = "Field `RXGBOCTIM` reader - MMC Receive Good Bad Octet Counter Interrupt Mask. Setting this bit masks the interrupt when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
pub type RXGBOCTIM_R = crate::BitReader<bool>;
#[doc = "Field `RXGBOCTIM` writer - MMC Receive Good Bad Octet Counter Interrupt Mask. Setting this bit masks the interrupt when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
pub type RXGBOCTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMC_INTR_MASK_RX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxctrlframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxctrlfim(&self) -> RXCTRLFIM_R {
        RXCTRLFIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxrcverror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxrcverrfim(&self) -> RXRCVERRFIM_R {
        RXRCVERRFIM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxwatchdog counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxwdogfim(&self) -> RXWDOGFIM_R {
        RXWDOGFIM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxvlanframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxvlangbfim(&self) -> RXVLANGBFIM_R {
        RXVLANGBFIM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxfovfim(&self) -> RXFOVFIM_R {
        RXFOVFIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxpauseframes counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxpausfim(&self) -> RXPAUSFIM_R {
        RXPAUSFIM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxorangefim(&self) -> RXORANGEFIM_R {
        RXORANGEFIM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxlenerfim(&self) -> RXLENERFIM_R {
        RXLENERFIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxunicastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxucgfim(&self) -> RXUCGFIM_R {
        RXUCGFIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask. Setting this bit masks the interrupt when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfim(&self) -> RX1024TMAXOCTGBFIM_R {
        RX1024TMAXOCTGBFIM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx512t1023octgbfim(&self) -> RX512T1023OCTGBFIM_R {
        RX512T1023OCTGBFIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx256t511octgbfim(&self) -> RX256T511OCTGBFIM_R {
        RX256T511OCTGBFIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx128t255octgbfim(&self) -> RX128T255OCTGBFIM_R {
        RX128T255OCTGBFIM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx65t127octgbfim(&self) -> RX65T127OCTGBFIM_R {
        RX65T127OCTGBFIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx64octgbfim(&self) -> RX64OCTGBFIM_R {
        RX64OCTGBFIM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxosizegfim(&self) -> RXOSIZEGFIM_R {
        RXOSIZEGFIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxusizegfim(&self) -> RXUSIZEGFIM_R {
        RXUSIZEGFIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxjaberfim(&self) -> RXJABERFIM_R {
        RXJABERFIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxrunterror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxruntfim(&self) -> RXRUNTFIM_R {
        RXRUNTFIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxalgnerfim(&self) -> RXALGNERFIM_R {
        RXALGNERFIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxcrcerfim(&self) -> RXCRCERFIM_R {
        RXCRCERFIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxmulticastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxmcgfim(&self) -> RXMCGFIM_R {
        RXMCGFIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxbcgfim(&self) -> RXBCGFIM_R {
        RXBCGFIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxgoctim(&self) -> RXGOCTIM_R {
        RXGOCTIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Mask. Setting this bit masks the interrupt when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxgboctim(&self) -> RXGBOCTIM_R {
        RXGBOCTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - MMC Receive Control Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxctrlframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxctrlfim(&mut self) -> RXCTRLFIM_W<25> {
        RXCTRLFIM_W::new(self)
    }
    #[doc = "Bit 24 - MMC Receive Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxrcverror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxrcverrfim(&mut self) -> RXRCVERRFIM_W<24> {
        RXRCVERRFIM_W::new(self)
    }
    #[doc = "Bit 23 - MMC Receive Watchdog Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxwatchdog counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxwdogfim(&mut self) -> RXWDOGFIM_W<23> {
        RXWDOGFIM_W::new(self)
    }
    #[doc = "Bit 22 - MMC Receive VLAN Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxvlanframes_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxvlangbfim(&mut self) -> RXVLANGBFIM_W<22> {
        RXVLANGBFIM_W::new(self)
    }
    #[doc = "Bit 21 - MMC Receive FIFO Overflow Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxfifooverflow counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxfovfim(&mut self) -> RXFOVFIM_W<21> {
        RXFOVFIM_W::new(self)
    }
    #[doc = "Bit 20 - MMC Receive Pause Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxpauseframes counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxpausfim(&mut self) -> RXPAUSFIM_W<20> {
        RXPAUSFIM_W::new(self)
    }
    #[doc = "Bit 19 - MMC Receive Out Of Range Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxoutofrangetype counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxorangefim(&mut self) -> RXORANGEFIM_W<19> {
        RXORANGEFIM_W::new(self)
    }
    #[doc = "Bit 18 - MMC Receive Length Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxlengtherror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxlenerfim(&mut self) -> RXLENERFIM_W<18> {
        RXLENERFIM_W::new(self)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxunicastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxucgfim(&mut self) -> RXUCGFIM_W<17> {
        RXUCGFIM_W::new(self)
    }
    #[doc = "Bit 16 - MMC Receive 1024 to Maximum Octet Good Bad Frame Counter Interrupt Mask. Setting this bit masks the interrupt when the rx1024tomaxoctets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx1024tmaxoctgbfim(&mut self) -> RX1024TMAXOCTGBFIM_W<16> {
        RX1024TMAXOCTGBFIM_W::new(self)
    }
    #[doc = "Bit 15 - MMC Receive 512 to 1023 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx512to1023octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx512t1023octgbfim(&mut self) -> RX512T1023OCTGBFIM_W<15> {
        RX512T1023OCTGBFIM_W::new(self)
    }
    #[doc = "Bit 14 - MMC Receive 256 to 511 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx256to511octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx256t511octgbfim(&mut self) -> RX256T511OCTGBFIM_W<14> {
        RX256T511OCTGBFIM_W::new(self)
    }
    #[doc = "Bit 13 - MMC Receive 128 to 255 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx128to255octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx128t255octgbfim(&mut self) -> RX128T255OCTGBFIM_W<13> {
        RX128T255OCTGBFIM_W::new(self)
    }
    #[doc = "Bit 12 - MMC Receive 65 to 127 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx65to127octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx65t127octgbfim(&mut self) -> RX65T127OCTGBFIM_W<12> {
        RX65T127OCTGBFIM_W::new(self)
    }
    #[doc = "Bit 11 - MMC Receive 64 Octet Good Bad Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rx64octets_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rx64octgbfim(&mut self) -> RX64OCTGBFIM_W<11> {
        RX64OCTGBFIM_W::new(self)
    }
    #[doc = "Bit 10 - MMC Receive Oversize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxoversize_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxosizegfim(&mut self) -> RXOSIZEGFIM_W<10> {
        RXOSIZEGFIM_W::new(self)
    }
    #[doc = "Bit 9 - MMC Receive Undersize Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxundersize_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxusizegfim(&mut self) -> RXUSIZEGFIM_W<9> {
        RXUSIZEGFIM_W::new(self)
    }
    #[doc = "Bit 8 - MMC Receive Jabber Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxjabbererror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxjaberfim(&mut self) -> RXJABERFIM_W<8> {
        RXJABERFIM_W::new(self)
    }
    #[doc = "Bit 7 - MMC Receive Runt Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxrunterror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxruntfim(&mut self) -> RXRUNTFIM_W<7> {
        RXRUNTFIM_W::new(self)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxalignmenterror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxalgnerfim(&mut self) -> RXALGNERFIM_W<6> {
        RXALGNERFIM_W::new(self)
    }
    #[doc = "Bit 5 - MMC Receive CRC Error Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxcrcerror counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxcrcerfim(&mut self) -> RXCRCERFIM_W<5> {
        RXCRCERFIM_W::new(self)
    }
    #[doc = "Bit 4 - MMC Receive Multicast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxmulticastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxmcgfim(&mut self) -> RXMCGFIM_W<4> {
        RXMCGFIM_W::new(self)
    }
    #[doc = "Bit 3 - MMC Receive Broadcast Good Frame Counter Interrupt Mask Setting this bit masks the interrupt when the rxbroadcastframes_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxbcgfim(&mut self) -> RXBCGFIM_W<3> {
        RXBCGFIM_W::new(self)
    }
    #[doc = "Bit 2 - MMC Receive Good Octet Counter Interrupt Mask Setting this bit masks the interrupt when the rxoctetcount_g counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxgoctim(&mut self) -> RXGOCTIM_W<2> {
        RXGOCTIM_W::new(self)
    }
    #[doc = "Bit 1 - MMC Receive Good Bad Octet Counter Interrupt Mask. Setting this bit masks the interrupt when the rxoctetcount_gb counter reaches half of the maximum value or the maximum value."]
    #[inline(always)]
    pub fn rxgboctim(&mut self) -> RXGBOCTIM_W<1> {
        RXGBOCTIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Receive Interrupt mask maintains the mask for the interrupt generated from all of the receive statistic counters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_intr_mask_rx](index.html) module"]
pub struct MMC_INTR_MASK_RX_SPEC;
impl crate::RegisterSpec for MMC_INTR_MASK_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_intr_mask_rx::R](R) reader structure"]
impl crate::Readable for MMC_INTR_MASK_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_intr_mask_rx::W](W) writer structure"]
impl crate::Writable for MMC_INTR_MASK_RX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMC_INTR_MASK_RX to value 0"]
impl crate::Resettable for MMC_INTR_MASK_RX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
