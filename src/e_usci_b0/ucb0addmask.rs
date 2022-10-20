#[doc = "Register `UCB0ADDMASK` reader"]
pub struct R(crate::R<UCB0ADDMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0ADDMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0ADDMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0ADDMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0ADDMASK` writer"]
pub struct W(crate::W<UCB0ADDMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0ADDMASK_SPEC>;
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
impl From<crate::W<UCB0ADDMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0ADDMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDMASK` reader - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
pub type ADDMASK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDMASK` writer - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
pub type ADDMASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, UCB0ADDMASK_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&self) -> ADDMASK_R {
        ADDMASK_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&mut self) -> ADDMASK_W<0> {
        ADDMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx I2C Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0addmask](index.html) module"]
pub struct UCB0ADDMASK_SPEC;
impl crate::RegisterSpec for UCB0ADDMASK_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0addmask::R](R) reader structure"]
impl crate::Readable for UCB0ADDMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0addmask::W](W) writer structure"]
impl crate::Writable for UCB0ADDMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0ADDMASK to value 0"]
impl crate::Resettable for UCB0ADDMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
