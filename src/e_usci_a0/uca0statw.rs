#[doc = "Register `UCA0STATW` reader"]
pub type R = crate::R<Uca0statwSpec>;
#[doc = "Register `UCA0STATW` writer"]
pub type W = crate::W<Uca0statwSpec>;
#[doc = "eUSCI_A busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucbusy {
    #[doc = "0: eUSCI_A inactive"]
    Idle = 0,
    #[doc = "1: eUSCI_A transmitting or receiving"]
    Busy = 1,
}
impl From<Ucbusy> for bool {
    #[inline(always)]
    fn from(variant: Ucbusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBUSY` reader - eUSCI_A busy"]
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
    #[doc = "eUSCI_A inactive"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Ucbusy::Idle
    }
    #[doc = "eUSCI_A transmitting or receiving"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Ucbusy::Busy
    }
}
#[doc = "Address received / Idle line detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UcaddrUcidle {
    #[doc = "0: UCADDR: Received character is data. UCIDLE: No idle line detected"]
    UcaddrUcidle0 = 0,
    #[doc = "1: UCADDR: Received character is an address. UCIDLE: Idle line detected"]
    UcaddrUcidle1 = 1,
}
impl From<UcaddrUcidle> for bool {
    #[inline(always)]
    fn from(variant: UcaddrUcidle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCADDR_UCIDLE` reader - Address received / Idle line detected"]
pub type UcaddrUcidleR = crate::BitReader<UcaddrUcidle>;
impl UcaddrUcidleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UcaddrUcidle {
        match self.bits {
            false => UcaddrUcidle::UcaddrUcidle0,
            true => UcaddrUcidle::UcaddrUcidle1,
        }
    }
    #[doc = "UCADDR: Received character is data. UCIDLE: No idle line detected"]
    #[inline(always)]
    pub fn is_ucaddr_ucidle_0(&self) -> bool {
        *self == UcaddrUcidle::UcaddrUcidle0
    }
    #[doc = "UCADDR: Received character is an address. UCIDLE: Idle line detected"]
    #[inline(always)]
    pub fn is_ucaddr_ucidle_1(&self) -> bool {
        *self == UcaddrUcidle::UcaddrUcidle1
    }
}
#[doc = "Field `UCADDR_UCIDLE` writer - Address received / Idle line detected"]
pub type UcaddrUcidleW<'a, REG> = crate::BitWriter<'a, REG, UcaddrUcidle>;
impl<'a, REG> UcaddrUcidleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UCADDR: Received character is data. UCIDLE: No idle line detected"]
    #[inline(always)]
    pub fn ucaddr_ucidle_0(self) -> &'a mut crate::W<REG> {
        self.variant(UcaddrUcidle::UcaddrUcidle0)
    }
    #[doc = "UCADDR: Received character is an address. UCIDLE: Idle line detected"]
    #[inline(always)]
    pub fn ucaddr_ucidle_1(self) -> &'a mut crate::W<REG> {
        self.variant(UcaddrUcidle::UcaddrUcidle1)
    }
}
#[doc = "Receive error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucrxerr {
    #[doc = "0: No receive errors detected"]
    Ucrxerr0 = 0,
    #[doc = "1: Receive error detected"]
    Ucrxerr1 = 1,
}
impl From<Ucrxerr> for bool {
    #[inline(always)]
    fn from(variant: Ucrxerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXERR` reader - Receive error flag"]
pub type UcrxerrR = crate::BitReader<Ucrxerr>;
impl UcrxerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucrxerr {
        match self.bits {
            false => Ucrxerr::Ucrxerr0,
            true => Ucrxerr::Ucrxerr1,
        }
    }
    #[doc = "No receive errors detected"]
    #[inline(always)]
    pub fn is_ucrxerr_0(&self) -> bool {
        *self == Ucrxerr::Ucrxerr0
    }
    #[doc = "Receive error detected"]
    #[inline(always)]
    pub fn is_ucrxerr_1(&self) -> bool {
        *self == Ucrxerr::Ucrxerr1
    }
}
#[doc = "Field `UCRXERR` writer - Receive error flag"]
pub type UcrxerrW<'a, REG> = crate::BitWriter<'a, REG, Ucrxerr>;
impl<'a, REG> UcrxerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No receive errors detected"]
    #[inline(always)]
    pub fn ucrxerr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxerr::Ucrxerr0)
    }
    #[doc = "Receive error detected"]
    #[inline(always)]
    pub fn ucrxerr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxerr::Ucrxerr1)
    }
}
#[doc = "Break detect flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucbrk {
    #[doc = "0: No break condition"]
    Ucbrk0 = 0,
    #[doc = "1: Break condition occurred"]
    Ucbrk1 = 1,
}
impl From<Ucbrk> for bool {
    #[inline(always)]
    fn from(variant: Ucbrk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBRK` reader - Break detect flag"]
pub type UcbrkR = crate::BitReader<Ucbrk>;
impl UcbrkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucbrk {
        match self.bits {
            false => Ucbrk::Ucbrk0,
            true => Ucbrk::Ucbrk1,
        }
    }
    #[doc = "No break condition"]
    #[inline(always)]
    pub fn is_ucbrk_0(&self) -> bool {
        *self == Ucbrk::Ucbrk0
    }
    #[doc = "Break condition occurred"]
    #[inline(always)]
    pub fn is_ucbrk_1(&self) -> bool {
        *self == Ucbrk::Ucbrk1
    }
}
#[doc = "Field `UCBRK` writer - Break detect flag"]
pub type UcbrkW<'a, REG> = crate::BitWriter<'a, REG, Ucbrk>;
impl<'a, REG> UcbrkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No break condition"]
    #[inline(always)]
    pub fn ucbrk_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrk::Ucbrk0)
    }
    #[doc = "Break condition occurred"]
    #[inline(always)]
    pub fn ucbrk_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrk::Ucbrk1)
    }
}
#[doc = "Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucpe {
    #[doc = "0: No error"]
    Ucpe0 = 0,
    #[doc = "1: Character received with parity error"]
    Ucpe1 = 1,
}
impl From<Ucpe> for bool {
    #[inline(always)]
    fn from(variant: Ucpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCPE` reader - Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read."]
pub type UcpeR = crate::BitReader<Ucpe>;
impl UcpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucpe {
        match self.bits {
            false => Ucpe::Ucpe0,
            true => Ucpe::Ucpe1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_ucpe_0(&self) -> bool {
        *self == Ucpe::Ucpe0
    }
    #[doc = "Character received with parity error"]
    #[inline(always)]
    pub fn is_ucpe_1(&self) -> bool {
        *self == Ucpe::Ucpe1
    }
}
#[doc = "Field `UCPE` writer - Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read."]
pub type UcpeW<'a, REG> = crate::BitWriter<'a, REG, Ucpe>;
impl<'a, REG> UcpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn ucpe_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpe::Ucpe0)
    }
    #[doc = "Character received with parity error"]
    #[inline(always)]
    pub fn ucpe_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpe::Ucpe1)
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
    #[doc = "1: Character received with low stop bit"]
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
    #[doc = "Character received with low stop bit"]
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
    #[doc = "Character received with low stop bit"]
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
    #[doc = "1: Enabled. UCAxTXD is internally fed back to the receiver"]
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
    #[doc = "Enabled. UCAxTXD is internally fed back to the receiver"]
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
    #[doc = "Enabled. UCAxTXD is internally fed back to the receiver"]
    #[inline(always)]
    pub fn uclisten_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uclisten::Uclisten1)
    }
}
impl R {
    #[doc = "Bit 0 - eUSCI_A busy"]
    #[inline(always)]
    pub fn ucbusy(&self) -> UcbusyR {
        UcbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address received / Idle line detected"]
    #[inline(always)]
    pub fn ucaddr_ucidle(&self) -> UcaddrUcidleR {
        UcaddrUcidleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive error flag"]
    #[inline(always)]
    pub fn ucrxerr(&self) -> UcrxerrR {
        UcrxerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Break detect flag"]
    #[inline(always)]
    pub fn ucbrk(&self) -> UcbrkR {
        UcbrkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read."]
    #[inline(always)]
    pub fn ucpe(&self) -> UcpeR {
        UcpeR::new(((self.bits >> 4) & 1) != 0)
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
    #[doc = "Bit 1 - Address received / Idle line detected"]
    #[inline(always)]
    pub fn ucaddr_ucidle(&mut self) -> UcaddrUcidleW<'_, Uca0statwSpec> {
        UcaddrUcidleW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive error flag"]
    #[inline(always)]
    pub fn ucrxerr(&mut self) -> UcrxerrW<'_, Uca0statwSpec> {
        UcrxerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Break detect flag"]
    #[inline(always)]
    pub fn ucbrk(&mut self) -> UcbrkW<'_, Uca0statwSpec> {
        UcbrkW::new(self, 3)
    }
    #[doc = "Bit 4 - Parity error flag. When UCPEN = 0, UCPE is read as 0. UCPE is cleared when UCAxRXBUF is read."]
    #[inline(always)]
    pub fn ucpe(&mut self) -> UcpeW<'_, Uca0statwSpec> {
        UcpeW::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun error flag"]
    #[inline(always)]
    pub fn ucoe(&mut self) -> UcoeW<'_, Uca0statwSpec> {
        UcoeW::new(self, 5)
    }
    #[doc = "Bit 6 - Framing error flag"]
    #[inline(always)]
    pub fn ucfe(&mut self) -> UcfeW<'_, Uca0statwSpec> {
        UcfeW::new(self, 6)
    }
    #[doc = "Bit 7 - Listen enable"]
    #[inline(always)]
    pub fn uclisten(&mut self) -> UclistenW<'_, Uca0statwSpec> {
        UclistenW::new(self, 7)
    }
}
#[doc = "eUSCI_Ax Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uca0statw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca0statw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca0statwSpec;
impl crate::RegisterSpec for Uca0statwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca0statw::R`](R) reader structure"]
impl crate::Readable for Uca0statwSpec {}
#[doc = "`write(|w| ..)` method takes [`uca0statw::W`](W) writer structure"]
impl crate::Writable for Uca0statwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA0STATW to value 0"]
impl crate::Resettable for Uca0statwSpec {}
