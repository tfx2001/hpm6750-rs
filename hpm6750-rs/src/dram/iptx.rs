#[doc = "Register `IPTX` reader"]
pub struct R(crate::R<IPTX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPTX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPTX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPTX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPTX` writer"]
pub struct W(crate::W<IPTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPTX_SPEC>;
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
impl From<crate::W<IPTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT` reader - Data"]
pub type DAT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DAT` writer - Data"]
pub type DAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPTX_SPEC, u32, u32, 32, O>;
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
#[doc = "TX DATA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptx](index.html) module"]
pub struct IPTX_SPEC;
impl crate::RegisterSpec for IPTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iptx::R](R) reader structure"]
impl crate::Readable for IPTX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iptx::W](W) writer structure"]
impl crate::Writable for IPTX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPTX to value 0"]
impl crate::Resettable for IPTX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
