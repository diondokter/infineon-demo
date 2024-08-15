#[doc = "Register `FLASH_CMD` reader"]
pub type R = crate::R<FlashCmdSpec>;
#[doc = "Register `FLASH_CMD` writer"]
pub type W = crate::W<FlashCmdSpec>;
#[doc = "Field `INV` reader - Invalidation of ALL caches (for CM0+ and CM4) and ALL buffers. SW writes a '1' to clear the caches. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The caches' LRU structures are also reset to their default state."]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - Invalidation of ALL caches (for CM0+ and CM4) and ALL buffers. SW writes a '1' to clear the caches. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The caches' LRU structures are also reset to their default state."]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFF_INV` reader - Invalidation of ALL buffers (does not invalidate the caches). SW writes a '1' to clear the buffers. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. Note: the caches only capture FLASH macro main array data. Therefore, invalidating just the buffers (BUFF_INV) does not invalidate captures main array data in the caches."]
pub type BuffInvR = crate::BitReader;
#[doc = "Field `BUFF_INV` writer - Invalidation of ALL buffers (does not invalidate the caches). SW writes a '1' to clear the buffers. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. Note: the caches only capture FLASH macro main array data. Therefore, invalidating just the buffers (BUFF_INV) does not invalidate captures main array data in the caches."]
pub type BuffInvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Invalidation of ALL caches (for CM0+ and CM4) and ALL buffers. SW writes a '1' to clear the caches. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The caches' LRU structures are also reset to their default state."]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invalidation of ALL buffers (does not invalidate the caches). SW writes a '1' to clear the buffers. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. Note: the caches only capture FLASH macro main array data. Therefore, invalidating just the buffers (BUFF_INV) does not invalidate captures main array data in the caches."]
    #[inline(always)]
    pub fn buff_inv(&self) -> BuffInvR {
        BuffInvR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalidation of ALL caches (for CM0+ and CM4) and ALL buffers. SW writes a '1' to clear the caches. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. The caches' LRU structures are also reset to their default state."]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> InvW<FlashCmdSpec> {
        InvW::new(self, 0)
    }
    #[doc = "Bit 1 - Invalidation of ALL buffers (does not invalidate the caches). SW writes a '1' to clear the buffers. HW sets this field to '0' when the operation is completed. The operation takes a maximum of three clock cycles on the slowest of the clk_slow and clk_fast clocks. Note: the caches only capture FLASH macro main array data. Therefore, invalidating just the buffers (BUFF_INV) does not invalidate captures main array data in the caches."]
    #[inline(always)]
    #[must_use]
    pub fn buff_inv(&mut self) -> BuffInvW<FlashCmdSpec> {
        BuffInvW::new(self, 1)
    }
}
#[doc = "Command\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashCmdSpec;
impl crate::RegisterSpec for FlashCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_cmd::R`](R) reader structure"]
impl crate::Readable for FlashCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_cmd::W`](W) writer structure"]
impl crate::Writable for FlashCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_CMD to value 0"]
impl crate::Resettable for FlashCmdSpec {
    const RESET_VALUE: u32 = 0;
}
