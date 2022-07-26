#[doc = "Register `PS_1_SCALE` reader"]
pub struct R(crate::R<PS_1_SCALE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS_1_SCALE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS_1_SCALE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS_1_SCALE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PS_1_SCALE` writer"]
pub struct W(crate::W<PS_1_SCALE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PS_1_SCALE_SPEC>;
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
impl From<crate::W<PS_1_SCALE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PS_1_SCALE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Y` reader - This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the X scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
pub type Y_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Y` writer - This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the X scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
pub type Y_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_1_SCALE_SPEC, u16, u16, 15, O>;
#[doc = "Field `X` reader - This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the Y scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
pub type X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `X` writer - This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the Y scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
pub type X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_1_SCALE_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 16:30 - This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the X scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bits 0:14 - This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the Y scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:30 - This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the X scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
    #[inline(always)]
    pub fn y(&mut self) -> Y_W<16> {
        Y_W::new(self)
    }
    #[doc = "Bits 0:14 - This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the Y scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
    #[inline(always)]
    pub fn x(&mut self) -> X_W<0> {
        X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer scale register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_1_scale](index.html) module"]
pub struct PS_1_SCALE_SPEC;
impl crate::RegisterSpec for PS_1_SCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps_1_scale::R](R) reader structure"]
impl crate::Readable for PS_1_SCALE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ps_1_scale::W](W) writer structure"]
impl crate::Writable for PS_1_SCALE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PS_1_SCALE to value 0"]
impl crate::Resettable for PS_1_SCALE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
