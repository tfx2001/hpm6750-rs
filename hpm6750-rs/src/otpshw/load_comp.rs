#[doc = "Register `LOAD_COMP` reader"]
pub struct R(crate::R<LOAD_COMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOAD_COMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOAD_COMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOAD_COMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOAD_COMP` writer"]
pub struct W(crate::W<LOAD_COMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOAD_COMP_SPEC>;
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
impl From<crate::W<LOAD_COMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOAD_COMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPLETE` reader - reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3"]
pub type COMPLETE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPLETE` writer - reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3"]
pub type COMPLETE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOAD_COMP_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3"]
    #[inline(always)]
    pub fn complete(&self) -> COMPLETE_R {
        COMPLETE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - reload complete sign for 4 regions bit0: region 0 bit1: region1 bit2: region2 bit3: region3"]
    #[inline(always)]
    #[must_use]
    pub fn complete(&mut self) -> COMPLETE_W<0> {
        COMPLETE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LOAD complete\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load_comp](index.html) module"]
pub struct LOAD_COMP_SPEC;
impl crate::RegisterSpec for LOAD_COMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [load_comp::R](R) reader structure"]
impl crate::Readable for LOAD_COMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [load_comp::W](W) writer structure"]
impl crate::Writable for LOAD_COMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOAD_COMP to value 0x07"]
impl crate::Resettable for LOAD_COMP_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
