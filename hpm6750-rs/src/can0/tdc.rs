#[doc = "Register `TDC` reader"]
pub struct R(crate::R<TDC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TDC` writer"]
pub struct W(crate::W<TDC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDC_SPEC>;
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
impl From<crate::W<TDC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDCEN` reader - Transmitter Delay Compensation ENable TDC will be activated during the data phase of a CAN FD frame if BRS is active if TDCEN=1."]
pub type TDCEN_R = crate::BitReader<bool>;
#[doc = "Field `TDCEN` writer - Transmitter Delay Compensation ENable TDC will be activated during the data phase of a CAN FD frame if BRS is active if TDCEN=1."]
pub type TDCEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, TDC_SPEC, bool, O>;
#[doc = "Field `SSPOFF` reader - Secondary Sample Point OFFset The transmitter delay plus SSPOFF defines the time of the secondary sample point for TDC. SSPOFF is given as a number of TQ."]
pub type SSPOFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSPOFF` writer - Secondary Sample Point OFFset The transmitter delay plus SSPOFF defines the time of the secondary sample point for TDC. SSPOFF is given as a number of TQ."]
pub type SSPOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TDC_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 7 - Transmitter Delay Compensation ENable TDC will be activated during the data phase of a CAN FD frame if BRS is active if TDCEN=1."]
    #[inline(always)]
    pub fn tdcen(&self) -> TDCEN_R {
        TDCEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 0:6 - Secondary Sample Point OFFset The transmitter delay plus SSPOFF defines the time of the secondary sample point for TDC. SSPOFF is given as a number of TQ."]
    #[inline(always)]
    pub fn sspoff(&self) -> SSPOFF_R {
        SSPOFF_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Transmitter Delay Compensation ENable TDC will be activated during the data phase of a CAN FD frame if BRS is active if TDCEN=1."]
    #[inline(always)]
    pub fn tdcen(&mut self) -> TDCEN_W<7> {
        TDCEN_W::new(self)
    }
    #[doc = "Bits 0:6 - Secondary Sample Point OFFset The transmitter delay plus SSPOFF defines the time of the secondary sample point for TDC. SSPOFF is given as a number of TQ."]
    #[inline(always)]
    pub fn sspoff(&mut self) -> SSPOFF_W<0> {
        SSPOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmitter Delay Compensation Register TDC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdc](index.html) module"]
pub struct TDC_SPEC;
impl crate::RegisterSpec for TDC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tdc::R](R) reader structure"]
impl crate::Readable for TDC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tdc::W](W) writer structure"]
impl crate::Writable for TDC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TDC to value 0"]
impl crate::Resettable for TDC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
