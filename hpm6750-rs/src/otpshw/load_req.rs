#[doc = "Register `LOAD_REQ` reader"]
pub struct R(crate::R<LOAD_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOAD_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOAD_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOAD_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOAD_REQ` writer"]
pub struct W(crate::W<LOAD_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOAD_REQ_SPEC>;
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
impl From<crate::W<LOAD_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOAD_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQUEST` reader - reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3"]
pub type REQUEST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REQUEST` writer - reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3"]
pub type REQUEST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOAD_REQ_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3"]
    #[inline(always)]
    pub fn request(&self) -> REQUEST_R {
        REQUEST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - reload request for 4 regions bit0: region0 bit1: region1 bit2: region2 bit3: region3"]
    #[inline(always)]
    pub fn request(&mut self) -> REQUEST_W<0> {
        REQUEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LOAD Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load_req](index.html) module"]
pub struct LOAD_REQ_SPEC;
impl crate::RegisterSpec for LOAD_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [load_req::R](R) reader structure"]
impl crate::Readable for LOAD_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [load_req::W](W) writer structure"]
impl crate::Writable for LOAD_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOAD_REQ to value 0x07"]
impl crate::Resettable for LOAD_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
