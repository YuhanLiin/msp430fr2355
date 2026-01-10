#[doc = "Register `UCA1CTLW0` reader"]
pub type R = crate::R<Uca1ctlw0Spec>;
#[doc = "Register `UCA1CTLW0` writer"]
pub type W = crate::W<Uca1ctlw0Spec>;
#[doc = "Software reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucswrst {
    #[doc = "0: Disabled. eUSCI_A reset released for operation"]
    Disable = 0,
    #[doc = "1: Enabled. eUSCI_A logic held in reset state"]
    Enable = 1,
}
impl From<Ucswrst> for bool {
    #[inline(always)]
    fn from(variant: Ucswrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSWRST` reader - Software reset enable"]
pub type UcswrstR = crate::BitReader<Ucswrst>;
impl UcswrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucswrst {
        match self.bits {
            false => Ucswrst::Disable,
            true => Ucswrst::Enable,
        }
    }
    #[doc = "Disabled. eUSCI_A reset released for operation"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ucswrst::Disable
    }
    #[doc = "Enabled. eUSCI_A logic held in reset state"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ucswrst::Enable
    }
}
#[doc = "Field `UCSWRST` writer - Software reset enable"]
pub type UcswrstW<'a, REG> = crate::BitWriter<'a, REG, Ucswrst>;
impl<'a, REG> UcswrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. eUSCI_A reset released for operation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ucswrst::Disable)
    }
    #[doc = "Enabled. eUSCI_A logic held in reset state"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ucswrst::Enable)
    }
}
#[doc = "Transmit break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxbrk {
    #[doc = "0: Next frame transmitted is not a break"]
    Uctxbrk0 = 0,
    #[doc = "1: Next frame transmitted is a break or a break/synch"]
    Uctxbrk1 = 1,
}
impl From<Uctxbrk> for bool {
    #[inline(always)]
    fn from(variant: Uctxbrk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXBRK` reader - Transmit break"]
pub type UctxbrkR = crate::BitReader<Uctxbrk>;
impl UctxbrkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxbrk {
        match self.bits {
            false => Uctxbrk::Uctxbrk0,
            true => Uctxbrk::Uctxbrk1,
        }
    }
    #[doc = "Next frame transmitted is not a break"]
    #[inline(always)]
    pub fn is_uctxbrk_0(&self) -> bool {
        *self == Uctxbrk::Uctxbrk0
    }
    #[doc = "Next frame transmitted is a break or a break/synch"]
    #[inline(always)]
    pub fn is_uctxbrk_1(&self) -> bool {
        *self == Uctxbrk::Uctxbrk1
    }
}
#[doc = "Field `UCTXBRK` writer - Transmit break"]
pub type UctxbrkW<'a, REG> = crate::BitWriter<'a, REG, Uctxbrk>;
impl<'a, REG> UctxbrkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next frame transmitted is not a break"]
    #[inline(always)]
    pub fn uctxbrk_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxbrk::Uctxbrk0)
    }
    #[doc = "Next frame transmitted is a break or a break/synch"]
    #[inline(always)]
    pub fn uctxbrk_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxbrk::Uctxbrk1)
    }
}
#[doc = "Transmit address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uctxaddr {
    #[doc = "0: Next frame transmitted is data"]
    Uctxaddr0 = 0,
    #[doc = "1: Next frame transmitted is an address"]
    Uctxaddr1 = 1,
}
impl From<Uctxaddr> for bool {
    #[inline(always)]
    fn from(variant: Uctxaddr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXADDR` reader - Transmit address"]
pub type UctxaddrR = crate::BitReader<Uctxaddr>;
impl UctxaddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uctxaddr {
        match self.bits {
            false => Uctxaddr::Uctxaddr0,
            true => Uctxaddr::Uctxaddr1,
        }
    }
    #[doc = "Next frame transmitted is data"]
    #[inline(always)]
    pub fn is_uctxaddr_0(&self) -> bool {
        *self == Uctxaddr::Uctxaddr0
    }
    #[doc = "Next frame transmitted is an address"]
    #[inline(always)]
    pub fn is_uctxaddr_1(&self) -> bool {
        *self == Uctxaddr::Uctxaddr1
    }
}
#[doc = "Field `UCTXADDR` writer - Transmit address"]
pub type UctxaddrW<'a, REG> = crate::BitWriter<'a, REG, Uctxaddr>;
impl<'a, REG> UctxaddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next frame transmitted is data"]
    #[inline(always)]
    pub fn uctxaddr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxaddr::Uctxaddr0)
    }
    #[doc = "Next frame transmitted is an address"]
    #[inline(always)]
    pub fn uctxaddr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Uctxaddr::Uctxaddr1)
    }
}
#[doc = "Dormant\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucdorm {
    #[doc = "0: Not dormant. All received characters set UCRXIFG."]
    Ucdorm0 = 0,
    #[doc = "1: Dormant. Only characters that are preceded by an idle-line or with address bit set UCRXIFG. In UART mode with automatic baud-rate detection, only the combination of a break and synch field sets UCRXIFG."]
    Ucdorm1 = 1,
}
impl From<Ucdorm> for bool {
    #[inline(always)]
    fn from(variant: Ucdorm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCDORM` reader - Dormant"]
pub type UcdormR = crate::BitReader<Ucdorm>;
impl UcdormR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucdorm {
        match self.bits {
            false => Ucdorm::Ucdorm0,
            true => Ucdorm::Ucdorm1,
        }
    }
    #[doc = "Not dormant. All received characters set UCRXIFG."]
    #[inline(always)]
    pub fn is_ucdorm_0(&self) -> bool {
        *self == Ucdorm::Ucdorm0
    }
    #[doc = "Dormant. Only characters that are preceded by an idle-line or with address bit set UCRXIFG. In UART mode with automatic baud-rate detection, only the combination of a break and synch field sets UCRXIFG."]
    #[inline(always)]
    pub fn is_ucdorm_1(&self) -> bool {
        *self == Ucdorm::Ucdorm1
    }
}
#[doc = "Field `UCDORM` writer - Dormant"]
pub type UcdormW<'a, REG> = crate::BitWriter<'a, REG, Ucdorm>;
impl<'a, REG> UcdormW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not dormant. All received characters set UCRXIFG."]
    #[inline(always)]
    pub fn ucdorm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucdorm::Ucdorm0)
    }
    #[doc = "Dormant. Only characters that are preceded by an idle-line or with address bit set UCRXIFG. In UART mode with automatic baud-rate detection, only the combination of a break and synch field sets UCRXIFG."]
    #[inline(always)]
    pub fn ucdorm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucdorm::Ucdorm1)
    }
}
#[doc = "Receive break character interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucbrkie {
    #[doc = "0: Received break characters do not set UCRXIFG"]
    Ucbrkie0 = 0,
    #[doc = "1: Received break characters set UCRXIFG"]
    Ucbrkie1 = 1,
}
impl From<Ucbrkie> for bool {
    #[inline(always)]
    fn from(variant: Ucbrkie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBRKIE` reader - Receive break character interrupt enable"]
pub type UcbrkieR = crate::BitReader<Ucbrkie>;
impl UcbrkieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucbrkie {
        match self.bits {
            false => Ucbrkie::Ucbrkie0,
            true => Ucbrkie::Ucbrkie1,
        }
    }
    #[doc = "Received break characters do not set UCRXIFG"]
    #[inline(always)]
    pub fn is_ucbrkie_0(&self) -> bool {
        *self == Ucbrkie::Ucbrkie0
    }
    #[doc = "Received break characters set UCRXIFG"]
    #[inline(always)]
    pub fn is_ucbrkie_1(&self) -> bool {
        *self == Ucbrkie::Ucbrkie1
    }
}
#[doc = "Field `UCBRKIE` writer - Receive break character interrupt enable"]
pub type UcbrkieW<'a, REG> = crate::BitWriter<'a, REG, Ucbrkie>;
impl<'a, REG> UcbrkieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Received break characters do not set UCRXIFG"]
    #[inline(always)]
    pub fn ucbrkie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrkie::Ucbrkie0)
    }
    #[doc = "Received break characters set UCRXIFG"]
    #[inline(always)]
    pub fn ucbrkie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucbrkie::Ucbrkie1)
    }
}
#[doc = "Receive erroneous-character interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucrxeie {
    #[doc = "0: Erroneous characters rejected and UCRXIFG is not set"]
    Ucrxeie0 = 0,
    #[doc = "1: Erroneous characters received set UCRXIFG"]
    Ucrxeie1 = 1,
}
impl From<Ucrxeie> for bool {
    #[inline(always)]
    fn from(variant: Ucrxeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXEIE` reader - Receive erroneous-character interrupt enable"]
pub type UcrxeieR = crate::BitReader<Ucrxeie>;
impl UcrxeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucrxeie {
        match self.bits {
            false => Ucrxeie::Ucrxeie0,
            true => Ucrxeie::Ucrxeie1,
        }
    }
    #[doc = "Erroneous characters rejected and UCRXIFG is not set"]
    #[inline(always)]
    pub fn is_ucrxeie_0(&self) -> bool {
        *self == Ucrxeie::Ucrxeie0
    }
    #[doc = "Erroneous characters received set UCRXIFG"]
    #[inline(always)]
    pub fn is_ucrxeie_1(&self) -> bool {
        *self == Ucrxeie::Ucrxeie1
    }
}
#[doc = "Field `UCRXEIE` writer - Receive erroneous-character interrupt enable"]
pub type UcrxeieW<'a, REG> = crate::BitWriter<'a, REG, Ucrxeie>;
impl<'a, REG> UcrxeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Erroneous characters rejected and UCRXIFG is not set"]
    #[inline(always)]
    pub fn ucrxeie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxeie::Ucrxeie0)
    }
    #[doc = "Erroneous characters received set UCRXIFG"]
    #[inline(always)]
    pub fn ucrxeie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucrxeie::Ucrxeie1)
    }
}
#[doc = "eUSCI_A clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucssel {
    #[doc = "0: UCLK"]
    Uclk = 0,
    #[doc = "1: ACLK"]
    Aclk = 1,
    #[doc = "2: SMCLK"]
    Smclk = 2,
    #[doc = "3: SMCLK"]
    Ucssel3 = 3,
}
impl From<Ucssel> for u8 {
    #[inline(always)]
    fn from(variant: Ucssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucssel {
    type Ux = u8;
}
impl crate::IsEnum for Ucssel {}
#[doc = "Field `UCSSEL` reader - eUSCI_A clock source select"]
pub type UcsselR = crate::FieldReader<Ucssel>;
impl UcsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucssel {
        match self.bits {
            0 => Ucssel::Uclk,
            1 => Ucssel::Aclk,
            2 => Ucssel::Smclk,
            3 => Ucssel::Ucssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "UCLK"]
    #[inline(always)]
    pub fn is_uclk(&self) -> bool {
        *self == Ucssel::Uclk
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == Ucssel::Aclk
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == Ucssel::Smclk
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn is_ucssel_3(&self) -> bool {
        *self == Ucssel::Ucssel3
    }
}
#[doc = "Field `UCSSEL` writer - eUSCI_A clock source select"]
pub type UcsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucssel, crate::Safe>;
impl<'a, REG> UcsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UCLK"]
    #[inline(always)]
    pub fn uclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Uclk)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Aclk)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Smclk)
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn ucssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucssel::Ucssel3)
    }
}
#[doc = "Synchronous mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucsync {
    #[doc = "0: Asynchronous mode"]
    Async = 0,
    #[doc = "1: Synchronous mode"]
    Sync = 1,
}
impl From<Ucsync> for bool {
    #[inline(always)]
    fn from(variant: Ucsync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSYNC` reader - Synchronous mode enable"]
pub type UcsyncR = crate::BitReader<Ucsync>;
impl UcsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucsync {
        match self.bits {
            false => Ucsync::Async,
            true => Ucsync::Sync,
        }
    }
    #[doc = "Asynchronous mode"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == Ucsync::Async
    }
    #[doc = "Synchronous mode"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Ucsync::Sync
    }
}
#[doc = "Field `UCSYNC` writer - Synchronous mode enable"]
pub type UcsyncW<'a, REG> = crate::BitWriter<'a, REG, Ucsync>;
impl<'a, REG> UcsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous mode"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsync::Async)
    }
    #[doc = "Synchronous mode"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Ucsync::Sync)
    }
}
#[doc = "eUSCI_A mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ucmode {
    #[doc = "0: UART mode"]
    Ucmode0 = 0,
    #[doc = "1: Idle-line multiprocessor mode"]
    Ucmode1 = 1,
    #[doc = "2: Address-bit multiprocessor mode"]
    Ucmode2 = 2,
    #[doc = "3: UART mode with automatic baud-rate detection"]
    Ucmode3 = 3,
}
impl From<Ucmode> for u8 {
    #[inline(always)]
    fn from(variant: Ucmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ucmode {
    type Ux = u8;
}
impl crate::IsEnum for Ucmode {}
#[doc = "Field `UCMODE` reader - eUSCI_A mode"]
pub type UcmodeR = crate::FieldReader<Ucmode>;
impl UcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucmode {
        match self.bits {
            0 => Ucmode::Ucmode0,
            1 => Ucmode::Ucmode1,
            2 => Ucmode::Ucmode2,
            3 => Ucmode::Ucmode3,
            _ => unreachable!(),
        }
    }
    #[doc = "UART mode"]
    #[inline(always)]
    pub fn is_ucmode_0(&self) -> bool {
        *self == Ucmode::Ucmode0
    }
    #[doc = "Idle-line multiprocessor mode"]
    #[inline(always)]
    pub fn is_ucmode_1(&self) -> bool {
        *self == Ucmode::Ucmode1
    }
    #[doc = "Address-bit multiprocessor mode"]
    #[inline(always)]
    pub fn is_ucmode_2(&self) -> bool {
        *self == Ucmode::Ucmode2
    }
    #[doc = "UART mode with automatic baud-rate detection"]
    #[inline(always)]
    pub fn is_ucmode_3(&self) -> bool {
        *self == Ucmode::Ucmode3
    }
}
#[doc = "Field `UCMODE` writer - eUSCI_A mode"]
pub type UcmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ucmode, crate::Safe>;
impl<'a, REG> UcmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UART mode"]
    #[inline(always)]
    pub fn ucmode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode0)
    }
    #[doc = "Idle-line multiprocessor mode"]
    #[inline(always)]
    pub fn ucmode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode1)
    }
    #[doc = "Address-bit multiprocessor mode"]
    #[inline(always)]
    pub fn ucmode_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode2)
    }
    #[doc = "UART mode with automatic baud-rate detection"]
    #[inline(always)]
    pub fn ucmode_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmode::Ucmode3)
    }
}
#[doc = "Stop bit select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucspb {
    #[doc = "0: One stop bit"]
    Ucspb0 = 0,
    #[doc = "1: Two stop bits"]
    Ucspb1 = 1,
}
impl From<Ucspb> for bool {
    #[inline(always)]
    fn from(variant: Ucspb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSPB` reader - Stop bit select"]
pub type UcspbR = crate::BitReader<Ucspb>;
impl UcspbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucspb {
        match self.bits {
            false => Ucspb::Ucspb0,
            true => Ucspb::Ucspb1,
        }
    }
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn is_ucspb_0(&self) -> bool {
        *self == Ucspb::Ucspb0
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn is_ucspb_1(&self) -> bool {
        *self == Ucspb::Ucspb1
    }
}
#[doc = "Field `UCSPB` writer - Stop bit select"]
pub type UcspbW<'a, REG> = crate::BitWriter<'a, REG, Ucspb>;
impl<'a, REG> UcspbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn ucspb_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucspb::Ucspb0)
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn ucspb_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucspb::Ucspb1)
    }
}
#[doc = "Character length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uc7bit {
    #[doc = "0: 8-bit data"]
    _8bit = 0,
    #[doc = "1: 7-bit data"]
    _7bit = 1,
}
impl From<Uc7bit> for bool {
    #[inline(always)]
    fn from(variant: Uc7bit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UC7BIT` reader - Character length"]
pub type Uc7bitR = crate::BitReader<Uc7bit>;
impl Uc7bitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uc7bit {
        match self.bits {
            false => Uc7bit::_8bit,
            true => Uc7bit::_7bit,
        }
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Uc7bit::_8bit
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == Uc7bit::_7bit
    }
}
#[doc = "Field `UC7BIT` writer - Character length"]
pub type Uc7bitW<'a, REG> = crate::BitWriter<'a, REG, Uc7bit>;
impl<'a, REG> Uc7bitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Uc7bit::_8bit)
    }
    #[doc = "7-bit data"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut crate::W<REG> {
        self.variant(Uc7bit::_7bit)
    }
}
#[doc = "MSB first select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucmsb {
    #[doc = "0: LSB first"]
    Ucmsb0 = 0,
    #[doc = "1: MSB first"]
    Ucmsb1 = 1,
}
impl From<Ucmsb> for bool {
    #[inline(always)]
    fn from(variant: Ucmsb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCMSB` reader - MSB first select"]
pub type UcmsbR = crate::BitReader<Ucmsb>;
impl UcmsbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucmsb {
        match self.bits {
            false => Ucmsb::Ucmsb0,
            true => Ucmsb::Ucmsb1,
        }
    }
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn is_ucmsb_0(&self) -> bool {
        *self == Ucmsb::Ucmsb0
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn is_ucmsb_1(&self) -> bool {
        *self == Ucmsb::Ucmsb1
    }
}
#[doc = "Field `UCMSB` writer - MSB first select"]
pub type UcmsbW<'a, REG> = crate::BitWriter<'a, REG, Ucmsb>;
impl<'a, REG> UcmsbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSB first"]
    #[inline(always)]
    pub fn ucmsb_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmsb::Ucmsb0)
    }
    #[doc = "MSB first"]
    #[inline(always)]
    pub fn ucmsb_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucmsb::Ucmsb1)
    }
}
#[doc = "Parity select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucpar {
    #[doc = "0: Odd parity"]
    Odd = 0,
    #[doc = "1: Even parity"]
    Even = 1,
}
impl From<Ucpar> for bool {
    #[inline(always)]
    fn from(variant: Ucpar) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCPAR` reader - Parity select"]
pub type UcparR = crate::BitReader<Ucpar>;
impl UcparR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucpar {
        match self.bits {
            false => Ucpar::Odd,
            true => Ucpar::Even,
        }
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Ucpar::Odd
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Ucpar::Even
    }
}
#[doc = "Field `UCPAR` writer - Parity select"]
pub type UcparW<'a, REG> = crate::BitWriter<'a, REG, Ucpar>;
impl<'a, REG> UcparW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpar::Odd)
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpar::Even)
    }
}
#[doc = "Parity enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ucpen {
    #[doc = "0: Parity disabled"]
    Ucpen0 = 0,
    #[doc = "1: Parity enabled. Parity bit is generated (UCAxTXD) and expected (UCAxRXD). In address-bit multiprocessor mode, the address bit is included in the parity calculation."]
    Ucpen1 = 1,
}
impl From<Ucpen> for bool {
    #[inline(always)]
    fn from(variant: Ucpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCPEN` reader - Parity enable"]
pub type UcpenR = crate::BitReader<Ucpen>;
impl UcpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ucpen {
        match self.bits {
            false => Ucpen::Ucpen0,
            true => Ucpen::Ucpen1,
        }
    }
    #[doc = "Parity disabled"]
    #[inline(always)]
    pub fn is_ucpen_0(&self) -> bool {
        *self == Ucpen::Ucpen0
    }
    #[doc = "Parity enabled. Parity bit is generated (UCAxTXD) and expected (UCAxRXD). In address-bit multiprocessor mode, the address bit is included in the parity calculation."]
    #[inline(always)]
    pub fn is_ucpen_1(&self) -> bool {
        *self == Ucpen::Ucpen1
    }
}
#[doc = "Field `UCPEN` writer - Parity enable"]
pub type UcpenW<'a, REG> = crate::BitWriter<'a, REG, Ucpen>;
impl<'a, REG> UcpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity disabled"]
    #[inline(always)]
    pub fn ucpen_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpen::Ucpen0)
    }
    #[doc = "Parity enabled. Parity bit is generated (UCAxTXD) and expected (UCAxRXD). In address-bit multiprocessor mode, the address bit is included in the parity calculation."]
    #[inline(always)]
    pub fn ucpen_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ucpen::Ucpen1)
    }
}
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&self) -> UcswrstR {
        UcswrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit break"]
    #[inline(always)]
    pub fn uctxbrk(&self) -> UctxbrkR {
        UctxbrkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit address"]
    #[inline(always)]
    pub fn uctxaddr(&self) -> UctxaddrR {
        UctxaddrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dormant"]
    #[inline(always)]
    pub fn ucdorm(&self) -> UcdormR {
        UcdormR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive break character interrupt enable"]
    #[inline(always)]
    pub fn ucbrkie(&self) -> UcbrkieR {
        UcbrkieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive erroneous-character interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&self) -> UcrxeieR {
        UcrxeieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - eUSCI_A clock source select"]
    #[inline(always)]
    pub fn ucssel(&self) -> UcsselR {
        UcsselR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&self) -> UcsyncR {
        UcsyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - eUSCI_A mode"]
    #[inline(always)]
    pub fn ucmode(&self) -> UcmodeR {
        UcmodeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Stop bit select"]
    #[inline(always)]
    pub fn ucspb(&self) -> UcspbR {
        UcspbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&self) -> Uc7bitR {
        Uc7bitR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&self) -> UcmsbR {
        UcmsbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Parity select"]
    #[inline(always)]
    pub fn ucpar(&self) -> UcparR {
        UcparR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Parity enable"]
    #[inline(always)]
    pub fn ucpen(&self) -> UcpenR {
        UcpenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn ucswrst(&mut self) -> UcswrstW<'_, Uca1ctlw0Spec> {
        UcswrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit break"]
    #[inline(always)]
    pub fn uctxbrk(&mut self) -> UctxbrkW<'_, Uca1ctlw0Spec> {
        UctxbrkW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit address"]
    #[inline(always)]
    pub fn uctxaddr(&mut self) -> UctxaddrW<'_, Uca1ctlw0Spec> {
        UctxaddrW::new(self, 2)
    }
    #[doc = "Bit 3 - Dormant"]
    #[inline(always)]
    pub fn ucdorm(&mut self) -> UcdormW<'_, Uca1ctlw0Spec> {
        UcdormW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive break character interrupt enable"]
    #[inline(always)]
    pub fn ucbrkie(&mut self) -> UcbrkieW<'_, Uca1ctlw0Spec> {
        UcbrkieW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive erroneous-character interrupt enable"]
    #[inline(always)]
    pub fn ucrxeie(&mut self) -> UcrxeieW<'_, Uca1ctlw0Spec> {
        UcrxeieW::new(self, 5)
    }
    #[doc = "Bits 6:7 - eUSCI_A clock source select"]
    #[inline(always)]
    pub fn ucssel(&mut self) -> UcsselW<'_, Uca1ctlw0Spec> {
        UcsselW::new(self, 6)
    }
    #[doc = "Bit 8 - Synchronous mode enable"]
    #[inline(always)]
    pub fn ucsync(&mut self) -> UcsyncW<'_, Uca1ctlw0Spec> {
        UcsyncW::new(self, 8)
    }
    #[doc = "Bits 9:10 - eUSCI_A mode"]
    #[inline(always)]
    pub fn ucmode(&mut self) -> UcmodeW<'_, Uca1ctlw0Spec> {
        UcmodeW::new(self, 9)
    }
    #[doc = "Bit 11 - Stop bit select"]
    #[inline(always)]
    pub fn ucspb(&mut self) -> UcspbW<'_, Uca1ctlw0Spec> {
        UcspbW::new(self, 11)
    }
    #[doc = "Bit 12 - Character length"]
    #[inline(always)]
    pub fn uc7bit(&mut self) -> Uc7bitW<'_, Uca1ctlw0Spec> {
        Uc7bitW::new(self, 12)
    }
    #[doc = "Bit 13 - MSB first select"]
    #[inline(always)]
    pub fn ucmsb(&mut self) -> UcmsbW<'_, Uca1ctlw0Spec> {
        UcmsbW::new(self, 13)
    }
    #[doc = "Bit 14 - Parity select"]
    #[inline(always)]
    pub fn ucpar(&mut self) -> UcparW<'_, Uca1ctlw0Spec> {
        UcparW::new(self, 14)
    }
    #[doc = "Bit 15 - Parity enable"]
    #[inline(always)]
    pub fn ucpen(&mut self) -> UcpenW<'_, Uca1ctlw0Spec> {
        UcpenW::new(self, 15)
    }
}
#[doc = "eUSCI_Ax Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uca1ctlw0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uca1ctlw0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uca1ctlw0Spec;
impl crate::RegisterSpec for Uca1ctlw0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`uca1ctlw0::R`](R) reader structure"]
impl crate::Readable for Uca1ctlw0Spec {}
#[doc = "`write(|w| ..)` method takes [`uca1ctlw0::W`](W) writer structure"]
impl crate::Writable for Uca1ctlw0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCA1CTLW0 to value 0"]
impl crate::Resettable for Uca1ctlw0Spec {}
