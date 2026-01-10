#[doc = "Register `ADCIV` reader"]
pub type R = crate::R<AdcivSpec>;
#[doc = "Register `ADCIV` writer"]
pub type W = crate::W<AdcivSpec>;
#[doc = "interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Adciv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: ADCMEM0 overflow; Interrupt Flag: ADCOVIFG; Interrupt Priority: Highest"]
    Adcovifg = 2,
    #[doc = "4: Interrupt Source: Conversion time overflow; Interrupt Flag: ADCTOVIFG"]
    Adctovifg = 4,
    #[doc = "6: Interrupt Source: ADCHI Interrupt flag; Interrupt Flag: ADCHIIFG"]
    Adchiifg = 6,
    #[doc = "8: Interrupt Source: ADCLO Interrupt flag; Interrupt Flag: ADCLOIFG"]
    Adcloifg = 8,
    #[doc = "10: nterrupt Source: ADCIN Interrupt flag; Interrupt Flag: ADCINIFG"]
    Adcinifg = 10,
    #[doc = "12: Interrupt Source: ADC memory Interrupt flag; Interrupt Flag: ADCIFG0; Interrupt Priority: Lowest"]
    Adcifg0 = 12,
}
impl From<Adciv> for u16 {
    #[inline(always)]
    fn from(variant: Adciv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adciv {
    type Ux = u16;
}
impl crate::IsEnum for Adciv {}
#[doc = "Field `ADCIV` reader - interrupt vector value"]
pub type AdcivR = crate::FieldReader<Adciv>;
impl AdcivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adciv> {
        match self.bits {
            0 => Some(Adciv::None),
            2 => Some(Adciv::Adcovifg),
            4 => Some(Adciv::Adctovifg),
            6 => Some(Adciv::Adchiifg),
            8 => Some(Adciv::Adcloifg),
            10 => Some(Adciv::Adcinifg),
            12 => Some(Adciv::Adcifg0),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Adciv::None
    }
    #[doc = "Interrupt Source: ADCMEM0 overflow; Interrupt Flag: ADCOVIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_adcovifg(&self) -> bool {
        *self == Adciv::Adcovifg
    }
    #[doc = "Interrupt Source: Conversion time overflow; Interrupt Flag: ADCTOVIFG"]
    #[inline(always)]
    pub fn is_adctovifg(&self) -> bool {
        *self == Adciv::Adctovifg
    }
    #[doc = "Interrupt Source: ADCHI Interrupt flag; Interrupt Flag: ADCHIIFG"]
    #[inline(always)]
    pub fn is_adchiifg(&self) -> bool {
        *self == Adciv::Adchiifg
    }
    #[doc = "Interrupt Source: ADCLO Interrupt flag; Interrupt Flag: ADCLOIFG"]
    #[inline(always)]
    pub fn is_adcloifg(&self) -> bool {
        *self == Adciv::Adcloifg
    }
    #[doc = "nterrupt Source: ADCIN Interrupt flag; Interrupt Flag: ADCINIFG"]
    #[inline(always)]
    pub fn is_adcinifg(&self) -> bool {
        *self == Adciv::Adcinifg
    }
    #[doc = "Interrupt Source: ADC memory Interrupt flag; Interrupt Flag: ADCIFG0; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_adcifg0(&self) -> bool {
        *self == Adciv::Adcifg0
    }
}
#[doc = "Field `ADCIV` writer - interrupt vector value"]
pub type AdcivW<'a, REG> = crate::FieldWriter<'a, REG, 16, Adciv>;
impl<'a, REG> AdcivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::None)
    }
    #[doc = "Interrupt Source: ADCMEM0 overflow; Interrupt Flag: ADCOVIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn adcovifg(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::Adcovifg)
    }
    #[doc = "Interrupt Source: Conversion time overflow; Interrupt Flag: ADCTOVIFG"]
    #[inline(always)]
    pub fn adctovifg(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::Adctovifg)
    }
    #[doc = "Interrupt Source: ADCHI Interrupt flag; Interrupt Flag: ADCHIIFG"]
    #[inline(always)]
    pub fn adchiifg(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::Adchiifg)
    }
    #[doc = "Interrupt Source: ADCLO Interrupt flag; Interrupt Flag: ADCLOIFG"]
    #[inline(always)]
    pub fn adcloifg(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::Adcloifg)
    }
    #[doc = "nterrupt Source: ADCIN Interrupt flag; Interrupt Flag: ADCINIFG"]
    #[inline(always)]
    pub fn adcinifg(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::Adcinifg)
    }
    #[doc = "Interrupt Source: ADC memory Interrupt flag; Interrupt Flag: ADCIFG0; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn adcifg0(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::Adcifg0)
    }
}
impl R {
    #[doc = "Bits 0:15 - interrupt vector value"]
    #[inline(always)]
    pub fn adciv(&self) -> AdcivR {
        AdcivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - interrupt vector value"]
    #[inline(always)]
    pub fn adciv(&mut self) -> AdcivW<'_, AdcivSpec> {
        AdcivW::new(self, 0)
    }
}
#[doc = "ADC Interrupt Vector\n\nYou can [`read`](crate::Reg::read) this register and get [`adciv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adciv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcivSpec;
impl crate::RegisterSpec for AdcivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adciv::R`](R) reader structure"]
impl crate::Readable for AdcivSpec {}
#[doc = "`write(|w| ..)` method takes [`adciv::W`](W) writer structure"]
impl crate::Writable for AdcivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCIV to value 0"]
impl crate::Resettable for AdcivSpec {}
