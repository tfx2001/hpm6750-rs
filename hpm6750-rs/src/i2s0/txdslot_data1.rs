#[doc = "Register `TXDSLOT_DATA1` reader"]
pub struct R(crate::R<TXDSLOT_DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXDSLOT_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXDSLOT_DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXDSLOT_DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDSLOT_DATA1` writer"]
pub struct W(crate::W<TXDSLOT_DATA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXDSLOT_DATA1_SPEC>;
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
impl From<crate::W<TXDSLOT_DATA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXDSLOT_DATA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - No description avaiable"]
pub type EN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EN` writer - No description avaiable"]
pub type EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXDSLOT_DATA1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Slots Enable for Tx Data1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdslot_data1](index.html) module"]
pub struct TXDSLOT_DATA1_SPEC;
impl crate::RegisterSpec for TXDSLOT_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txdslot_data1::R](R) reader structure"]
impl crate::Readable for TXDSLOT_DATA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdslot_data1::W](W) writer structure"]
impl crate::Writable for TXDSLOT_DATA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDSLOT_DATA1 to value 0xffff"]
impl crate::Resettable for TXDSLOT_DATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
