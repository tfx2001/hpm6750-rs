#[doc = "Register `HASH_TABLE_REGISTER3` reader"]
pub struct R(crate::R<HASH_TABLE_REGISTER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_TABLE_REGISTER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_TABLE_REGISTER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_TABLE_REGISTER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_TABLE_REGISTER3` writer"]
pub struct W(crate::W<HASH_TABLE_REGISTER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_TABLE_REGISTER3_SPEC>;
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
impl From<crate::W<HASH_TABLE_REGISTER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_TABLE_REGISTER3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT31T0` reader - First 32 bits of Hash Table This field contains the first 32 Bits (31:0) of the Hash table."]
pub type HT31T0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HT31T0` writer - First 32 bits of Hash Table This field contains the first 32 Bits (31:0) of the Hash table."]
pub type HT31T0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HASH_TABLE_REGISTER3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - First 32 bits of Hash Table This field contains the first 32 Bits (31:0) of the Hash table."]
    #[inline(always)]
    pub fn ht31t0(&self) -> HT31T0_R {
        HT31T0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - First 32 bits of Hash Table This field contains the first 32 Bits (31:0) of the Hash table."]
    #[inline(always)]
    pub fn ht31t0(&mut self) -> HT31T0_W<0> {
        HT31T0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Table Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_table_register3](index.html) module"]
pub struct HASH_TABLE_REGISTER3_SPEC;
impl crate::RegisterSpec for HASH_TABLE_REGISTER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_table_register3::R](R) reader structure"]
impl crate::Readable for HASH_TABLE_REGISTER3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_table_register3::W](W) writer structure"]
impl crate::Writable for HASH_TABLE_REGISTER3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_TABLE_REGISTER3 to value 0"]
impl crate::Resettable for HASH_TABLE_REGISTER3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
