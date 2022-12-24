#[doc = "Register `UNLK` reader"]
pub struct R(crate::R<UNLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UNLK` writer"]
pub struct W(crate::W<UNLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNLK_SPEC>;
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
impl From<crate::W<UNLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHUNLK` reader - write 0xB0382607 to unlock the shadow registers of register offset from 0x04 to 0x78, otherwise the shadow registers can not be written."]
pub type SHUNLK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHUNLK` writer - write 0xB0382607 to unlock the shadow registers of register offset from 0x04 to 0x78, otherwise the shadow registers can not be written."]
pub type SHUNLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UNLK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - write 0xB0382607 to unlock the shadow registers of register offset from 0x04 to 0x78, otherwise the shadow registers can not be written."]
    #[inline(always)]
    pub fn shunlk(&self) -> SHUNLK_R {
        SHUNLK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - write 0xB0382607 to unlock the shadow registers of register offset from 0x04 to 0x78, otherwise the shadow registers can not be written."]
    #[inline(always)]
    #[must_use]
    pub fn shunlk(&mut self) -> SHUNLK_W<0> {
        SHUNLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow registers unlock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unlk](index.html) module"]
pub struct UNLK_SPEC;
impl crate::RegisterSpec for UNLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [unlk::R](R) reader structure"]
impl crate::Readable for UNLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [unlk::W](W) writer structure"]
impl crate::Writable for UNLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UNLK to value 0"]
impl crate::Resettable for UNLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
