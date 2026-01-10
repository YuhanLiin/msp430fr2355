#[doc = "Register `RTCCTL` reader"]
pub type R = crate::R<RtcctlSpec>;
#[doc = "Register `RTCCTL` writer"]
pub type W = crate::W<RtcctlSpec>;
#[doc = "Real-time interrupt flag. This bit reports the status of a pending interrupt. This read only bit can be cleared by reading RTCIV register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcifg {
    #[doc = "0: No interrupt pending"]
    Rtcifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Rtcifg1 = 1,
}
impl From<Rtcifg> for bool {
    #[inline(always)]
    fn from(variant: Rtcifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCIFG` reader - Real-time interrupt flag. This bit reports the status of a pending interrupt. This read only bit can be cleared by reading RTCIV register."]
pub type RtcifgR = crate::BitReader<Rtcifg>;
impl RtcifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcifg {
        match self.bits {
            false => Rtcifg::Rtcifg0,
            true => Rtcifg::Rtcifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_rtcifg_0(&self) -> bool {
        *self == Rtcifg::Rtcifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_rtcifg_1(&self) -> bool {
        *self == Rtcifg::Rtcifg1
    }
}
#[doc = "Real-time interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcie {
    #[doc = "0: Interrupt disabled"]
    Rtcie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Rtcie1 = 1,
}
impl From<Rtcie> for bool {
    #[inline(always)]
    fn from(variant: Rtcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCIE` reader - Real-time interrupt enable"]
pub type RtcieR = crate::BitReader<Rtcie>;
impl RtcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcie {
        match self.bits {
            false => Rtcie::Rtcie0,
            true => Rtcie::Rtcie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_rtcie_0(&self) -> bool {
        *self == Rtcie::Rtcie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_rtcie_1(&self) -> bool {
        *self == Rtcie::Rtcie1
    }
}
#[doc = "Field `RTCIE` writer - Real-time interrupt enable"]
pub type RtcieW<'a, REG> = crate::BitWriter<'a, REG, Rtcie>;
impl<'a, REG> RtcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn rtcie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcie::Rtcie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn rtcie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcie::Rtcie1)
    }
}
#[doc = "Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcsr {
    #[doc = "0: Write 0 has no effect"]
    Rtcsr0 = 0,
    #[doc = "1: Write 1 to this bit clears the counter value and reloads the shadow register value from the modulo register at the next tick of the selected source clock. No overflow event or interrupt is generated."]
    Rtcsr1 = 1,
}
impl From<Rtcsr> for bool {
    #[inline(always)]
    fn from(variant: Rtcsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCSR` reader - Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect"]
pub type RtcsrR = crate::BitReader<Rtcsr>;
impl RtcsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcsr {
        match self.bits {
            false => Rtcsr::Rtcsr0,
            true => Rtcsr::Rtcsr1,
        }
    }
    #[doc = "Write 0 has no effect"]
    #[inline(always)]
    pub fn is_rtcsr_0(&self) -> bool {
        *self == Rtcsr::Rtcsr0
    }
    #[doc = "Write 1 to this bit clears the counter value and reloads the shadow register value from the modulo register at the next tick of the selected source clock. No overflow event or interrupt is generated."]
    #[inline(always)]
    pub fn is_rtcsr_1(&self) -> bool {
        *self == Rtcsr::Rtcsr1
    }
}
#[doc = "Field `RTCSR` writer - Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect"]
pub type RtcsrW<'a, REG> = crate::BitWriter<'a, REG, Rtcsr>;
impl<'a, REG> RtcsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write 0 has no effect"]
    #[inline(always)]
    pub fn rtcsr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsr::Rtcsr0)
    }
    #[doc = "Write 1 to this bit clears the counter value and reloads the shadow register value from the modulo register at the next tick of the selected source clock. No overflow event or interrupt is generated."]
    #[inline(always)]
    pub fn rtcsr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcsr::Rtcsr1)
    }
}
#[doc = "Real-time clock pre-divider select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcps {
    #[doc = "0: /1"]
    _1 = 0,
    #[doc = "1: /10"]
    _10 = 1,
    #[doc = "2: /100"]
    _100 = 2,
    #[doc = "3: /1000"]
    _1000 = 3,
    #[doc = "4: /16"]
    _16 = 4,
    #[doc = "5: /64"]
    _64 = 5,
    #[doc = "6: /256"]
    _256 = 6,
    #[doc = "7: /1024"]
    _1024 = 7,
}
impl From<Rtcps> for u8 {
    #[inline(always)]
    fn from(variant: Rtcps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcps {
    type Ux = u8;
}
impl crate::IsEnum for Rtcps {}
#[doc = "Field `RTCPS` reader - Real-time clock pre-divider select"]
pub type RtcpsR = crate::FieldReader<Rtcps>;
impl RtcpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcps {
        match self.bits {
            0 => Rtcps::_1,
            1 => Rtcps::_10,
            2 => Rtcps::_100,
            3 => Rtcps::_1000,
            4 => Rtcps::_16,
            5 => Rtcps::_64,
            6 => Rtcps::_256,
            7 => Rtcps::_1024,
            _ => unreachable!(),
        }
    }
    #[doc = "/1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtcps::_1
    }
    #[doc = "/10"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Rtcps::_10
    }
    #[doc = "/100"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == Rtcps::_100
    }
    #[doc = "/1000"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == Rtcps::_1000
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Rtcps::_16
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Rtcps::_64
    }
    #[doc = "/256"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Rtcps::_256
    }
    #[doc = "/1024"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Rtcps::_1024
    }
}
#[doc = "Field `RTCPS` writer - Real-time clock pre-divider select"]
pub type RtcpsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rtcps, crate::Safe>;
impl<'a, REG> RtcpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "/1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_1)
    }
    #[doc = "/10"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_10)
    }
    #[doc = "/100"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_100)
    }
    #[doc = "/1000"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_1000)
    }
    #[doc = "/16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_16)
    }
    #[doc = "/64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_64)
    }
    #[doc = "/256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_256)
    }
    #[doc = "/1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcps::_1024)
    }
}
#[doc = "Real-time clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rtcss {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: SMCLK"]
    Smclk = 1,
    #[doc = "2: XT1CLK"]
    Xt1clk = 2,
    #[doc = "3: VLOCLK"]
    Vloclk = 3,
}
impl From<Rtcss> for u8 {
    #[inline(always)]
    fn from(variant: Rtcss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rtcss {
    type Ux = u8;
}
impl crate::IsEnum for Rtcss {}
#[doc = "Field `RTCSS` reader - Real-time clock source select"]
pub type RtcssR = crate::FieldReader<Rtcss>;
impl RtcssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcss {
        match self.bits {
            0 => Rtcss::Disabled,
            1 => Rtcss::Smclk,
            2 => Rtcss::Xt1clk,
            3 => Rtcss::Vloclk,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rtcss::Disabled
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == Rtcss::Smclk
    }
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == Rtcss::Xt1clk
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == Rtcss::Vloclk
    }
}
#[doc = "Field `RTCSS` writer - Real-time clock source select"]
pub type RtcssW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rtcss, crate::Safe>;
impl<'a, REG> RtcssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcss::Disabled)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcss::Smclk)
    }
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcss::Xt1clk)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcss::Vloclk)
    }
}
impl R {
    #[doc = "Bit 0 - Real-time interrupt flag. This bit reports the status of a pending interrupt. This read only bit can be cleared by reading RTCIV register."]
    #[inline(always)]
    pub fn rtcifg(&self) -> RtcifgR {
        RtcifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real-time interrupt enable"]
    #[inline(always)]
    pub fn rtcie(&self) -> RtcieR {
        RtcieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect"]
    #[inline(always)]
    pub fn rtcsr(&self) -> RtcsrR {
        RtcsrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Real-time clock pre-divider select"]
    #[inline(always)]
    pub fn rtcps(&self) -> RtcpsR {
        RtcpsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Real-time clock source select"]
    #[inline(always)]
    pub fn rtcss(&self) -> RtcssR {
        RtcssR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Real-time interrupt enable"]
    #[inline(always)]
    pub fn rtcie(&mut self) -> RtcieW<'_, RtcctlSpec> {
        RtcieW::new(self, 1)
    }
    #[doc = "Bit 6 - Real-time software reset. This is a write only bit and is always read with logic 0. 0b = Write 0 has no effect"]
    #[inline(always)]
    pub fn rtcsr(&mut self) -> RtcsrW<'_, RtcctlSpec> {
        RtcsrW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Real-time clock pre-divider select"]
    #[inline(always)]
    pub fn rtcps(&mut self) -> RtcpsW<'_, RtcctlSpec> {
        RtcpsW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Real-time clock source select"]
    #[inline(always)]
    pub fn rtcss(&mut self) -> RtcssW<'_, RtcctlSpec> {
        RtcssW::new(self, 12)
    }
}
#[doc = "RTCCTL0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcctlSpec;
impl crate::RegisterSpec for RtcctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rtcctl::R`](R) reader structure"]
impl crate::Readable for RtcctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcctl::W`](W) writer structure"]
impl crate::Writable for RtcctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCTL to value 0"]
impl crate::Resettable for RtcctlSpec {}
