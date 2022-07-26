#[doc = "Register `LPI_TCR` reader"]
pub struct R(crate::R<LPI_TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPI_TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPI_TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPI_TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPI_TCR` writer"]
pub struct W(crate::W<LPI_TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPI_TCR_SPEC>;
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
impl From<crate::W<LPI_TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPI_TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LST` reader - LPI LS TIMER This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY. The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count. The default value of the LPI LS Timer is 1000 (1 sec) as defined in the IEEE standard."]
pub type LST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LST` writer - LPI LS TIMER This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY. The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count. The default value of the LPI LS Timer is 1000 (1 sec) as defined in the IEEE standard."]
pub type LST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPI_TCR_SPEC, u16, u16, 10, O>;
#[doc = "Field `TWT` reader - LPI TW TIMER This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission. The TLPIEX status bit is set after the expiry of this timer."]
pub type TWT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TWT` writer - LPI TW TIMER This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission. The TLPIEX status bit is set after the expiry of this timer."]
pub type TWT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPI_TCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:25 - LPI LS TIMER This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY. The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count. The default value of the LPI LS Timer is 1000 (1 sec) as defined in the IEEE standard."]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:15 - LPI TW TIMER This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission. The TLPIEX status bit is set after the expiry of this timer."]
    #[inline(always)]
    pub fn twt(&self) -> TWT_R {
        TWT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - LPI LS TIMER This field specifies the minimum time (in milliseconds) for which the link status from the PHY should be up (OKAY) before the LPI pattern can be transmitted to the PHY. The MAC does not transmit the LPI pattern even when the LPIEN bit is set unless the LPI LS Timer reaches the programmed terminal count. The default value of the LPI LS Timer is 1000 (1 sec) as defined in the IEEE standard."]
    #[inline(always)]
    pub fn lst(&mut self) -> LST_W<16> {
        LST_W::new(self)
    }
    #[doc = "Bits 0:15 - LPI TW TIMER This field specifies the minimum time (in microseconds) for which the MAC waits after it stops transmitting the LPI pattern to the PHY and before it resumes the normal transmission. The TLPIEX status bit is set after the expiry of this timer."]
    #[inline(always)]
    pub fn twt(&mut self) -> TWT_W<0> {
        TWT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPI Timers Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpi_tcr](index.html) module"]
pub struct LPI_TCR_SPEC;
impl crate::RegisterSpec for LPI_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpi_tcr::R](R) reader structure"]
impl crate::Readable for LPI_TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpi_tcr::W](W) writer structure"]
impl crate::Writable for LPI_TCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPI_TCR to value 0"]
impl crate::Resettable for LPI_TCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
