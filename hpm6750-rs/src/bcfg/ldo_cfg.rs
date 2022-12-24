#[doc = "Register `LDO_CFG` reader"]
pub struct R(crate::R<LDO_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDO_CFG` writer"]
pub struct W(crate::W<LDO_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO_CFG_SPEC>;
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
impl From<crate::W<LDO_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOLT` reader - LDO voltage setting in mV, valid range through 600mV to 1100mV, step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1100mV. 600: 600mV 620: 620mV . . . 1100:1100mV"]
pub type VOLT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VOLT` writer - LDO voltage setting in mV, valid range through 600mV to 1100mV, step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1100mV. 600: 600mV 620: 620mV . . . 1100:1100mV"]
pub type VOLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_CFG_SPEC, u16, u16, 12, O>;
#[doc = "Field `ENABLE` reader - LDO enable 0: LDO is disabled 1: LDO is enabled"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - LDO enable 0: LDO is disabled 1: LDO is enabled"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO_CFG_SPEC, bool, O>;
#[doc = "Field `DIS_PD` reader - disable pull down resistor, enable pull down may lead to more power but better response 0: pulldown resistor enabled 1: pulldown resistor disabled"]
pub type DIS_PD_R = crate::BitReader<bool>;
#[doc = "Field `DIS_PD` writer - disable pull down resistor, enable pull down may lead to more power but better response 0: pulldown resistor enabled 1: pulldown resistor disabled"]
pub type DIS_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO_CFG_SPEC, bool, O>;
#[doc = "Field `EN_SL` reader - enable selfload, this bit helps improve LDO performance when current less than 200nA 0: self load disabled 1: selfload enabled"]
pub type EN_SL_R = crate::BitReader<bool>;
#[doc = "Field `EN_SL` writer - enable selfload, this bit helps improve LDO performance when current less than 200nA 0: self load disabled 1: selfload enabled"]
pub type EN_SL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LDO_CFG_SPEC, bool, O>;
#[doc = "Field `CP_TRIM` reader - Capacitor trim"]
pub type CP_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CP_TRIM` writer - Capacitor trim"]
pub type CP_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `RES_TRIM` reader - Resistor trim"]
pub type RES_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RES_TRIM` writer - Resistor trim"]
pub type RES_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDO_CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:11 - LDO voltage setting in mV, valid range through 600mV to 1100mV, step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1100mV. 600: 600mV 620: 620mV . . . 1100:1100mV"]
    #[inline(always)]
    pub fn volt(&self) -> VOLT_R {
        VOLT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - LDO enable 0: LDO is disabled 1: LDO is enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - disable pull down resistor, enable pull down may lead to more power but better response 0: pulldown resistor enabled 1: pulldown resistor disabled"]
    #[inline(always)]
    pub fn dis_pd(&self) -> DIS_PD_R {
        DIS_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - enable selfload, this bit helps improve LDO performance when current less than 200nA 0: self load disabled 1: selfload enabled"]
    #[inline(always)]
    pub fn en_sl(&self) -> EN_SL_R {
        EN_SL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Capacitor trim"]
    #[inline(always)]
    pub fn cp_trim(&self) -> CP_TRIM_R {
        CP_TRIM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Resistor trim"]
    #[inline(always)]
    pub fn res_trim(&self) -> RES_TRIM_R {
        RES_TRIM_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - LDO voltage setting in mV, valid range through 600mV to 1100mV, step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1100mV. 600: 600mV 620: 620mV . . . 1100:1100mV"]
    #[inline(always)]
    #[must_use]
    pub fn volt(&mut self) -> VOLT_W<0> {
        VOLT_W::new(self)
    }
    #[doc = "Bit 16 - LDO enable 0: LDO is disabled 1: LDO is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<16> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 17 - disable pull down resistor, enable pull down may lead to more power but better response 0: pulldown resistor enabled 1: pulldown resistor disabled"]
    #[inline(always)]
    #[must_use]
    pub fn dis_pd(&mut self) -> DIS_PD_W<17> {
        DIS_PD_W::new(self)
    }
    #[doc = "Bit 18 - enable selfload, this bit helps improve LDO performance when current less than 200nA 0: self load disabled 1: selfload enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en_sl(&mut self) -> EN_SL_W<18> {
        EN_SL_W::new(self)
    }
    #[doc = "Bits 20:21 - Capacitor trim"]
    #[inline(always)]
    #[must_use]
    pub fn cp_trim(&mut self) -> CP_TRIM_W<20> {
        CP_TRIM_W::new(self)
    }
    #[doc = "Bits 24:25 - Resistor trim"]
    #[inline(always)]
    #[must_use]
    pub fn res_trim(&mut self) -> RES_TRIM_W<24> {
        RES_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LDO config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo_cfg](index.html) module"]
pub struct LDO_CFG_SPEC;
impl crate::RegisterSpec for LDO_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo_cfg::R](R) reader structure"]
impl crate::Readable for LDO_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo_cfg::W](W) writer structure"]
impl crate::Writable for LDO_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LDO_CFG to value 0x0001_0000"]
impl crate::Resettable for LDO_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
