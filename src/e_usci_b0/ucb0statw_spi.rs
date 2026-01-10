#[doc = "Register `UCB0STATW_SPI` reader"]
pub type R = crate::R<Ucb0statwSpiSpec>;
#[doc = "Register `UCB0STATW_SPI` writer"]
pub type W = crate::W<Ucb0statwSpiSpec>;
#[doc = "eUSCI_B0 busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucbusy {
    #[doc = "0: eUSCI_B0 inactive"]
    Idle = 0,
    #[doc = "1: eUSCI_B0 transmitting or receiving"]
    Busy = 1,
}
impl From<Ucbusy> for bool {
    #[inline(always)]
    fn from(variant: Ucbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBUSY` reader - eUSCI_B0 busy"]
pub type UcbusyR = crate::BitReader<Ucbusy>;
impl UcbusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucbusy {
        match self.bits {
            false => Ucbusy::Idle,
            true => Ucbusy::Busy,
        }
    }
    #[doc = "eUSCI_B0 inactive"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Ucbusy::Idle
    }
    #[doc = "eUSCI_B0 transmitting or receiving"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Ucbusy::Busy
    }
}
#[doc = "Overrun error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucoe {
    #[doc = "0: No error"]
    Ucoe0 = 0,
    #[doc = "1: Overrun error occurred"]
    Ucoe1 = 1,
}
impl From<Ucoe> for bool {
    #[inline(always)]
    fn from(variant: Ucoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCOE` reader - Overrun error flag"]
pub type UcoeR = crate::BitReader<Ucoe>;
impl UcoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucoe {
        match self.bits {
            false => Ucoe::Ucoe0,
            true => Ucoe::Ucoe1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_ucoe_0(&self) -> bool {
        *self == Ucoe::Ucoe0
    }
    #[doc = "Overrun error occurred"]
    #[inline(always)]
    pub fn is_ucoe_1(&self) -> bool {
        *self == Ucoe::Ucoe1
    }
}
#[doc = "Field `UCOE` writer - Overrun error flag"]
pub type UcoeW<'a, REG> = crate::BitWriter<'a, REG, Ucoe>;
impl<'a, REG> UcoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucoe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucoe::Ucoe0)
    }
    #[doc = "Overrun error occurred"]
    #[inline(always)]
    pub fn ucoe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucoe::Ucoe1)
    }
}
#[doc = "Framing error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucfe {
    #[doc = "0: No error"]
    Ucfe0 = 0,
    #[doc = "1: Bus conflict occurred"]
    Ucfe1 = 1,
}
impl From<Ucfe> for bool {
    #[inline(always)]
    fn from(variant: Ucfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCFE` reader - Framing error flag"]
pub type UcfeR = crate::BitReader<Ucfe>;
impl UcfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucfe {
        match self.bits {
            false => Ucfe::Ucfe0,
            true => Ucfe::Ucfe1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_ucfe_0(&self) -> bool {
        *self == Ucfe::Ucfe0
    }
    #[doc = "Bus conflict occurred"]
    #[inline(always)]
    pub fn is_ucfe_1(&self) -> bool {
        *self == Ucfe::Ucfe1
    }
}
#[doc = "Field `UCFE` writer - Framing error flag"]
pub type UcfeW<'a, REG> = crate::BitWriter<'a, REG, Ucfe>;
impl<'a, REG> UcfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucfe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucfe::Ucfe0)
    }
    #[doc = "Bus conflict occurred"]
    #[inline(always)]
    pub fn ucfe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucfe::Ucfe1)
    }
}
#[doc = "Listen enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uclisten {
    #[doc = "0: Disabled"]
    Uclisten0 = 0,
    #[doc = "1: Enabled. UCBxTXD is internally fed back to the receiver"]
    Uclisten1 = 1,
}
impl From<Uclisten> for bool {
    #[inline(always)]
    fn from(variant: Uclisten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCLISTEN` reader - Listen enable"]
pub type UclistenR = crate::BitReader<Uclisten>;
impl UclistenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uclisten {
        match self.bits {
            false => Uclisten::Uclisten0,
            true => Uclisten::Uclisten1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_uclisten_0(&self) -> bool {
        *self == Uclisten::Uclisten0
    }
    #[doc = "Enabled. UCBxTXD is internally fed back to the receiver"]
    #[inline(always)]
    pub fn is_uclisten_1(&self) -> bool {
        *self == Uclisten::Uclisten1
    }
}
#[doc = "Field `UCLISTEN` writer - Listen enable"]
pub type UclistenW<'a, REG> = crate::BitWriter<'a, REG, Uclisten>;
impl<'a, REG> UclistenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn uclisten_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uclisten::Uclisten0)
    }
    #[doc = "Enabled. UCBxTXD is internally fed back to the receiver"]
    #[inline(always)]
    pub fn uclisten_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uclisten::Uclisten1)
    }
}
impl R {
    #[doc = "Bit 0 - eUSCI_B0 busy"]
    #[inline(always)]
    pub fn ucbusy(&self) -> UcbusyR {
        UcbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun error flag"]
    #[inline(always)]
    pub fn ucoe(&self) -> UcoeR {
        UcoeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing error flag"]
    #[inline(always)]
    pub fn ucfe(&self) -> UcfeR {
        UcfeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Listen enable"]
    #[inline(always)]
    pub fn uclisten(&self) -> UclistenR {
        UclistenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Overrun error flag"]
    #[inline(always)]
    pub fn ucoe(&mut self) -> UcoeW<'_, Ucb0statwSpiSpec> {
        UcoeW::new(self, 5)
    }
    #[doc = "Bit 6 - Framing error flag"]
    #[inline(always)]
    pub fn ucfe(&mut self) -> UcfeW<'_, Ucb0statwSpiSpec> {
        UcfeW::new(self, 6)
    }
    #[doc = "Bit 7 - Listen enable"]
    #[inline(always)]
    pub fn uclisten(&mut self) -> UclistenW<'_, Ucb0statwSpiSpec> {
        UclistenW::new(self, 7)
    }
}
#[doc = "UCB0STATW_SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb0statw_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb0statw_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb0statwSpiSpec;
impl crate::RegisterSpec for Ucb0statwSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb0statw_spi::R`](R) reader structure"]
impl crate::Readable for Ucb0statwSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb0statw_spi::W`](W) writer structure"]
impl crate::Writable for Ucb0statwSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB0STATW_SPI to value 0"]
impl crate::Resettable for Ucb0statwSpiSpec {}
