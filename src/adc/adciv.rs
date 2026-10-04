#[doc = "Register `ADCIV` reader"]
pub type R = crate::R<AdcivSpec>;
#[doc = "Register `ADCIV` writer"]
pub type W = crate::W<AdcivSpec>;
#[doc = "interrupt vector value\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Adciv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: ADCMEM0 overflow; Interrupt Flag: ADCOVIFG; Interrupt Priority: Highest"]
    Overflow = 2,
    #[doc = "4: Interrupt Source: Conversion time overflow; Interrupt Flag: ADCTOVIFG"]
    TimeOverflow = 4,
    #[doc = "6: Interrupt Source: ADCHI Interrupt flag; Interrupt Flag: ADCHIIFG"]
    AboveWindow = 6,
    #[doc = "8: Interrupt Source: ADCLO Interrupt flag; Interrupt Flag: ADCLOIFG"]
    BelowWindow = 8,
    #[doc = "10: Interrupt Source: ADCIN Interrupt flag; Interrupt Flag: ADCINIFG"]
    InsideWindow = 10,
    #[doc = "12: Interrupt Source: ADC memory Interrupt flag; Interrupt Flag: ADCIFG0; Interrupt Priority: Lowest"]
    ResultReady = 12,
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
            2 => Some(Adciv::Overflow),
            4 => Some(Adciv::TimeOverflow),
            6 => Some(Adciv::AboveWindow),
            8 => Some(Adciv::BelowWindow),
            10 => Some(Adciv::InsideWindow),
            12 => Some(Adciv::ResultReady),
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
    pub fn is_overflow(&self) -> bool {
        *self == Adciv::Overflow
    }
    #[doc = "Interrupt Source: Conversion time overflow; Interrupt Flag: ADCTOVIFG"]
    #[inline(always)]
    pub fn is_time_overflow(&self) -> bool {
        *self == Adciv::TimeOverflow
    }
    #[doc = "Interrupt Source: ADCHI Interrupt flag; Interrupt Flag: ADCHIIFG"]
    #[inline(always)]
    pub fn is_above_window(&self) -> bool {
        *self == Adciv::AboveWindow
    }
    #[doc = "Interrupt Source: ADCLO Interrupt flag; Interrupt Flag: ADCLOIFG"]
    #[inline(always)]
    pub fn is_below_window(&self) -> bool {
        *self == Adciv::BelowWindow
    }
    #[doc = "Interrupt Source: ADCIN Interrupt flag; Interrupt Flag: ADCINIFG"]
    #[inline(always)]
    pub fn is_inside_window(&self) -> bool {
        *self == Adciv::InsideWindow
    }
    #[doc = "Interrupt Source: ADC memory Interrupt flag; Interrupt Flag: ADCIFG0; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_result_ready(&self) -> bool {
        *self == Adciv::ResultReady
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
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::Overflow)
    }
    #[doc = "Interrupt Source: Conversion time overflow; Interrupt Flag: ADCTOVIFG"]
    #[inline(always)]
    pub fn time_overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::TimeOverflow)
    }
    #[doc = "Interrupt Source: ADCHI Interrupt flag; Interrupt Flag: ADCHIIFG"]
    #[inline(always)]
    pub fn above_window(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::AboveWindow)
    }
    #[doc = "Interrupt Source: ADCLO Interrupt flag; Interrupt Flag: ADCLOIFG"]
    #[inline(always)]
    pub fn below_window(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::BelowWindow)
    }
    #[doc = "Interrupt Source: ADCIN Interrupt flag; Interrupt Flag: ADCINIFG"]
    #[inline(always)]
    pub fn inside_window(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::InsideWindow)
    }
    #[doc = "Interrupt Source: ADC memory Interrupt flag; Interrupt Flag: ADCIFG0; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn result_ready(self) -> &'a mut crate::W<REG> {
        self.variant(Adciv::ResultReady)
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
