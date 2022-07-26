#[doc = "Register `MAC_ADDR_4_LOW` reader"]
pub struct R(crate::R<MAC_ADDR_4_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDR_4_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDR_4_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDR_4_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDR_4_LOW` writer"]
pub struct W(crate::W<MAC_ADDR_4_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDR_4_LOW_SPEC>;
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
impl From<crate::W<MAC_ADDR_4_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDR_4_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRLO` reader - MAC Address1 \\[31:0\\]
This field contains the lower 32 bits of the second 6-byte MAC address. The content of this field is undefined until loaded by the Application after the initialization process."]
pub type ADDRLO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRLO` writer - MAC Address1 \\[31:0\\]
This field contains the lower 32 bits of the second 6-byte MAC address. The content of this field is undefined until loaded by the Application after the initialization process."]
pub type ADDRLO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_ADDR_4_LOW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - MAC Address1 \\[31:0\\]
This field contains the lower 32 bits of the second 6-byte MAC address. The content of this field is undefined until loaded by the Application after the initialization process."]
    #[inline(always)]
    pub fn addrlo(&self) -> ADDRLO_R {
        ADDRLO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address1 \\[31:0\\]
This field contains the lower 32 bits of the second 6-byte MAC address. The content of this field is undefined until loaded by the Application after the initialization process."]
    #[inline(always)]
    pub fn addrlo(&mut self) -> ADDRLO_W<0> {
        ADDRLO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Address4 Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr_4_low](index.html) module"]
pub struct MAC_ADDR_4_LOW_SPEC;
impl crate::RegisterSpec for MAC_ADDR_4_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_addr_4_low::R](R) reader structure"]
impl crate::Readable for MAC_ADDR_4_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_addr_4_low::W](W) writer structure"]
impl crate::Writable for MAC_ADDR_4_LOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_ADDR_4_LOW to value 0"]
impl crate::Resettable for MAC_ADDR_4_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
