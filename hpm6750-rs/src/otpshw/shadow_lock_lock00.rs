#[doc = "Register `SHADOW_LOCK_LOCK00` reader"]
pub struct R(crate::R<SHADOW_LOCK_LOCK00_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHADOW_LOCK_LOCK00_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHADOW_LOCK_LOCK00_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHADOW_LOCK_LOCK00_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHADOW_LOCK_LOCK00` writer"]
pub struct W(crate::W<SHADOW_LOCK_LOCK00_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHADOW_LOCK_LOCK00_SPEC>;
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
impl From<crate::W<SHADOW_LOCK_LOCK00_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHADOW_LOCK_LOCK00_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - lock for pmic part shadow registers, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked"]
pub type LOCK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOCK` writer - lock for pmic part shadow registers, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked"]
pub type LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHADOW_LOCK_LOCK00_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - lock for pmic part shadow registers, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - lock for pmic part shadow registers, 2 bits per 32 bit word, lock behavior is different between different fuse types 00: not locked 01: soft locked 10: not locked, and cannot lock in furture 11: double locked"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fuse shadow lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shadow_lock_lock00](index.html) module"]
pub struct SHADOW_LOCK_LOCK00_SPEC;
impl crate::RegisterSpec for SHADOW_LOCK_LOCK00_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shadow_lock_lock00::R](R) reader structure"]
impl crate::Readable for SHADOW_LOCK_LOCK00_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shadow_lock_lock00::W](W) writer structure"]
impl crate::Writable for SHADOW_LOCK_LOCK00_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHADOW_LOCK_LOCK00 to value 0"]
impl crate::Resettable for SHADOW_LOCK_LOCK00_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
