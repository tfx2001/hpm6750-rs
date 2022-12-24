#[doc = "Register `PS_1_OFFSET` reader"]
pub struct R(crate::R<PS_1_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PS_1_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PS_1_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PS_1_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PS_1_OFFSET` writer"]
pub struct W(crate::W<PS_1_OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PS_1_OFFSET_SPEC>;
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
impl From<crate::W<PS_1_OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PS_1_OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X` reader - This is a 12 bit fractional representation (0.####_####_####) of the X scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
pub type X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `X` writer - This is a 12 bit fractional representation (0.####_####_####) of the X scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
pub type X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_1_OFFSET_SPEC, u16, u16, 12, O>;
#[doc = "Field `Y` reader - This is a 12 bit fractional representation (0.####_####_####) of the Y scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
pub type Y_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Y` writer - This is a 12 bit fractional representation (0.####_####_####) of the Y scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
pub type Y_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PS_1_OFFSET_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - This is a 12 bit fractional representation (0.####_####_####) of the X scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - This is a 12 bit fractional representation (0.####_####_####) of the Y scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This is a 12 bit fractional representation (0.####_####_####) of the X scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
    #[inline(always)]
    #[must_use]
    pub fn x(&mut self) -> X_W<0> {
        X_W::new(self)
    }
    #[doc = "Bits 16:27 - This is a 12 bit fractional representation (0.####_####_####) of the Y scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
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
#[doc = "Layer offset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ps_1_offset](index.html) module"]
pub struct PS_1_OFFSET_SPEC;
impl crate::RegisterSpec for PS_1_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ps_1_offset::R](R) reader structure"]
impl crate::Readable for PS_1_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ps_1_offset::W](W) writer structure"]
impl crate::Writable for PS_1_OFFSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PS_1_OFFSET to value 0"]
impl crate::Resettable for PS_1_OFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
