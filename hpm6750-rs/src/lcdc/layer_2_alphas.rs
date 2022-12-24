#[doc = "Register `LAYER_2_ALPHAS` reader"]
pub struct R(crate::R<LAYER_2_ALPHAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYER_2_ALPHAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYER_2_ALPHAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYER_2_ALPHAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYER_2_ALPHAS` writer"]
pub struct W(crate::W<LAYER_2_ALPHAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYER_2_ALPHAS_SPEC>;
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
impl From<crate::W<LAYER_2_ALPHAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYER_2_ALPHAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IND` reader - The system alpha value for the input stream from previous stage (DST)"]
pub type IND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IND` writer - The system alpha value for the input stream from previous stage (DST)"]
pub type IND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LAYER_2_ALPHAS_SPEC, u8, u8, 8, O>;
#[doc = "Field `LOCD` reader - The system alpha value for the data stream of current layer stream (SRC)"]
pub type LOCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCD` writer - The system alpha value for the data stream of current layer stream (SRC)"]
pub type LOCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LAYER_2_ALPHAS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The system alpha value for the input stream from previous stage (DST)"]
    #[inline(always)]
    pub fn ind(&self) -> IND_R {
        IND_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The system alpha value for the data stream of current layer stream (SRC)"]
    #[inline(always)]
    pub fn locd(&self) -> LOCD_R {
        LOCD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The system alpha value for the input stream from previous stage (DST)"]
    #[inline(always)]
    #[must_use]
    pub fn ind(&mut self) -> IND_W<0> {
        IND_W::new(self)
    }
    #[doc = "Bits 8:15 - The system alpha value for the data stream of current layer stream (SRC)"]
    #[inline(always)]
    #[must_use]
    pub fn locd(&mut self) -> LOCD_W<8> {
        LOCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer Alpha Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layer_2_alphas](index.html) module"]
pub struct LAYER_2_ALPHAS_SPEC;
impl crate::RegisterSpec for LAYER_2_ALPHAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layer_2_alphas::R](R) reader structure"]
impl crate::Readable for LAYER_2_ALPHAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layer_2_alphas::W](W) writer structure"]
impl crate::Writable for LAYER_2_ALPHAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYER_2_ALPHAS to value 0"]
impl crate::Resettable for LAYER_2_ALPHAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
