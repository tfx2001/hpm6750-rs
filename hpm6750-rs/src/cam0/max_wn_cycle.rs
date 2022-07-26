#[doc = "Register `MAX_WN_CYCLE` reader"]
pub struct R(crate::R<MAX_WN_CYCLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAX_WN_CYCLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAX_WN_CYCLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAX_WN_CYCLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAX_WN_CYCLE` writer"]
pub struct W(crate::W<MAX_WN_CYCLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAX_WN_CYCLE_SPEC>;
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
impl From<crate::W<MAX_WN_CYCLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAX_WN_CYCLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROW` reader - Max Width-1"]
pub type ROW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ROW` writer - Max Width-1"]
pub type ROW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAX_WN_CYCLE_SPEC, u16, u16, 16, O>;
#[doc = "Field `COL` reader - Max Height-1"]
pub type COL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COL` writer - Max Height-1"]
pub type COL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAX_WN_CYCLE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - Max Width-1"]
    #[inline(always)]
    pub fn row(&self) -> ROW_R {
        ROW_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Max Height-1"]
    #[inline(always)]
    pub fn col(&self) -> COL_R {
        COL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Max Width-1"]
    #[inline(always)]
    pub fn row(&mut self) -> ROW_W<16> {
        ROW_W::new(self)
    }
    #[doc = "Bits 0:15 - Max Height-1"]
    #[inline(always)]
    pub fn col(&mut self) -> COL_W<0> {
        COL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Max Window Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [max_wn_cycle](index.html) module"]
pub struct MAX_WN_CYCLE_SPEC;
impl crate::RegisterSpec for MAX_WN_CYCLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [max_wn_cycle::R](R) reader structure"]
impl crate::Readable for MAX_WN_CYCLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [max_wn_cycle::W](W) writer structure"]
impl crate::Writable for MAX_WN_CYCLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAX_WN_CYCLE to value 0"]
impl crate::Resettable for MAX_WN_CYCLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
