#[doc = "Register `USB_CLK_EN` reader"]
pub type R = crate::R<UsbClkEnSpec>;
#[doc = "Register `USB_CLK_EN` writer"]
pub type W = crate::W<UsbClkEnSpec>;
#[doc = "Field `CSR_CLK_EN` reader - Clock Enable for Core Logic clocked by AHB bus clock"]
pub type CsrClkEnR = crate::BitReader;
#[doc = "Field `CSR_CLK_EN` writer - Clock Enable for Core Logic clocked by AHB bus clock"]
pub type CsrClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Enable for Core Logic clocked by AHB bus clock"]
    #[inline(always)]
    pub fn csr_clk_en(&self) -> CsrClkEnR {
        CsrClkEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable for Core Logic clocked by AHB bus clock"]
    #[inline(always)]
    #[must_use]
    pub fn csr_clk_en(&mut self) -> CsrClkEnW<UsbClkEnSpec> {
        CsrClkEnW::new(self, 0)
    }
}
#[doc = "USB Block Clock Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbClkEnSpec;
impl crate::RegisterSpec for UsbClkEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_clk_en::R`](R) reader structure"]
impl crate::Readable for UsbClkEnSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_clk_en::W`](W) writer structure"]
impl crate::Writable for UsbClkEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB_CLK_EN to value 0"]
impl crate::Resettable for UsbClkEnSpec {
    const RESET_VALUE: u32 = 0;
}
