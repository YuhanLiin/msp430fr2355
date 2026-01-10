#[doc = "Register `PM5CTL0` reader"]
pub type R = crate::R<Pm5ctl0Spec>;
#[doc = "Register `PM5CTL0` writer"]
pub type W = crate::W<Pm5ctl0Spec>;
#[doc = "LPMx.5 Lock Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Locklpm5 {
    #[doc = "0: LPMx.5 configuration is not locked and defaults to its reset condition."]
    Locklpm5_0 = 0,
    #[doc = "1: LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit."]
    Locklpm5_1 = 1,
}
impl From<Locklpm5> for bool {
    #[inline(always)]
    fn from(variant: Locklpm5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKLPM5` reader - LPMx.5 Lock Bit"]
pub type Locklpm5R = crate::BitReader<Locklpm5>;
impl Locklpm5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Locklpm5 {
        match self.bits {
            false => Locklpm5::Locklpm5_0,
            true => Locklpm5::Locklpm5_1,
        }
    }
    #[doc = "LPMx.5 configuration is not locked and defaults to its reset condition."]
    #[inline(always)]
    pub fn is_locklpm5_0(&self) -> bool {
        *self == Locklpm5::Locklpm5_0
    }
    #[doc = "LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit."]
    #[inline(always)]
    pub fn is_locklpm5_1(&self) -> bool {
        *self == Locklpm5::Locklpm5_1
    }
}
#[doc = "Field `LOCKLPM5` writer - LPMx.5 Lock Bit"]
pub type Locklpm5W<'a, REG> = crate::BitWriter<'a, REG, Locklpm5>;
impl<'a, REG> Locklpm5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPMx.5 configuration is not locked and defaults to its reset condition."]
    #[inline(always)]
    pub fn locklpm5_0(self) -> &'a mut crate::W<REG> {
        self.variant(Locklpm5::Locklpm5_0)
    }
    #[doc = "LPMx.5 configuration remains locked. Pin state is held during LPMx.5 entry and exit."]
    #[inline(always)]
    pub fn locklpm5_1(self) -> &'a mut crate::W<REG> {
        self.variant(Locklpm5::Locklpm5_1)
    }
}
#[doc = "Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpm5sw {
    #[doc = "0: LPMx.5 switch disconnected"]
    Lpm5sw0 = 0,
    #[doc = "1: LPMx.5 switch connected"]
    Lpm5sw1 = 1,
}
impl From<Lpm5sw> for bool {
    #[inline(always)]
    fn from(variant: Lpm5sw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPM5SW` reader - Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected."]
pub type Lpm5swR = crate::BitReader<Lpm5sw>;
impl Lpm5swR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpm5sw {
        match self.bits {
            false => Lpm5sw::Lpm5sw0,
            true => Lpm5sw::Lpm5sw1,
        }
    }
    #[doc = "LPMx.5 switch disconnected"]
    #[inline(always)]
    pub fn is_lpm5sw_0(&self) -> bool {
        *self == Lpm5sw::Lpm5sw0
    }
    #[doc = "LPMx.5 switch connected"]
    #[inline(always)]
    pub fn is_lpm5sw_1(&self) -> bool {
        *self == Lpm5sw::Lpm5sw1
    }
}
#[doc = "Field `LPM5SW` writer - Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected."]
pub type Lpm5swW<'a, REG> = crate::BitWriter<'a, REG, Lpm5sw>;
impl<'a, REG> Lpm5swW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPMx.5 switch disconnected"]
    #[inline(always)]
    pub fn lpm5sw_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpm5sw::Lpm5sw0)
    }
    #[doc = "LPMx.5 switch connected"]
    #[inline(always)]
    pub fn lpm5sw_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpm5sw::Lpm5sw1)
    }
}
#[doc = "Specifies the operation mode of the LPM3.5 switch.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpm5sm {
    #[doc = "0: Automatic mode for LPM3.5 switch that the switch is fully handled by the circuitry during mode switch."]
    Lpm5sm0 = 0,
    #[doc = "1: Manual mode for LPM3.5 switch that the switch is specified by LPM5SW bit setting in software."]
    Lpm5sm1 = 1,
}
impl From<Lpm5sm> for bool {
    #[inline(always)]
    fn from(variant: Lpm5sm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPM5SM` reader - Specifies the operation mode of the LPM3.5 switch."]
pub type Lpm5smR = crate::BitReader<Lpm5sm>;
impl Lpm5smR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpm5sm {
        match self.bits {
            false => Lpm5sm::Lpm5sm0,
            true => Lpm5sm::Lpm5sm1,
        }
    }
    #[doc = "Automatic mode for LPM3.5 switch that the switch is fully handled by the circuitry during mode switch."]
    #[inline(always)]
    pub fn is_lpm5sm_0(&self) -> bool {
        *self == Lpm5sm::Lpm5sm0
    }
    #[doc = "Manual mode for LPM3.5 switch that the switch is specified by LPM5SW bit setting in software."]
    #[inline(always)]
    pub fn is_lpm5sm_1(&self) -> bool {
        *self == Lpm5sm::Lpm5sm1
    }
}
#[doc = "Field `LPM5SM` writer - Specifies the operation mode of the LPM3.5 switch."]
pub type Lpm5smW<'a, REG> = crate::BitWriter<'a, REG, Lpm5sm>;
impl<'a, REG> Lpm5smW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Automatic mode for LPM3.5 switch that the switch is fully handled by the circuitry during mode switch."]
    #[inline(always)]
    pub fn lpm5sm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpm5sm::Lpm5sm0)
    }
    #[doc = "Manual mode for LPM3.5 switch that the switch is specified by LPM5SW bit setting in software."]
    #[inline(always)]
    pub fn lpm5sm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpm5sm::Lpm5sm1)
    }
}
impl R {
    #[doc = "Bit 0 - LPMx.5 Lock Bit"]
    #[inline(always)]
    pub fn locklpm5(&self) -> Locklpm5R {
        Locklpm5R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected."]
    #[inline(always)]
    pub fn lpm5sw(&self) -> Lpm5swR {
        Lpm5swR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Specifies the operation mode of the LPM3.5 switch."]
    #[inline(always)]
    pub fn lpm5sm(&self) -> Lpm5smR {
        Lpm5smR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPMx.5 Lock Bit"]
    #[inline(always)]
    pub fn locklpm5(&mut self) -> Locklpm5W<'_, Pm5ctl0Spec> {
        Locklpm5W::new(self, 0)
    }
    #[doc = "Bit 4 - Reports or sets the LPM3.5 switch connection upon the switch mode set by LPM5SM. When this bit is set, the VLPM3.5 domain can accept full-speed read and write operation by CPU MCLK. If the switch is disconnected, all peripherals within this domain can only accept the operation no more than 40 kHz. In automatic mode (LPM5SM = 0), this bit represents the switch connection between Vcore and VLPM3.5. Any write to this bit has no effect. In manual mode (LPM5SM = 1), this bit can be fully read and written by software. When this bit is set, the switch connection between Vcore and VLPM3.5 is connected. Otherwise, the switch is disconnected."]
    #[inline(always)]
    pub fn lpm5sw(&mut self) -> Lpm5swW<'_, Pm5ctl0Spec> {
        Lpm5swW::new(self, 4)
    }
    #[doc = "Bit 5 - Specifies the operation mode of the LPM3.5 switch."]
    #[inline(always)]
    pub fn lpm5sm(&mut self) -> Lpm5smW<'_, Pm5ctl0Spec> {
        Lpm5smW::new(self, 5)
    }
}
#[doc = "Power mode 5 control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pm5ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pm5ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pm5ctl0Spec;
impl crate::RegisterSpec for Pm5ctl0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pm5ctl0::R`](R) reader structure"]
impl crate::Readable for Pm5ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pm5ctl0::W`](W) writer structure"]
impl crate::Writable for Pm5ctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PM5CTL0 to value 0"]
impl crate::Resettable for Pm5ctl0Spec {}
