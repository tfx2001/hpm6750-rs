#[doc = "Register `BYTEMSK` reader"]
pub struct R(crate::R<BYTEMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BYTEMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BYTEMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BYTEMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BYTEMSK` writer"]
pub struct W(crate::W<BYTEMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BYTEMSK_SPEC>;
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
impl From<crate::W<BYTEMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BYTEMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BM0` reader - Byte Mask for Byte 0 (IPTXD bit 7:0) 0b - Byte Unmasked 1b - Byte Masked"]
pub type BM0_R = crate::BitReader<bool>;
#[doc = "Field `BM0` writer - Byte Mask for Byte 0 (IPTXD bit 7:0) 0b - Byte Unmasked 1b - Byte Masked"]
pub type BM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BYTEMSK_SPEC, bool, O>;
#[doc = "Field `BM1` reader - Byte Mask for Byte 1 (IPTXD bit 15:8) 0b - Byte Unmasked 1b - Byte Masked"]
pub type BM1_R = crate::BitReader<bool>;
#[doc = "Field `BM1` writer - Byte Mask for Byte 1 (IPTXD bit 15:8) 0b - Byte Unmasked 1b - Byte Masked"]
pub type BM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BYTEMSK_SPEC, bool, O>;
#[doc = "Field `BM2` reader - Byte Mask for Byte 2 (IPTXD bit 23:16) 0b - Byte Unmasked 1b - Byte Masked"]
pub type BM2_R = crate::BitReader<bool>;
#[doc = "Field `BM2` writer - Byte Mask for Byte 2 (IPTXD bit 23:16) 0b - Byte Unmasked 1b - Byte Masked"]
pub type BM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BYTEMSK_SPEC, bool, O>;
#[doc = "Field `BM3` reader - Byte Mask for Byte 3 (IPTXD bit 31:24) 0b - Byte Unmasked 1b - Byte Masked"]
pub type BM3_R = crate::BitReader<bool>;
#[doc = "Field `BM3` writer - Byte Mask for Byte 3 (IPTXD bit 31:24) 0b - Byte Unmasked 1b - Byte Masked"]
pub type BM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BYTEMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Byte Mask for Byte 0 (IPTXD bit 7:0) 0b - Byte Unmasked 1b - Byte Masked"]
    #[inline(always)]
    pub fn bm0(&self) -> BM0_R {
        BM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Byte Mask for Byte 1 (IPTXD bit 15:8) 0b - Byte Unmasked 1b - Byte Masked"]
    #[inline(always)]
    pub fn bm1(&self) -> BM1_R {
        BM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte Mask for Byte 2 (IPTXD bit 23:16) 0b - Byte Unmasked 1b - Byte Masked"]
    #[inline(always)]
    pub fn bm2(&self) -> BM2_R {
        BM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Byte Mask for Byte 3 (IPTXD bit 31:24) 0b - Byte Unmasked 1b - Byte Masked"]
    #[inline(always)]
    pub fn bm3(&self) -> BM3_R {
        BM3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Byte Mask for Byte 0 (IPTXD bit 7:0) 0b - Byte Unmasked 1b - Byte Masked"]
    #[inline(always)]
    #[must_use]
    pub fn bm0(&mut self) -> BM0_W<0> {
        BM0_W::new(self)
    }
    #[doc = "Bit 1 - Byte Mask for Byte 1 (IPTXD bit 15:8) 0b - Byte Unmasked 1b - Byte Masked"]
    #[inline(always)]
    #[must_use]
    pub fn bm1(&mut self) -> BM1_W<1> {
        BM1_W::new(self)
    }
    #[doc = "Bit 2 - Byte Mask for Byte 2 (IPTXD bit 23:16) 0b - Byte Unmasked 1b - Byte Masked"]
    #[inline(always)]
    #[must_use]
    pub fn bm2(&mut self) -> BM2_W<2> {
        BM2_W::new(self)
    }
    #[doc = "Bit 3 - Byte Mask for Byte 3 (IPTXD bit 31:24) 0b - Byte Unmasked 1b - Byte Masked"]
    #[inline(always)]
    #[must_use]
    pub fn bm3(&mut self) -> BM3_W<3> {
        BM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Command Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bytemsk](index.html) module"]
pub struct BYTEMSK_SPEC;
impl crate::RegisterSpec for BYTEMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bytemsk::R](R) reader structure"]
impl crate::Readable for BYTEMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bytemsk::W](W) writer structure"]
impl crate::Writable for BYTEMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BYTEMSK to value 0"]
impl crate::Resettable for BYTEMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
