#[doc = "Register `UCB1I2COA1` reader"]
pub type R = crate::R<Ucb1i2coa1Spec>;
#[doc = "Register `UCB1I2COA1` writer"]
pub type W = crate::W<Ucb1i2coa1Spec>;
#[doc = "Field `I2COA1` reader - I2C own address"]
pub type I2coa1R = crate::FieldReader<u16>;
#[doc = "Field `I2COA1` writer - I2C own address"]
pub type I2coa1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Own Address enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucoaen {
    #[doc = "0: The slave address defined in I2COA1 is disabled"]
    Disable = 0,
    #[doc = "1: The slave address defined in I2COA1 is enabled"]
    Enable = 1,
}
impl From<Ucoaen> for bool {
    #[inline(always)]
    fn from(variant: Ucoaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCOAEN` reader - Own Address enable register"]
pub type UcoaenR = crate::BitReader<Ucoaen>;
impl UcoaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucoaen {
        match self.bits {
            false => Ucoaen::Disable,
            true => Ucoaen::Enable,
        }
    }
    #[doc = "The slave address defined in I2COA1 is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ucoaen::Disable
    }
    #[doc = "The slave address defined in I2COA1 is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ucoaen::Enable
    }
}
#[doc = "Field `UCOAEN` writer - Own Address enable register"]
pub type UcoaenW<'a, REG> = crate::BitWriter<'a, REG, Ucoaen>;
impl<'a, REG> UcoaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The slave address defined in I2COA1 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ucoaen::Disable)
    }
    #[doc = "The slave address defined in I2COA1 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ucoaen::Enable)
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C own address"]
    #[inline(always)]
    pub fn i2coa1(&self) -> I2coa1R {
        I2coa1R::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10 - Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&self) -> UcoaenR {
        UcoaenR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C own address"]
    #[inline(always)]
    pub fn i2coa1(&mut self) -> I2coa1W<'_, Ucb1i2coa1Spec> {
        I2coa1W::new(self, 0)
    }
    #[doc = "Bit 10 - Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&mut self) -> UcoaenW<'_, Ucb1i2coa1Spec> {
        UcoaenW::new(self, 10)
    }
}
#[doc = "eUSCI_Bx I2C Own Address 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb1i2coa1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb1i2coa1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb1i2coa1Spec;
impl crate::RegisterSpec for Ucb1i2coa1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb1i2coa1::R`](R) reader structure"]
impl crate::Readable for Ucb1i2coa1Spec {}
#[doc = "`write(|w| ..)` method takes [`ucb1i2coa1::W`](W) writer structure"]
impl crate::Writable for Ucb1i2coa1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB1I2COA1 to value 0"]
impl crate::Resettable for Ucb1i2coa1Spec {}
