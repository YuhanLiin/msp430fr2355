#[doc = "Register `ADCIFG` reader"]
pub type R = crate::R<AdcifgSpec>;
#[doc = "Register `ADCIFG` writer"]
pub type W = crate::W<AdcifgSpec>;
#[doc = "ADCMEM0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcifg0 {
    #[doc = "0: No interrupt pending"]
    Adcifg0_0 = 0,
    #[doc = "1: Interrupt pending"]
    Adcifg0_1 = 1,
}
impl From<Adcifg0> for bool {
    #[inline(always)]
    fn from(variant: Adcifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCIFG0` reader - ADCMEM0 interrupt flag"]
pub type Adcifg0R = crate::BitReader<Adcifg0>;
impl Adcifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcifg0 {
        match self.bits {
            false => Adcifg0::Adcifg0_0,
            true => Adcifg0::Adcifg0_1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adcifg0_0(&self) -> bool {
        *self == Adcifg0::Adcifg0_0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adcifg0_1(&self) -> bool {
        *self == Adcifg0::Adcifg0_1
    }
}
#[doc = "Field `ADCIFG0` writer - ADCMEM0 interrupt flag"]
pub type Adcifg0W<'a, REG> = crate::BitWriter<'a, REG, Adcifg0>;
impl<'a, REG> Adcifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcifg0_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcifg0::Adcifg0_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adcifg0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcifg0::Adcifg0_1)
    }
}
#[doc = "The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcinifg {
    #[doc = "0: No interrupt pending"]
    Adcinifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adcinifg1 = 1,
}
impl From<Adcinifg> for bool {
    #[inline(always)]
    fn from(variant: Adcinifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCINIFG` reader - The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers."]
pub type AdcinifgR = crate::BitReader<Adcinifg>;
impl AdcinifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcinifg {
        match self.bits {
            false => Adcinifg::Adcinifg0,
            true => Adcinifg::Adcinifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adcinifg_0(&self) -> bool {
        *self == Adcinifg::Adcinifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adcinifg_1(&self) -> bool {
        *self == Adcinifg::Adcinifg1
    }
}
#[doc = "Field `ADCINIFG` writer - The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers."]
pub type AdcinifgW<'a, REG> = crate::BitWriter<'a, REG, Adcinifg>;
impl<'a, REG> AdcinifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcinifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinifg::Adcinifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adcinifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcinifg::Adcinifg1)
    }
}
#[doc = "The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcloifg {
    #[doc = "0: No interrupt pending"]
    Adcloifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adcloifg1 = 1,
}
impl From<Adcloifg> for bool {
    #[inline(always)]
    fn from(variant: Adcloifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCLOIFG` reader - The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register."]
pub type AdcloifgR = crate::BitReader<Adcloifg>;
impl AdcloifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcloifg {
        match self.bits {
            false => Adcloifg::Adcloifg0,
            true => Adcloifg::Adcloifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adcloifg_0(&self) -> bool {
        *self == Adcloifg::Adcloifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adcloifg_1(&self) -> bool {
        *self == Adcloifg::Adcloifg1
    }
}
#[doc = "Field `ADCLOIFG` writer - The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register."]
pub type AdcloifgW<'a, REG> = crate::BitWriter<'a, REG, Adcloifg>;
impl<'a, REG> AdcloifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcloifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcloifg::Adcloifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adcloifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcloifg::Adcloifg1)
    }
}
#[doc = "The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adchiifg {
    #[doc = "0: No interrupt pending"]
    Adchiifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adchiifg1 = 1,
}
impl From<Adchiifg> for bool {
    #[inline(always)]
    fn from(variant: Adchiifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCHIIFG` reader - The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register."]
pub type AdchiifgR = crate::BitReader<Adchiifg>;
impl AdchiifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adchiifg {
        match self.bits {
            false => Adchiifg::Adchiifg0,
            true => Adchiifg::Adchiifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adchiifg_0(&self) -> bool {
        *self == Adchiifg::Adchiifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adchiifg_1(&self) -> bool {
        *self == Adchiifg::Adchiifg1
    }
}
#[doc = "Field `ADCHIIFG` writer - The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register."]
pub type AdchiifgW<'a, REG> = crate::BitWriter<'a, REG, Adchiifg>;
impl<'a, REG> AdchiifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adchiifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adchiifg::Adchiifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adchiifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adchiifg::Adchiifg1)
    }
}
#[doc = "The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcovifg {
    #[doc = "0: No interrupt pending"]
    Adcovifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adcovifg1 = 1,
}
impl From<Adcovifg> for bool {
    #[inline(always)]
    fn from(variant: Adcovifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCOVIFG` reader - The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read."]
pub type AdcovifgR = crate::BitReader<Adcovifg>;
impl AdcovifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcovifg {
        match self.bits {
            false => Adcovifg::Adcovifg0,
            true => Adcovifg::Adcovifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adcovifg_0(&self) -> bool {
        *self == Adcovifg::Adcovifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adcovifg_1(&self) -> bool {
        *self == Adcovifg::Adcovifg1
    }
}
#[doc = "Field `ADCOVIFG` writer - The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read."]
pub type AdcovifgW<'a, REG> = crate::BitWriter<'a, REG, Adcovifg>;
impl<'a, REG> AdcovifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcovifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcovifg::Adcovifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adcovifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcovifg::Adcovifg1)
    }
}
#[doc = "The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adctovifg {
    #[doc = "0: No interrupt pending"]
    Adcovifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Adctovifg1 = 1,
}
impl From<Adctovifg> for bool {
    #[inline(always)]
    fn from(variant: Adctovifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCTOVIFG` reader - The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed."]
pub type AdctovifgR = crate::BitReader<Adctovifg>;
impl AdctovifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adctovifg {
        match self.bits {
            false => Adctovifg::Adcovifg0,
            true => Adctovifg::Adctovifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_adcovifg_0(&self) -> bool {
        *self == Adctovifg::Adcovifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_adctovifg_1(&self) -> bool {
        *self == Adctovifg::Adctovifg1
    }
}
#[doc = "Field `ADCTOVIFG` writer - The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed."]
pub type AdctovifgW<'a, REG> = crate::BitWriter<'a, REG, Adctovifg>;
impl<'a, REG> AdctovifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn adcovifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adctovifg::Adcovifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn adctovifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adctovifg::Adctovifg1)
    }
}
impl R {
    #[doc = "Bit 0 - ADCMEM0 interrupt flag"]
    #[inline(always)]
    pub fn adcifg0(&self) -> Adcifg0R {
        Adcifg0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers."]
    #[inline(always)]
    pub fn adcinifg(&self) -> AdcinifgR {
        AdcinifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register."]
    #[inline(always)]
    pub fn adcloifg(&self) -> AdcloifgR {
        AdcloifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register."]
    #[inline(always)]
    pub fn adchiifg(&self) -> AdchiifgR {
        AdchiifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read."]
    #[inline(always)]
    pub fn adcovifg(&self) -> AdcovifgR {
        AdcovifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed."]
    #[inline(always)]
    pub fn adctovifg(&self) -> AdctovifgR {
        AdctovifgR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADCMEM0 interrupt flag"]
    #[inline(always)]
    pub fn adcifg0(&mut self) -> Adcifg0W<'_, AdcifgSpec> {
        Adcifg0W::new(self, 0)
    }
    #[doc = "Bit 1 - The ADCINIFG is set when the result of the current ADC conversion is within the thresholds defined by the window comparator threshold registers."]
    #[inline(always)]
    pub fn adcinifg(&mut self) -> AdcinifgW<'_, AdcifgSpec> {
        AdcinifgW::new(self, 1)
    }
    #[doc = "Bit 2 - The ADCLOIFG is set when the result of the current ADC conversion is below the lower threshold defined by the window comparator lower threshold register."]
    #[inline(always)]
    pub fn adcloifg(&mut self) -> AdcloifgW<'_, AdcifgSpec> {
        AdcloifgW::new(self, 2)
    }
    #[doc = "Bit 3 - The ADCHIIFG is set when the result of the current ADC conversion is greater than the upper threshold defined by the window comparator upper threshold register."]
    #[inline(always)]
    pub fn adchiifg(&mut self) -> AdchiifgW<'_, AdcifgSpec> {
        AdchiifgW::new(self, 3)
    }
    #[doc = "Bit 4 - The ADCOVIFG is set when the ADCMEM0 register is written before the last conversion result has been read."]
    #[inline(always)]
    pub fn adcovifg(&mut self) -> AdcovifgW<'_, AdcifgSpec> {
        AdcovifgW::new(self, 4)
    }
    #[doc = "Bit 5 - The ADCTOVIFG is set when an ADC conversion is triggered before the actual conversion has completed."]
    #[inline(always)]
    pub fn adctovifg(&mut self) -> AdctovifgW<'_, AdcifgSpec> {
        AdctovifgW::new(self, 5)
    }
}
#[doc = "ADC Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`adcifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcifgSpec;
impl crate::RegisterSpec for AdcifgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcifg::R`](R) reader structure"]
impl crate::Readable for AdcifgSpec {}
#[doc = "`write(|w| ..)` method takes [`adcifg::W`](W) writer structure"]
impl crate::Writable for AdcifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCIFG to value 0"]
impl crate::Resettable for AdcifgSpec {}
