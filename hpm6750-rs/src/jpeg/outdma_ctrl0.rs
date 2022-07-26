#[doc = "Register `OUTDMA_CTRL0` reader"]
pub struct R(crate::R<OUTDMA_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTDMA_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTDMA_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTDMA_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTDMA_CTRL0` writer"]
pub struct W(crate::W<OUTDMA_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTDMA_CTRL0_SPEC>;
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
impl From<crate::W<OUTDMA_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTDMA_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTLEN` reader - Total length (Low 16 bits) in Bytes -1 for transfer when Out_DMA_ID!=Pixel. If Out_DMA_ID=ECS, it can be any value greater than the length of the ECS, for example, the number of encoded bytes."]
pub type TTLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TTLEN` writer - Total length (Low 16 bits) in Bytes -1 for transfer when Out_DMA_ID!=Pixel. If Out_DMA_ID=ECS, it can be any value greater than the length of the ECS, for example, the number of encoded bytes."]
pub type TTLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTDMA_CTRL0_SPEC, u16, u16, 16, O>;
#[doc = "Field `PITCH` reader - Pitch between the starting point of Rows when Out_DMA_ID==Pixel"]
pub type PITCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PITCH` writer - Pitch between the starting point of Rows when Out_DMA_ID==Pixel"]
pub type PITCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTDMA_CTRL0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - Total length (Low 16 bits) in Bytes -1 for transfer when Out_DMA_ID!=Pixel. If Out_DMA_ID=ECS, it can be any value greater than the length of the ECS, for example, the number of encoded bytes."]
    #[inline(always)]
    pub fn ttlen(&self) -> TTLEN_R {
        TTLEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Pitch between the starting point of Rows when Out_DMA_ID==Pixel"]
    #[inline(always)]
    pub fn pitch(&self) -> PITCH_R {
        PITCH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Total length (Low 16 bits) in Bytes -1 for transfer when Out_DMA_ID!=Pixel. If Out_DMA_ID=ECS, it can be any value greater than the length of the ECS, for example, the number of encoded bytes."]
    #[inline(always)]
    pub fn ttlen(&mut self) -> TTLEN_W<16> {
        TTLEN_W::new(self)
    }
    #[doc = "Bits 0:15 - Pitch between the starting point of Rows when Out_DMA_ID==Pixel"]
    #[inline(always)]
    pub fn pitch(&mut self) -> PITCH_W<0> {
        PITCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Out DMA Buf Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outdma_ctrl0](index.html) module"]
pub struct OUTDMA_CTRL0_SPEC;
impl crate::RegisterSpec for OUTDMA_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outdma_ctrl0::R](R) reader structure"]
impl crate::Readable for OUTDMA_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outdma_ctrl0::W](W) writer structure"]
impl crate::Writable for OUTDMA_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTDMA_CTRL0 to value 0"]
impl crate::Resettable for OUTDMA_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}