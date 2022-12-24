#[doc = "Register `TT_WTRIG` reader"]
pub struct R(crate::R<TT_WTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TT_WTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TT_WTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TT_WTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TT_WTRIG` writer"]
pub struct W(crate::W<TT_WTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TT_WTRIG_SPEC>;
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
impl From<crate::W<TT_WTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TT_WTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TT_WTRIG` reader - Watch Trigger Time TT_WTRIG(15:0) defines the cycle time for a watch trigger. The initial watch trigger isthe maximum cycle time 0xffff."]
pub type TT_WTRIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TT_WTRIG` writer - Watch Trigger Time TT_WTRIG(15:0) defines the cycle time for a watch trigger. The initial watch trigger isthe maximum cycle time 0xffff."]
pub type TT_WTRIG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TT_WTRIG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Watch Trigger Time TT_WTRIG(15:0) defines the cycle time for a watch trigger. The initial watch trigger isthe maximum cycle time 0xffff."]
    #[inline(always)]
    pub fn tt_wtrig(&self) -> TT_WTRIG_R {
        TT_WTRIG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Watch Trigger Time TT_WTRIG(15:0) defines the cycle time for a watch trigger. The initial watch trigger isthe maximum cycle time 0xffff."]
    #[inline(always)]
    #[must_use]
    pub fn tt_wtrig(&mut self) -> TT_WTRIG_W<0> {
        TT_WTRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TTCAN: Watch Trigger Time TT_WTRIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tt_wtrig](index.html) module"]
pub struct TT_WTRIG_SPEC;
impl crate::RegisterSpec for TT_WTRIG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tt_wtrig::R](R) reader structure"]
impl crate::Readable for TT_WTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tt_wtrig::W](W) writer structure"]
impl crate::Writable for TT_WTRIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TT_WTRIG to value 0"]
impl crate::Resettable for TT_WTRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
