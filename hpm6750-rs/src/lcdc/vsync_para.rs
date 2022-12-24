#[doc = "Register `VSYNC_PARA` reader"]
pub struct R(crate::R<VSYNC_PARA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VSYNC_PARA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VSYNC_PARA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VSYNC_PARA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VSYNC_PARA` writer"]
pub struct W(crate::W<VSYNC_PARA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VSYNC_PARA_SPEC>;
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
impl From<crate::W<VSYNC_PARA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VSYNC_PARA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PW` reader - VSYNC active pulse width (in horizontal line cycles). Pulse width has a minimum value of 1."]
pub type PW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PW` writer - VSYNC active pulse width (in horizontal line cycles). Pulse width has a minimum value of 1."]
pub type PW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VSYNC_PARA_SPEC, u16, u16, 9, O>;
#[doc = "Field `BP` reader - VSYNC back-porch pulse width (in horizontal line cycles). If zero, means no back-porch for VSYNC"]
pub type BP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BP` writer - VSYNC back-porch pulse width (in horizontal line cycles). If zero, means no back-porch for VSYNC"]
pub type BP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VSYNC_PARA_SPEC, u16, u16, 9, O>;
#[doc = "Field `FP` reader - VSYNC front-porch pulse width (in horizontal line cycles). If zero, means no front-porch for VSYNC"]
pub type FP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FP` writer - VSYNC front-porch pulse width (in horizontal line cycles). If zero, means no front-porch for VSYNC"]
pub type FP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VSYNC_PARA_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - VSYNC active pulse width (in horizontal line cycles). Pulse width has a minimum value of 1."]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 11:19 - VSYNC back-porch pulse width (in horizontal line cycles). If zero, means no back-porch for VSYNC"]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:30 - VSYNC front-porch pulse width (in horizontal line cycles). If zero, means no front-porch for VSYNC"]
    #[inline(always)]
    pub fn fp(&self) -> FP_R {
        FP_R::new(((self.bits >> 22) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - VSYNC active pulse width (in horizontal line cycles). Pulse width has a minimum value of 1."]
    #[inline(always)]
    #[must_use]
    pub fn pw(&mut self) -> PW_W<0> {
        PW_W::new(self)
    }
    #[doc = "Bits 11:19 - VSYNC back-porch pulse width (in horizontal line cycles). If zero, means no back-porch for VSYNC"]
    #[inline(always)]
    #[must_use]
    pub fn bp(&mut self) -> BP_W<11> {
        BP_W::new(self)
    }
    #[doc = "Bits 22:30 - VSYNC front-porch pulse width (in horizontal line cycles). If zero, means no front-porch for VSYNC"]
    #[inline(always)]
    #[must_use]
    pub fn fp(&mut self) -> FP_W<22> {
        FP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VSYNC Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vsync_para](index.html) module"]
pub struct VSYNC_PARA_SPEC;
impl crate::RegisterSpec for VSYNC_PARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vsync_para::R](R) reader structure"]
impl crate::Readable for VSYNC_PARA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vsync_para::W](W) writer structure"]
impl crate::Writable for VSYNC_PARA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VSYNC_PARA to value 0"]
impl crate::Resettable for VSYNC_PARA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
