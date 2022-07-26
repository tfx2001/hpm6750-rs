#[doc = "Register `TIMECFG` reader"]
pub struct R(crate::R<TIMECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMECFG` writer"]
pub struct W(crate::W<TIMECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMECFG_SPEC>;
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
impl From<crate::W<TIMECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEPOS` reader - TIME-stamping POSition 0 – SOF1 – EOF (see Chapter 7)TIMEPOS can only be changed if TIMEEN=0, but it is possible to modify TIMPOS withthe same write access that sets TIMEEN=1."]
pub type TIMEPOS_R = crate::BitReader<bool>;
#[doc = "Field `TIMEPOS` writer - TIME-stamping POSition 0 – SOF1 – EOF (see Chapter 7)TIMEPOS can only be changed if TIMEEN=0, but it is possible to modify TIMPOS withthe same write access that sets TIMEEN=1."]
pub type TIMEPOS_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMECFG_SPEC, bool, O>;
#[doc = "Field `TIMEEN` reader - TIME-stamping ENable 0 – disabled 1 – enabled"]
pub type TIMEEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMEEN` writer - TIME-stamping ENable 0 – disabled 1 – enabled"]
pub type TIMEEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, TIMECFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - TIME-stamping POSition 0 – SOF1 – EOF (see Chapter 7)TIMEPOS can only be changed if TIMEEN=0, but it is possible to modify TIMPOS withthe same write access that sets TIMEEN=1."]
    #[inline(always)]
    pub fn timepos(&self) -> TIMEPOS_R {
        TIMEPOS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - TIME-stamping ENable 0 – disabled 1 – enabled"]
    #[inline(always)]
    pub fn timeen(&self) -> TIMEEN_R {
        TIMEEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIME-stamping POSition 0 – SOF1 – EOF (see Chapter 7)TIMEPOS can only be changed if TIMEEN=0, but it is possible to modify TIMPOS withthe same write access that sets TIMEEN=1."]
    #[inline(always)]
    pub fn timepos(&mut self) -> TIMEPOS_W<1> {
        TIMEPOS_W::new(self)
    }
    #[doc = "Bit 0 - TIME-stamping ENable 0 – disabled 1 – enabled"]
    #[inline(always)]
    pub fn timeen(&mut self) -> TIMEEN_W<0> {
        TIMEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CiA 603 Time-Stamping TIMECFG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timecfg](index.html) module"]
pub struct TIMECFG_SPEC;
impl crate::RegisterSpec for TIMECFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [timecfg::R](R) reader structure"]
impl crate::Readable for TIMECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timecfg::W](W) writer structure"]
impl crate::Writable for TIMECFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMECFG to value 0"]
impl crate::Resettable for TIMECFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
