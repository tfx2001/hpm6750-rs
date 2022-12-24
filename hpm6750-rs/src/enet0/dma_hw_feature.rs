#[doc = "Register `DMA_HW_FEATURE` reader"]
pub struct R(crate::R<DMA_HW_FEATURE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_HW_FEATURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_HW_FEATURE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_HW_FEATURE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_HW_FEATURE` writer"]
pub struct W(crate::W<DMA_HW_FEATURE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_HW_FEATURE_SPEC>;
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
impl From<crate::W<DMA_HW_FEATURE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_HW_FEATURE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIISEL` reader - 10 or 100 Mbps support"]
pub type MIISEL_R = crate::BitReader<bool>;
#[doc = "Field `MIISEL` writer - 10 or 100 Mbps support"]
pub type MIISEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `GMIISEL` reader - 1000 Mbps support"]
pub type GMIISEL_R = crate::BitReader<bool>;
#[doc = "Field `GMIISEL` writer - 1000 Mbps support"]
pub type GMIISEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `HDSEL` reader - Half-duplex support"]
pub type HDSEL_R = crate::BitReader<bool>;
#[doc = "Field `HDSEL` writer - Half-duplex support"]
pub type HDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `EXTHASHEN` reader - Expanded DA Hash filter"]
pub type EXTHASHEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTHASHEN` writer - Expanded DA Hash filter"]
pub type EXTHASHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `HASHSEL` reader - HASH filter"]
pub type HASHSEL_R = crate::BitReader<bool>;
#[doc = "Field `HASHSEL` writer - HASH filter"]
pub type HASHSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `ADDMACADRSEL` reader - Multiple MAC Address registers"]
pub type ADDMACADRSEL_R = crate::BitReader<bool>;
#[doc = "Field `ADDMACADRSEL` writer - Multiple MAC Address registers"]
pub type ADDMACADRSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `PCSSEL` reader - PCS registers (TBI, SGMII, or RTBI PHY interface)"]
pub type PCSSEL_R = crate::BitReader<bool>;
#[doc = "Field `PCSSEL` writer - PCS registers (TBI, SGMII, or RTBI PHY interface)"]
pub type PCSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `L3L4FLTREN` reader - Layer 3 and Layer 4 feature"]
pub type L3L4FLTREN_R = crate::BitReader<bool>;
#[doc = "Field `L3L4FLTREN` writer - Layer 3 and Layer 4 feature"]
pub type L3L4FLTREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `SMASEL` reader - SMA (MDIO) Interface"]
pub type SMASEL_R = crate::BitReader<bool>;
#[doc = "Field `SMASEL` writer - SMA (MDIO) Interface"]
pub type SMASEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `RWKSEL` reader - PMT remote wake-up frame"]
pub type RWKSEL_R = crate::BitReader<bool>;
#[doc = "Field `RWKSEL` writer - PMT remote wake-up frame"]
pub type RWKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `MGKSEL` reader - PMT magic packet"]
pub type MGKSEL_R = crate::BitReader<bool>;
#[doc = "Field `MGKSEL` writer - PMT magic packet"]
pub type MGKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `MMCSEL` reader - RMON module"]
pub type MMCSEL_R = crate::BitReader<bool>;
#[doc = "Field `MMCSEL` writer - RMON module"]
pub type MMCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `TSVER1SEL` reader - Only IEEE 1588-2002 timestamp"]
pub type TSVER1SEL_R = crate::BitReader<bool>;
#[doc = "Field `TSVER1SEL` writer - Only IEEE 1588-2002 timestamp"]
pub type TSVER1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `TSVER2SEL` reader - IEEE 1588-2008 Advanced timestamp"]
pub type TSVER2SEL_R = crate::BitReader<bool>;
#[doc = "Field `TSVER2SEL` writer - IEEE 1588-2008 Advanced timestamp"]
pub type TSVER2SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `EEESEL` reader - Energy Efficient Ethernet"]
pub type EEESEL_R = crate::BitReader<bool>;
#[doc = "Field `EEESEL` writer - Energy Efficient Ethernet"]
pub type EEESEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `AVSEL` reader - AV feature"]
pub type AVSEL_R = crate::BitReader<bool>;
#[doc = "Field `AVSEL` writer - AV feature"]
pub type AVSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `TXCOESEL` reader - Checksum Offload in Tx"]
pub type TXCOESEL_R = crate::BitReader<bool>;
#[doc = "Field `TXCOESEL` writer - Checksum Offload in Tx"]
pub type TXCOESEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `RXTYP1COE` reader - IP Checksum Offload (Type 1) in Rx Note: If IPCHKSUM_EN = Enabled and IPC_FULL_OFFLOAD = Enabled, then RXTYP1COE = 0 and RXTYP2COE = 1."]
pub type RXTYP1COE_R = crate::BitReader<bool>;
#[doc = "Field `RXTYP1COE` writer - IP Checksum Offload (Type 1) in Rx Note: If IPCHKSUM_EN = Enabled and IPC_FULL_OFFLOAD = Enabled, then RXTYP1COE = 0 and RXTYP2COE = 1."]
pub type RXTYP1COE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `RXTYP2COE` reader - IP Checksum Offload (Type 2) in Rx"]
pub type RXTYP2COE_R = crate::BitReader<bool>;
#[doc = "Field `RXTYP2COE` writer - IP Checksum Offload (Type 2) in Rx"]
pub type RXTYP2COE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `RXFIFOSIZE` reader - Rx FIFO > 2,048 Bytes"]
pub type RXFIFOSIZE_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOSIZE` writer - Rx FIFO > 2,048 Bytes"]
pub type RXFIFOSIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `RXCHCNT` reader - Number of additional Rx Channels"]
pub type RXCHCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXCHCNT` writer - Number of additional Rx Channels"]
pub type RXCHCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_HW_FEATURE_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXCHCNT` reader - Number of additional Tx Channels"]
pub type TXCHCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCHCNT` writer - Number of additional Tx Channels"]
pub type TXCHCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_HW_FEATURE_SPEC, u8, u8, 2, O>;
#[doc = "Field `ENHDESSEL` reader - Alternate (Enhanced Descriptor)"]
pub type ENHDESSEL_R = crate::BitReader<bool>;
#[doc = "Field `ENHDESSEL` writer - Alternate (Enhanced Descriptor)"]
pub type ENHDESSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `INTTSEN` reader - Timestamping with Internal System Time"]
pub type INTTSEN_R = crate::BitReader<bool>;
#[doc = "Field `INTTSEN` writer - Timestamping with Internal System Time"]
pub type INTTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `FLEXIPPSEN` reader - Flexible Pulse-Per-Second Output"]
pub type FLEXIPPSEN_R = crate::BitReader<bool>;
#[doc = "Field `FLEXIPPSEN` writer - Flexible Pulse-Per-Second Output"]
pub type FLEXIPPSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `SAVLANINS` reader - Source Address or VLAN Insertion"]
pub type SAVLANINS_R = crate::BitReader<bool>;
#[doc = "Field `SAVLANINS` writer - Source Address or VLAN Insertion"]
pub type SAVLANINS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_HW_FEATURE_SPEC, bool, O>;
#[doc = "Field `ACTPHYIF` reader - Active or selected PHY interface When you have multiple PHY interfaces in your configuration, this field indicates the sampled value of phy_intf_sel_i during reset de-assertion. - 000: GMII or MII - 001: RGMII - 010: SGMII - 011: TBI - 100: RMII - 101: RTBI - 110: SMII - 111: RevMII - All Others: Reserved"]
pub type ACTPHYIF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACTPHYIF` writer - Active or selected PHY interface When you have multiple PHY interfaces in your configuration, this field indicates the sampled value of phy_intf_sel_i during reset de-assertion. - 000: GMII or MII - 001: RGMII - 010: SGMII - 011: TBI - 100: RMII - 101: RTBI - 110: SMII - 111: RevMII - All Others: Reserved"]
pub type ACTPHYIF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_HW_FEATURE_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - 10 or 100 Mbps support"]
    #[inline(always)]
    pub fn miisel(&self) -> MIISEL_R {
        MIISEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1000 Mbps support"]
    #[inline(always)]
    pub fn gmiisel(&self) -> GMIISEL_R {
        GMIISEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half-duplex support"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Expanded DA Hash filter"]
    #[inline(always)]
    pub fn exthashen(&self) -> EXTHASHEN_R {
        EXTHASHEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HASH filter"]
    #[inline(always)]
    pub fn hashsel(&self) -> HASHSEL_R {
        HASHSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multiple MAC Address registers"]
    #[inline(always)]
    pub fn addmacadrsel(&self) -> ADDMACADRSEL_R {
        ADDMACADRSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCS registers (TBI, SGMII, or RTBI PHY interface)"]
    #[inline(always)]
    pub fn pcssel(&self) -> PCSSEL_R {
        PCSSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Layer 3 and Layer 4 feature"]
    #[inline(always)]
    pub fn l3l4fltren(&self) -> L3L4FLTREN_R {
        L3L4FLTREN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SMA (MDIO) Interface"]
    #[inline(always)]
    pub fn smasel(&self) -> SMASEL_R {
        SMASEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PMT remote wake-up frame"]
    #[inline(always)]
    pub fn rwksel(&self) -> RWKSEL_R {
        RWKSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PMT magic packet"]
    #[inline(always)]
    pub fn mgksel(&self) -> MGKSEL_R {
        MGKSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RMON module"]
    #[inline(always)]
    pub fn mmcsel(&self) -> MMCSEL_R {
        MMCSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Only IEEE 1588-2002 timestamp"]
    #[inline(always)]
    pub fn tsver1sel(&self) -> TSVER1SEL_R {
        TSVER1SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IEEE 1588-2008 Advanced timestamp"]
    #[inline(always)]
    pub fn tsver2sel(&self) -> TSVER2SEL_R {
        TSVER2SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Energy Efficient Ethernet"]
    #[inline(always)]
    pub fn eeesel(&self) -> EEESEL_R {
        EEESEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AV feature"]
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Checksum Offload in Tx"]
    #[inline(always)]
    pub fn txcoesel(&self) -> TXCOESEL_R {
        TXCOESEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - IP Checksum Offload (Type 1) in Rx Note: If IPCHKSUM_EN = Enabled and IPC_FULL_OFFLOAD = Enabled, then RXTYP1COE = 0 and RXTYP2COE = 1."]
    #[inline(always)]
    pub fn rxtyp1coe(&self) -> RXTYP1COE_R {
        RXTYP1COE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IP Checksum Offload (Type 2) in Rx"]
    #[inline(always)]
    pub fn rxtyp2coe(&self) -> RXTYP2COE_R {
        RXTYP2COE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx FIFO > 2,048 Bytes"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Number of additional Rx Channels"]
    #[inline(always)]
    pub fn rxchcnt(&self) -> RXCHCNT_R {
        RXCHCNT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Number of additional Tx Channels"]
    #[inline(always)]
    pub fn txchcnt(&self) -> TXCHCNT_R {
        TXCHCNT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Alternate (Enhanced Descriptor)"]
    #[inline(always)]
    pub fn enhdessel(&self) -> ENHDESSEL_R {
        ENHDESSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Timestamping with Internal System Time"]
    #[inline(always)]
    pub fn inttsen(&self) -> INTTSEN_R {
        INTTSEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Flexible Pulse-Per-Second Output"]
    #[inline(always)]
    pub fn flexippsen(&self) -> FLEXIPPSEN_R {
        FLEXIPPSEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Source Address or VLAN Insertion"]
    #[inline(always)]
    pub fn savlanins(&self) -> SAVLANINS_R {
        SAVLANINS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Active or selected PHY interface When you have multiple PHY interfaces in your configuration, this field indicates the sampled value of phy_intf_sel_i during reset de-assertion. - 000: GMII or MII - 001: RGMII - 010: SGMII - 011: TBI - 100: RMII - 101: RTBI - 110: SMII - 111: RevMII - All Others: Reserved"]
    #[inline(always)]
    pub fn actphyif(&self) -> ACTPHYIF_R {
        ACTPHYIF_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 10 or 100 Mbps support"]
    #[inline(always)]
    #[must_use]
    pub fn miisel(&mut self) -> MIISEL_W<0> {
        MIISEL_W::new(self)
    }
    #[doc = "Bit 1 - 1000 Mbps support"]
    #[inline(always)]
    #[must_use]
    pub fn gmiisel(&mut self) -> GMIISEL_W<1> {
        GMIISEL_W::new(self)
    }
    #[doc = "Bit 2 - Half-duplex support"]
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<2> {
        HDSEL_W::new(self)
    }
    #[doc = "Bit 3 - Expanded DA Hash filter"]
    #[inline(always)]
    #[must_use]
    pub fn exthashen(&mut self) -> EXTHASHEN_W<3> {
        EXTHASHEN_W::new(self)
    }
    #[doc = "Bit 4 - HASH filter"]
    #[inline(always)]
    #[must_use]
    pub fn hashsel(&mut self) -> HASHSEL_W<4> {
        HASHSEL_W::new(self)
    }
    #[doc = "Bit 5 - Multiple MAC Address registers"]
    #[inline(always)]
    #[must_use]
    pub fn addmacadrsel(&mut self) -> ADDMACADRSEL_W<5> {
        ADDMACADRSEL_W::new(self)
    }
    #[doc = "Bit 6 - PCS registers (TBI, SGMII, or RTBI PHY interface)"]
    #[inline(always)]
    #[must_use]
    pub fn pcssel(&mut self) -> PCSSEL_W<6> {
        PCSSEL_W::new(self)
    }
    #[doc = "Bit 7 - Layer 3 and Layer 4 feature"]
    #[inline(always)]
    #[must_use]
    pub fn l3l4fltren(&mut self) -> L3L4FLTREN_W<7> {
        L3L4FLTREN_W::new(self)
    }
    #[doc = "Bit 8 - SMA (MDIO) Interface"]
    #[inline(always)]
    #[must_use]
    pub fn smasel(&mut self) -> SMASEL_W<8> {
        SMASEL_W::new(self)
    }
    #[doc = "Bit 9 - PMT remote wake-up frame"]
    #[inline(always)]
    #[must_use]
    pub fn rwksel(&mut self) -> RWKSEL_W<9> {
        RWKSEL_W::new(self)
    }
    #[doc = "Bit 10 - PMT magic packet"]
    #[inline(always)]
    #[must_use]
    pub fn mgksel(&mut self) -> MGKSEL_W<10> {
        MGKSEL_W::new(self)
    }
    #[doc = "Bit 11 - RMON module"]
    #[inline(always)]
    #[must_use]
    pub fn mmcsel(&mut self) -> MMCSEL_W<11> {
        MMCSEL_W::new(self)
    }
    #[doc = "Bit 12 - Only IEEE 1588-2002 timestamp"]
    #[inline(always)]
    #[must_use]
    pub fn tsver1sel(&mut self) -> TSVER1SEL_W<12> {
        TSVER1SEL_W::new(self)
    }
    #[doc = "Bit 13 - IEEE 1588-2008 Advanced timestamp"]
    #[inline(always)]
    #[must_use]
    pub fn tsver2sel(&mut self) -> TSVER2SEL_W<13> {
        TSVER2SEL_W::new(self)
    }
    #[doc = "Bit 14 - Energy Efficient Ethernet"]
    #[inline(always)]
    #[must_use]
    pub fn eeesel(&mut self) -> EEESEL_W<14> {
        EEESEL_W::new(self)
    }
    #[doc = "Bit 15 - AV feature"]
    #[inline(always)]
    #[must_use]
    pub fn avsel(&mut self) -> AVSEL_W<15> {
        AVSEL_W::new(self)
    }
    #[doc = "Bit 16 - Checksum Offload in Tx"]
    #[inline(always)]
    #[must_use]
    pub fn txcoesel(&mut self) -> TXCOESEL_W<16> {
        TXCOESEL_W::new(self)
    }
    #[doc = "Bit 17 - IP Checksum Offload (Type 1) in Rx Note: If IPCHKSUM_EN = Enabled and IPC_FULL_OFFLOAD = Enabled, then RXTYP1COE = 0 and RXTYP2COE = 1."]
    #[inline(always)]
    #[must_use]
    pub fn rxtyp1coe(&mut self) -> RXTYP1COE_W<17> {
        RXTYP1COE_W::new(self)
    }
    #[doc = "Bit 18 - IP Checksum Offload (Type 2) in Rx"]
    #[inline(always)]
    #[must_use]
    pub fn rxtyp2coe(&mut self) -> RXTYP2COE_W<18> {
        RXTYP2COE_W::new(self)
    }
    #[doc = "Bit 19 - Rx FIFO > 2,048 Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifosize(&mut self) -> RXFIFOSIZE_W<19> {
        RXFIFOSIZE_W::new(self)
    }
    #[doc = "Bits 20:21 - Number of additional Rx Channels"]
    #[inline(always)]
    #[must_use]
    pub fn rxchcnt(&mut self) -> RXCHCNT_W<20> {
        RXCHCNT_W::new(self)
    }
    #[doc = "Bits 22:23 - Number of additional Tx Channels"]
    #[inline(always)]
    #[must_use]
    pub fn txchcnt(&mut self) -> TXCHCNT_W<22> {
        TXCHCNT_W::new(self)
    }
    #[doc = "Bit 24 - Alternate (Enhanced Descriptor)"]
    #[inline(always)]
    #[must_use]
    pub fn enhdessel(&mut self) -> ENHDESSEL_W<24> {
        ENHDESSEL_W::new(self)
    }
    #[doc = "Bit 25 - Timestamping with Internal System Time"]
    #[inline(always)]
    #[must_use]
    pub fn inttsen(&mut self) -> INTTSEN_W<25> {
        INTTSEN_W::new(self)
    }
    #[doc = "Bit 26 - Flexible Pulse-Per-Second Output"]
    #[inline(always)]
    #[must_use]
    pub fn flexippsen(&mut self) -> FLEXIPPSEN_W<26> {
        FLEXIPPSEN_W::new(self)
    }
    #[doc = "Bit 27 - Source Address or VLAN Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn savlanins(&mut self) -> SAVLANINS_W<27> {
        SAVLANINS_W::new(self)
    }
    #[doc = "Bits 28:30 - Active or selected PHY interface When you have multiple PHY interfaces in your configuration, this field indicates the sampled value of phy_intf_sel_i during reset de-assertion. - 000: GMII or MII - 001: RGMII - 010: SGMII - 011: TBI - 100: RMII - 101: RTBI - 110: SMII - 111: RevMII - All Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn actphyif(&mut self) -> ACTPHYIF_W<28> {
        ACTPHYIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HW Feature Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_hw_feature](index.html) module"]
pub struct DMA_HW_FEATURE_SPEC;
impl crate::RegisterSpec for DMA_HW_FEATURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_hw_feature::R](R) reader structure"]
impl crate::Readable for DMA_HW_FEATURE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_hw_feature::W](W) writer structure"]
impl crate::Writable for DMA_HW_FEATURE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_HW_FEATURE to value 0"]
impl crate::Resettable for DMA_HW_FEATURE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
