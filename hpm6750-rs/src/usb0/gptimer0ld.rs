#[doc = "Register `GPTIMER0LD` reader"]
pub struct R(crate::R<GPTIMER0LD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTIMER0LD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTIMER0LD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTIMER0LD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTIMER0LD` writer"]
pub struct W(crate::W<GPTIMER0LD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTIMER0LD_SPEC>;
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
impl From<crate::W<GPTIMER0LD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTIMER0LD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPTLD` reader - GPTLD General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'. This value represents the time in microseconds minus 1 for the timer duration. Example: for a one millisecond timer, load 1000-1=999 or 0x0003E7. NOTE: Max value is 0xFFFFFF or 16.777215 seconds."]
pub type GPTLD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPTLD` writer - GPTLD General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'. This value represents the time in microseconds minus 1 for the timer duration. Example: for a one millisecond timer, load 1000-1=999 or 0x0003E7. NOTE: Max value is 0xFFFFFF or 16.777215 seconds."]
pub type GPTLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPTIMER0LD_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - GPTLD General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'. This value represents the time in microseconds minus 1 for the timer duration. Example: for a one millisecond timer, load 1000-1=999 or 0x0003E7. NOTE: Max value is 0xFFFFFF or 16.777215 seconds."]
    #[inline(always)]
    pub fn gptld(&self) -> GPTLD_R {
        GPTLD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - GPTLD General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'. This value represents the time in microseconds minus 1 for the timer duration. Example: for a one millisecond timer, load 1000-1=999 or 0x0003E7. NOTE: Max value is 0xFFFFFF or 16.777215 seconds."]
    #[inline(always)]
    pub fn gptld(&mut self) -> GPTLD_W<0> {
        GPTLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Timer #0 Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptimer0ld](index.html) module"]
pub struct GPTIMER0LD_SPEC;
impl crate::RegisterSpec for GPTIMER0LD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptimer0ld::R](R) reader structure"]
impl crate::Readable for GPTIMER0LD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptimer0ld::W](W) writer structure"]
impl crate::Writable for GPTIMER0LD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPTIMER0LD to value 0"]
impl crate::Resettable for GPTIMER0LD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
