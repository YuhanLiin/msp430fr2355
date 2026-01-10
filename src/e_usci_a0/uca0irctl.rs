#[doc = "Register `UCA0IRCTL` reader"]
pub type R = crate::R<Uca0irctlSpec>;
#[doc = "Register `UCA0IRCTL` writer"]
pub type W = crate::W<Uca0irctlSpec>;
#[doc = "IrDA encoder/decoder enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uciren {
    #[doc = "0: IrDA encoder/decoder disabled"]
    Uciren0 = 0,
    #[doc = "1: IrDA encoder/decoder enabled"]
    Uciren1 = 1,
}
impl From<Uciren> for bool {
    #[inline(always)]
    fn from(variant: Uciren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCIREN` reader - IrDA encoder/decoder enable"]
pub type UcirenR = crate::BitReader<Uciren>;
impl UcirenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uciren {
        match self.bits {
            false => Uciren::Uciren0,
            true => Uciren::Uciren1,
        }
    }
    #[doc = "IrDA encoder/decoder disabled"]
    #[inline(always)]
    pub fn is_uciren_0(&self) -> bool {
        *self == Uciren::Uciren0
    }
    #[doc = "IrDA encoder/decoder enabled"]
    #[inline(always)]
    pub fn is_uciren_1(&self) -> bool {
        *self == Uciren::Uciren1
    }
}
#[doc = "Field `UCIREN` writer - IrDA encoder/decoder enable"]
pub type UcirenW<'a, REG> = crate::BitWriter<'a, REG, Uciren>;
impl<'a, REG> UcirenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA encoder/decoder disabled"]
    #[inline(always)]
    pub fn uciren_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uciren::Uciren0)
    }
    #[doc = "IrDA encoder/decoder enabled"]
    #[inline(always)]
    pub fn uciren_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uciren::Uciren1)
    }
}
#[doc = "IrDA transmit pulse clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucirtxclk {
    #[doc = "0: BRCLK"]
    Ucirtxclk0 = 0,
    #[doc = "1: BITCLK16 when UCOS16 = 1. Otherwise, BRCLK."]
    Ucirtxclk1 = 1,
}
impl From<Ucirtxclk> for bool {
    #[inline(always)]
    fn from(variant: Ucirtxclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCIRTXCLK` reader - IrDA transmit pulse clock select"]
pub type UcirtxclkR = crate::BitReader<Ucirtxclk>;
impl UcirtxclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucirtxclk {
        match self.bits {
            false => Ucirtxclk::Ucirtxclk0,
            true => Ucirtxclk::Ucirtxclk1,
        }
    }
    #[doc = "BRCLK"]
    #[inline(always)]
    pub fn is_ucirtxclk_0(&self) -> bool {
        *self == Ucirtxclk::Ucirtxclk0
    }
    #[doc = "BITCLK16 when UCOS16 = 1. Otherwise, BRCLK."]
    #[inline(always)]
    pub fn is_ucirtxclk_1(&self) -> bool {
        *self == Ucirtxclk::Ucirtxclk1
    }
}
#[doc = "Field `UCIRTXCLK` writer - IrDA transmit pulse clock select"]
pub type UcirtxclkW<'a, REG> = crate::BitWriter<'a, REG, Ucirtxclk>;
impl<'a, REG> UcirtxclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BRCLK"]
    #[inline(always)]
    pub fn ucirtxclk_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucirtxclk::Ucirtxclk0)
    }
    #[doc = "BITCLK16 when UCOS16 = 1. Otherwise, BRCLK."]
    #[inline(always)]
    pub fn ucirtxclk_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucirtxclk::Ucirtxclk1)
    }
}
#[doc = "Field `UCIRTXPL` reader - Transmit pulse length"]
pub type UcirtxplR = crate::FieldReader;
#[doc = "Field `UCIRTXPL` writer - Transmit pulse length"]
pub type UcirtxplW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "IrDA receive filter enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucirrxfe {
    #[doc = "0: Receive filter disabled"]
    Ucirrxfe0 = 0,
    #[doc = "1: Receive filter enabled"]
    Ucirrxfe1 = 1,
}
impl From<Ucirrxfe> for bool {
    #[inline(always)]
    fn from(variant: Ucirrxfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCIRRXFE` reader - IrDA receive filter enabled"]
pub type UcirrxfeR = crate::BitReader<Ucirrxfe>;
impl UcirrxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucirrxfe {
        match self.bits {
            false => Ucirrxfe::Ucirrxfe0,
            true => Ucirrxfe::Ucirrxfe1,
        }
    }
    #[doc = "Receive filter disabled"]
    #[inline(always)]
    pub fn is_ucirrxfe_0(&self) -> bool {
        *self == Ucirrxfe::Ucirrxfe0
    }
    #[doc = "Receive filter enabled"]
    #[inline(always)]
    pub fn is_ucirrxfe_1(&self) -> bool {
        *self == Ucirrxfe::Ucirrxfe1
    }
}
#[doc = "Field `UCIRRXFE` writer - IrDA receive filter enabled"]
pub type UcirrxfeW<'a, REG> = crate::BitWriter<'a, REG, Ucirrxfe>;
impl<'a, REG> UcirrxfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive filter disabled"]
    #[inline(always)]
    pub fn ucirrxfe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucirrxfe::Ucirrxfe0)
    }
    #[doc = "Receive filter enabled"]
    #[inline(always)]
    pub fn ucirrxfe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucirrxfe::Ucirrxfe1)
    }
}
#[doc = "IrDA receive input UCAxRXD polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucirrxpl {
    #[doc = "0: IrDA transceiver delivers a high pulse when a light pulse is seen"]
    High = 0,
    #[doc = "1: IrDA transceiver delivers a low pulse when a light pulse is seen"]
    Low = 1,
}
impl From<Ucirrxpl> for bool {
    #[inline(always)]
    fn from(variant: Ucirrxpl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCIRRXPL` reader - IrDA receive input UCAxRXD polarity"]
pub type UcirrxplR = crate::BitReader<Ucirrxpl>;
impl UcirrxplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucirrxpl {
        match self.bits {
            false => Ucirrxpl::High,
            true => Ucirrxpl::Low,
        }
    }
    #[doc = "IrDA transceiver delivers a high pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ucirrxpl::High
    }
    #[doc = "IrDA transceiver delivers a low pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Ucirrxpl::Low
    }
}
#[doc = "Field `UCIRRXPL` writer - IrDA receive input UCAxRXD polarity"]
pub type UcirrxplW<'a, REG> = crate::BitWriter<'a, REG, Ucirrxpl>;
impl<'a, REG> UcirrxplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA transceiver delivers a high pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ucirrxpl::High)
    }
    #[doc = "IrDA transceiver delivers a low pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Ucirrxpl::Low)
    }
}
#[doc = "Field `UCIRRXFL` reader - Receive filter length"]
pub type UcirrxflR = crate::FieldReader;
#[doc = "Field `UCIRRXFL` writer - Receive filter length"]
pub type UcirrxflW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - IrDA encoder/decoder enable"]
    #[inline(always)]
    pub fn uciren(&self) -> UcirenR {
        UcirenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn ucirtxclk(&self) -> UcirtxclkR {
        UcirtxclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Transmit pulse length"]
    #[inline(always)]
    pub fn ucirtxpl(&self) -> UcirtxplR {
        UcirtxplR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - IrDA receive filter enabled"]
    #[inline(always)]
    pub fn ucirrxfe(&self) -> UcirrxfeR {
        UcirrxfeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IrDA receive input UCAxRXD polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&self) -> UcirrxplR {
        UcirrxplR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - Receive filter length"]
    #[inline(always)]
    pub fn ucirrxfl(&self) -> UcirrxflR {
        UcirrxflR::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA encoder/decoder enable"]
    #[inline(always)]
    pub fn uciren(&mut self) -> UcirenW<'_, Uca0irctlSpec> {
        UcirenW::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn ucirtxclk(&mut self) -> UcirtxclkW<'_, Uca0irctlSpec> {
        UcirtxclkW::new(self, 1)
    }
    #[doc = "Bits 2:7 - Transmit pulse length"]
    #[inline(always)]
    pub fn ucirtxpl(&mut self) -> UcirtxplW<'_, Uca0irctlSpec> {
        UcirtxplW::new(self, 2)
    }
    #[doc = "Bit 8 - IrDA receive filter enabled"]
    #[inline(always)]
    pub fn ucirrxfe(&mut self) -> UcirrxfeW<'_, Uca0irctlSpec> {
        UcirrxfeW::new(self, 8)
    }
    #[doc = "Bit 9 - IrDA receive input UCAxRXD polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&mut self) -> UcirrxplW<'_, Uca0irctlSpec> {
        UcirrxplW::new(self, 9)
    }
    #[doc = "Bits 10:15 - Receive filter length"]
    #[inline(always)]
    pub fn ucirrxfl(&mut self) -> UcirrxflW<'_, Uca0irctlSpec> {
        UcirrxflW::new(self, 10)
    }
}
#[doc = "eUSCI_Ax IrDA Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0irctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0irctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0irctlSpec;
impl crate::RegisterSpec for Uca0irctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0irctl::R`](R) reader structure"]
impl crate::Readable for Uca0irctlSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0irctl::W`](W) writer structure"]
impl crate::Writable for Uca0irctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0IRCTL to value 0"]
impl crate::Resettable for Uca0irctlSpec {}
