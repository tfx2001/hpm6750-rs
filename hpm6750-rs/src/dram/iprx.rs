#[doc = "Register `IPRX` reader"]
pub struct R(crate::R<IPRX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPRX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPRX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPRX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPRX` writer"]
pub struct W(crate::W<IPRX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPRX_SPEC>;
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
impl From<crate::W<IPRX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPRX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT` reader - Data"]
pub type DAT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DAT` writer - Data"]
pub type DAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPRX_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn dat(&mut self) -> DAT_W<0> {
        DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX DATA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iprx](index.html) module"]
pub struct IPRX_SPEC;
impl crate::RegisterSpec for IPRX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iprx::R](R) reader structure"]
impl crate::Readable for IPRX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iprx::W](W) writer structure"]
impl crate::Writable for IPRX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPRX to value 0"]
impl crate::Resettable for IPRX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
