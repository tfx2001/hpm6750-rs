#[doc = "Register `GROUP1_2_SET` reader"]
pub struct R(crate::R<GROUP1_2_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GROUP1_2_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GROUP1_2_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GROUP1_2_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GROUP1_2_SET` writer"]
pub struct W(crate::W<GROUP1_2_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GROUP1_2_SET_SPEC>;
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
impl From<crate::W<GROUP1_2_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GROUP1_2_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINK` reader - denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
pub type LINK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LINK` writer - denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
pub type LINK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GROUP1_2_SET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LINK_W<0> {
        LINK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Goup setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [group1_2_set](index.html) module"]
pub struct GROUP1_2_SET_SPEC;
impl crate::RegisterSpec for GROUP1_2_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [group1_2_set::R](R) reader structure"]
impl crate::Readable for GROUP1_2_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [group1_2_set::W](W) writer structure"]
impl crate::Writable for GROUP1_2_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GROUP1_2_SET to value 0"]
impl crate::Resettable for GROUP1_2_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
