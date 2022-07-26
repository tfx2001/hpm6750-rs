#[doc = "Register `SOFTPKEY_SPK0` reader"]
pub struct R(crate::R<SOFTPKEY_SPK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFTPKEY_SPK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFTPKEY_SPK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFTPKEY_SPK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOFTPKEY_SPK0` writer"]
pub struct W(crate::W<SOFTPKEY_SPK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTPKEY_SPK0_SPEC>;
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
impl From<crate::W<SOFTPKEY_SPK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTPKEY_SPK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
pub type KEY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEY` writer - software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SOFTPKEY_SPK0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "system asymmetric key\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softpkey_spk0](index.html) module"]
pub struct SOFTPKEY_SPK0_SPEC;
impl crate::RegisterSpec for SOFTPKEY_SPK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [softpkey_spk0::R](R) reader structure"]
impl crate::Readable for SOFTPKEY_SPK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [softpkey_spk0::W](W) writer structure"]
impl crate::Writable for SOFTPKEY_SPK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOFTPKEY_SPK0 to value 0"]
impl crate::Resettable for SOFTPKEY_SPK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
