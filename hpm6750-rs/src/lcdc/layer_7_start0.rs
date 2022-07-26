#[doc = "Register `LAYER_7_START0` reader"]
pub struct R(crate::R<LAYER_7_START0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYER_7_START0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYER_7_START0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYER_7_START0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYER_7_START0` writer"]
pub struct W(crate::W<LAYER_7_START0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYER_7_START0_SPEC>;
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
impl From<crate::W<LAYER_7_START0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYER_7_START0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR0` reader - Input buffer Start address 0"]
pub type ADDR0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR0` writer - Input buffer Start address 0"]
pub type ADDR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_7_START0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Input buffer Start address 0"]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input buffer Start address 0"]
    #[inline(always)]
    pub fn addr0(&mut self) -> ADDR0_W<0> {
        ADDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer Buffer Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layer_7_start0](index.html) module"]
pub struct LAYER_7_START0_SPEC;
impl crate::RegisterSpec for LAYER_7_START0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layer_7_start0::R](R) reader structure"]
impl crate::Readable for LAYER_7_START0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layer_7_start0::W](W) writer structure"]
impl crate::Writable for LAYER_7_START0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LAYER_7_START0 to value 0"]
impl crate::Resettable for LAYER_7_START0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
