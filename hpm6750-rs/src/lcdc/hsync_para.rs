#[doc = "Register `HSYNC_PARA` reader"]
pub struct R(crate::R<HSYNC_PARA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSYNC_PARA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSYNC_PARA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSYNC_PARA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSYNC_PARA` writer"]
pub struct W(crate::W<HSYNC_PARA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSYNC_PARA_SPEC>;
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
impl From<crate::W<HSYNC_PARA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSYNC_PARA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PW` reader - HSYNC active pulse width (in pixel clock cycles). Pulse width has a minimum value of 1."]
pub type PW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PW` writer - HSYNC active pulse width (in pixel clock cycles). Pulse width has a minimum value of 1."]
pub type PW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSYNC_PARA_SPEC, u16, u16, 9, O>;
#[doc = "Field `BP` reader - HSYNC back-porch pulse width (in pixel clock cycles). If zero, indicates no back-porch for HSYNC"]
pub type BP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BP` writer - HSYNC back-porch pulse width (in pixel clock cycles). If zero, indicates no back-porch for HSYNC"]
pub type BP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSYNC_PARA_SPEC, u16, u16, 9, O>;
#[doc = "Field `FP` reader - HSYNC front-porch pulse width (in pixel clock cycles). If zero, indicates no front-porch for HSYNC"]
pub type FP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FP` writer - HSYNC front-porch pulse width (in pixel clock cycles). If zero, indicates no front-porch for HSYNC"]
pub type FP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSYNC_PARA_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - HSYNC active pulse width (in pixel clock cycles). Pulse width has a minimum value of 1."]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 11:19 - HSYNC back-porch pulse width (in pixel clock cycles). If zero, indicates no back-porch for HSYNC"]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bits 22:30 - HSYNC front-porch pulse width (in pixel clock cycles). If zero, indicates no front-porch for HSYNC"]
    #[inline(always)]
    pub fn fp(&self) -> FP_R {
        FP_R::new(((self.bits >> 22) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - HSYNC active pulse width (in pixel clock cycles). Pulse width has a minimum value of 1."]
    #[inline(always)]
    #[must_use]
    pub fn pw(&mut self) -> PW_W<0> {
        PW_W::new(self)
    }
    #[doc = "Bits 11:19 - HSYNC back-porch pulse width (in pixel clock cycles). If zero, indicates no back-porch for HSYNC"]
    #[inline(always)]
    #[must_use]
    pub fn bp(&mut self) -> BP_W<11> {
        BP_W::new(self)
    }
    #[doc = "Bits 22:30 - HSYNC front-porch pulse width (in pixel clock cycles). If zero, indicates no front-porch for HSYNC"]
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
#[doc = "HSYNC Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsync_para](index.html) module"]
pub struct HSYNC_PARA_SPEC;
impl crate::RegisterSpec for HSYNC_PARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsync_para::R](R) reader structure"]
impl crate::Readable for HSYNC_PARA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsync_para::W](W) writer structure"]
impl crate::Writable for HSYNC_PARA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSYNC_PARA to value 0"]
impl crate::Resettable for HSYNC_PARA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
