#[doc = "Register `UCB0I2CSA` reader"]
pub struct R(crate::R<UCB0I2CSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCB0I2CSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCB0I2CSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCB0I2CSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCB0I2CSA` writer"]
pub struct W(crate::W<UCB0I2CSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCB0I2CSA_SPEC>;
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
impl From<crate::W<UCB0I2CSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCB0I2CSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2CSA` reader - I2C slave address"]
pub type I2CSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `I2CSA` writer - I2C slave address"]
pub type I2CSA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, UCB0I2CSA_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - I2C slave address"]
    #[inline(always)]
    pub fn i2csa(&self) -> I2CSA_R {
        I2CSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C slave address"]
    #[inline(always)]
    pub fn i2csa(&mut self) -> I2CSA_W<0> {
        I2CSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx I2C Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucb0i2csa](index.html) module"]
pub struct UCB0I2CSA_SPEC;
impl crate::RegisterSpec for UCB0I2CSA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucb0i2csa::R](R) reader structure"]
impl crate::Readable for UCB0I2CSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucb0i2csa::W](W) writer structure"]
impl crate::Writable for UCB0I2CSA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCB0I2CSA to value 0"]
impl crate::Resettable for UCB0I2CSA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
