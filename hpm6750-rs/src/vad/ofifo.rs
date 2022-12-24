#[doc = "Register `OFIFO` reader"]
pub struct R(crate::R<OFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFIFO` writer"]
pub struct W(crate::W<OFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFIFO_SPEC>;
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
impl From<crate::W<OFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D` reader - The PCM data. When there is only one channel, the samples are from Ch0, and the 2 samples in the 32-bits are: bit \\[31:16\\]: the samples earlier in time (\\[T-1\\]). Bit \\[15:0\\]: the samples later in time (\\[T\\]). When there is two channels, the samples in the 32-bits are: bit \\[31:16\\]: the samples belong to Ch 1 (when ch_pol\\[1:0\\]==2, the data is captured at the positive part of the pdm clk). bit \\[15:0\\]: the samples belong to Ch 0 (when ch_pol\\[1:0\\]==2, the data is captured at the negtive part of the pdm clk)."]
pub type D_R = crate::FieldReader<u32, u32>;
#[doc = "Field `D` writer - The PCM data. When there is only one channel, the samples are from Ch0, and the 2 samples in the 32-bits are: bit \\[31:16\\]: the samples earlier in time (\\[T-1\\]). Bit \\[15:0\\]: the samples later in time (\\[T\\]). When there is two channels, the samples in the 32-bits are: bit \\[31:16\\]: the samples belong to Ch 1 (when ch_pol\\[1:0\\]==2, the data is captured at the positive part of the pdm clk). bit \\[15:0\\]: the samples belong to Ch 0 (when ch_pol\\[1:0\\]==2, the data is captured at the negtive part of the pdm clk)."]
pub type D_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFIFO_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The PCM data. When there is only one channel, the samples are from Ch0, and the 2 samples in the 32-bits are: bit \\[31:16\\]: the samples earlier in time (\\[T-1\\]). Bit \\[15:0\\]: the samples later in time (\\[T\\]). When there is two channels, the samples in the 32-bits are: bit \\[31:16\\]: the samples belong to Ch 1 (when ch_pol\\[1:0\\]==2, the data is captured at the positive part of the pdm clk). bit \\[15:0\\]: the samples belong to Ch 0 (when ch_pol\\[1:0\\]==2, the data is captured at the negtive part of the pdm clk)."]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The PCM data. When there is only one channel, the samples are from Ch0, and the 2 samples in the 32-bits are: bit \\[31:16\\]: the samples earlier in time (\\[T-1\\]). Bit \\[15:0\\]: the samples later in time (\\[T\\]). When there is two channels, the samples in the 32-bits are: bit \\[31:16\\]: the samples belong to Ch 1 (when ch_pol\\[1:0\\]==2, the data is captured at the positive part of the pdm clk). bit \\[15:0\\]: the samples belong to Ch 0 (when ch_pol\\[1:0\\]==2, the data is captured at the negtive part of the pdm clk)."]
    #[inline(always)]
    #[must_use]
    pub fn d(&mut self) -> D_W<0> {
        D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Out FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofifo](index.html) module"]
pub struct OFIFO_SPEC;
impl crate::RegisterSpec for OFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofifo::R](R) reader structure"]
impl crate::Readable for OFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofifo::W](W) writer structure"]
impl crate::Writable for OFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFIFO to value 0"]
impl crate::Resettable for OFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
