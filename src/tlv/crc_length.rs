#[doc = "Register `CRC_LENGTH` reader"]
pub type R = crate::R<CrcLengthSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "CRC length\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_length::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcLengthSpec;
impl crate::RegisterSpec for CrcLengthSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`crc_length::R`](R) reader structure"]
impl crate::Readable for CrcLengthSpec {}
#[doc = "`reset()` method sets CRC_LENGTH to value 0"]
impl crate::Resettable for CrcLengthSpec {}
