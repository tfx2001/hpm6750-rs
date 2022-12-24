#[doc = "Register `AUTO_TUNING_STAT` reader"]
pub struct R(crate::R<AUTO_TUNING_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTO_TUNING_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTO_TUNING_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTO_TUNING_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTO_TUNING_STAT` writer"]
pub struct W(crate::W<AUTO_TUNING_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTO_TUNING_STAT_SPEC>;
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
impl From<crate::W<AUTO_TUNING_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTO_TUNING_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CENTER_PH_CODE` reader - Centered Phase code. Reading this field returns the current value on tuning_cclk_sel output. Setting AT_CTRL_R.SW_TUNE_EN enables software to write to this field and its contents are reflected on tuning_cclk_sel"]
pub type CENTER_PH_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CENTER_PH_CODE` writer - Centered Phase code. Reading this field returns the current value on tuning_cclk_sel output. Setting AT_CTRL_R.SW_TUNE_EN enables software to write to this field and its contents are reflected on tuning_cclk_sel"]
pub type CENTER_PH_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AUTO_TUNING_STAT_SPEC, u8, u8, 8, O>;
#[doc = "Field `R_EDGE_PH_CODE` reader - Right Edge Phase code. Reading this field returns the phase code value used by Auto-tuning engine to sample data on Right edge of sampling window."]
pub type R_EDGE_PH_CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `L_EDGE_PH_CODE` reader - Left Edge Phase code. Reading this field returns the phase code value used by Auto-tuning engine to sample data on Left edge of sampling window."]
pub type L_EDGE_PH_CODE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Centered Phase code. Reading this field returns the current value on tuning_cclk_sel output. Setting AT_CTRL_R.SW_TUNE_EN enables software to write to this field and its contents are reflected on tuning_cclk_sel"]
    #[inline(always)]
    pub fn center_ph_code(&self) -> CENTER_PH_CODE_R {
        CENTER_PH_CODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Right Edge Phase code. Reading this field returns the phase code value used by Auto-tuning engine to sample data on Right edge of sampling window."]
    #[inline(always)]
    pub fn r_edge_ph_code(&self) -> R_EDGE_PH_CODE_R {
        R_EDGE_PH_CODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Left Edge Phase code. Reading this field returns the phase code value used by Auto-tuning engine to sample data on Left edge of sampling window."]
    #[inline(always)]
    pub fn l_edge_ph_code(&self) -> L_EDGE_PH_CODE_R {
        L_EDGE_PH_CODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Centered Phase code. Reading this field returns the current value on tuning_cclk_sel output. Setting AT_CTRL_R.SW_TUNE_EN enables software to write to this field and its contents are reflected on tuning_cclk_sel"]
    #[inline(always)]
    #[must_use]
    pub fn center_ph_code(&mut self) -> CENTER_PH_CODE_W<0> {
        CENTER_PH_CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No description avaiable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auto_tuning_stat](index.html) module"]
pub struct AUTO_TUNING_STAT_SPEC;
impl crate::RegisterSpec for AUTO_TUNING_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auto_tuning_stat::R](R) reader structure"]
impl crate::Readable for AUTO_TUNING_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auto_tuning_stat::W](W) writer structure"]
impl crate::Writable for AUTO_TUNING_STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTO_TUNING_STAT to value 0"]
impl crate::Resettable for AUTO_TUNING_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
