#[doc = "Register `MSR` reader"]
pub struct R(crate::R<MSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSR` writer"]
pub struct W(crate::W<MSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSR_SPEC>;
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
impl From<crate::W<MSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTS` reader - Clear to send 0: The modem_ctsn input signal is HIGH. 1: The modem_ctsn input signal is LOW."]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `DCTS` reader - Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read."]
pub type DCTS_R = crate::BitReader<bool>;
#[doc = "Field `DCTS` writer - Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read."]
pub type DCTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - Clear to send 0: The modem_ctsn input signal is HIGH. 1: The modem_ctsn input signal is LOW."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 0 - Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read."]
    #[inline(always)]
    pub fn dcts(&self) -> DCTS_R {
        DCTS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read."]
    #[inline(always)]
    pub fn dcts(&mut self) -> DCTS_W<0> {
        DCTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Modem Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](index.html) module"]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msr::R](R) reader structure"]
impl crate::Readable for MSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msr::W](W) writer structure"]
impl crate::Writable for MSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
