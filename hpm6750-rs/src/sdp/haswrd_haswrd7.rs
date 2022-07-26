#[doc = "Register `HASWRD_HASWRD7` reader"]
pub struct R(crate::R<HASWRD_HASWRD7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASWRD_HASWRD7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASWRD_HASWRD7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASWRD_HASWRD7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASWRD_HASWRD7` writer"]
pub struct W(crate::W<HASWRD_HASWRD7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASWRD_HASWRD7_SPEC>;
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
impl From<crate::W<HASWRD_HASWRD7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASWRD_HASWRD7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HASWRD` reader - Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\]
here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
pub type HASWRD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HASWRD` writer - Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\]
here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
pub type HASWRD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HASWRD_HASWRD7_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\]
here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
    #[inline(always)]
    pub fn haswrd(&self) -> HASWRD_R {
        HASWRD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\]
here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
    #[inline(always)]
    pub fn haswrd(&mut self) -> HASWRD_W<0> {
        HASWRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Data Word 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haswrd_haswrd7](index.html) module"]
pub struct HASWRD_HASWRD7_SPEC;
impl crate::RegisterSpec for HASWRD_HASWRD7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [haswrd_haswrd7::R](R) reader structure"]
impl crate::Readable for HASWRD_HASWRD7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [haswrd_haswrd7::W](W) writer structure"]
impl crate::Writable for HASWRD_HASWRD7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASWRD_HASWRD7 to value 0x30"]
impl crate::Resettable for HASWRD_HASWRD7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
