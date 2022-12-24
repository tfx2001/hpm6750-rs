#[doc = "Register `DI_GPIOB_VALUE` reader"]
pub struct R(crate::R<DI_GPIOB_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DI_GPIOB_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DI_GPIOB_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DI_GPIOB_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DI_GPIOB_VALUE` writer"]
pub struct W(crate::W<DI_GPIOB_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DI_GPIOB_VALUE_SPEC>;
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
impl From<crate::W<DI_GPIOB_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DI_GPIOB_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUT` reader - GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
pub type INPUT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INPUT` writer - GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
pub type INPUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DI_GPIOB_VALUE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    #[inline(always)]
    #[must_use]
    pub fn input(&mut self) -> INPUT_W<0> {
        INPUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOB input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [di_gpiob_value](index.html) module"]
pub struct DI_GPIOB_VALUE_SPEC;
impl crate::RegisterSpec for DI_GPIOB_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [di_gpiob_value::R](R) reader structure"]
impl crate::Readable for DI_GPIOB_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [di_gpiob_value::W](W) writer structure"]
impl crate::Writable for DI_GPIOB_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DI_GPIOB_VALUE to value 0"]
impl crate::Resettable for DI_GPIOB_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
