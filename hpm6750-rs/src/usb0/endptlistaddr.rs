#[doc = "Register `ENDPTLISTADDR` reader"]
pub struct R(crate::R<ENDPTLISTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPTLISTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPTLISTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPTLISTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPTLISTADDR` writer"]
pub struct W(crate::W<ENDPTLISTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDPTLISTADDR_SPEC>;
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
impl From<crate::W<ENDPTLISTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDPTLISTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPBASE` reader - EPBASE Endpoint List Pointer(Low). These bits correspond to memory address signals \\[31:11\\], respectively. This field will reference a list of up to 32 Queue Head (QH) (that is, one queue head per endpoint & direction)."]
pub type EPBASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EPBASE` writer - EPBASE Endpoint List Pointer(Low). These bits correspond to memory address signals \\[31:11\\], respectively. This field will reference a list of up to 32 Queue Head (QH) (that is, one queue head per endpoint & direction)."]
pub type EPBASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENDPTLISTADDR_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 11:31 - EPBASE Endpoint List Pointer(Low). These bits correspond to memory address signals \\[31:11\\], respectively. This field will reference a list of up to 32 Queue Head (QH) (that is, one queue head per endpoint & direction)."]
    #[inline(always)]
    pub fn epbase(&self) -> EPBASE_R {
        EPBASE_R::new(((self.bits >> 11) & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 11:31 - EPBASE Endpoint List Pointer(Low). These bits correspond to memory address signals \\[31:11\\], respectively. This field will reference a list of up to 32 Queue Head (QH) (that is, one queue head per endpoint & direction)."]
    #[inline(always)]
    pub fn epbase(&mut self) -> EPBASE_W<11> {
        EPBASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint List Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endptlistaddr](index.html) module"]
pub struct ENDPTLISTADDR_SPEC;
impl crate::RegisterSpec for ENDPTLISTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [endptlistaddr::R](R) reader structure"]
impl crate::Readable for ENDPTLISTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endptlistaddr::W](W) writer structure"]
impl crate::Writable for ENDPTLISTADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENDPTLISTADDR to value 0"]
impl crate::Resettable for ENDPTLISTADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
