#[doc = "Register `OE_GPIOB_SET` reader"]
pub struct R(crate::R<OE_GPIOB_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OE_GPIOB_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OE_GPIOB_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OE_GPIOB_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OE_GPIOB_SET` writer"]
pub struct W(crate::W<OE_GPIOB_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OE_GPIOB_SET_SPEC>;
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
impl From<crate::W<OE_GPIOB_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OE_GPIOB_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIRECTION` reader - GPIO direction, each bit represents a bus bit 0: input 1: output"]
pub type DIRECTION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIRECTION` writer - GPIO direction, each bit represents a bus bit 0: input 1: output"]
pub type DIRECTION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OE_GPIOB_SET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - GPIO direction, each bit represents a bus bit 0: input 1: output"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO direction, each bit represents a bus bit 0: input 1: output"]
    #[inline(always)]
    #[must_use]
    pub fn direction(&mut self) -> DIRECTION_W<0> {
        DIRECTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO direction set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oe_gpiob_set](index.html) module"]
pub struct OE_GPIOB_SET_SPEC;
impl crate::RegisterSpec for OE_GPIOB_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oe_gpiob_set::R](R) reader structure"]
impl crate::Readable for OE_GPIOB_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oe_gpiob_set::W](W) writer structure"]
impl crate::Writable for OE_GPIOB_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OE_GPIOB_SET to value 0"]
impl crate::Resettable for OE_GPIOB_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
