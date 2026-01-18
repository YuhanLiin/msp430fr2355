#[doc = "Register `ADCIE` reader"]
pub type R = crate::R<AdcieSpec>;
#[doc = "Register `ADCIE` writer"]
pub type W = crate::W<AdcieSpec>;
#[doc = "Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcie0 {
    #[doc = "0: 0b = Interrupt disabled"]
    Adcie0_0 = 0,
    #[doc = "1: 1b = Interrupt enabled"]
    Adcie0_1 = 1,
}
impl From<Adcie0> for bool {
    #[inline(always)]
    fn from(variant: Adcie0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCIE0` reader - Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion."]
pub type Adcie0R = crate::BitReader<Adcie0>;
impl Adcie0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcie0 {
        match self.bits {
            false => Adcie0::Adcie0_0,
            true => Adcie0::Adcie0_1,
        }
    }
    #[doc = "0b = Interrupt disabled"]
    #[inline(always)]
    pub fn is_adcie0_0(&self) -> bool {
        *self == Adcie0::Adcie0_0
    }
    #[doc = "1b = Interrupt enabled"]
    #[inline(always)]
    pub fn is_adcie0_1(&self) -> bool {
        *self == Adcie0::Adcie0_1
    }
}
#[doc = "Field `ADCIE0` writer - Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion."]
pub type Adcie0W<'a, REG> = crate::BitWriter<'a, REG, Adcie0>;
impl<'a, REG> Adcie0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0b = Interrupt disabled"]
    #[inline(always)]
    pub fn adcie0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcie0::Adcie0_0)
    }
    #[doc = "1b = Interrupt enabled"]
    #[inline(always)]
    pub fn adcie0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcie0::Adcie0_1)
    }
}
#[doc = "Interrupt enable for the inside of window interrupt of the window comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcinie {
    #[doc = "0: 0b = Inside of window interrupt disabled"]
    Adcinie0 = 0,
    #[doc = "1: 1b = Inside of window interrupt enabled"]
    Adcinie1 = 1,
}
impl From<Adcinie> for bool {
    #[inline(always)]
    fn from(variant: Adcinie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCINIE` reader - Interrupt enable for the inside of window interrupt of the window comparator."]
pub type AdcinieR = crate::BitReader<Adcinie>;
impl AdcinieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcinie {
        match self.bits {
            false => Adcinie::Adcinie0,
            true => Adcinie::Adcinie1,
        }
    }
    #[doc = "0b = Inside of window interrupt disabled"]
    #[inline(always)]
    pub fn is_adcinie_0(&self) -> bool {
        *self == Adcinie::Adcinie0
    }
    #[doc = "1b = Inside of window interrupt enabled"]
    #[inline(always)]
    pub fn is_adcinie_1(&self) -> bool {
        *self == Adcinie::Adcinie1
    }
}
#[doc = "Field `ADCINIE` writer - Interrupt enable for the inside of window interrupt of the window comparator."]
pub type AdcinieW<'a, REG> = crate::BitWriter<'a, REG, Adcinie>;
impl<'a, REG> AdcinieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0b = Inside of window interrupt disabled"]
    #[inline(always)]
    pub fn adcinie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinie::Adcinie0)
    }
    #[doc = "1b = Inside of window interrupt enabled"]
    #[inline(always)]
    pub fn adcinie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinie::Adcinie1)
    }
}
#[doc = "Interrupt enable for the below lower threshold interrupt of the window comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcloie {
    #[doc = "0: 0b = Below lower threshold interrupt disabled"]
    Adcloie0 = 0,
    #[doc = "1: 1b = Below lower threshold interrupt enabled"]
    Adcloie1 = 1,
}
impl From<Adcloie> for bool {
    #[inline(always)]
    fn from(variant: Adcloie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCLOIE` reader - Interrupt enable for the below lower threshold interrupt of the window comparator."]
pub type AdcloieR = crate::BitReader<Adcloie>;
impl AdcloieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcloie {
        match self.bits {
            false => Adcloie::Adcloie0,
            true => Adcloie::Adcloie1,
        }
    }
    #[doc = "0b = Below lower threshold interrupt disabled"]
    #[inline(always)]
    pub fn is_adcloie_0(&self) -> bool {
        *self == Adcloie::Adcloie0
    }
    #[doc = "1b = Below lower threshold interrupt enabled"]
    #[inline(always)]
    pub fn is_adcloie_1(&self) -> bool {
        *self == Adcloie::Adcloie1
    }
}
#[doc = "Field `ADCLOIE` writer - Interrupt enable for the below lower threshold interrupt of the window comparator."]
pub type AdcloieW<'a, REG> = crate::BitWriter<'a, REG, Adcloie>;
impl<'a, REG> AdcloieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0b = Below lower threshold interrupt disabled"]
    #[inline(always)]
    pub fn adcloie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcloie::Adcloie0)
    }
    #[doc = "1b = Below lower threshold interrupt enabled"]
    #[inline(always)]
    pub fn adcloie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcloie::Adcloie1)
    }
}
#[doc = "Interrupt enable for the above upper threshold interrupt of the window comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adchiie {
    #[doc = "0: 0b = Above upper threshold interrupt disabled"]
    Adchiie0 = 0,
    #[doc = "1: 1b = Above upper threshold interrupt enabled"]
    Adchiie1 = 1,
}
impl From<Adchiie> for bool {
    #[inline(always)]
    fn from(variant: Adchiie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCHIIE` reader - Interrupt enable for the above upper threshold interrupt of the window comparator."]
pub type AdchiieR = crate::BitReader<Adchiie>;
impl AdchiieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adchiie {
        match self.bits {
            false => Adchiie::Adchiie0,
            true => Adchiie::Adchiie1,
        }
    }
    #[doc = "0b = Above upper threshold interrupt disabled"]
    #[inline(always)]
    pub fn is_adchiie_0(&self) -> bool {
        *self == Adchiie::Adchiie0
    }
    #[doc = "1b = Above upper threshold interrupt enabled"]
    #[inline(always)]
    pub fn is_adchiie_1(&self) -> bool {
        *self == Adchiie::Adchiie1
    }
}
#[doc = "Field `ADCHIIE` writer - Interrupt enable for the above upper threshold interrupt of the window comparator."]
pub type AdchiieW<'a, REG> = crate::BitWriter<'a, REG, Adchiie>;
impl<'a, REG> AdchiieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0b = Above upper threshold interrupt disabled"]
    #[inline(always)]
    pub fn adchiie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adchiie::Adchiie0)
    }
    #[doc = "1b = Above upper threshold interrupt enabled"]
    #[inline(always)]
    pub fn adchiie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adchiie::Adchiie1)
    }
}
#[doc = "ADCMEM0 overflow interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcovie {
    #[doc = "0: 0b = Overflow interrupt disabled"]
    Adcovie0 = 0,
    #[doc = "1: 1b = Overflow interrupt enabled"]
    Adcovie1 = 1,
}
impl From<Adcovie> for bool {
    #[inline(always)]
    fn from(variant: Adcovie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCOVIE` reader - ADCMEM0 overflow interrupt enable."]
pub type AdcovieR = crate::BitReader<Adcovie>;
impl AdcovieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcovie {
        match self.bits {
            false => Adcovie::Adcovie0,
            true => Adcovie::Adcovie1,
        }
    }
    #[doc = "0b = Overflow interrupt disabled"]
    #[inline(always)]
    pub fn is_adcovie_0(&self) -> bool {
        *self == Adcovie::Adcovie0
    }
    #[doc = "1b = Overflow interrupt enabled"]
    #[inline(always)]
    pub fn is_adcovie_1(&self) -> bool {
        *self == Adcovie::Adcovie1
    }
}
#[doc = "Field `ADCOVIE` writer - ADCMEM0 overflow interrupt enable."]
pub type AdcovieW<'a, REG> = crate::BitWriter<'a, REG, Adcovie>;
impl<'a, REG> AdcovieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0b = Overflow interrupt disabled"]
    #[inline(always)]
    pub fn adcovie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcovie::Adcovie0)
    }
    #[doc = "1b = Overflow interrupt enabled"]
    #[inline(always)]
    pub fn adcovie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcovie::Adcovie1)
    }
}
#[doc = "ADC conversion-time-overflow interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adctovie {
    #[doc = "0: 0b = Conversion time overflow interrupt disabled"]
    Adctovie0 = 0,
    #[doc = "1: 1b = Conversion time overflow interrupt enabled"]
    Adctovie1 = 1,
}
impl From<Adctovie> for bool {
    #[inline(always)]
    fn from(variant: Adctovie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCTOVIE` reader - ADC conversion-time-overflow interrupt enable."]
pub type AdctovieR = crate::BitReader<Adctovie>;
impl AdctovieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adctovie {
        match self.bits {
            false => Adctovie::Adctovie0,
            true => Adctovie::Adctovie1,
        }
    }
    #[doc = "0b = Conversion time overflow interrupt disabled"]
    #[inline(always)]
    pub fn is_adctovie_0(&self) -> bool {
        *self == Adctovie::Adctovie0
    }
    #[doc = "1b = Conversion time overflow interrupt enabled"]
    #[inline(always)]
    pub fn is_adctovie_1(&self) -> bool {
        *self == Adctovie::Adctovie1
    }
}
#[doc = "Field `ADCTOVIE` writer - ADC conversion-time-overflow interrupt enable."]
pub type AdctovieW<'a, REG> = crate::BitWriter<'a, REG, Adctovie>;
impl<'a, REG> AdctovieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0b = Conversion time overflow interrupt disabled"]
    #[inline(always)]
    pub fn adctovie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adctovie::Adctovie0)
    }
    #[doc = "1b = Conversion time overflow interrupt enabled"]
    #[inline(always)]
    pub fn adctovie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adctovie::Adctovie1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion."]
    #[inline(always)]
    pub fn adcie0(&self) -> Adcie0R {
        Adcie0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable for the inside of window interrupt of the window comparator."]
    #[inline(always)]
    pub fn adcinie(&self) -> AdcinieR {
        AdcinieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for the below lower threshold interrupt of the window comparator."]
    #[inline(always)]
    pub fn adcloie(&self) -> AdcloieR {
        AdcloieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for the above upper threshold interrupt of the window comparator."]
    #[inline(always)]
    pub fn adchiie(&self) -> AdchiieR {
        AdchiieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADCMEM0 overflow interrupt enable."]
    #[inline(always)]
    pub fn adcovie(&self) -> AdcovieR {
        AdcovieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow interrupt enable."]
    #[inline(always)]
    pub fn adctovie(&self) -> AdctovieR {
        AdctovieR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable. This bits enable or disable the interrupt request for a completed ADC conversion."]
    #[inline(always)]
    pub fn adcie0(&mut self) -> Adcie0W<'_, AdcieSpec> {
        Adcie0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt enable for the inside of window interrupt of the window comparator."]
    #[inline(always)]
    pub fn adcinie(&mut self) -> AdcinieW<'_, AdcieSpec> {
        AdcinieW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable for the below lower threshold interrupt of the window comparator."]
    #[inline(always)]
    pub fn adcloie(&mut self) -> AdcloieW<'_, AdcieSpec> {
        AdcloieW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable for the above upper threshold interrupt of the window comparator."]
    #[inline(always)]
    pub fn adchiie(&mut self) -> AdchiieW<'_, AdcieSpec> {
        AdchiieW::new(self, 3)
    }
    #[doc = "Bit 4 - ADCMEM0 overflow interrupt enable."]
    #[inline(always)]
    pub fn adcovie(&mut self) -> AdcovieW<'_, AdcieSpec> {
        AdcovieW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC conversion-time-overflow interrupt enable."]
    #[inline(always)]
    pub fn adctovie(&mut self) -> AdctovieW<'_, AdcieSpec> {
        AdctovieW::new(self, 5)
    }
}
#[doc = "ADC Interrupt Enable 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcieSpec;
impl crate::RegisterSpec for AdcieSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcie::R`](R) reader structure"]
impl crate::Readable for AdcieSpec {}
#[doc = "`write(|w| ..)` method takes [`adcie::W`](W) writer structure"]
impl crate::Writable for AdcieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCIE to value 0"]
impl crate::Resettable for AdcieSpec {}
