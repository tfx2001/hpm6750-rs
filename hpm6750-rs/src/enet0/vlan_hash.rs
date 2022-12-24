#[doc = "Register `VLAN_HASH` reader"]
pub struct R(crate::R<VLAN_HASH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLAN_HASH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLAN_HASH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLAN_HASH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLAN_HASH` writer"]
pub struct W(crate::W<VLAN_HASH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLAN_HASH_SPEC>;
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
impl From<crate::W<VLAN_HASH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLAN_HASH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLHT` reader - VLAN Hash Table This field contains the 16-bit VLAN Hash Table."]
pub type VLHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VLHT` writer - VLAN Hash Table This field contains the 16-bit VLAN Hash Table."]
pub type VLHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VLAN_HASH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - VLAN Hash Table This field contains the 16-bit VLAN Hash Table."]
    #[inline(always)]
    pub fn vlht(&self) -> VLHT_R {
        VLHT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN Hash Table This field contains the 16-bit VLAN Hash Table."]
    #[inline(always)]
    #[must_use]
    pub fn vlht(&mut self) -> VLHT_W<0> {
        VLHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VLAN Hash Table Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vlan_hash](index.html) module"]
pub struct VLAN_HASH_SPEC;
impl crate::RegisterSpec for VLAN_HASH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vlan_hash::R](R) reader structure"]
impl crate::Readable for VLAN_HASH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vlan_hash::W](W) writer structure"]
impl crate::Writable for VLAN_HASH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VLAN_HASH to value 0"]
impl crate::Resettable for VLAN_HASH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
