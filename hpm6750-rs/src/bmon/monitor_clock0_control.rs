#[doc = "Register `MONITOR_CLOCK0_CONTROL` reader"]
pub struct R(crate::R<MONITOR_CLOCK0_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MONITOR_CLOCK0_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MONITOR_CLOCK0_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MONITOR_CLOCK0_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MONITOR_CLOCK0_CONTROL` writer"]
pub struct W(crate::W<MONITOR_CLOCK0_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MONITOR_CLOCK0_CONTROL_SPEC>;
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
impl From<crate::W<MONITOR_CLOCK0_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MONITOR_CLOCK0_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - enable glitch detector 0: detector disabled 1: detector enabled"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - enable glitch detector 0: detector disabled 1: detector enabled"]
pub type ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MONITOR_CLOCK0_CONTROL_SPEC, bool, O>;
#[doc = "Field `ACTIVE` reader - select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destory DFF value 1: active mode, check glitch by DFF chain"]
pub type ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE` writer - select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destory DFF value 1: active mode, check glitch by DFF chain"]
pub type ACTIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MONITOR_CLOCK0_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - enable glitch detector 0: detector disabled 1: detector enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destory DFF value 1: active mode, check glitch by DFF chain"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable glitch detector 0: detector disabled 1: detector enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 4 - select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destory DFF value 1: active mode, check glitch by DFF chain"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<4> {
        ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Glitch and clock monitor control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monitor_clock0_control](index.html) module"]
pub struct MONITOR_CLOCK0_CONTROL_SPEC;
impl crate::RegisterSpec for MONITOR_CLOCK0_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [monitor_clock0_control::R](R) reader structure"]
impl crate::Readable for MONITOR_CLOCK0_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [monitor_clock0_control::W](W) writer structure"]
impl crate::Writable for MONITOR_CLOCK0_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MONITOR_CLOCK0_CONTROL to value 0"]
impl crate::Resettable for MONITOR_CLOCK0_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
