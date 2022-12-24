#[doc = "Register `IDEAL_WN_SIZE` reader"]
pub struct R(crate::R<IDEAL_WN_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDEAL_WN_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDEAL_WN_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDEAL_WN_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDEAL_WN_SIZE` writer"]
pub struct W(crate::W<IDEAL_WN_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDEAL_WN_SIZE_SPEC>;
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
impl From<crate::W<IDEAL_WN_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDEAL_WN_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIDTH` reader - Image Width. Indicates how many active pixels in a line of the image from the sensor. The number of bytes to be transfered is re-calculated automatically in hardware based on cr1\\[color_ext\\]
and cr1\\[store_mode\\]. Default value is 2*pixel number. As the input data from the sensor is 8-bit/pixel format, the IMAGE_WIDTH should be a multiple of 8 pixels."]
pub type WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WIDTH` writer - Image Width. Indicates how many active pixels in a line of the image from the sensor. The number of bytes to be transfered is re-calculated automatically in hardware based on cr1\\[color_ext\\]
and cr1\\[store_mode\\]. Default value is 2*pixel number. As the input data from the sensor is 8-bit/pixel format, the IMAGE_WIDTH should be a multiple of 8 pixels."]
pub type WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IDEAL_WN_SIZE_SPEC, u16, u16, 16, O>;
#[doc = "Field `HEIGHT` reader - Image Height. Indicates how many active pixels in a column of the image from the sensor."]
pub type HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HEIGHT` writer - Image Height. Indicates how many active pixels in a column of the image from the sensor."]
pub type HEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IDEAL_WN_SIZE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Image Width. Indicates how many active pixels in a line of the image from the sensor. The number of bytes to be transfered is re-calculated automatically in hardware based on cr1\\[color_ext\\]
and cr1\\[store_mode\\]. Default value is 2*pixel number. As the input data from the sensor is 8-bit/pixel format, the IMAGE_WIDTH should be a multiple of 8 pixels."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Image Height. Indicates how many active pixels in a column of the image from the sensor."]
    #[inline(always)]
    pub fn height(&self) -> HEIGHT_R {
        HEIGHT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Image Width. Indicates how many active pixels in a line of the image from the sensor. The number of bytes to be transfered is re-calculated automatically in hardware based on cr1\\[color_ext\\]
and cr1\\[store_mode\\]. Default value is 2*pixel number. As the input data from the sensor is 8-bit/pixel format, the IMAGE_WIDTH should be a multiple of 8 pixels."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<0> {
        WIDTH_W::new(self)
    }
    #[doc = "Bits 16:31 - Image Height. Indicates how many active pixels in a column of the image from the sensor."]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HEIGHT_W<16> {
        HEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ideal Image Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ideal_wn_size](index.html) module"]
pub struct IDEAL_WN_SIZE_SPEC;
impl crate::RegisterSpec for IDEAL_WN_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ideal_wn_size::R](R) reader structure"]
impl crate::Readable for IDEAL_WN_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ideal_wn_size::W](W) writer structure"]
impl crate::Writable for IDEAL_WN_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDEAL_WN_SIZE to value 0"]
impl crate::Resettable for IDEAL_WN_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
