#[doc = "Register `DEBUG_STOP` reader"]
pub struct R(crate::R<DEBUG_STOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_STOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_STOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_STOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_STOP` writer"]
pub struct W(crate::W<DEBUG_STOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_STOP_SPEC>;
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
impl From<crate::W<DEBUG_STOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_STOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU1` reader - Stop peripheral when CPU1 enter debug mode 0: peripheral keep running when CPU1 in debug mode 1: peripheral enter debug mode when CPU1 enter debug"]
pub type CPU1_R = crate::BitReader<bool>;
#[doc = "Field `CPU1` writer - Stop peripheral when CPU1 enter debug mode 0: peripheral keep running when CPU1 in debug mode 1: peripheral enter debug mode when CPU1 enter debug"]
pub type CPU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_STOP_SPEC, bool, O>;
#[doc = "Field `CPU0` reader - Stop peripheral when CPU0 enter debug mode 0: peripheral keep running when CPU0 in debug mode 1: peripheral enter debug mode when CPU0 enter debug"]
pub type CPU0_R = crate::BitReader<bool>;
#[doc = "Field `CPU0` writer - Stop peripheral when CPU0 enter debug mode 0: peripheral keep running when CPU0 in debug mode 1: peripheral enter debug mode when CPU0 enter debug"]
pub type CPU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_STOP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Stop peripheral when CPU1 enter debug mode 0: peripheral keep running when CPU1 in debug mode 1: peripheral enter debug mode when CPU1 enter debug"]
    #[inline(always)]
    pub fn cpu1(&self) -> CPU1_R {
        CPU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Stop peripheral when CPU0 enter debug mode 0: peripheral keep running when CPU0 in debug mode 1: peripheral enter debug mode when CPU0 enter debug"]
    #[inline(always)]
    pub fn cpu0(&self) -> CPU0_R {
        CPU0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Stop peripheral when CPU1 enter debug mode 0: peripheral keep running when CPU1 in debug mode 1: peripheral enter debug mode when CPU1 enter debug"]
    #[inline(always)]
    pub fn cpu1(&mut self) -> CPU1_W<1> {
        CPU1_W::new(self)
    }
    #[doc = "Bit 0 - Stop peripheral when CPU0 enter debug mode 0: peripheral keep running when CPU0 in debug mode 1: peripheral enter debug mode when CPU0 enter debug"]
    #[inline(always)]
    pub fn cpu0(&mut self) -> CPU0_W<0> {
        CPU0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug stop config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_stop](index.html) module"]
pub struct DEBUG_STOP_SPEC;
impl crate::RegisterSpec for DEBUG_STOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_stop::R](R) reader structure"]
impl crate::Readable for DEBUG_STOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_stop::W](W) writer structure"]
impl crate::Writable for DEBUG_STOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEBUG_STOP to value 0x01"]
impl crate::Resettable for DEBUG_STOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
