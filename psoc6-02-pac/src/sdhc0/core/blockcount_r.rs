#[doc = "Register `BLOCKCOUNT_R` reader"]
pub type R = crate::R<BlockcountRSpec>;
#[doc = "Register `BLOCKCOUNT_R` writer"]
pub type W = crate::W<BlockcountRSpec>;
#[doc = "Field `BLOCK_CNT` reader - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
pub type BlockCntR = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_CNT` writer - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
pub type BlockCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
    #[inline(always)]
    pub fn block_cnt(&self) -> BlockCntR {
        BlockCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - ... - ... - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
    #[inline(always)]
    #[must_use]
    pub fn block_cnt(&mut self) -> BlockCntW<BlockcountRSpec> {
        BlockCntW::new(self, 0)
    }
}
#[doc = "16-bit Block Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`blockcount_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blockcount_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlockcountRSpec;
impl crate::RegisterSpec for BlockcountRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`blockcount_r::R`](R) reader structure"]
impl crate::Readable for BlockcountRSpec {}
#[doc = "`write(|w| ..)` method takes [`blockcount_r::W`](W) writer structure"]
impl crate::Writable for BlockcountRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BLOCKCOUNT_R to value 0"]
impl crate::Resettable for BlockcountRSpec {
    const RESET_VALUE: u16 = 0;
}
