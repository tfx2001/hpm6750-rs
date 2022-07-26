#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFTRST` reader - Self-clear"]
pub type SFTRST_R = crate::BitReader<bool>;
#[doc = "Field `SFTRST` writer - Self-clear"]
pub type SFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RUN` reader - Enable this module to run."]
pub type RUN_R = crate::BitReader<bool>;
#[doc = "Field `RUN` writer - Enable this module to run."]
pub type RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Self-clear"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Enable this module to run."]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Self-clear"]
    #[inline(always)]
    pub fn sftrst(&mut self) -> SFTRST_W<1> {
        SFTRST_W::new(self)
    }
    #[doc = "Bit 0 - Enable this module to run."]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W<0> {
        RUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
