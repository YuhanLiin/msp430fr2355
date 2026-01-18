#[doc = "Register `UCB0I2COA0` reader"]
pub type R = crate::R<Ucb0i2coa0Spec>;
#[doc = "Register `UCB0I2COA0` writer"]
pub type W = crate::W<Ucb0i2coa0Spec>;
#[doc = "Field `I2COA0` reader - I2C own address"]
pub type I2coa0R = crate::FieldReader<u16>;
#[doc = "Field `I2COA0` writer - I2C own address"]
pub type I2coa0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Own Address enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucoaen {
    #[doc = "0: The slave address defined in I2COA0 is disabled"]
    Disable = 0,
    #[doc = "1: The slave address defined in I2COA0 is enabled"]
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
    #[doc = "The slave address defined in I2COA0 is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ucoaen::Disable
    }
    #[doc = "The slave address defined in I2COA0 is enabled"]
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
    #[doc = "The slave address defined in I2COA0 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ucoaen::Disable)
    }
    #[doc = "The slave address defined in I2COA0 is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ucoaen::Enable)
    }
}
#[doc = "General call response enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucgcen {
    #[doc = "0: Do not respond to a general call"]
    Ucgcen0 = 0,
    #[doc = "1: Respond to a general call"]
    Ucgcen1 = 1,
}
impl From<Ucgcen> for bool {
    #[inline(always)]
    fn from(variant: Ucgcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCGCEN` reader - General call response enable"]
pub type UcgcenR = crate::BitReader<Ucgcen>;
impl UcgcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucgcen {
        match self.bits {
            false => Ucgcen::Ucgcen0,
            true => Ucgcen::Ucgcen1,
        }
    }
    #[doc = "Do not respond to a general call"]
    #[inline(always)]
    pub fn is_ucgcen_0(&self) -> bool {
        *self == Ucgcen::Ucgcen0
    }
    #[doc = "Respond to a general call"]
    #[inline(always)]
    pub fn is_ucgcen_1(&self) -> bool {
        *self == Ucgcen::Ucgcen1
    }
}
#[doc = "Field `UCGCEN` writer - General call response enable"]
pub type UcgcenW<'a, REG> = crate::BitWriter<'a, REG, Ucgcen>;
impl<'a, REG> UcgcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not respond to a general call"]
    #[inline(always)]
    pub fn ucgcen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucgcen::Ucgcen0)
    }
    #[doc = "Respond to a general call"]
    #[inline(always)]
    pub fn ucgcen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucgcen::Ucgcen1)
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C own address"]
    #[inline(always)]
    pub fn i2coa0(&self) -> I2coa0R {
        I2coa0R::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 10 - Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&self) -> UcoaenR {
        UcoaenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - General call response enable"]
    #[inline(always)]
    pub fn ucgcen(&self) -> UcgcenR {
        UcgcenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C own address"]
    #[inline(always)]
    pub fn i2coa0(&mut self) -> I2coa0W<'_, Ucb0i2coa0Spec> {
        I2coa0W::new(self, 0)
    }
    #[doc = "Bit 10 - Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&mut self) -> UcoaenW<'_, Ucb0i2coa0Spec> {
        UcoaenW::new(self, 10)
    }
    #[doc = "Bit 15 - General call response enable"]
    #[inline(always)]
    pub fn ucgcen(&mut self) -> UcgcenW<'_, Ucb0i2coa0Spec> {
        UcgcenW::new(self, 15)
    }
}
#[doc = "eUSCI_Bx I2C Own Address 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0i2coa0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0i2coa0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0i2coa0Spec;
impl crate::RegisterSpec for Ucb0i2coa0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0i2coa0::R`](R) reader structure"]
impl crate::Readable for Ucb0i2coa0Spec {}
#[doc = "`write(|w| ..)` method takes [`ucb0i2coa0::W`](W) writer structure"]
impl crate::Writable for Ucb0i2coa0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0I2COA0 to value 0"]
impl crate::Resettable for Ucb0i2coa0Spec {}
