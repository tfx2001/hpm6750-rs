#[doc = "Register `LAYER_5_LAYPOS` reader"]
pub struct R(crate::R<LAYER_5_LAYPOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYER_5_LAYPOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYER_5_LAYPOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYER_5_LAYPOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYER_5_LAYPOS` writer"]
pub struct W(crate::W<LAYER_5_LAYPOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYER_5_LAYPOS_SPEC>;
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
impl From<crate::W<LAYER_5_LAYPOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYER_5_LAYPOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X` reader - The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
pub type X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `X` writer - The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
pub type X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LAYER_5_LAYPOS_SPEC, u16, u16, 16, O>;
#[doc = "Field `Y` reader - The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
pub type Y_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Y` writer - The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
pub type Y_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LAYER_5_LAYPOS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
    #[inline(always)]
    #[must_use]
    pub fn x(&mut self) -> X_W<0> {
        X_W::new(self)
    }
    #[doc = "Bits 16:31 - The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
    #[inline(always)]
    #[must_use]
    pub fn y(&mut self) -> Y_W<16> {
        Y_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer Position Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layer_5_laypos](index.html) module"]
pub struct LAYER_5_LAYPOS_SPEC;
impl crate::RegisterSpec for LAYER_5_LAYPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layer_5_laypos::R](R) reader structure"]
impl crate::Readable for LAYER_5_LAYPOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layer_5_laypos::W](W) writer structure"]
impl crate::Writable for LAYER_5_LAYPOS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYER_5_LAYPOS to value 0"]
impl crate::Resettable for LAYER_5_LAYPOS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
