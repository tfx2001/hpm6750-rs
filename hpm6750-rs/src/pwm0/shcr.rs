#[doc = "Register `SHCR` reader"]
pub struct R(crate::R<SHCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHCR` writer"]
pub struct W(crate::W<SHCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHCR_SPEC>;
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
impl From<crate::W<SHCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHLKEN` reader - 1- enable shadow registers lock feature, 0- disable shadow registers lock, shlk bit will always be 0"]
pub type SHLKEN_R = crate::BitReader<bool>;
#[doc = "Field `SHLKEN` writer - 1- enable shadow registers lock feature, 0- disable shadow registers lock, shlk bit will always be 0"]
pub type SHLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHCR_SPEC, bool, O>;
#[doc = "Field `CNTSHDWUPT` reader - This bitfield select when the counter related shadow registers (STA and RLD) will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
pub type CNTSHDWUPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNTSHDWUPT` writer - This bitfield select when the counter related shadow registers (STA and RLD) will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
pub type CNTSHDWUPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CNTSHDWSEL` reader - This bitfield select one of the comparators as hardware event time to load the counter related shadow registers (STA and RLD)"]
pub type CNTSHDWSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNTSHDWSEL` writer - This bitfield select one of the comparators as hardware event time to load the counter related shadow registers (STA and RLD)"]
pub type CNTSHDWSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHCR_SPEC, u8, u8, 5, O>;
#[doc = "Field `FRCSHDWSEL` reader - This bitfield select one of the comparators as hardware event time to load FRCMD shadow registers"]
pub type FRCSHDWSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRCSHDWSEL` writer - This bitfield select one of the comparators as hardware event time to load FRCMD shadow registers"]
pub type FRCSHDWSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - 1- enable shadow registers lock feature, 0- disable shadow registers lock, shlk bit will always be 0"]
    #[inline(always)]
    pub fn shlken(&self) -> SHLKEN_R {
        SHLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - This bitfield select when the counter related shadow registers (STA and RLD) will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    #[inline(always)]
    pub fn cntshdwupt(&self) -> CNTSHDWUPT_R {
        CNTSHDWUPT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:7 - This bitfield select one of the comparators as hardware event time to load the counter related shadow registers (STA and RLD)"]
    #[inline(always)]
    pub fn cntshdwsel(&self) -> CNTSHDWSEL_R {
        CNTSHDWSEL_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - This bitfield select one of the comparators as hardware event time to load FRCMD shadow registers"]
    #[inline(always)]
    pub fn frcshdwsel(&self) -> FRCSHDWSEL_R {
        FRCSHDWSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1- enable shadow registers lock feature, 0- disable shadow registers lock, shlk bit will always be 0"]
    #[inline(always)]
    #[must_use]
    pub fn shlken(&mut self) -> SHLKEN_W<0> {
        SHLKEN_W::new(self)
    }
    #[doc = "Bits 1:2 - This bitfield select when the counter related shadow registers (STA and RLD) will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    #[inline(always)]
    #[must_use]
    pub fn cntshdwupt(&mut self) -> CNTSHDWUPT_W<1> {
        CNTSHDWUPT_W::new(self)
    }
    #[doc = "Bits 3:7 - This bitfield select one of the comparators as hardware event time to load the counter related shadow registers (STA and RLD)"]
    #[inline(always)]
    #[must_use]
    pub fn cntshdwsel(&mut self) -> CNTSHDWSEL_W<3> {
        CNTSHDWSEL_W::new(self)
    }
    #[doc = "Bits 8:12 - This bitfield select one of the comparators as hardware event time to load FRCMD shadow registers"]
    #[inline(always)]
    #[must_use]
    pub fn frcshdwsel(&mut self) -> FRCSHDWSEL_W<8> {
        FRCSHDWSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow register control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shcr](index.html) module"]
pub struct SHCR_SPEC;
impl crate::RegisterSpec for SHCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shcr::R](R) reader structure"]
impl crate::Readable for SHCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shcr::W](W) writer structure"]
impl crate::Writable for SHCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHCR to value 0"]
impl crate::Resettable for SHCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
