#[doc = "Register `BLOCKSIZE_R` reader"]
pub type R = crate::R<BlocksizeRSpec>;
#[doc = "Register `BLOCKSIZE_R` writer"]
pub type W = crate::W<BlocksizeRSpec>;
#[doc = "Field `XFER_BLOCK_SIZE` reader - Transfer Block Size These bits specify the block size of data transfers. In case of memory, it is set to 512 bytes. It can be accessed only if no transaction is executing. Read operations during transfers may return an invalid value, and write operations are ignored. Following are the values for XFER_BLOCK_SIZE: - 0x1: 1 byte - 0x2: 2 bytes - 0x3: 3 bytes - ...... - 0x1FF: 511 byte - 0x200: 512 bytes - ...... - 0x800: 2048 bytes Note: This register must be programmed with a non-zero value for data transfer."]
pub type XferBlockSizeR = crate::FieldReader<u16>;
#[doc = "Field `XFER_BLOCK_SIZE` writer - Transfer Block Size These bits specify the block size of data transfers. In case of memory, it is set to 512 bytes. It can be accessed only if no transaction is executing. Read operations during transfers may return an invalid value, and write operations are ignored. Following are the values for XFER_BLOCK_SIZE: - 0x1: 1 byte - 0x2: 2 bytes - 0x3: 3 bytes - ...... - 0x1FF: 511 byte - 0x200: 512 bytes - ...... - 0x800: 2048 bytes Note: This register must be programmed with a non-zero value for data transfer."]
pub type XferBlockSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SDMA_BUF_BDARY` reader - SDMA Buffer Boundary These bits specify the size of contiguous buffer in system memory. The SDMA transfer waits at every boundary specified by these fields and the Host Controller generates the DMA interrupt to request the Host Driver to update the SDMA System Address register. Values: - 0x0 (BYTES_4K): 4K bytes SDMA Buffer Boundary - 0x1 (BYTES_8K): 8K bytes SDMA Buffer Boundary - 0x2 (BYTES_16K): 16K bytes SDMA Buffer Boundary - 0x3 (BYTES_32K): 32K bytes SDMA Buffer Boundary - 0x4 (BYTES_64K): 64K bytes SDMA Buffer Boundary - 0x5 (BYTES_128K): 128K bytes SDMA Buffer Boundary - 0x6 (BYTES_256K): 256K bytes SDMA Buffer Boundary - 0x7 (BYTES_512K): 512K bytes SDMA Buffer Boundary"]
pub type SdmaBufBdaryR = crate::FieldReader;
#[doc = "Field `SDMA_BUF_BDARY` writer - SDMA Buffer Boundary These bits specify the size of contiguous buffer in system memory. The SDMA transfer waits at every boundary specified by these fields and the Host Controller generates the DMA interrupt to request the Host Driver to update the SDMA System Address register. Values: - 0x0 (BYTES_4K): 4K bytes SDMA Buffer Boundary - 0x1 (BYTES_8K): 8K bytes SDMA Buffer Boundary - 0x2 (BYTES_16K): 16K bytes SDMA Buffer Boundary - 0x3 (BYTES_32K): 32K bytes SDMA Buffer Boundary - 0x4 (BYTES_64K): 64K bytes SDMA Buffer Boundary - 0x5 (BYTES_128K): 128K bytes SDMA Buffer Boundary - 0x6 (BYTES_256K): 256K bytes SDMA Buffer Boundary - 0x7 (BYTES_512K): 512K bytes SDMA Buffer Boundary"]
pub type SdmaBufBdaryW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - Transfer Block Size These bits specify the block size of data transfers. In case of memory, it is set to 512 bytes. It can be accessed only if no transaction is executing. Read operations during transfers may return an invalid value, and write operations are ignored. Following are the values for XFER_BLOCK_SIZE: - 0x1: 1 byte - 0x2: 2 bytes - 0x3: 3 bytes - ...... - 0x1FF: 511 byte - 0x200: 512 bytes - ...... - 0x800: 2048 bytes Note: This register must be programmed with a non-zero value for data transfer."]
    #[inline(always)]
    pub fn xfer_block_size(&self) -> XferBlockSizeR {
        XferBlockSizeR::new(self.bits & 0x0fff)
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary These bits specify the size of contiguous buffer in system memory. The SDMA transfer waits at every boundary specified by these fields and the Host Controller generates the DMA interrupt to request the Host Driver to update the SDMA System Address register. Values: - 0x0 (BYTES_4K): 4K bytes SDMA Buffer Boundary - 0x1 (BYTES_8K): 8K bytes SDMA Buffer Boundary - 0x2 (BYTES_16K): 16K bytes SDMA Buffer Boundary - 0x3 (BYTES_32K): 32K bytes SDMA Buffer Boundary - 0x4 (BYTES_64K): 64K bytes SDMA Buffer Boundary - 0x5 (BYTES_128K): 128K bytes SDMA Buffer Boundary - 0x6 (BYTES_256K): 256K bytes SDMA Buffer Boundary - 0x7 (BYTES_512K): 512K bytes SDMA Buffer Boundary"]
    #[inline(always)]
    pub fn sdma_buf_bdary(&self) -> SdmaBufBdaryR {
        SdmaBufBdaryR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer Block Size These bits specify the block size of data transfers. In case of memory, it is set to 512 bytes. It can be accessed only if no transaction is executing. Read operations during transfers may return an invalid value, and write operations are ignored. Following are the values for XFER_BLOCK_SIZE: - 0x1: 1 byte - 0x2: 2 bytes - 0x3: 3 bytes - ...... - 0x1FF: 511 byte - 0x200: 512 bytes - ...... - 0x800: 2048 bytes Note: This register must be programmed with a non-zero value for data transfer."]
    #[inline(always)]
    #[must_use]
    pub fn xfer_block_size(&mut self) -> XferBlockSizeW<BlocksizeRSpec> {
        XferBlockSizeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - SDMA Buffer Boundary These bits specify the size of contiguous buffer in system memory. The SDMA transfer waits at every boundary specified by these fields and the Host Controller generates the DMA interrupt to request the Host Driver to update the SDMA System Address register. Values: - 0x0 (BYTES_4K): 4K bytes SDMA Buffer Boundary - 0x1 (BYTES_8K): 8K bytes SDMA Buffer Boundary - 0x2 (BYTES_16K): 16K bytes SDMA Buffer Boundary - 0x3 (BYTES_32K): 32K bytes SDMA Buffer Boundary - 0x4 (BYTES_64K): 64K bytes SDMA Buffer Boundary - 0x5 (BYTES_128K): 128K bytes SDMA Buffer Boundary - 0x6 (BYTES_256K): 256K bytes SDMA Buffer Boundary - 0x7 (BYTES_512K): 512K bytes SDMA Buffer Boundary"]
    #[inline(always)]
    #[must_use]
    pub fn sdma_buf_bdary(&mut self) -> SdmaBufBdaryW<BlocksizeRSpec> {
        SdmaBufBdaryW::new(self, 12)
    }
}
#[doc = "Block Size register\n\nYou can [`read`](crate::Reg::read) this register and get [`blocksize_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blocksize_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlocksizeRSpec;
impl crate::RegisterSpec for BlocksizeRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`blocksize_r::R`](R) reader structure"]
impl crate::Readable for BlocksizeRSpec {}
#[doc = "`write(|w| ..)` method takes [`blocksize_r::W`](W) writer structure"]
impl crate::Writable for BlocksizeRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BLOCKSIZE_R to value 0"]
impl crate::Resettable for BlocksizeRSpec {
    const RESET_VALUE: u16 = 0;
}
