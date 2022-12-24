#[doc = "Register `XMII_CSR` reader"]
pub struct R(crate::R<XMII_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XMII_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XMII_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XMII_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XMII_CSR` writer"]
pub struct W(crate::W<XMII_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XMII_CSR_SPEC>;
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
impl From<crate::W<XMII_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XMII_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LNKMOD` reader - Link Mode This bit indicates the current mode of operation of the link: - 1’b0: Half-duplex mode - 1’b1: Full-duplex mode"]
pub type LNKMOD_R = crate::BitReader<bool>;
#[doc = "Field `LNKMOD` writer - Link Mode This bit indicates the current mode of operation of the link: - 1’b0: Half-duplex mode - 1’b1: Full-duplex mode"]
pub type LNKMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, XMII_CSR_SPEC, bool, O>;
#[doc = "Field `LNKSPEED` reader - Link Speed This bit indicates the current speed of the link: - 00: 2.5 MHz - 01: 25 MHz - 10: 125 MHz Bit 2 is reserved when the MAC is configured for the SMII PHY interface."]
pub type LNKSPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LNKSPEED` writer - Link Speed This bit indicates the current speed of the link: - 00: 2.5 MHz - 01: 25 MHz - 10: 125 MHz Bit 2 is reserved when the MAC is configured for the SMII PHY interface."]
pub type LNKSPEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XMII_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LNKSTS` reader - Link Status This bit indicates whether the link between the local PHY and the remote PHY is up or down. It gives the status of the link between the SGMII of MAC and the SGMII of the local PHY. The status bits are received from the local PHY during ANEG betweent he MAC and PHY on the SGMII link."]
pub type LNKSTS_R = crate::BitReader<bool>;
#[doc = "Field `LNKSTS` writer - Link Status This bit indicates whether the link between the local PHY and the remote PHY is up or down. It gives the status of the link between the SGMII of MAC and the SGMII of the local PHY. The status bits are received from the local PHY during ANEG betweent he MAC and PHY on the SGMII link."]
pub type LNKSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, XMII_CSR_SPEC, bool, O>;
#[doc = "Field `JABTO` reader - Jabber Timeout This bit indicates whether there is jabber timeout error (1'b1) in the received frame. This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface."]
pub type JABTO_R = crate::BitReader<bool>;
#[doc = "Field `JABTO` writer - Jabber Timeout This bit indicates whether there is jabber timeout error (1'b1) in the received frame. This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface."]
pub type JABTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, XMII_CSR_SPEC, bool, O>;
#[doc = "Field `FALSCARDET` reader - False Carrier Detected This bit indicates whether the SMII PHY detected false carrier (1'b1). This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface."]
pub type FALSCARDET_R = crate::BitReader<bool>;
#[doc = "Field `FALSCARDET` writer - False Carrier Detected This bit indicates whether the SMII PHY detected false carrier (1'b1). This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface."]
pub type FALSCARDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, XMII_CSR_SPEC, bool, O>;
#[doc = "Field `SMIDRXS` reader - Delay SMII RX Data Sampling with respect to the SMII SYNC Signal When set, the first bit of the SMII RX data is sampled one cycle after the SMII SYNC signal. When reset, the first bit of the SMII RX data is sampled along with the SMII SYNC signal. If the SMII PHY Interface with source synchronous mode is selected during core configuration, this bit is reserved (RO with default value)."]
pub type SMIDRXS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Link Mode This bit indicates the current mode of operation of the link: - 1’b0: Half-duplex mode - 1’b1: Full-duplex mode"]
    #[inline(always)]
    pub fn lnkmod(&self) -> LNKMOD_R {
        LNKMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Link Speed This bit indicates the current speed of the link: - 00: 2.5 MHz - 01: 25 MHz - 10: 125 MHz Bit 2 is reserved when the MAC is configured for the SMII PHY interface."]
    #[inline(always)]
    pub fn lnkspeed(&self) -> LNKSPEED_R {
        LNKSPEED_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Link Status This bit indicates whether the link between the local PHY and the remote PHY is up or down. It gives the status of the link between the SGMII of MAC and the SGMII of the local PHY. The status bits are received from the local PHY during ANEG betweent he MAC and PHY on the SGMII link."]
    #[inline(always)]
    pub fn lnksts(&self) -> LNKSTS_R {
        LNKSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Jabber Timeout This bit indicates whether there is jabber timeout error (1'b1) in the received frame. This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface."]
    #[inline(always)]
    pub fn jabto(&self) -> JABTO_R {
        JABTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - False Carrier Detected This bit indicates whether the SMII PHY detected false carrier (1'b1). This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface."]
    #[inline(always)]
    pub fn falscardet(&self) -> FALSCARDET_R {
        FALSCARDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Delay SMII RX Data Sampling with respect to the SMII SYNC Signal When set, the first bit of the SMII RX data is sampled one cycle after the SMII SYNC signal. When reset, the first bit of the SMII RX data is sampled along with the SMII SYNC signal. If the SMII PHY Interface with source synchronous mode is selected during core configuration, this bit is reserved (RO with default value)."]
    #[inline(always)]
    pub fn smidrxs(&self) -> SMIDRXS_R {
        SMIDRXS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Link Mode This bit indicates the current mode of operation of the link: - 1’b0: Half-duplex mode - 1’b1: Full-duplex mode"]
    #[inline(always)]
    #[must_use]
    pub fn lnkmod(&mut self) -> LNKMOD_W<0> {
        LNKMOD_W::new(self)
    }
    #[doc = "Bits 1:2 - Link Speed This bit indicates the current speed of the link: - 00: 2.5 MHz - 01: 25 MHz - 10: 125 MHz Bit 2 is reserved when the MAC is configured for the SMII PHY interface."]
    #[inline(always)]
    #[must_use]
    pub fn lnkspeed(&mut self) -> LNKSPEED_W<1> {
        LNKSPEED_W::new(self)
    }
    #[doc = "Bit 3 - Link Status This bit indicates whether the link between the local PHY and the remote PHY is up or down. It gives the status of the link between the SGMII of MAC and the SGMII of the local PHY. The status bits are received from the local PHY during ANEG betweent he MAC and PHY on the SGMII link."]
    #[inline(always)]
    #[must_use]
    pub fn lnksts(&mut self) -> LNKSTS_W<3> {
        LNKSTS_W::new(self)
    }
    #[doc = "Bit 4 - Jabber Timeout This bit indicates whether there is jabber timeout error (1'b1) in the received frame. This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface."]
    #[inline(always)]
    #[must_use]
    pub fn jabto(&mut self) -> JABTO_W<4> {
        JABTO_W::new(self)
    }
    #[doc = "Bit 5 - False Carrier Detected This bit indicates whether the SMII PHY detected false carrier (1'b1). This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface."]
    #[inline(always)]
    #[must_use]
    pub fn falscardet(&mut self) -> FALSCARDET_W<5> {
        FALSCARDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SGMII/RGMII/SMII Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xmii_csr](index.html) module"]
pub struct XMII_CSR_SPEC;
impl crate::RegisterSpec for XMII_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xmii_csr::R](R) reader structure"]
impl crate::Readable for XMII_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xmii_csr::W](W) writer structure"]
impl crate::Writable for XMII_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XMII_CSR to value 0"]
impl crate::Resettable for XMII_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
