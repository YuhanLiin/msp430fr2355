#[doc = "Register `ADCCTL2` reader"]
pub type R = crate::R<Adcctl2Spec>;
#[doc = "Register `ADCCTL2` writer"]
pub type W = crate::W<Adcctl2Spec>;
#[doc = "ADC sampling rate.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcsr {
    #[doc = "0: ADC buffer supports up to approximately 200 ksps"]
    Max200ksps = 0,
    #[doc = "1: ADC buffer supports up to approximately 50 ksps"]
    Max50ksps = 1,
}
impl From<Adcsr> for bool {
    #[inline(always)]
    fn from(variant: Adcsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCSR` reader - ADC sampling rate."]
pub type AdcsrR = crate::BitReader<Adcsr>;
impl AdcsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcsr {
        match self.bits {
            false => Adcsr::Max200ksps,
            true => Adcsr::Max50ksps,
        }
    }
    #[doc = "ADC buffer supports up to approximately 200 ksps"]
    #[inline(always)]
    pub fn is_max_200ksps(&self) -> bool {
        *self == Adcsr::Max200ksps
    }
    #[doc = "ADC buffer supports up to approximately 50 ksps"]
    #[inline(always)]
    pub fn is_max_50ksps(&self) -> bool {
        *self == Adcsr::Max50ksps
    }
}
#[doc = "Field `ADCSR` writer - ADC sampling rate."]
pub type AdcsrW<'a, REG> = crate::BitWriter<'a, REG, Adcsr>;
impl<'a, REG> AdcsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC buffer supports up to approximately 200 ksps"]
    #[inline(always)]
    pub fn max_200ksps(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsr::Max200ksps)
    }
    #[doc = "ADC buffer supports up to approximately 50 ksps"]
    #[inline(always)]
    pub fn max_50ksps(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsr::Max50ksps)
    }
}
#[doc = "data read-back format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcdf {
    #[doc = "0: Binary unsigned. Theoretically, the analog input voltage -VREF results in 0000h, and the analog input voltage +VREF results in 03FFh."]
    Unsigned = 0,
    #[doc = "1: Signed binary (2s complement), left aligned. Theoretically, the analog input voltage -VREF results in 8000h, and the analog input voltage +VREF results in 7FC0h."]
    Signed = 1,
}
impl From<Adcdf> for bool {
    #[inline(always)]
    fn from(variant: Adcdf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCDF` reader - data read-back format"]
pub type AdcdfR = crate::BitReader<Adcdf>;
impl AdcdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcdf {
        match self.bits {
            false => Adcdf::Unsigned,
            true => Adcdf::Signed,
        }
    }
    #[doc = "Binary unsigned. Theoretically, the analog input voltage -VREF results in 0000h, and the analog input voltage +VREF results in 03FFh."]
    #[inline(always)]
    pub fn is_unsigned(&self) -> bool {
        *self == Adcdf::Unsigned
    }
    #[doc = "Signed binary (2s complement), left aligned. Theoretically, the analog input voltage -VREF results in 8000h, and the analog input voltage +VREF results in 7FC0h."]
    #[inline(always)]
    pub fn is_signed(&self) -> bool {
        *self == Adcdf::Signed
    }
}
#[doc = "Field `ADCDF` writer - data read-back format"]
pub type AdcdfW<'a, REG> = crate::BitWriter<'a, REG, Adcdf>;
impl<'a, REG> AdcdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Binary unsigned. Theoretically, the analog input voltage -VREF results in 0000h, and the analog input voltage +VREF results in 03FFh."]
    #[inline(always)]
    pub fn unsigned(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdf::Unsigned)
    }
    #[doc = "Signed binary (2s complement), left aligned. Theoretically, the analog input voltage -VREF results in 8000h, and the analog input voltage +VREF results in 7FC0h."]
    #[inline(always)]
    pub fn signed(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdf::Signed)
    }
}
#[doc = "resolution\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcres {
    #[doc = "0: 8 bit (10 clock cycle conversion time)"]
    Bits8 = 0,
    #[doc = "1: 10 bit (12 clock cycle conversion time)"]
    Bits10 = 1,
    #[doc = "2: 12 bit (14 clock cycle conversion time)"]
    Bits12 = 2,
}
impl From<Adcres> for u8 {
    #[inline(always)]
    fn from(variant: Adcres) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcres {
    type Ux = u8;
}
impl crate::IsEnum for Adcres {}
#[doc = "Field `ADCRES` reader - resolution"]
pub type AdcresR = crate::FieldReader<Adcres>;
impl AdcresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adcres> {
        match self.bits {
            0 => Some(Adcres::Bits8),
            1 => Some(Adcres::Bits10),
            2 => Some(Adcres::Bits12),
            _ => None,
        }
    }
    #[doc = "8 bit (10 clock cycle conversion time)"]
    #[inline(always)]
    pub fn is_bits_8(&self) -> bool {
        *self == Adcres::Bits8
    }
    #[doc = "10 bit (12 clock cycle conversion time)"]
    #[inline(always)]
    pub fn is_bits_10(&self) -> bool {
        *self == Adcres::Bits10
    }
    #[doc = "12 bit (14 clock cycle conversion time)"]
    #[inline(always)]
    pub fn is_bits_12(&self) -> bool {
        *self == Adcres::Bits12
    }
}
#[doc = "Field `ADCRES` writer - resolution"]
pub type AdcresW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcres>;
impl<'a, REG> AdcresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit (10 clock cycle conversion time)"]
    #[inline(always)]
    pub fn bits_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adcres::Bits8)
    }
    #[doc = "10 bit (12 clock cycle conversion time)"]
    #[inline(always)]
    pub fn bits_10(self) -> &'a mut crate::W<REG> {
        self.variant(Adcres::Bits10)
    }
    #[doc = "12 bit (14 clock cycle conversion time)"]
    #[inline(always)]
    pub fn bits_12(self) -> &'a mut crate::W<REG> {
        self.variant(Adcres::Bits12)
    }
}
#[doc = "ADC predivider. This bit predivides the selected ADC clock source before it gets divided again using ADCDIVx.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcpdiv {
    #[doc = "0: Predivide by 1"]
    _1 = 0,
    #[doc = "1: Predivide by 4"]
    _4 = 1,
    #[doc = "2: Predivide by 64"]
    _64 = 2,
}
impl From<Adcpdiv> for u8 {
    #[inline(always)]
    fn from(variant: Adcpdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcpdiv {
    type Ux = u8;
}
impl crate::IsEnum for Adcpdiv {}
#[doc = "Field `ADCPDIV` reader - ADC predivider. This bit predivides the selected ADC clock source before it gets divided again using ADCDIVx."]
pub type AdcpdivR = crate::FieldReader<Adcpdiv>;
impl AdcpdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adcpdiv> {
        match self.bits {
            0 => Some(Adcpdiv::_1),
            1 => Some(Adcpdiv::_4),
            2 => Some(Adcpdiv::_64),
            _ => None,
        }
    }
    #[doc = "Predivide by 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adcpdiv::_1
    }
    #[doc = "Predivide by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Adcpdiv::_4
    }
    #[doc = "Predivide by 64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Adcpdiv::_64
    }
}
#[doc = "Field `ADCPDIV` writer - ADC predivider. This bit predivides the selected ADC clock source before it gets divided again using ADCDIVx."]
pub type AdcpdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcpdiv>;
impl<'a, REG> AdcpdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Predivide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpdiv::_1)
    }
    #[doc = "Predivide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpdiv::_4)
    }
    #[doc = "Predivide by 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Adcpdiv::_64)
    }
}
impl R {
    #[doc = "Bit 2 - ADC sampling rate."]
    #[inline(always)]
    pub fn adcsr(&self) -> AdcsrR {
        AdcsrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - data read-back format"]
    #[inline(always)]
    pub fn adcdf(&self) -> AdcdfR {
        AdcdfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - resolution"]
    #[inline(always)]
    pub fn adcres(&self) -> AdcresR {
        AdcresR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ADC predivider. This bit predivides the selected ADC clock source before it gets divided again using ADCDIVx."]
    #[inline(always)]
    pub fn adcpdiv(&self) -> AdcpdivR {
        AdcpdivR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - ADC sampling rate."]
    #[inline(always)]
    pub fn adcsr(&mut self) -> AdcsrW<'_, Adcctl2Spec> {
        AdcsrW::new(self, 2)
    }
    #[doc = "Bit 3 - data read-back format"]
    #[inline(always)]
    pub fn adcdf(&mut self) -> AdcdfW<'_, Adcctl2Spec> {
        AdcdfW::new(self, 3)
    }
    #[doc = "Bits 4:5 - resolution"]
    #[inline(always)]
    pub fn adcres(&mut self) -> AdcresW<'_, Adcctl2Spec> {
        AdcresW::new(self, 4)
    }
    #[doc = "Bits 8:9 - ADC predivider. This bit predivides the selected ADC clock source before it gets divided again using ADCDIVx."]
    #[inline(always)]
    pub fn adcpdiv(&mut self) -> AdcpdivW<'_, Adcctl2Spec> {
        AdcpdivW::new(self, 8)
    }
}
#[doc = "ADC Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcctl2Spec;
impl crate::RegisterSpec for Adcctl2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcctl2::R`](R) reader structure"]
impl crate::Readable for Adcctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`adcctl2::W`](W) writer structure"]
impl crate::Writable for Adcctl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCCTL2 to value 0"]
impl crate::Resettable for Adcctl2Spec {}
