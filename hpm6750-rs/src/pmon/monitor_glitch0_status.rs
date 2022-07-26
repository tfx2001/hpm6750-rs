#[doc = "Register `MONITOR_GLITCH0_STATUS` reader"]
pub struct R(crate::R<MONITOR_GLITCH0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MONITOR_GLITCH0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MONITOR_GLITCH0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MONITOR_GLITCH0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MONITOR_GLITCH0_STATUS` writer"]
pub struct W(crate::W<MONITOR_GLITCH0_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MONITOR_GLITCH0_STATUS_SPEC>;
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
impl From<crate::W<MONITOR_GLITCH0_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MONITOR_GLITCH0_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLAG` reader - flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected"]
pub type FLAG_R = crate::BitReader<bool>;
#[doc = "Field `FLAG` writer - flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected"]
pub type FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MONITOR_GLITCH0_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected"]
    #[inline(always)]
    pub fn flag(&self) -> FLAG_R {
        FLAG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected"]
    #[inline(always)]
    pub fn flag(&mut self) -> FLAG_W<0> {
        FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Glitch and clock monitor status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monitor_glitch0_status](index.html) module"]
pub struct MONITOR_GLITCH0_STATUS_SPEC;
impl crate::RegisterSpec for MONITOR_GLITCH0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [monitor_glitch0_status::R](R) reader structure"]
impl crate::Readable for MONITOR_GLITCH0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [monitor_glitch0_status::W](W) writer structure"]
impl crate::Writable for MONITOR_GLITCH0_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MONITOR_GLITCH0_STATUS to value 0"]
impl crate::Resettable for MONITOR_GLITCH0_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
