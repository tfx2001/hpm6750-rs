#[doc = "Register `RXDSLOT_DATA3` reader"]
pub struct R(crate::R<RXDSLOT_DATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDSLOT_DATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDSLOT_DATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDSLOT_DATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDSLOT_DATA3` writer"]
pub struct W(crate::W<RXDSLOT_DATA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDSLOT_DATA3_SPEC>;
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
impl From<crate::W<RXDSLOT_DATA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDSLOT_DATA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - No description avaiable"]
pub type EN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EN` writer - No description avaiable"]
pub type EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXDSLOT_DATA3_SPEC, u32, u32, 32, O>;
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
    #[must_use]
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
#[doc = "Rx Slots Enable for Rx Data3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdslot_data3](index.html) module"]
pub struct RXDSLOT_DATA3_SPEC;
impl crate::RegisterSpec for RXDSLOT_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdslot_data3::R](R) reader structure"]
impl crate::Readable for RXDSLOT_DATA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdslot_data3::W](W) writer structure"]
impl crate::Writable for RXDSLOT_DATA3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXDSLOT_DATA3 to value 0xffff"]
impl crate::Resettable for RXDSLOT_DATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
