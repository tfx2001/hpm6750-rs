#[doc = "Register `DISP_WN_SIZE` reader"]
pub struct R(crate::R<DISP_WN_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISP_WN_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISP_WN_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISP_WN_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DISP_WN_SIZE` writer"]
pub struct W(crate::W<DISP_WN_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISP_WN_SIZE_SPEC>;
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
impl From<crate::W<DISP_WN_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISP_WN_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X` reader - Sets the display size horizontal resolution in pixels."]
pub type X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `X` writer - Sets the display size horizontal resolution in pixels."]
pub type X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DISP_WN_SIZE_SPEC, u16, u16, 12, O>;
#[doc = "Field `Y` reader - Sets the display size vertical resolution in pixels."]
pub type Y_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Y` writer - Sets the display size vertical resolution in pixels."]
pub type Y_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DISP_WN_SIZE_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Sets the display size horizontal resolution in pixels."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Sets the display size vertical resolution in pixels."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Sets the display size horizontal resolution in pixels."]
    #[inline(always)]
    #[must_use]
    pub fn x(&mut self) -> X_W<0> {
        X_W::new(self)
    }
    #[doc = "Bits 16:27 - Sets the display size vertical resolution in pixels."]
    #[inline(always)]
    #[must_use]
    pub fn y(&mut self) -> Y_W<16> {
        Y_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Display Window Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [disp_wn_size](index.html) module"]
pub struct DISP_WN_SIZE_SPEC;
impl crate::RegisterSpec for DISP_WN_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [disp_wn_size::R](R) reader structure"]
impl crate::Readable for DISP_WN_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [disp_wn_size::W](W) writer structure"]
impl crate::Writable for DISP_WN_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DISP_WN_SIZE to value 0"]
impl crate::Resettable for DISP_WN_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
