#[doc = "Register `LIMIT` reader"]
pub struct R(crate::R<LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIMIT` writer"]
pub struct W(crate::W<LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIMIT_SPEC>;
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
impl From<crate::W<LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWL` reader - Programmable Error Warning Limit = (EWL+1)*8. Possible Limit values: 8, 16, … 128. The value of EWL controls EIF."]
pub type EWL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EWL` writer - Programmable Error Warning Limit = (EWL+1)*8. Possible Limit values: 8, 16, … 128. The value of EWL controls EIF."]
pub type EWL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LIMIT_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFWL` reader - receive buffer Almost Full Warning Limit AFWL defines the internal warning limit AFWL_i with being the number of availableRB slots. AFWL_i is compared to the number of filled RB slots and triggers RAFIF if equal. Thevalid range of . AFWL = 0 is meaningless and automatically treated as 0x1. (Note that AFWL is meant in this rule and not AFWL_i.) AFWL_i > nRB is meaningless and automatically treated as nRB. AFWL_i = nRB is a valid value, but note that RFIF also exists."]
pub type AFWL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFWL` writer - receive buffer Almost Full Warning Limit AFWL defines the internal warning limit AFWL_i with being the number of availableRB slots. AFWL_i is compared to the number of filled RB slots and triggers RAFIF if equal. Thevalid range of . AFWL = 0 is meaningless and automatically treated as 0x1. (Note that AFWL is meant in this rule and not AFWL_i.) AFWL_i > nRB is meaningless and automatically treated as nRB. AFWL_i = nRB is a valid value, but note that RFIF also exists."]
pub type AFWL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LIMIT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Programmable Error Warning Limit = (EWL+1)*8. Possible Limit values: 8, 16, … 128. The value of EWL controls EIF."]
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - receive buffer Almost Full Warning Limit AFWL defines the internal warning limit AFWL_i with being the number of availableRB slots. AFWL_i is compared to the number of filled RB slots and triggers RAFIF if equal. Thevalid range of . AFWL = 0 is meaningless and automatically treated as 0x1. (Note that AFWL is meant in this rule and not AFWL_i.) AFWL_i > nRB is meaningless and automatically treated as nRB. AFWL_i = nRB is a valid value, but note that RFIF also exists."]
    #[inline(always)]
    pub fn afwl(&self) -> AFWL_R {
        AFWL_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Programmable Error Warning Limit = (EWL+1)*8. Possible Limit values: 8, 16, … 128. The value of EWL controls EIF."]
    #[inline(always)]
    #[must_use]
    pub fn ewl(&mut self) -> EWL_W<0> {
        EWL_W::new(self)
    }
    #[doc = "Bits 4:7 - receive buffer Almost Full Warning Limit AFWL defines the internal warning limit AFWL_i with being the number of availableRB slots. AFWL_i is compared to the number of filled RB slots and triggers RAFIF if equal. Thevalid range of . AFWL = 0 is meaningless and automatically treated as 0x1. (Note that AFWL is meant in this rule and not AFWL_i.) AFWL_i > nRB is meaningless and automatically treated as nRB. AFWL_i = nRB is a valid value, but note that RFIF also exists."]
    #[inline(always)]
    #[must_use]
    pub fn afwl(&mut self) -> AFWL_W<4> {
        AFWL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Warning Limits Register LIMIT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limit](index.html) module"]
pub struct LIMIT_SPEC;
impl crate::RegisterSpec for LIMIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [limit::R](R) reader structure"]
impl crate::Readable for LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [limit::W](W) writer structure"]
impl crate::Writable for LIMIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LIMIT to value 0x1b"]
impl crate::Resettable for LIMIT_SPEC {
    const RESET_VALUE: Self::Ux = 0x1b;
}
