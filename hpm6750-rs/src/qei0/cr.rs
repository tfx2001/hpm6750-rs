#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENCTYP` reader - 00-abz; 01-pd; 10-ud; 11-reserved"]
pub type ENCTYP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENCTYP` writer - 00-abz; 01-pd; 10-ud; 11-reserved"]
pub type ENCTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSTCNT` reader - 1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx"]
pub type RSTCNT_R = crate::BitReader<bool>;
#[doc = "Field `RSTCNT` writer - 1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx"]
pub type RSTCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SNAPEN` reader - 1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert"]
pub type SNAPEN_R = crate::BitReader<bool>;
#[doc = "Field `SNAPEN` writer - 1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert"]
pub type SNAPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HFDIR0` reader - 1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)"]
pub type HFDIR0_R = crate::BitReader<bool>;
#[doc = "Field `HFDIR0` writer - 1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)"]
pub type HFDIR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HFDIR1` reader - 1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)"]
pub type HFDIR1_R = crate::BitReader<bool>;
#[doc = "Field `HFDIR1` writer - 1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)"]
pub type HFDIR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HRDIR0` reader - 1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)"]
pub type HRDIR0_R = crate::BitReader<bool>;
#[doc = "Field `HRDIR0` writer - 1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)"]
pub type HRDIR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HRDIR1` reader - 1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)"]
pub type HRDIR1_R = crate::BitReader<bool>;
#[doc = "Field `HRDIR1` writer - 1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)"]
pub type HRDIR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PAUSEZ` reader - 1- pause zcnt when PAUSE assert"]
pub type PAUSEZ_R = crate::BitReader<bool>;
#[doc = "Field `PAUSEZ` writer - 1- pause zcnt when PAUSE assert"]
pub type PAUSEZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PAUSEPH` reader - 1- pause phcnt when PAUSE assert"]
pub type PAUSEPH_R = crate::BitReader<bool>;
#[doc = "Field `PAUSEPH` writer - 1- pause phcnt when PAUSE assert"]
pub type PAUSEPH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `PAUSESPD` reader - 1- pause spdcnt when PAUSE assert"]
pub type PAUSESPD_R = crate::BitReader<bool>;
#[doc = "Field `PAUSESPD` writer - 1- pause spdcnt when PAUSE assert"]
pub type PAUSESPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HRSTZ` reader - 1- reset zcnt when H assert"]
pub type HRSTZ_R = crate::BitReader<bool>;
#[doc = "Field `HRSTZ` writer - 1- reset zcnt when H assert"]
pub type HRSTZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HRSTPH` reader - 1- reset phcnt when H assert"]
pub type HRSTPH_R = crate::BitReader<bool>;
#[doc = "Field `HRSTPH` writer - 1- reset phcnt when H assert"]
pub type HRSTPH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `HRSTSPD` reader - 1- reset spdcnt when H assert"]
pub type HRSTSPD_R = crate::BitReader<bool>;
#[doc = "Field `HRSTSPD` writer - 1- reset spdcnt when H assert"]
pub type HRSTSPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `READ` writer - 1- load phcnt, zcnt, spdcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0"]
pub type READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - 00-abz; 01-pd; 10-ud; 11-reserved"]
    #[inline(always)]
    pub fn enctyp(&self) -> ENCTYP_R {
        ENCTYP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - 1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx"]
    #[inline(always)]
    pub fn rstcnt(&self) -> RSTCNT_R {
        RSTCNT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert"]
    #[inline(always)]
    pub fn snapen(&self) -> SNAPEN_R {
        SNAPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - 1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)"]
    #[inline(always)]
    pub fn hfdir0(&self) -> HFDIR0_R {
        HFDIR0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)"]
    #[inline(always)]
    pub fn hfdir1(&self) -> HFDIR1_R {
        HFDIR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)"]
    #[inline(always)]
    pub fn hrdir0(&self) -> HRDIR0_R {
        HRDIR0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)"]
    #[inline(always)]
    pub fn hrdir1(&self) -> HRDIR1_R {
        HRDIR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1- pause zcnt when PAUSE assert"]
    #[inline(always)]
    pub fn pausez(&self) -> PAUSEZ_R {
        PAUSEZ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1- pause phcnt when PAUSE assert"]
    #[inline(always)]
    pub fn pauseph(&self) -> PAUSEPH_R {
        PAUSEPH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1- pause spdcnt when PAUSE assert"]
    #[inline(always)]
    pub fn pausespd(&self) -> PAUSESPD_R {
        PAUSESPD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 1- reset zcnt when H assert"]
    #[inline(always)]
    pub fn hrstz(&self) -> HRSTZ_R {
        HRSTZ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1- reset phcnt when H assert"]
    #[inline(always)]
    pub fn hrstph(&self) -> HRSTPH_R {
        HRSTPH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1- reset spdcnt when H assert"]
    #[inline(always)]
    pub fn hrstspd(&self) -> HRSTSPD_R {
        HRSTSPD_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00-abz; 01-pd; 10-ud; 11-reserved"]
    #[inline(always)]
    #[must_use]
    pub fn enctyp(&mut self) -> ENCTYP_W<0> {
        ENCTYP_W::new(self)
    }
    #[doc = "Bit 4 - 1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx"]
    #[inline(always)]
    #[must_use]
    pub fn rstcnt(&mut self) -> RSTCNT_W<4> {
        RSTCNT_W::new(self)
    }
    #[doc = "Bit 5 - 1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert"]
    #[inline(always)]
    #[must_use]
    pub fn snapen(&mut self) -> SNAPEN_W<5> {
        SNAPEN_W::new(self)
    }
    #[doc = "Bit 8 - 1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)"]
    #[inline(always)]
    #[must_use]
    pub fn hfdir0(&mut self) -> HFDIR0_W<8> {
        HFDIR0_W::new(self)
    }
    #[doc = "Bit 9 - 1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)"]
    #[inline(always)]
    #[must_use]
    pub fn hfdir1(&mut self) -> HFDIR1_W<9> {
        HFDIR1_W::new(self)
    }
    #[doc = "Bit 10 - 1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)"]
    #[inline(always)]
    #[must_use]
    pub fn hrdir0(&mut self) -> HRDIR0_W<10> {
        HRDIR0_W::new(self)
    }
    #[doc = "Bit 11 - 1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)"]
    #[inline(always)]
    #[must_use]
    pub fn hrdir1(&mut self) -> HRDIR1_W<11> {
        HRDIR1_W::new(self)
    }
    #[doc = "Bit 12 - 1- pause zcnt when PAUSE assert"]
    #[inline(always)]
    #[must_use]
    pub fn pausez(&mut self) -> PAUSEZ_W<12> {
        PAUSEZ_W::new(self)
    }
    #[doc = "Bit 13 - 1- pause phcnt when PAUSE assert"]
    #[inline(always)]
    #[must_use]
    pub fn pauseph(&mut self) -> PAUSEPH_W<13> {
        PAUSEPH_W::new(self)
    }
    #[doc = "Bit 14 - 1- pause spdcnt when PAUSE assert"]
    #[inline(always)]
    #[must_use]
    pub fn pausespd(&mut self) -> PAUSESPD_W<14> {
        PAUSESPD_W::new(self)
    }
    #[doc = "Bit 16 - 1- reset zcnt when H assert"]
    #[inline(always)]
    #[must_use]
    pub fn hrstz(&mut self) -> HRSTZ_W<16> {
        HRSTZ_W::new(self)
    }
    #[doc = "Bit 17 - 1- reset phcnt when H assert"]
    #[inline(always)]
    #[must_use]
    pub fn hrstph(&mut self) -> HRSTPH_W<17> {
        HRSTPH_W::new(self)
    }
    #[doc = "Bit 18 - 1- reset spdcnt when H assert"]
    #[inline(always)]
    #[must_use]
    pub fn hrstspd(&mut self) -> HRSTSPD_W<18> {
        HRSTSPD_W::new(self)
    }
    #[doc = "Bit 31 - 1- load phcnt, zcnt, spdcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<31> {
        READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
