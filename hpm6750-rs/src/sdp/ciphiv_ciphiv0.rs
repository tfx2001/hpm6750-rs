#[doc = "Register `CIPHIV_CIPHIV0` reader"]
pub struct R(crate::R<CIPHIV_CIPHIV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIPHIV_CIPHIV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIPHIV_CIPHIV0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIPHIV_CIPHIV0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIPHIV_CIPHIV0` writer"]
pub struct W(crate::W<CIPHIV_CIPHIV0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIPHIV_CIPHIV0_SPEC>;
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
impl From<crate::W<CIPHIV_CIPHIV0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIPHIV_CIPHIV0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CIPHIV` reader - cipher initialization vector."]
pub type CIPHIV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CIPHIV` writer - cipher initialization vector."]
pub type CIPHIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CIPHIV_CIPHIV0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - cipher initialization vector."]
    #[inline(always)]
    pub fn ciphiv(&self) -> CIPHIV_R {
        CIPHIV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cipher initialization vector."]
    #[inline(always)]
    pub fn ciphiv(&mut self) -> CIPHIV_W<0> {
        CIPHIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cipher Initializtion Vector 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ciphiv_ciphiv0](index.html) module"]
pub struct CIPHIV_CIPHIV0_SPEC;
impl crate::RegisterSpec for CIPHIV_CIPHIV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ciphiv_ciphiv0::R](R) reader structure"]
impl crate::Readable for CIPHIV_CIPHIV0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ciphiv_ciphiv0::W](W) writer structure"]
impl crate::Writable for CIPHIV_CIPHIV0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIPHIV_CIPHIV0 to value 0x3200_0000"]
impl crate::Resettable for CIPHIV_CIPHIV0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3200_0000
    }
}
