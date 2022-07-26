#[doc = "Register `CMPCFG_18` reader"]
pub struct R(crate::R<CMPCFG_18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPCFG_18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPCFG_18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPCFG_18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPCFG_18` writer"]
pub struct W(crate::W<CMPCFG_18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPCFG_18_SPEC>;
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
impl From<crate::W<CMPCFG_18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPCFG_18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XCNTCMPEN` reader - This bitfield enable the comparator to compare xcmp with xcnt."]
pub type XCNTCMPEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XCNTCMPEN` writer - This bitfield enable the comparator to compare xcmp with xcnt."]
pub type XCNTCMPEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPCFG_18_SPEC, u8, u8, 4, O>;
#[doc = "Field `CMPSHDWUPT` reader - This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
pub type CMPSHDWUPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPSHDWUPT` writer - This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
pub type CMPSHDWUPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPCFG_18_SPEC, u8, u8, 2, O>;
#[doc = "Field `CMPMODE` reader - comparator mode 0- output compare mode 1- input capture mode"]
pub type CMPMODE_R = crate::BitReader<bool>;
#[doc = "Field `CMPMODE` writer - comparator mode 0- output compare mode 1- input capture mode"]
pub type CMPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPCFG_18_SPEC, bool, O>;
impl R {
    #[doc = "Bits 4:7 - This bitfield enable the comparator to compare xcmp with xcnt."]
    #[inline(always)]
    pub fn xcntcmpen(&self) -> XCNTCMPEN_R {
        XCNTCMPEN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 2:3 - This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    #[inline(always)]
    pub fn cmpshdwupt(&self) -> CMPSHDWUPT_R {
        CMPSHDWUPT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 1 - comparator mode 0- output compare mode 1- input capture mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7 - This bitfield enable the comparator to compare xcmp with xcnt."]
    #[inline(always)]
    pub fn xcntcmpen(&mut self) -> XCNTCMPEN_W<4> {
        XCNTCMPEN_W::new(self)
    }
    #[doc = "Bits 2:3 - This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    #[inline(always)]
    pub fn cmpshdwupt(&mut self) -> CMPSHDWUPT_W<2> {
        CMPSHDWUPT_W::new(self)
    }
    #[doc = "Bit 1 - comparator mode 0- output compare mode 1- input capture mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CMPMODE_W<1> {
        CMPMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpcfg_18](index.html) module"]
pub struct CMPCFG_18_SPEC;
impl crate::RegisterSpec for CMPCFG_18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpcfg_18::R](R) reader structure"]
impl crate::Readable for CMPCFG_18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpcfg_18::W](W) writer structure"]
impl crate::Writable for CMPCFG_18_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPCFG_18 to value 0"]
impl crate::Resettable for CMPCFG_18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
