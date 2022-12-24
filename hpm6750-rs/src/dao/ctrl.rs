#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FALSE_RUN` reader - the module continues to comsume data, but all the pads are constant, thus no audio out"]
pub type FALSE_RUN_R = crate::BitReader<bool>;
#[doc = "Field `FALSE_RUN` writer - the module continues to comsume data, but all the pads are constant, thus no audio out"]
pub type FALSE_RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `FALSE_LEVEL` reader - the pad output in False run mode, or when the module is disabled 0: all low 1: all high 2: P-high, N-low 3. output is not enabled"]
pub type FALSE_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FALSE_LEVEL` writer - the pad output in False run mode, or when the module is disabled 0: all low 1: all high 2: P-high, N-low 3. output is not enabled"]
pub type FALSE_LEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `INVERT` reader - all the outputs are inverted before sending to pad"]
pub type INVERT_R = crate::BitReader<bool>;
#[doc = "Field `INVERT` writer - all the outputs are inverted before sending to pad"]
pub type INVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `REMAP` reader - 1: Use remap pwm version. The remap version is a version that one pwm output is tied to zero when the input pcm signal is positive or negative 0: Don't use remap pwm version"]
pub type REMAP_R = crate::BitReader<bool>;
#[doc = "Field `REMAP` writer - 1: Use remap pwm version. The remap version is a version that one pwm output is tied to zero when the input pcm signal is positive or negative 0: Don't use remap pwm version"]
pub type REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LEFT_EN` reader - Asserted to enable the left channel"]
pub type LEFT_EN_R = crate::BitReader<bool>;
#[doc = "Field `LEFT_EN` writer - Asserted to enable the left channel"]
pub type LEFT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RIGHT_EN` reader - Asserted to enable the right channel"]
pub type RIGHT_EN_R = crate::BitReader<bool>;
#[doc = "Field `RIGHT_EN` writer - Asserted to enable the right channel"]
pub type RIGHT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `MONO` reader - Asserted to let the left and right channel output the same value."]
pub type MONO_R = crate::BitReader<bool>;
#[doc = "Field `MONO` writer - Asserted to let the left and right channel output the same value."]
pub type MONO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SAT_ERR_IE` reader - Error interrupt enable This bit controls the generation of an interrupt when an error condition (saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
pub type SAT_ERR_IE_R = crate::BitReader<bool>;
#[doc = "Field `SAT_ERR_IE` writer - Error interrupt enable This bit controls the generation of an interrupt when an error condition (saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
pub type SAT_ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `HPF_EN` reader - Whether HPF is enabled. This HPF is used to filter out the DC part."]
pub type HPF_EN_R = crate::BitReader<bool>;
#[doc = "Field `HPF_EN` writer - Whether HPF is enabled. This HPF is used to filter out the DC part."]
pub type HPF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - the module continues to comsume data, but all the pads are constant, thus no audio out"]
    #[inline(always)]
    pub fn false_run(&self) -> FALSE_RUN_R {
        FALSE_RUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - the pad output in False run mode, or when the module is disabled 0: all low 1: all high 2: P-high, N-low 3. output is not enabled"]
    #[inline(always)]
    pub fn false_level(&self) -> FALSE_LEVEL_R {
        FALSE_LEVEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - all the outputs are inverted before sending to pad"]
    #[inline(always)]
    pub fn invert(&self) -> INVERT_R {
        INVERT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Use remap pwm version. The remap version is a version that one pwm output is tied to zero when the input pcm signal is positive or negative 0: Don't use remap pwm version"]
    #[inline(always)]
    pub fn remap(&self) -> REMAP_R {
        REMAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asserted to enable the left channel"]
    #[inline(always)]
    pub fn left_en(&self) -> LEFT_EN_R {
        LEFT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Asserted to enable the right channel"]
    #[inline(always)]
    pub fn right_en(&self) -> RIGHT_EN_R {
        RIGHT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Asserted to let the left and right channel output the same value."]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Error interrupt enable This bit controls the generation of an interrupt when an error condition (saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
    #[inline(always)]
    pub fn sat_err_ie(&self) -> SAT_ERR_IE_R {
        SAT_ERR_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Whether HPF is enabled. This HPF is used to filter out the DC part."]
    #[inline(always)]
    pub fn hpf_en(&self) -> HPF_EN_R {
        HPF_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - the module continues to comsume data, but all the pads are constant, thus no audio out"]
    #[inline(always)]
    #[must_use]
    pub fn false_run(&mut self) -> FALSE_RUN_W<0> {
        FALSE_RUN_W::new(self)
    }
    #[doc = "Bits 1:2 - the pad output in False run mode, or when the module is disabled 0: all low 1: all high 2: P-high, N-low 3. output is not enabled"]
    #[inline(always)]
    #[must_use]
    pub fn false_level(&mut self) -> FALSE_LEVEL_W<1> {
        FALSE_LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - all the outputs are inverted before sending to pad"]
    #[inline(always)]
    #[must_use]
    pub fn invert(&mut self) -> INVERT_W<3> {
        INVERT_W::new(self)
    }
    #[doc = "Bit 4 - 1: Use remap pwm version. The remap version is a version that one pwm output is tied to zero when the input pcm signal is positive or negative 0: Don't use remap pwm version"]
    #[inline(always)]
    #[must_use]
    pub fn remap(&mut self) -> REMAP_W<4> {
        REMAP_W::new(self)
    }
    #[doc = "Bit 5 - Asserted to enable the left channel"]
    #[inline(always)]
    #[must_use]
    pub fn left_en(&mut self) -> LEFT_EN_W<5> {
        LEFT_EN_W::new(self)
    }
    #[doc = "Bit 6 - Asserted to enable the right channel"]
    #[inline(always)]
    #[must_use]
    pub fn right_en(&mut self) -> RIGHT_EN_W<6> {
        RIGHT_EN_W::new(self)
    }
    #[doc = "Bit 7 - Asserted to let the left and right channel output the same value."]
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MONO_W<7> {
        MONO_W::new(self)
    }
    #[doc = "Bit 16 - Error interrupt enable This bit controls the generation of an interrupt when an error condition (saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sat_err_ie(&mut self) -> SAT_ERR_IE_W<16> {
        SAT_ERR_IE_W::new(self)
    }
    #[doc = "Bit 17 - Whether HPF is enabled. This HPF is used to filter out the DC part."]
    #[inline(always)]
    #[must_use]
    pub fn hpf_en(&mut self) -> HPF_EN_W<17> {
        HPF_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
