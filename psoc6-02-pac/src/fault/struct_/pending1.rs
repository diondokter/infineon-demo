#[doc = "Register `PENDING1` reader"]
pub type R = crate::R<Pending1Spec>;
#[doc = "Field `SOURCE` reader - This field specifies the following sources: Bit 0: Peripheral group 0 PPU. Bit 1: Peripheral group 1 PPU. Bit 2: Peripheral group 2 PPU. Bit 3: Peripheral group 3 PPU. Bit 4: Peripheral group 4 PPU. Bit 5: Peripheral group 5 PPU. Bit 6: Peripheral group 6 PPU. Bit 7: Peripheral group 7 PPU. ... Bit 15: Peripheral group 15 PPU. Bit 16 - 31: See STATUS register."]
pub type SourceR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field specifies the following sources: Bit 0: Peripheral group 0 PPU. Bit 1: Peripheral group 1 PPU. Bit 2: Peripheral group 2 PPU. Bit 3: Peripheral group 3 PPU. Bit 4: Peripheral group 4 PPU. Bit 5: Peripheral group 5 PPU. Bit 6: Peripheral group 6 PPU. Bit 7: Peripheral group 7 PPU. ... Bit 15: Peripheral group 15 PPU. Bit 16 - 31: See STATUS register."]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(self.bits)
    }
}
#[doc = "Fault pending 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pending1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pending1Spec;
impl crate::RegisterSpec for Pending1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending1::R`](R) reader structure"]
impl crate::Readable for Pending1Spec {}
#[doc = "`reset()` method sets PENDING1 to value 0"]
impl crate::Resettable for Pending1Spec {
    const RESET_VALUE: u32 = 0;
}
