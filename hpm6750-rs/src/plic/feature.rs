#[doc = "Register `FEATURE` reader"]
pub struct R(crate::R<FEATURE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEATURE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEATURE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEATURE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEATURE` writer"]
pub struct W(crate::W<FEATURE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEATURE_SPEC>;
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
impl From<crate::W<FEATURE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEATURE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTORED` reader - Vector mode enable 0: Disabled 1: Enabled"]
pub type VECTORED_R = crate::BitReader<bool>;
#[doc = "Field `VECTORED` writer - Vector mode enable 0: Disabled 1: Enabled"]
pub type VECTORED_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEATURE_SPEC, bool, O>;
#[doc = "Field `PREEMPT` reader - Preemptive priority interrupt enable 0: Disabled 1: Enabled"]
pub type PREEMPT_R = crate::BitReader<bool>;
#[doc = "Field `PREEMPT` writer - Preemptive priority interrupt enable 0: Disabled 1: Enabled"]
pub type PREEMPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEATURE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Vector mode enable 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn vectored(&self) -> VECTORED_R {
        VECTORED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Preemptive priority interrupt enable 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn preempt(&self) -> PREEMPT_R {
        PREEMPT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Vector mode enable 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn vectored(&mut self) -> VECTORED_W<1> {
        VECTORED_W::new(self)
    }
    #[doc = "Bit 0 - Preemptive priority interrupt enable 0: Disabled 1: Enabled"]
    #[inline(always)]
    pub fn preempt(&mut self) -> PREEMPT_W<0> {
        PREEMPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Feature enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feature](index.html) module"]
pub struct FEATURE_SPEC;
impl crate::RegisterSpec for FEATURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [feature::R](R) reader structure"]
impl crate::Readable for FEATURE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [feature::W](W) writer structure"]
impl crate::Writable for FEATURE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FEATURE to value 0"]
impl crate::Resettable for FEATURE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
