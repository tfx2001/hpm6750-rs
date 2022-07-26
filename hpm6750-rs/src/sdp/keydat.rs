#[doc = "Register `KEYDAT` reader"]
pub struct R(crate::R<KEYDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYDAT` writer"]
pub struct W(crate::W<KEYDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYDAT_SPEC>;
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
impl From<crate::W<KEYDAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYDAT` reader - This register provides the write access to the key/key subword specified by the key index register. Writing this location updates the selected subword for the key located at the index specified by the key index register. The write also triggers the SUBWORD field of the KEY register to increment to the next higher word in the key"]
pub type KEYDAT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEYDAT` writer - This register provides the write access to the key/key subword specified by the key index register. Writing this location updates the selected subword for the key located at the index specified by the key index register. The write also triggers the SUBWORD field of the KEY register to increment to the next higher word in the key"]
pub type KEYDAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYDAT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - This register provides the write access to the key/key subword specified by the key index register. Writing this location updates the selected subword for the key located at the index specified by the key index register. The write also triggers the SUBWORD field of the KEY register to increment to the next higher word in the key"]
    #[inline(always)]
    pub fn keydat(&self) -> KEYDAT_R {
        KEYDAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register provides the write access to the key/key subword specified by the key index register. Writing this location updates the selected subword for the key located at the index specified by the key index register. The write also triggers the SUBWORD field of the KEY register to increment to the next higher word in the key"]
    #[inline(always)]
    pub fn keydat(&mut self) -> KEYDAT_W<0> {
        KEYDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keydat](index.html) module"]
pub struct KEYDAT_SPEC;
impl crate::RegisterSpec for KEYDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keydat::R](R) reader structure"]
impl crate::Readable for KEYDAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keydat::W](W) writer structure"]
impl crate::Writable for KEYDAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYDAT to value 0x30"]
impl crate::Resettable for KEYDAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
