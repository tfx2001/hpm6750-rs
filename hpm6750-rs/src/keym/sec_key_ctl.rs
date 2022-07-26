#[doc = "Register `SEC_KEY_CTL` reader"]
pub struct R(crate::R<SEC_KEY_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_KEY_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_KEY_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_KEY_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_KEY_CTL` writer"]
pub struct W(crate::W<SEC_KEY_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_KEY_CTL_SPEC>;
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
impl From<crate::W<SEC_KEY_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_KEY_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_SEC_CTL` reader - block secure state key setting being changed"]
pub type LOCK_SEC_CTL_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_SEC_CTL` writer - block secure state key setting being changed"]
pub type LOCK_SEC_CTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEC_KEY_CTL_SPEC, bool, O>;
#[doc = "Field `SK_VAL` reader - session key valid 0: session key is all 0's and not usable 1: session key is valid"]
pub type SK_VAL_R = crate::BitReader<bool>;
#[doc = "Field `SMK_SEL` reader - software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key"]
pub type SMK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SMK_SEL` writer - software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key"]
pub type SMK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEC_KEY_CTL_SPEC, bool, O>;
#[doc = "Field `ZMK_SEL` reader - batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
pub type ZMK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `ZMK_SEL` writer - batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
pub type ZMK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEC_KEY_CTL_SPEC, bool, O>;
#[doc = "Field `FMK_SEL` reader - fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key"]
pub type FMK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `FMK_SEL` writer - fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key"]
pub type FMK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEC_KEY_CTL_SPEC, bool, O>;
#[doc = "Field `KEY_SEL` reader - secure symmtric key synthesize setting, key is a XOR of followings bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected"]
pub type KEY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_SEL` writer - secure symmtric key synthesize setting, key is a XOR of followings bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected"]
pub type KEY_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEC_KEY_CTL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 31 - block secure state key setting being changed"]
    #[inline(always)]
    pub fn lock_sec_ctl(&self) -> LOCK_SEC_CTL_R {
        LOCK_SEC_CTL_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 16 - session key valid 0: session key is all 0's and not usable 1: session key is valid"]
    #[inline(always)]
    pub fn sk_val(&self) -> SK_VAL_R {
        SK_VAL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 12 - software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key"]
    #[inline(always)]
    pub fn smk_sel(&self) -> SMK_SEL_R {
        SMK_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 8 - batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
    #[inline(always)]
    pub fn zmk_sel(&self) -> ZMK_SEL_R {
        ZMK_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 4 - fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key"]
    #[inline(always)]
    pub fn fmk_sel(&self) -> FMK_SEL_R {
        FMK_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:2 - secure symmtric key synthesize setting, key is a XOR of followings bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected"]
    #[inline(always)]
    pub fn key_sel(&self) -> KEY_SEL_R {
        KEY_SEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - block secure state key setting being changed"]
    #[inline(always)]
    pub fn lock_sec_ctl(&mut self) -> LOCK_SEC_CTL_W<31> {
        LOCK_SEC_CTL_W::new(self)
    }
    #[doc = "Bit 12 - software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key"]
    #[inline(always)]
    pub fn smk_sel(&mut self) -> SMK_SEL_W<12> {
        SMK_SEL_W::new(self)
    }
    #[doc = "Bit 8 - batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
    #[inline(always)]
    pub fn zmk_sel(&mut self) -> ZMK_SEL_W<8> {
        ZMK_SEL_W::new(self)
    }
    #[doc = "Bit 4 - fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key"]
    #[inline(always)]
    pub fn fmk_sel(&mut self) -> FMK_SEL_W<4> {
        FMK_SEL_W::new(self)
    }
    #[doc = "Bits 0:2 - secure symmtric key synthesize setting, key is a XOR of followings bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected"]
    #[inline(always)]
    pub fn key_sel(&mut self) -> KEY_SEL_W<0> {
        KEY_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "secure key generation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_key_ctl](index.html) module"]
pub struct SEC_KEY_CTL_SPEC;
impl crate::RegisterSpec for SEC_KEY_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_key_ctl::R](R) reader structure"]
impl crate::Readable for SEC_KEY_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_key_ctl::W](W) writer structure"]
impl crate::Writable for SEC_KEY_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEC_KEY_CTL to value 0"]
impl crate::Resettable for SEC_KEY_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
