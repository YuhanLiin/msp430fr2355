#[doc = "Register `ADCCTL0` reader"]
pub type R = crate::R<Adcctl0Spec>;
#[doc = "Register `ADCCTL0` writer"]
pub type W = crate::W<Adcctl0Spec>;
#[doc = "start conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcsc {
    #[doc = "0: No sample-and-conversion-start"]
    Adcsc0 = 0,
    #[doc = "1: Start sample-and-conversion"]
    Adcsc1 = 1,
}
impl From<Adcsc> for bool {
    #[inline(always)]
    fn from(variant: Adcsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCSC` reader - start conversion"]
pub type AdcscR = crate::BitReader<Adcsc>;
impl AdcscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcsc {
        match self.bits {
            false => Adcsc::Adcsc0,
            true => Adcsc::Adcsc1,
        }
    }
    #[doc = "No sample-and-conversion-start"]
    #[inline(always)]
    pub fn is_adcsc_0(&self) -> bool {
        *self == Adcsc::Adcsc0
    }
    #[doc = "Start sample-and-conversion"]
    #[inline(always)]
    pub fn is_adcsc_1(&self) -> bool {
        *self == Adcsc::Adcsc1
    }
}
#[doc = "Field `ADCSC` writer - start conversion"]
pub type AdcscW<'a, REG> = crate::BitWriter<'a, REG, Adcsc>;
impl<'a, REG> AdcscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No sample-and-conversion-start"]
    #[inline(always)]
    pub fn adcsc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsc::Adcsc0)
    }
    #[doc = "Start sample-and-conversion"]
    #[inline(always)]
    pub fn adcsc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsc::Adcsc1)
    }
}
#[doc = "enable conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcenc {
    #[doc = "0: ADC disabled"]
    Adcenc0 = 0,
    #[doc = "1: ADC enabled"]
    Adcenc1 = 1,
}
impl From<Adcenc> for bool {
    #[inline(always)]
    fn from(variant: Adcenc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCENC` reader - enable conversion"]
pub type AdcencR = crate::BitReader<Adcenc>;
impl AdcencR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcenc {
        match self.bits {
            false => Adcenc::Adcenc0,
            true => Adcenc::Adcenc1,
        }
    }
    #[doc = "ADC disabled"]
    #[inline(always)]
    pub fn is_adcenc_0(&self) -> bool {
        *self == Adcenc::Adcenc0
    }
    #[doc = "ADC enabled"]
    #[inline(always)]
    pub fn is_adcenc_1(&self) -> bool {
        *self == Adcenc::Adcenc1
    }
}
#[doc = "Field `ADCENC` writer - enable conversion"]
pub type AdcencW<'a, REG> = crate::BitWriter<'a, REG, Adcenc>;
impl<'a, REG> AdcencW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC disabled"]
    #[inline(always)]
    pub fn adcenc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcenc::Adcenc0)
    }
    #[doc = "ADC enabled"]
    #[inline(always)]
    pub fn adcenc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcenc::Adcenc1)
    }
}
#[doc = "ADC on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcon {
    #[doc = "0: ADC off"]
    Adcon0 = 0,
    #[doc = "1: ADC on"]
    Adcon1 = 1,
}
impl From<Adcon> for bool {
    #[inline(always)]
    fn from(variant: Adcon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCON` reader - ADC on"]
pub type AdconR = crate::BitReader<Adcon>;
impl AdconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcon {
        match self.bits {
            false => Adcon::Adcon0,
            true => Adcon::Adcon1,
        }
    }
    #[doc = "ADC off"]
    #[inline(always)]
    pub fn is_adcon_0(&self) -> bool {
        *self == Adcon::Adcon0
    }
    #[doc = "ADC on"]
    #[inline(always)]
    pub fn is_adcon_1(&self) -> bool {
        *self == Adcon::Adcon1
    }
}
#[doc = "Field `ADCON` writer - ADC on"]
pub type AdconW<'a, REG> = crate::BitWriter<'a, REG, Adcon>;
impl<'a, REG> AdconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC off"]
    #[inline(always)]
    pub fn adcon_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcon::Adcon0)
    }
    #[doc = "ADC on"]
    #[inline(always)]
    pub fn adcon_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcon::Adcon1)
    }
}
#[doc = "sample-and-hold time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcmsc {
    #[doc = "0: The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert."]
    Adcmsc0 = 0,
    #[doc = "1: The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed."]
    Adcmsc1 = 1,
}
impl From<Adcmsc> for bool {
    #[inline(always)]
    fn from(variant: Adcmsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCMSC` reader - sample-and-hold time."]
pub type AdcmscR = crate::BitReader<Adcmsc>;
impl AdcmscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcmsc {
        match self.bits {
            false => Adcmsc::Adcmsc0,
            true => Adcmsc::Adcmsc1,
        }
    }
    #[doc = "The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert."]
    #[inline(always)]
    pub fn is_adcmsc_0(&self) -> bool {
        *self == Adcmsc::Adcmsc0
    }
    #[doc = "The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed."]
    #[inline(always)]
    pub fn is_adcmsc_1(&self) -> bool {
        *self == Adcmsc::Adcmsc1
    }
}
#[doc = "Field `ADCMSC` writer - sample-and-hold time."]
pub type AdcmscW<'a, REG> = crate::BitWriter<'a, REG, Adcmsc>;
impl<'a, REG> AdcmscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The sampling timer requires a rising edge of the SHI signal to trigger each sample-and-convert."]
    #[inline(always)]
    pub fn adcmsc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcmsc::Adcmsc0)
    }
    #[doc = "The incidence of a positive(or for devices first rising edge of the) SHI signal triggers the sampling timer, but further sample-and-conversions are performed automatically as soon as the prior conversion is completed."]
    #[inline(always)]
    pub fn adcmsc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcmsc::Adcmsc1)
    }
}
#[doc = "sample-and-hold time.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcsht {
    #[doc = "0: 4 ADCCLK cycles"]
    Adcsht0 = 0,
    #[doc = "1: 8 ADCCLK cycles"]
    Adcsht1 = 1,
    #[doc = "2: 16 ADCCLK cycles"]
    Adcsht2 = 2,
    #[doc = "3: 32 ADCCLK cycles"]
    Adcsht3 = 3,
    #[doc = "4: 64 ADCCLK cycles"]
    Adcsht4 = 4,
    #[doc = "5: 96 ADCCLK cycles"]
    Adcsht5 = 5,
    #[doc = "6: 128 ADCCLK cycles"]
    Adcsht6 = 6,
    #[doc = "7: 192 ADCCLK cycles"]
    Adcsht7 = 7,
    #[doc = "8: 256 ADCCLK cycles"]
    Adcsht8 = 8,
    #[doc = "9: 384 ADCCLK cycles"]
    Adcsht9 = 9,
    #[doc = "10: 512 ADCCLK cycles"]
    Adcsht10 = 10,
    #[doc = "11: 768 ADCCLK cycles"]
    Adcsht11 = 11,
    #[doc = "12: 1024 ADCCLK cycles"]
    Adcsht12 = 12,
    #[doc = "13: 1024 ADCCLK cycles"]
    Adcsht13 = 13,
    #[doc = "14: 1024 ADCCLK cycles"]
    Adcsht14 = 14,
    #[doc = "15: 1024 ADCCLK cycles"]
    Adcsht15 = 15,
}
impl From<Adcsht> for u8 {
    #[inline(always)]
    fn from(variant: Adcsht) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcsht {
    type Ux = u8;
}
impl crate::IsEnum for Adcsht {}
#[doc = "Field `ADCSHT` reader - sample-and-hold time."]
pub type AdcshtR = crate::FieldReader<Adcsht>;
impl AdcshtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcsht {
        match self.bits {
            0 => Adcsht::Adcsht0,
            1 => Adcsht::Adcsht1,
            2 => Adcsht::Adcsht2,
            3 => Adcsht::Adcsht3,
            4 => Adcsht::Adcsht4,
            5 => Adcsht::Adcsht5,
            6 => Adcsht::Adcsht6,
            7 => Adcsht::Adcsht7,
            8 => Adcsht::Adcsht8,
            9 => Adcsht::Adcsht9,
            10 => Adcsht::Adcsht10,
            11 => Adcsht::Adcsht11,
            12 => Adcsht::Adcsht12,
            13 => Adcsht::Adcsht13,
            14 => Adcsht::Adcsht14,
            15 => Adcsht::Adcsht15,
            _ => unreachable!(),
        }
    }
    #[doc = "4 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_0(&self) -> bool {
        *self == Adcsht::Adcsht0
    }
    #[doc = "8 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_1(&self) -> bool {
        *self == Adcsht::Adcsht1
    }
    #[doc = "16 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_2(&self) -> bool {
        *self == Adcsht::Adcsht2
    }
    #[doc = "32 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_3(&self) -> bool {
        *self == Adcsht::Adcsht3
    }
    #[doc = "64 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_4(&self) -> bool {
        *self == Adcsht::Adcsht4
    }
    #[doc = "96 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_5(&self) -> bool {
        *self == Adcsht::Adcsht5
    }
    #[doc = "128 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_6(&self) -> bool {
        *self == Adcsht::Adcsht6
    }
    #[doc = "192 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_7(&self) -> bool {
        *self == Adcsht::Adcsht7
    }
    #[doc = "256 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_8(&self) -> bool {
        *self == Adcsht::Adcsht8
    }
    #[doc = "384 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_9(&self) -> bool {
        *self == Adcsht::Adcsht9
    }
    #[doc = "512 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_10(&self) -> bool {
        *self == Adcsht::Adcsht10
    }
    #[doc = "768 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_11(&self) -> bool {
        *self == Adcsht::Adcsht11
    }
    #[doc = "1024 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_12(&self) -> bool {
        *self == Adcsht::Adcsht12
    }
    #[doc = "1024 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_13(&self) -> bool {
        *self == Adcsht::Adcsht13
    }
    #[doc = "1024 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_14(&self) -> bool {
        *self == Adcsht::Adcsht14
    }
    #[doc = "1024 ADCCLK cycles"]
    #[inline(always)]
    pub fn is_adcsht_15(&self) -> bool {
        *self == Adcsht::Adcsht15
    }
}
#[doc = "Field `ADCSHT` writer - sample-and-hold time."]
pub type AdcshtW<'a, REG> = crate::FieldWriter<'a, REG, 4, Adcsht, crate::Safe>;
impl<'a, REG> AdcshtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht0)
    }
    #[doc = "8 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht1)
    }
    #[doc = "16 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht2)
    }
    #[doc = "32 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht3)
    }
    #[doc = "64 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_4(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht4)
    }
    #[doc = "96 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_5(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht5)
    }
    #[doc = "128 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_6(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht6)
    }
    #[doc = "192 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_7(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht7)
    }
    #[doc = "256 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_8(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht8)
    }
    #[doc = "384 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_9(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht9)
    }
    #[doc = "512 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_10(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht10)
    }
    #[doc = "768 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_11(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht11)
    }
    #[doc = "1024 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_12(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht12)
    }
    #[doc = "1024 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_13(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht13)
    }
    #[doc = "1024 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_14(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht14)
    }
    #[doc = "1024 ADCCLK cycles"]
    #[inline(always)]
    pub fn adcsht_15(self) -> &'a mut crate::W<REG> {
        self.variant(Adcsht::Adcsht15)
    }
}
impl R {
    #[doc = "Bit 0 - start conversion"]
    #[inline(always)]
    pub fn adcsc(&self) -> AdcscR {
        AdcscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable conversion"]
    #[inline(always)]
    pub fn adcenc(&self) -> AdcencR {
        AdcencR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC on"]
    #[inline(always)]
    pub fn adcon(&self) -> AdconR {
        AdconR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - sample-and-hold time."]
    #[inline(always)]
    pub fn adcmsc(&self) -> AdcmscR {
        AdcmscR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - sample-and-hold time."]
    #[inline(always)]
    pub fn adcsht(&self) -> AdcshtR {
        AdcshtR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - start conversion"]
    #[inline(always)]
    pub fn adcsc(&mut self) -> AdcscW<'_, Adcctl0Spec> {
        AdcscW::new(self, 0)
    }
    #[doc = "Bit 1 - enable conversion"]
    #[inline(always)]
    pub fn adcenc(&mut self) -> AdcencW<'_, Adcctl0Spec> {
        AdcencW::new(self, 1)
    }
    #[doc = "Bit 4 - ADC on"]
    #[inline(always)]
    pub fn adcon(&mut self) -> AdconW<'_, Adcctl0Spec> {
        AdconW::new(self, 4)
    }
    #[doc = "Bit 7 - sample-and-hold time."]
    #[inline(always)]
    pub fn adcmsc(&mut self) -> AdcmscW<'_, Adcctl0Spec> {
        AdcmscW::new(self, 7)
    }
    #[doc = "Bits 8:11 - sample-and-hold time."]
    #[inline(always)]
    pub fn adcsht(&mut self) -> AdcshtW<'_, Adcctl0Spec> {
        AdcshtW::new(self, 8)
    }
}
#[doc = "ADC Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcctl0Spec;
impl crate::RegisterSpec for Adcctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcctl0::R`](R) reader structure"]
impl crate::Readable for Adcctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`adcctl0::W`](W) writer structure"]
impl crate::Writable for Adcctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCCTL0 to value 0"]
impl crate::Resettable for Adcctl0Spec {}
