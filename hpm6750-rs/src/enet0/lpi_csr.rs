#[doc = "Register `LPI_CSR` reader"]
pub struct R(crate::R<LPI_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPI_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPI_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPI_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPI_CSR` writer"]
pub struct W(crate::W<LPI_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPI_CSR_SPEC>;
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
impl From<crate::W<LPI_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPI_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLPIEN` reader - Transmit LPI Entry When set, this bit indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit. This bit is cleared by a read into this register."]
pub type TLPIEN_R = crate::BitReader<bool>;
#[doc = "Field `TLPIEN` writer - Transmit LPI Entry When set, this bit indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit. This bit is cleared by a read into this register."]
pub type TLPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPI_CSR_SPEC, bool, O>;
#[doc = "Field `TLPIEX` reader - Transmit LPI Exit When set, this bit indicates that the MAC transmitter has exited the LPI state after the user has cleared the LPIEN bit and the LPI TW Timer has expired. This bit is cleared by a read into this register."]
pub type TLPIEX_R = crate::BitReader<bool>;
#[doc = "Field `TLPIEX` writer - Transmit LPI Exit When set, this bit indicates that the MAC transmitter has exited the LPI state after the user has cleared the LPIEN bit and the LPI TW Timer has expired. This bit is cleared by a read into this register."]
pub type TLPIEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPI_CSR_SPEC, bool, O>;
#[doc = "Field `RLPIEN` reader - Receive LPI Entry When set, this bit indicates that the MAC Receiver has received an LPI pattern and entered the LPI state. This bit is cleared by a read into this register. Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock."]
pub type RLPIEN_R = crate::BitReader<bool>;
#[doc = "Field `RLPIEN` writer - Receive LPI Entry When set, this bit indicates that the MAC Receiver has received an LPI pattern and entered the LPI state. This bit is cleared by a read into this register. Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock."]
pub type RLPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPI_CSR_SPEC, bool, O>;
#[doc = "Field `RLPIEX` reader - Receive LPI Exit When set, this bit indicates that the MAC Receiver has stopped receiving the LPI pattern on the GMII or MII interface, exited the LPI state, and resumed the normal reception. This bit is cleared by a read into this register. Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock."]
pub type RLPIEX_R = crate::BitReader<bool>;
#[doc = "Field `RLPIEX` writer - Receive LPI Exit When set, this bit indicates that the MAC Receiver has stopped receiving the LPI pattern on the GMII or MII interface, exited the LPI state, and resumed the normal reception. This bit is cleared by a read into this register. Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock."]
pub type RLPIEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPI_CSR_SPEC, bool, O>;
#[doc = "Field `TLPIST` reader - Transmit LPI State When set, this bit indicates that the MAC is transmitting the LPI pattern on the GMII or MII interface."]
pub type TLPIST_R = crate::BitReader<bool>;
#[doc = "Field `TLPIST` writer - Transmit LPI State When set, this bit indicates that the MAC is transmitting the LPI pattern on the GMII or MII interface."]
pub type TLPIST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPI_CSR_SPEC, bool, O>;
#[doc = "Field `RLPIST` reader - Receive LPI State When set, this bit indicates that the MAC is receiving the LPI pattern on the GMII or MII interface."]
pub type RLPIST_R = crate::BitReader<bool>;
#[doc = "Field `RLPIST` writer - Receive LPI State When set, this bit indicates that the MAC is receiving the LPI pattern on the GMII or MII interface."]
pub type RLPIST_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPI_CSR_SPEC, bool, O>;
#[doc = "Field `LPIEN` reader - LPI Enable When set, this bit instructs the MAC Transmitter to enter the LPI state. When reset, this bit instructs the MAC to exit the LPI state and resume normal transmission. This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission."]
pub type LPIEN_R = crate::BitReader<bool>;
#[doc = "Field `LPIEN` writer - LPI Enable When set, this bit instructs the MAC Transmitter to enter the LPI state. When reset, this bit instructs the MAC to exit the LPI state and resume normal transmission. This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission."]
pub type LPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPI_CSR_SPEC, bool, O>;
#[doc = "Field `PLS` reader - PHY Link Status This bit indicates the link status of the PHY. The MAC Transmitter asserts the LPI pattern only when the link status is up (okay) at least for the time indicated by the LPI LS TIMER. When set, the link is considered to be okay (up) and when reset, the link is considered to be down."]
pub type PLS_R = crate::BitReader<bool>;
#[doc = "Field `PLS` writer - PHY Link Status This bit indicates the link status of the PHY. The MAC Transmitter asserts the LPI pattern only when the link status is up (okay) at least for the time indicated by the LPI LS TIMER. When set, the link is considered to be okay (up) and when reset, the link is considered to be down."]
pub type PLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPI_CSR_SPEC, bool, O>;
#[doc = "Field `PLSEN` reader - PHY Link Status Enable This bit enables the link status received on the RGMII, SGMII, or SMII receive paths to be used for activating the LPI LS TIMER. When set, the MAC uses the link-status bits of Register 54 (SGMII/RGMII/SMII Control and Status Register) and Bit 17 (PLS) for the LPI LS Timer trigger. When cleared, the MAC ignores the link-status bits of Register 54 and takes only the PLS bit. This bit is RO and reserved if you have not selected the RGMII, SGMII, or SMII PHY interface."]
pub type PLSEN_R = crate::BitReader<bool>;
#[doc = "Field `PLSEN` writer - PHY Link Status Enable This bit enables the link status received on the RGMII, SGMII, or SMII receive paths to be used for activating the LPI LS TIMER. When set, the MAC uses the link-status bits of Register 54 (SGMII/RGMII/SMII Control and Status Register) and Bit 17 (PLS) for the LPI LS Timer trigger. When cleared, the MAC ignores the link-status bits of Register 54 and takes only the PLS bit. This bit is RO and reserved if you have not selected the RGMII, SGMII, or SMII PHY interface."]
pub type PLSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPI_CSR_SPEC, bool, O>;
#[doc = "Field `LPITXA` reader - LPI TX Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the transmit side. This bit is not functional in the GMAC-CORE configuration in which the Tx clock gating is done during the LPI mode. If the LPITXA and LPIEN bits are set to 1, the MAC enters the LPI mode only after all outstanding frames (in the core) and pending frames (in the application interface) have been transmitted. The MAC comes out of the LPI mode when the application sends any frame for transmission or the application issues a TX FIFO Flush command. In addition, the MAC automatically clears the LPIEN bit when it exits the LPI state. If TX FIFO Flush is set in Bit 20 of Register 6 (Operation Mode Register), when the MAC is in the LPI mode, the MAC exits the LPI mode. When this bit is 0, the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode."]
pub type LPITXA_R = crate::BitReader<bool>;
#[doc = "Field `LPITXA` writer - LPI TX Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the transmit side. This bit is not functional in the GMAC-CORE configuration in which the Tx clock gating is done during the LPI mode. If the LPITXA and LPIEN bits are set to 1, the MAC enters the LPI mode only after all outstanding frames (in the core) and pending frames (in the application interface) have been transmitted. The MAC comes out of the LPI mode when the application sends any frame for transmission or the application issues a TX FIFO Flush command. In addition, the MAC automatically clears the LPIEN bit when it exits the LPI state. If TX FIFO Flush is set in Bit 20 of Register 6 (Operation Mode Register), when the MAC is in the LPI mode, the MAC exits the LPI mode. When this bit is 0, the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode."]
pub type LPITXA_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPI_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit LPI Entry When set, this bit indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit. This bit is cleared by a read into this register."]
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit LPI Exit When set, this bit indicates that the MAC transmitter has exited the LPI state after the user has cleared the LPIEN bit and the LPI TW Timer has expired. This bit is cleared by a read into this register."]
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive LPI Entry When set, this bit indicates that the MAC Receiver has received an LPI pattern and entered the LPI state. This bit is cleared by a read into this register. Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock."]
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive LPI Exit When set, this bit indicates that the MAC Receiver has stopped receiving the LPI pattern on the GMII or MII interface, exited the LPI state, and resumed the normal reception. This bit is cleared by a read into this register. Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock."]
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit LPI State When set, this bit indicates that the MAC is transmitting the LPI pattern on the GMII or MII interface."]
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive LPI State When set, this bit indicates that the MAC is receiving the LPI pattern on the GMII or MII interface."]
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - LPI Enable When set, this bit instructs the MAC Transmitter to enter the LPI state. When reset, this bit instructs the MAC to exit the LPI state and resume normal transmission. This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission."]
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PHY Link Status This bit indicates the link status of the PHY. The MAC Transmitter asserts the LPI pattern only when the link status is up (okay) at least for the time indicated by the LPI LS TIMER. When set, the link is considered to be okay (up) and when reset, the link is considered to be down."]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PHY Link Status Enable This bit enables the link status received on the RGMII, SGMII, or SMII receive paths to be used for activating the LPI LS TIMER. When set, the MAC uses the link-status bits of Register 54 (SGMII/RGMII/SMII Control and Status Register) and Bit 17 (PLS) for the LPI LS Timer trigger. When cleared, the MAC ignores the link-status bits of Register 54 and takes only the PLS bit. This bit is RO and reserved if you have not selected the RGMII, SGMII, or SMII PHY interface."]
    #[inline(always)]
    pub fn plsen(&self) -> PLSEN_R {
        PLSEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LPI TX Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the transmit side. This bit is not functional in the GMAC-CORE configuration in which the Tx clock gating is done during the LPI mode. If the LPITXA and LPIEN bits are set to 1, the MAC enters the LPI mode only after all outstanding frames (in the core) and pending frames (in the application interface) have been transmitted. The MAC comes out of the LPI mode when the application sends any frame for transmission or the application issues a TX FIFO Flush command. In addition, the MAC automatically clears the LPIEN bit when it exits the LPI state. If TX FIFO Flush is set in Bit 20 of Register 6 (Operation Mode Register), when the MAC is in the LPI mode, the MAC exits the LPI mode. When this bit is 0, the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode."]
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit LPI Entry When set, this bit indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit. This bit is cleared by a read into this register."]
    #[inline(always)]
    #[must_use]
    pub fn tlpien(&mut self) -> TLPIEN_W<0> {
        TLPIEN_W::new(self)
    }
    #[doc = "Bit 1 - Transmit LPI Exit When set, this bit indicates that the MAC transmitter has exited the LPI state after the user has cleared the LPIEN bit and the LPI TW Timer has expired. This bit is cleared by a read into this register."]
    #[inline(always)]
    #[must_use]
    pub fn tlpiex(&mut self) -> TLPIEX_W<1> {
        TLPIEX_W::new(self)
    }
    #[doc = "Bit 2 - Receive LPI Entry When set, this bit indicates that the MAC Receiver has received an LPI pattern and entered the LPI state. This bit is cleared by a read into this register. Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock."]
    #[inline(always)]
    #[must_use]
    pub fn rlpien(&mut self) -> RLPIEN_W<2> {
        RLPIEN_W::new(self)
    }
    #[doc = "Bit 3 - Receive LPI Exit When set, this bit indicates that the MAC Receiver has stopped receiving the LPI pattern on the GMII or MII interface, exited the LPI state, and resumed the normal reception. This bit is cleared by a read into this register. Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock."]
    #[inline(always)]
    #[must_use]
    pub fn rlpiex(&mut self) -> RLPIEX_W<3> {
        RLPIEX_W::new(self)
    }
    #[doc = "Bit 8 - Transmit LPI State When set, this bit indicates that the MAC is transmitting the LPI pattern on the GMII or MII interface."]
    #[inline(always)]
    #[must_use]
    pub fn tlpist(&mut self) -> TLPIST_W<8> {
        TLPIST_W::new(self)
    }
    #[doc = "Bit 9 - Receive LPI State When set, this bit indicates that the MAC is receiving the LPI pattern on the GMII or MII interface."]
    #[inline(always)]
    #[must_use]
    pub fn rlpist(&mut self) -> RLPIST_W<9> {
        RLPIST_W::new(self)
    }
    #[doc = "Bit 16 - LPI Enable When set, this bit instructs the MAC Transmitter to enter the LPI state. When reset, this bit instructs the MAC to exit the LPI state and resume normal transmission. This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission."]
    #[inline(always)]
    #[must_use]
    pub fn lpien(&mut self) -> LPIEN_W<16> {
        LPIEN_W::new(self)
    }
    #[doc = "Bit 17 - PHY Link Status This bit indicates the link status of the PHY. The MAC Transmitter asserts the LPI pattern only when the link status is up (okay) at least for the time indicated by the LPI LS TIMER. When set, the link is considered to be okay (up) and when reset, the link is considered to be down."]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<17> {
        PLS_W::new(self)
    }
    #[doc = "Bit 18 - PHY Link Status Enable This bit enables the link status received on the RGMII, SGMII, or SMII receive paths to be used for activating the LPI LS TIMER. When set, the MAC uses the link-status bits of Register 54 (SGMII/RGMII/SMII Control and Status Register) and Bit 17 (PLS) for the LPI LS Timer trigger. When cleared, the MAC ignores the link-status bits of Register 54 and takes only the PLS bit. This bit is RO and reserved if you have not selected the RGMII, SGMII, or SMII PHY interface."]
    #[inline(always)]
    #[must_use]
    pub fn plsen(&mut self) -> PLSEN_W<18> {
        PLSEN_W::new(self)
    }
    #[doc = "Bit 19 - LPI TX Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the transmit side. This bit is not functional in the GMAC-CORE configuration in which the Tx clock gating is done during the LPI mode. If the LPITXA and LPIEN bits are set to 1, the MAC enters the LPI mode only after all outstanding frames (in the core) and pending frames (in the application interface) have been transmitted. The MAC comes out of the LPI mode when the application sends any frame for transmission or the application issues a TX FIFO Flush command. In addition, the MAC automatically clears the LPIEN bit when it exits the LPI state. If TX FIFO Flush is set in Bit 20 of Register 6 (Operation Mode Register), when the MAC is in the LPI mode, the MAC exits the LPI mode. When this bit is 0, the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode."]
    #[inline(always)]
    #[must_use]
    pub fn lpitxa(&mut self) -> LPITXA_W<19> {
        LPITXA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPI Control and Status Regsiter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpi_csr](index.html) module"]
pub struct LPI_CSR_SPEC;
impl crate::RegisterSpec for LPI_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpi_csr::R](R) reader structure"]
impl crate::Readable for LPI_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpi_csr::W](W) writer structure"]
impl crate::Writable for LPI_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPI_CSR to value 0"]
impl crate::Resettable for LPI_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
