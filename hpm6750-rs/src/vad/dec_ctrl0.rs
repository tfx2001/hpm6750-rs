#[doc = "Register `DEC_CTRL0` reader"]
pub struct R(crate::R<DEC_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEC_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEC_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEC_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEC_CTRL0` writer"]
pub struct W(crate::W<DEC_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEC_CTRL0_SPEC>;
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
impl From<crate::W<DEC_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEC_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBBLK_LEN` reader - length of sub-block"]
pub type SUBBLK_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SUBBLK_LEN` writer - length of sub-block"]
pub type SUBBLK_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEC_CTRL0_SPEC, u16, u16, 9, O>;
#[doc = "Field `BLK_CFG` reader - asserted to have 3 sub-blocks, otherwise to have 2 sub-blocks"]
pub type BLK_CFG_R = crate::BitReader<bool>;
#[doc = "Field `BLK_CFG` writer - asserted to have 3 sub-blocks, otherwise to have 2 sub-blocks"]
pub type BLK_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEC_CTRL0_SPEC, bool, O>;
#[doc = "Field `NOISE_TOL` reader - the value of amplitude for noise determination when calculationg ZCR"]
pub type NOISE_TOL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NOISE_TOL` writer - the value of amplitude for noise determination when calculationg ZCR"]
pub type NOISE_TOL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEC_CTRL0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:8 - length of sub-block"]
    #[inline(always)]
    pub fn subblk_len(&self) -> SUBBLK_LEN_R {
        SUBBLK_LEN_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - asserted to have 3 sub-blocks, otherwise to have 2 sub-blocks"]
    #[inline(always)]
    pub fn blk_cfg(&self) -> BLK_CFG_R {
        BLK_CFG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:31 - the value of amplitude for noise determination when calculationg ZCR"]
    #[inline(always)]
    pub fn noise_tol(&self) -> NOISE_TOL_R {
        NOISE_TOL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - length of sub-block"]
    #[inline(always)]
    #[must_use]
    pub fn subblk_len(&mut self) -> SUBBLK_LEN_W<0> {
        SUBBLK_LEN_W::new(self)
    }
    #[doc = "Bit 9 - asserted to have 3 sub-blocks, otherwise to have 2 sub-blocks"]
    #[inline(always)]
    #[must_use]
    pub fn blk_cfg(&mut self) -> BLK_CFG_W<9> {
        BLK_CFG_W::new(self)
    }
    #[doc = "Bits 16:31 - the value of amplitude for noise determination when calculationg ZCR"]
    #[inline(always)]
    #[must_use]
    pub fn noise_tol(&mut self) -> NOISE_TOL_W<16> {
        NOISE_TOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Decision Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dec_ctrl0](index.html) module"]
pub struct DEC_CTRL0_SPEC;
impl crate::RegisterSpec for DEC_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dec_ctrl0::R](R) reader structure"]
impl crate::Readable for DEC_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dec_ctrl0::W](W) writer structure"]
impl crate::Writable for DEC_CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEC_CTRL0 to value 0"]
impl crate::Resettable for DEC_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
