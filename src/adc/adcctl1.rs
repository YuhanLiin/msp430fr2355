#[doc = "Register `ADCCTL1` reader"]
pub type R = crate::R<Adcctl1Spec>;
#[doc = "Register `ADCCTL1` writer"]
pub type W = crate::W<Adcctl1Spec>;
#[doc = "ADC busy\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcbusy {
    #[doc = "0: No operation is active."]
    Adcbusy0 = 0,
    #[doc = "1: A sequence, sample, or conversion is active."]
    Adcbusy1 = 1,
}
impl From<Adcbusy> for bool {
    #[inline(always)]
    fn from(variant: Adcbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCBUSY` reader - ADC busy"]
pub type AdcbusyR = crate::BitReader<Adcbusy>;
impl AdcbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcbusy {
        match self.bits {
            false => Adcbusy::Adcbusy0,
            true => Adcbusy::Adcbusy1,
        }
    }
    #[doc = "No operation is active."]
    #[inline(always)]
    pub fn is_adcbusy_0(&self) -> bool {
        *self == Adcbusy::Adcbusy0
    }
    #[doc = "A sequence, sample, or conversion is active."]
    #[inline(always)]
    pub fn is_adcbusy_1(&self) -> bool {
        *self == Adcbusy::Adcbusy1
    }
}
#[doc = "conversion sequence mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcconseq {
    #[doc = "0: Single-channel, single-conversion: a single channel is converted once"]
    Single = 0,
    #[doc = "1: Sequence-of-channels: a sequence of channels is converted once"]
    Sequence = 1,
    #[doc = "2: Repeat-single-channel: a single channel is converted repeatedly"]
    RepeatSingle = 2,
    #[doc = "3: Repeat-sequence-of-channels: a sequence of channels is converted repeatedly"]
    RepeatSequence = 3,
}
impl From<Adcconseq> for u8 {
    #[inline(always)]
    fn from(variant: Adcconseq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcconseq {
    type Ux = u8;
}
impl crate::IsEnum for Adcconseq {}
#[doc = "Field `ADCCONSEQ` reader - conversion sequence mode select"]
pub type AdcconseqR = crate::FieldReader<Adcconseq>;
impl AdcconseqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcconseq {
        match self.bits {
            0 => Adcconseq::Single,
            1 => Adcconseq::Sequence,
            2 => Adcconseq::RepeatSingle,
            3 => Adcconseq::RepeatSequence,
            _ => unreachable!(),
        }
    }
    #[doc = "Single-channel, single-conversion: a single channel is converted once"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Adcconseq::Single
    }
    #[doc = "Sequence-of-channels: a sequence of channels is converted once"]
    #[inline(always)]
    pub fn is_sequence(&self) -> bool {
        *self == Adcconseq::Sequence
    }
    #[doc = "Repeat-single-channel: a single channel is converted repeatedly"]
    #[inline(always)]
    pub fn is_repeat_single(&self) -> bool {
        *self == Adcconseq::RepeatSingle
    }
    #[doc = "Repeat-sequence-of-channels: a sequence of channels is converted repeatedly"]
    #[inline(always)]
    pub fn is_repeat_sequence(&self) -> bool {
        *self == Adcconseq::RepeatSequence
    }
}
#[doc = "Field `ADCCONSEQ` writer - conversion sequence mode select"]
pub type AdcconseqW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcconseq, crate::Safe>;
impl<'a, REG> AdcconseqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single-channel, single-conversion: a single channel is converted once"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Adcconseq::Single)
    }
    #[doc = "Sequence-of-channels: a sequence of channels is converted once"]
    #[inline(always)]
    pub fn sequence(self) -> &'a mut crate::W<REG> {
        self.variant(Adcconseq::Sequence)
    }
    #[doc = "Repeat-single-channel: a single channel is converted repeatedly"]
    #[inline(always)]
    pub fn repeat_single(self) -> &'a mut crate::W<REG> {
        self.variant(Adcconseq::RepeatSingle)
    }
    #[doc = "Repeat-sequence-of-channels: a sequence of channels is converted repeatedly"]
    #[inline(always)]
    pub fn repeat_sequence(self) -> &'a mut crate::W<REG> {
        self.variant(Adcconseq::RepeatSequence)
    }
}
#[doc = "clock source select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcssel {
    #[doc = "0: MODCLK"]
    Modclk = 0,
    #[doc = "1: ACLK"]
    Aclk = 1,
    #[doc = "2: SMCLK"]
    Smclk = 2,
}
impl From<Adcssel> for u8 {
    #[inline(always)]
    fn from(variant: Adcssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcssel {
    type Ux = u8;
}
impl crate::IsEnum for Adcssel {}
#[doc = "Field `ADCSSEL` reader - clock source select"]
pub type AdcsselR = crate::FieldReader<Adcssel>;
impl AdcsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adcssel> {
        match self.bits {
            0 => Some(Adcssel::Modclk),
            1 => Some(Adcssel::Aclk),
            2 => Some(Adcssel::Smclk),
            _ => None,
        }
    }
    #[doc = "MODCLK"]
    #[inline(always)]
    pub fn is_modclk(&self) -> bool {
        *self == Adcssel::Modclk
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == Adcssel::Aclk
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == Adcssel::Smclk
    }
}
#[doc = "Field `ADCSSEL` writer - clock source select"]
pub type AdcsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcssel>;
impl<'a, REG> AdcsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MODCLK"]
    #[inline(always)]
    pub fn modclk(self) -> &'a mut crate::W<REG> {
        self.variant(Adcssel::Modclk)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut crate::W<REG> {
        self.variant(Adcssel::Aclk)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut crate::W<REG> {
        self.variant(Adcssel::Smclk)
    }
}
#[doc = "clock divider\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcdiv {
    #[doc = "0: Divide by 1"]
    _1 = 0,
    #[doc = "1: Divide by 2"]
    _2 = 1,
    #[doc = "2: Divide by 3"]
    _3 = 2,
    #[doc = "3: Divide by 4"]
    _4 = 3,
    #[doc = "4: Divide by 5"]
    _5 = 4,
    #[doc = "5: Divide by 6"]
    _6 = 5,
    #[doc = "6: Divide by 7"]
    _7 = 6,
    #[doc = "7: Divide by 8"]
    _8 = 7,
}
impl From<Adcdiv> for u8 {
    #[inline(always)]
    fn from(variant: Adcdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcdiv {
    type Ux = u8;
}
impl crate::IsEnum for Adcdiv {}
#[doc = "Field `ADCDIV` reader - clock divider"]
pub type AdcdivR = crate::FieldReader<Adcdiv>;
impl AdcdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcdiv {
        match self.bits {
            0 => Adcdiv::_1,
            1 => Adcdiv::_2,
            2 => Adcdiv::_3,
            3 => Adcdiv::_4,
            4 => Adcdiv::_5,
            5 => Adcdiv::_6,
            6 => Adcdiv::_7,
            7 => Adcdiv::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adcdiv::_1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Adcdiv::_2
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == Adcdiv::_3
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Adcdiv::_4
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == Adcdiv::_5
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == Adcdiv::_6
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == Adcdiv::_7
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Adcdiv::_8
    }
}
#[doc = "Field `ADCDIV` writer - clock divider"]
pub type AdcdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Adcdiv, crate::Safe>;
impl<'a, REG> AdcdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Adcdiv::_8)
    }
}
#[doc = "invert signal sample-and-hold\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcissh {
    #[doc = "0: The sample-input signal is not inverted."]
    Adcissh0 = 0,
    #[doc = "1: The sample-input signal is inverted."]
    Adcissh1 = 1,
}
impl From<Adcissh> for bool {
    #[inline(always)]
    fn from(variant: Adcissh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCISSH` reader - invert signal sample-and-hold"]
pub type AdcisshR = crate::BitReader<Adcissh>;
impl AdcisshR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcissh {
        match self.bits {
            false => Adcissh::Adcissh0,
            true => Adcissh::Adcissh1,
        }
    }
    #[doc = "The sample-input signal is not inverted."]
    #[inline(always)]
    pub fn is_adcissh_0(&self) -> bool {
        *self == Adcissh::Adcissh0
    }
    #[doc = "The sample-input signal is inverted."]
    #[inline(always)]
    pub fn is_adcissh_1(&self) -> bool {
        *self == Adcissh::Adcissh1
    }
}
#[doc = "Field `ADCISSH` writer - invert signal sample-and-hold"]
pub type AdcisshW<'a, REG> = crate::BitWriter<'a, REG, Adcissh>;
impl<'a, REG> AdcisshW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The sample-input signal is not inverted."]
    #[inline(always)]
    pub fn adcissh_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcissh::Adcissh0)
    }
    #[doc = "The sample-input signal is inverted."]
    #[inline(always)]
    pub fn adcissh_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcissh::Adcissh1)
    }
}
#[doc = "sample-and-hold pulse-mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcshp {
    #[doc = "0: SAMPCON signal is sourced from the sample-input signal."]
    Adcshp0 = 0,
    #[doc = "1: SAMPCON signal is sourced from the sampling timer."]
    Adcshp1 = 1,
}
impl From<Adcshp> for bool {
    #[inline(always)]
    fn from(variant: Adcshp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCSHP` reader - sample-and-hold pulse-mode select"]
pub type AdcshpR = crate::BitReader<Adcshp>;
impl AdcshpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcshp {
        match self.bits {
            false => Adcshp::Adcshp0,
            true => Adcshp::Adcshp1,
        }
    }
    #[doc = "SAMPCON signal is sourced from the sample-input signal."]
    #[inline(always)]
    pub fn is_adcshp_0(&self) -> bool {
        *self == Adcshp::Adcshp0
    }
    #[doc = "SAMPCON signal is sourced from the sampling timer."]
    #[inline(always)]
    pub fn is_adcshp_1(&self) -> bool {
        *self == Adcshp::Adcshp1
    }
}
#[doc = "Field `ADCSHP` writer - sample-and-hold pulse-mode select"]
pub type AdcshpW<'a, REG> = crate::BitWriter<'a, REG, Adcshp>;
impl<'a, REG> AdcshpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SAMPCON signal is sourced from the sample-input signal."]
    #[inline(always)]
    pub fn adcshp_0(self) -> &'a mut crate::W<REG> {
        self.variant(Adcshp::Adcshp0)
    }
    #[doc = "SAMPCON signal is sourced from the sampling timer."]
    #[inline(always)]
    pub fn adcshp_1(self) -> &'a mut crate::W<REG> {
        self.variant(Adcshp::Adcshp1)
    }
}
#[doc = "sample-and-hold source select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcshs {
    #[doc = "0: ADCSC bit (software trigger)"]
    Software = 0,
    #[doc = "1: RTC event"]
    Rtc = 1,
    #[doc = "2: TB1.1B"]
    Timer = 2,
    #[doc = "3: eCOMP0 COUT"]
    Comparator = 3,
}
impl From<Adcshs> for u8 {
    #[inline(always)]
    fn from(variant: Adcshs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcshs {
    type Ux = u8;
}
impl crate::IsEnum for Adcshs {}
#[doc = "Field `ADCSHS` reader - sample-and-hold source select"]
pub type AdcshsR = crate::FieldReader<Adcshs>;
impl AdcshsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcshs {
        match self.bits {
            0 => Adcshs::Software,
            1 => Adcshs::Rtc,
            2 => Adcshs::Timer,
            3 => Adcshs::Comparator,
            _ => unreachable!(),
        }
    }
    #[doc = "ADCSC bit (software trigger)"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Adcshs::Software
    }
    #[doc = "RTC event"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == Adcshs::Rtc
    }
    #[doc = "TB1.1B"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Adcshs::Timer
    }
    #[doc = "eCOMP0 COUT"]
    #[inline(always)]
    pub fn is_comparator(&self) -> bool {
        *self == Adcshs::Comparator
    }
}
#[doc = "Field `ADCSHS` writer - sample-and-hold source select"]
pub type AdcshsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcshs, crate::Safe>;
impl<'a, REG> AdcshsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADCSC bit (software trigger)"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(Adcshs::Software)
    }
    #[doc = "RTC event"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut crate::W<REG> {
        self.variant(Adcshs::Rtc)
    }
    #[doc = "TB1.1B"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(Adcshs::Timer)
    }
    #[doc = "eCOMP0 COUT"]
    #[inline(always)]
    pub fn comparator(self) -> &'a mut crate::W<REG> {
        self.variant(Adcshs::Comparator)
    }
}
impl R {
    #[doc = "Bit 0 - ADC busy"]
    #[inline(always)]
    pub fn adcbusy(&self) -> AdcbusyR {
        AdcbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - conversion sequence mode select"]
    #[inline(always)]
    pub fn adcconseq(&self) -> AdcconseqR {
        AdcconseqR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - clock source select"]
    #[inline(always)]
    pub fn adcssel(&self) -> AdcsselR {
        AdcsselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - clock divider"]
    #[inline(always)]
    pub fn adcdiv(&self) -> AdcdivR {
        AdcdivR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - invert signal sample-and-hold"]
    #[inline(always)]
    pub fn adcissh(&self) -> AdcisshR {
        AdcisshR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - sample-and-hold pulse-mode select"]
    #[inline(always)]
    pub fn adcshp(&self) -> AdcshpR {
        AdcshpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - sample-and-hold source select"]
    #[inline(always)]
    pub fn adcshs(&self) -> AdcshsR {
        AdcshsR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - conversion sequence mode select"]
    #[inline(always)]
    pub fn adcconseq(&mut self) -> AdcconseqW<'_, Adcctl1Spec> {
        AdcconseqW::new(self, 1)
    }
    #[doc = "Bits 3:4 - clock source select"]
    #[inline(always)]
    pub fn adcssel(&mut self) -> AdcsselW<'_, Adcctl1Spec> {
        AdcsselW::new(self, 3)
    }
    #[doc = "Bits 5:7 - clock divider"]
    #[inline(always)]
    pub fn adcdiv(&mut self) -> AdcdivW<'_, Adcctl1Spec> {
        AdcdivW::new(self, 5)
    }
    #[doc = "Bit 8 - invert signal sample-and-hold"]
    #[inline(always)]
    pub fn adcissh(&mut self) -> AdcisshW<'_, Adcctl1Spec> {
        AdcisshW::new(self, 8)
    }
    #[doc = "Bit 9 - sample-and-hold pulse-mode select"]
    #[inline(always)]
    pub fn adcshp(&mut self) -> AdcshpW<'_, Adcctl1Spec> {
        AdcshpW::new(self, 9)
    }
    #[doc = "Bits 10:11 - sample-and-hold source select"]
    #[inline(always)]
    pub fn adcshs(&mut self) -> AdcshsW<'_, Adcctl1Spec> {
        AdcshsW::new(self, 10)
    }
}
#[doc = "ADC Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcctl1Spec;
impl crate::RegisterSpec for Adcctl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcctl1::R`](R) reader structure"]
impl crate::Readable for Adcctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcctl1::W`](W) writer structure"]
impl crate::Writable for Adcctl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCCTL1 to value 0"]
impl crate::Resettable for Adcctl1Spec {}
