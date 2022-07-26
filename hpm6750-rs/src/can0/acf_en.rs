#[doc = "Register `ACF_EN` reader"]
pub struct R(crate::R<ACF_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACF_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACF_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACF_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACF_EN` writer"]
pub struct W(crate::W<ACF_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACF_EN_SPEC>;
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
impl From<crate::W<ACF_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACF_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACF_EN` reader - Acceptance filter Enable 1 - acceptance filter enabled 0 - acceptance filter disable Each acceptance filter (AMASK / ACODE) can be individually enabled or disabled. Disabled filters reject a message. Only enabled filters can accept a message if the appropriate AMASK / ACODE configuration matches."]
pub type ACF_EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACF_EN` writer - Acceptance filter Enable 1 - acceptance filter enabled 0 - acceptance filter disable Each acceptance filter (AMASK / ACODE) can be individually enabled or disabled. Disabled filters reject a message. Only enabled filters can accept a message if the appropriate AMASK / ACODE configuration matches."]
pub type ACF_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, ACF_EN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Acceptance filter Enable 1 - acceptance filter enabled 0 - acceptance filter disable Each acceptance filter (AMASK / ACODE) can be individually enabled or disabled. Disabled filters reject a message. Only enabled filters can accept a message if the appropriate AMASK / ACODE configuration matches."]
    #[inline(always)]
    pub fn acf_en(&self) -> ACF_EN_R {
        ACF_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Acceptance filter Enable 1 - acceptance filter enabled 0 - acceptance filter disable Each acceptance filter (AMASK / ACODE) can be individually enabled or disabled. Disabled filters reject a message. Only enabled filters can accept a message if the appropriate AMASK / ACODE configuration matches."]
    #[inline(always)]
    pub fn acf_en(&mut self) -> ACF_EN_W<0> {
        ACF_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Acceptance Filter Enable ACF_EN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acf_en](index.html) module"]
pub struct ACF_EN_SPEC;
impl crate::RegisterSpec for ACF_EN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [acf_en::R](R) reader structure"]
impl crate::Readable for ACF_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acf_en::W](W) writer structure"]
impl crate::Writable for ACF_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACF_EN to value 0"]
impl crate::Resettable for ACF_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
