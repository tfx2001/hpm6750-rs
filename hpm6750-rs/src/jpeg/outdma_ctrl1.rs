#[doc = "Register `OUTDMA_CTRL1` reader"]
pub struct R(crate::R<OUTDMA_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTDMA_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTDMA_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTDMA_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTDMA_CTRL1` writer"]
pub struct W(crate::W<OUTDMA_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTDMA_CTRL1_SPEC>;
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
impl From<crate::W<OUTDMA_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTDMA_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROWLEN` reader - Total length (High 16 bits) in Bytes -1 for transfer. See reference in OutDMA_Ctrl0\\[TTLEN\\]"]
pub type ROWLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ROWLEN` writer - Total length (High 16 bits) in Bytes -1 for transfer. See reference in OutDMA_Ctrl0\\[TTLEN\\]"]
pub type ROWLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTDMA_CTRL1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Total length (High 16 bits) in Bytes -1 for transfer. See reference in OutDMA_Ctrl0\\[TTLEN\\]"]
    #[inline(always)]
    pub fn rowlen(&self) -> ROWLEN_R {
        ROWLEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Total length (High 16 bits) in Bytes -1 for transfer. See reference in OutDMA_Ctrl0\\[TTLEN\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rowlen(&mut self) -> ROWLEN_W<0> {
        ROWLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Out DMA Buf Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outdma_ctrl1](index.html) module"]
pub struct OUTDMA_CTRL1_SPEC;
impl crate::RegisterSpec for OUTDMA_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outdma_ctrl1::R](R) reader structure"]
impl crate::Readable for OUTDMA_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outdma_ctrl1::W](W) writer structure"]
impl crate::Writable for OUTDMA_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTDMA_CTRL1 to value 0"]
impl crate::Resettable for OUTDMA_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
