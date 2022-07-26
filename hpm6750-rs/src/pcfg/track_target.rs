#[doc = "Register `TRACK_TARGET` reader"]
pub struct R(crate::R<TRACK_TARGET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACK_TARGET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACK_TARGET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACK_TARGET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACK_TARGET` writer"]
pub struct W(crate::W<TRACK_TARGET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACK_TARGET_SPEC>;
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
impl From<crate::W<TRACK_TARGET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACK_TARGET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE_DIV` reader - Divider for reference source"]
pub type PRE_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRE_DIV` writer - Divider for reference source"]
pub type PRE_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRACK_TARGET_SPEC, u16, u16, 16, O>;
#[doc = "Field `TARGET` reader - Target frequency multiplier of divided source"]
pub type TARGET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TARGET` writer - Target frequency multiplier of divided source"]
pub type TARGET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRACK_TARGET_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - Divider for reference source"]
    #[inline(always)]
    pub fn pre_div(&self) -> PRE_DIV_R {
        PRE_DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Target frequency multiplier of divided source"]
    #[inline(always)]
    pub fn target(&self) -> TARGET_R {
        TARGET_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Divider for reference source"]
    #[inline(always)]
    pub fn pre_div(&mut self) -> PRE_DIV_W<16> {
        PRE_DIV_W::new(self)
    }
    #[doc = "Bits 0:15 - Target frequency multiplier of divided source"]
    #[inline(always)]
    pub fn target(&mut self) -> TARGET_W<0> {
        TARGET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RC 24M track target\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [track_target](index.html) module"]
pub struct TRACK_TARGET_SPEC;
impl crate::RegisterSpec for TRACK_TARGET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [track_target::R](R) reader structure"]
impl crate::Readable for TRACK_TARGET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [track_target::W](W) writer structure"]
impl crate::Writable for TRACK_TARGET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRACK_TARGET to value 0"]
impl crate::Resettable for TRACK_TARGET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
