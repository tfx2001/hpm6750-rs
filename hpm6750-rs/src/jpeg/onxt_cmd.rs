#[doc = "Register `ONXT_CMD` reader"]
pub struct R(crate::R<ONXT_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ONXT_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ONXT_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ONXT_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ONXT_CMD` writer"]
pub struct W(crate::W<ONXT_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ONXT_CMD_SPEC>;
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
impl From<crate::W<ONXT_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ONXT_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - NXTCMD phase Enable Bit"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - NXTCMD phase Enable Bit"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ONXT_CMD_SPEC, bool, O>;
#[doc = "Field `OP_VALID` reader - asserted if there is either a DATA DMA phase or NXTCMD phase. Automatically cleared. Will trigger the OutDMA and NXTCMD phase transfer if CFG\\[JPEG_EN\\]
is 1."]
pub type OP_VALID_R = crate::BitReader<bool>;
#[doc = "Field `OP_VALID` writer - asserted if there is either a DATA DMA phase or NXTCMD phase. Automatically cleared. Will trigger the OutDMA and NXTCMD phase transfer if CFG\\[JPEG_EN\\]
is 1."]
pub type OP_VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, ONXT_CMD_SPEC, bool, O>;
#[doc = "Field `ADDR` reader - The address pointing to the next command"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - The address pointing to the next command"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ONXT_CMD_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - NXTCMD phase Enable Bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - asserted if there is either a DATA DMA phase or NXTCMD phase. Automatically cleared. Will trigger the OutDMA and NXTCMD phase transfer if CFG\\[JPEG_EN\\]
is 1."]
    #[inline(always)]
    pub fn op_valid(&self) -> OP_VALID_R {
        OP_VALID_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - The address pointing to the next command"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - NXTCMD phase Enable Bit"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - asserted if there is either a DATA DMA phase or NXTCMD phase. Automatically cleared. Will trigger the OutDMA and NXTCMD phase transfer if CFG\\[JPEG_EN\\]
is 1."]
    #[inline(always)]
    #[must_use]
    pub fn op_valid(&mut self) -> OP_VALID_W<1> {
        OP_VALID_W::new(self)
    }
    #[doc = "Bits 2:31 - The address pointing to the next command"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<2> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Out DMA Next Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [onxt_cmd](index.html) module"]
pub struct ONXT_CMD_SPEC;
impl crate::RegisterSpec for ONXT_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [onxt_cmd::R](R) reader structure"]
impl crate::Readable for ONXT_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [onxt_cmd::W](W) writer structure"]
impl crate::Writable for ONXT_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ONXT_CMD to value 0"]
impl crate::Resettable for ONXT_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
