#[doc = "Register `BMW0` reader"]
pub struct R(crate::R<BMW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMW0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMW0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMW0` writer"]
pub struct W(crate::W<BMW0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMW0_SPEC>;
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
impl From<crate::W<BMW0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMW0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWS` reader - Weight of slave hit with Read/Write Switch. This weight score is valid when queue command's slave is same as current executing command with read/write operation switch."]
pub type RWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWS` writer - Weight of slave hit with Read/Write Switch. This weight score is valid when queue command's slave is same as current executing command with read/write operation switch."]
pub type RWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMW0_SPEC, u8, u8, 8, O>;
#[doc = "Field `SH` reader - Weight of Slave Hit without read/write switch. This weight score is valid when queue command's slave is same as current executing command without read/write operation switch."]
pub type SH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SH` writer - Weight of Slave Hit without read/write switch. This weight score is valid when queue command's slave is same as current executing command without read/write operation switch."]
pub type SH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMW0_SPEC, u8, u8, 8, O>;
#[doc = "Field `AGE` reader - Weight of AGE calculation. Each command in queue has an age signal to indicate its wait period. It is multiplied by WAGE to get weight score."]
pub type AGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AGE` writer - Weight of AGE calculation. Each command in queue has an age signal to indicate its wait period. It is multiplied by WAGE to get weight score."]
pub type AGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMW0_SPEC, u8, u8, 4, O>;
#[doc = "Field `QOS` reader - Weight of QOS calculation. AXI bus access has AxQOS signal set, which is used as a priority indicator for the associated write or read transaction. A higher value indicates a higher priority transaction. AxQOS is multiplied by WQOS to get weight score."]
pub type QOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QOS` writer - Weight of QOS calculation. AXI bus access has AxQOS signal set, which is used as a priority indicator for the associated write or read transaction. A higher value indicates a higher priority transaction. AxQOS is multiplied by WQOS to get weight score."]
pub type QOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMW0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 16:23 - Weight of slave hit with Read/Write Switch. This weight score is valid when queue command's slave is same as current executing command with read/write operation switch."]
    #[inline(always)]
    pub fn rws(&self) -> RWS_R {
        RWS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Weight of Slave Hit without read/write switch. This weight score is valid when queue command's slave is same as current executing command without read/write operation switch."]
    #[inline(always)]
    pub fn sh(&self) -> SH_R {
        SH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - Weight of AGE calculation. Each command in queue has an age signal to indicate its wait period. It is multiplied by WAGE to get weight score."]
    #[inline(always)]
    pub fn age(&self) -> AGE_R {
        AGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Weight of QOS calculation. AXI bus access has AxQOS signal set, which is used as a priority indicator for the associated write or read transaction. A higher value indicates a higher priority transaction. AxQOS is multiplied by WQOS to get weight score."]
    #[inline(always)]
    pub fn qos(&self) -> QOS_R {
        QOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Weight of slave hit with Read/Write Switch. This weight score is valid when queue command's slave is same as current executing command with read/write operation switch."]
    #[inline(always)]
    pub fn rws(&mut self) -> RWS_W<16> {
        RWS_W::new(self)
    }
    #[doc = "Bits 8:15 - Weight of Slave Hit without read/write switch. This weight score is valid when queue command's slave is same as current executing command without read/write operation switch."]
    #[inline(always)]
    pub fn sh(&mut self) -> SH_W<8> {
        SH_W::new(self)
    }
    #[doc = "Bits 4:7 - Weight of AGE calculation. Each command in queue has an age signal to indicate its wait period. It is multiplied by WAGE to get weight score."]
    #[inline(always)]
    pub fn age(&mut self) -> AGE_W<4> {
        AGE_W::new(self)
    }
    #[doc = "Bits 0:3 - Weight of QOS calculation. AXI bus access has AxQOS signal set, which is used as a priority indicator for the associated write or read transaction. A higher value indicates a higher priority transaction. AxQOS is multiplied by WQOS to get weight score."]
    #[inline(always)]
    pub fn qos(&mut self) -> QOS_W<0> {
        QOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus (AXI) Weight Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmw0](index.html) module"]
pub struct BMW0_SPEC;
impl crate::RegisterSpec for BMW0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmw0::R](R) reader structure"]
impl crate::Readable for BMW0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmw0::W](W) writer structure"]
impl crate::Writable for BMW0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMW0 to value 0"]
impl crate::Resettable for BMW0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
