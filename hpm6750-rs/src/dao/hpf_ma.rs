#[doc = "Register `HPF_MA` reader"]
pub struct R(crate::R<HPF_MA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPF_MA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPF_MA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPF_MA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPF_MA` writer"]
pub struct W(crate::W<HPF_MA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPF_MA_SPEC>;
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
impl From<crate::W<HPF_MA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPF_MA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COEF` reader - Composite value of coef A of the Order-1 HPF"]
pub type COEF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COEF` writer - Composite value of coef A of the Order-1 HPF"]
pub type COEF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPF_MA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Composite value of coef A of the Order-1 HPF"]
    #[inline(always)]
    pub fn coef(&self) -> COEF_R {
        COEF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Composite value of coef A of the Order-1 HPF"]
    #[inline(always)]
    #[must_use]
    pub fn coef(&mut self) -> COEF_W<0> {
        COEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HPF A Coef Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hpf_ma](index.html) module"]
pub struct HPF_MA_SPEC;
impl crate::RegisterSpec for HPF_MA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hpf_ma::R](R) reader structure"]
impl crate::Readable for HPF_MA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hpf_ma::W](W) writer structure"]
impl crate::Writable for HPF_MA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPF_MA to value 0"]
impl crate::Resettable for HPF_MA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
