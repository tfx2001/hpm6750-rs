#[doc = "Register `LAYER_0_LINECFG` reader"]
pub struct R(crate::R<LAYER_0_LINECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYER_0_LINECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYER_0_LINECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYER_0_LINECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYER_0_LINECFG` writer"]
pub struct W(crate::W<LAYER_0_LINECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYER_0_LINECFG_SPEC>;
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
impl From<crate::W<LAYER_0_LINECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYER_0_LINECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PITCH` reader - Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry."]
pub type PITCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PITCH` writer - Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry."]
pub type PITCH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_0_LINECFG_SPEC, u16, u16, 16, O>;
#[doc = "Field `MAX_OT` reader - the number of outstanding axi read transactions. If zero, it means max 8."]
pub type MAX_OT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_OT` writer - the number of outstanding axi read transactions. If zero, it means max 8."]
pub type MAX_OT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_0_LINECFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `MPT_SIZE` reader - Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes"]
pub type MPT_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MPT_SIZE` writer - Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes"]
pub type MPT_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYER_0_LINECFG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:15 - Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry."]
    #[inline(always)]
    pub fn pitch(&self) -> PITCH_R {
        PITCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 21:23 - the number of outstanding axi read transactions. If zero, it means max 8."]
    #[inline(always)]
    pub fn max_ot(&self) -> MAX_OT_R {
        MAX_OT_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes"]
    #[inline(always)]
    pub fn mpt_size(&self) -> MPT_SIZE_R {
        MPT_SIZE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry."]
    #[inline(always)]
    #[must_use]
    pub fn pitch(&mut self) -> PITCH_W<0> {
        PITCH_W::new(self)
    }
    #[doc = "Bits 21:23 - the number of outstanding axi read transactions. If zero, it means max 8."]
    #[inline(always)]
    #[must_use]
    pub fn max_ot(&mut self) -> MAX_OT_W<21> {
        MAX_OT_W::new(self)
    }
    #[doc = "Bits 29:31 - Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn mpt_size(&mut self) -> MPT_SIZE_W<29> {
        MPT_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer Bus Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layer_0_linecfg](index.html) module"]
pub struct LAYER_0_LINECFG_SPEC;
impl crate::RegisterSpec for LAYER_0_LINECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layer_0_linecfg::R](R) reader structure"]
impl crate::Readable for LAYER_0_LINECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layer_0_linecfg::W](W) writer structure"]
impl crate::Writable for LAYER_0_LINECFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYER_0_LINECFG to value 0"]
impl crate::Resettable for LAYER_0_LINECFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
