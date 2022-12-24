#[doc = "Register `TT_TRIG` reader"]
pub struct R(crate::R<TT_TRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TT_TRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TT_TRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TT_TRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TT_TRIG` writer"]
pub struct W(crate::W<TT_TRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TT_TRIG_SPEC>;
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
impl From<crate::W<TT_TRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TT_TRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TT_TRIG` reader - Trigger Time TT_TRIG(15:0) defines the cycle time for a trigger. For a transmission trigger theearliest point of transmission of the SOF of the appropriate frame will be TT_TRIG+1."]
pub type TT_TRIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TT_TRIG` writer - Trigger Time TT_TRIG(15:0) defines the cycle time for a trigger. For a transmission trigger theearliest point of transmission of the SOF of the appropriate frame will be TT_TRIG+1."]
pub type TT_TRIG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TT_TRIG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Trigger Time TT_TRIG(15:0) defines the cycle time for a trigger. For a transmission trigger theearliest point of transmission of the SOF of the appropriate frame will be TT_TRIG+1."]
    #[inline(always)]
    pub fn tt_trig(&self) -> TT_TRIG_R {
        TT_TRIG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Trigger Time TT_TRIG(15:0) defines the cycle time for a trigger. For a transmission trigger theearliest point of transmission of the SOF of the appropriate frame will be TT_TRIG+1."]
    #[inline(always)]
    #[must_use]
    pub fn tt_trig(&mut self) -> TT_TRIG_W<0> {
        TT_TRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TTCAN: Trigger Time TT_TRIG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tt_trig](index.html) module"]
pub struct TT_TRIG_SPEC;
impl crate::RegisterSpec for TT_TRIG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tt_trig::R](R) reader structure"]
impl crate::Readable for TT_TRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tt_trig::W](W) writer structure"]
impl crate::Writable for TT_TRIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TT_TRIG to value 0"]
impl crate::Resettable for TT_TRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
