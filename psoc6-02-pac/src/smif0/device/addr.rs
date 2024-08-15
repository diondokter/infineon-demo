#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `ADDR` reader - Specifies the base address of the device region. If the device region is 2^m Bytes, ADDR MUST be a multiple of 2^m. In dual quad SPI data transfer, the two devices should have the same ADDR and MASK register settings. The device control information (ADDR_CTL, RD_CMD_CTL, etc.) are provided by the MMIO control registers of the device with the lowest index. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\]
= SMIF_XIP_ADDR\\[31:24\\]."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Specifies the base address of the device region. If the device region is 2^m Bytes, ADDR MUST be a multiple of 2^m. In dual quad SPI data transfer, the two devices should have the same ADDR and MASK register settings. The device control information (ADDR_CTL, RD_CMD_CTL, etc.) are provided by the MMIO control registers of the device with the lowest index. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\]
= SMIF_XIP_ADDR\\[31:24\\]."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - Specifies the base address of the device region. If the device region is 2^m Bytes, ADDR MUST be a multiple of 2^m. In dual quad SPI data transfer, the two devices should have the same ADDR and MASK register settings. The device control information (ADDR_CTL, RD_CMD_CTL, etc.) are provided by the MMIO control registers of the device with the lowest index. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\]
= SMIF_XIP_ADDR\\[31:24\\]."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Specifies the base address of the device region. If the device region is 2^m Bytes, ADDR MUST be a multiple of 2^m. In dual quad SPI data transfer, the two devices should have the same ADDR and MASK register settings. The device control information (ADDR_CTL, RD_CMD_CTL, etc.) are provided by the MMIO control registers of the device with the lowest index. The most significant bit fields are constants and set based on the SMIF_XIP_ADDR parameter. The most significant bits are identified on the SMIF_XIP_MASK parameter. E.g., if SMIF_XIP_MASK is 0xff00:0000 (16 MB XIP memory region), ADDR\\[31:24\\]
= SMIF_XIP_ADDR\\[31:24\\]."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<AddrSpec> {
        AddrW::new(self, 8)
    }
}
#[doc = "Device region base address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {
    const RESET_VALUE: u32 = 0;
}
