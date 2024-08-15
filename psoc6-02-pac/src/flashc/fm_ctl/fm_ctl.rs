#[doc = "Register `FM_CTL` reader"]
pub type R = crate::R<FmCtlSpec>;
#[doc = "Register `FM_CTL` writer"]
pub type W = crate::W<FmCtlSpec>;
#[doc = "Field `FM_MODE` reader - Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
pub type FmModeR = crate::FieldReader;
#[doc = "Field `FM_MODE` writer - Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
pub type FmModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FM_SEQ` reader - Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
pub type FmSeqR = crate::FieldReader;
#[doc = "Field `FM_SEQ` writer - Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
pub type FmSeqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAA_MUX_SEL` reader - Direct memory cell access address."]
pub type DaaMuxSelR = crate::FieldReader;
#[doc = "Field `DAA_MUX_SEL` writer - Direct memory cell access address."]
pub type DaaMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `IF_SEL` reader - Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
pub type IfSelR = crate::BitReader;
#[doc = "Field `IF_SEL` writer - Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
pub type IfSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_EN` reader - 0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
pub type WrEnR = crate::BitReader;
#[doc = "Field `WR_EN` writer - 0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
pub type WrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
    #[inline(always)]
    pub fn fm_mode(&self) -> FmModeR {
        FmModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
    #[inline(always)]
    pub fn fm_seq(&self) -> FmSeqR {
        FmSeqR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Direct memory cell access address."]
    #[inline(always)]
    pub fn daa_mux_sel(&self) -> DaaMuxSelR {
        DaaMuxSelR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    pub fn if_sel(&self) -> IfSelR {
        IfSelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    pub fn wr_en(&self) -> WrEnR {
        WrEnR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Requires (IF_SEL|WR_EN)=1 Flash macro mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn fm_mode(&mut self) -> FmModeW<FmCtlSpec> {
        FmModeW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Requires (IF_SEL|WR_EN)=1 Flash macro sequence selection"]
    #[inline(always)]
    #[must_use]
    pub fn fm_seq(&mut self) -> FmSeqW<FmCtlSpec> {
        FmSeqW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Direct memory cell access address."]
    #[inline(always)]
    #[must_use]
    pub fn daa_mux_sel(&mut self) -> DaaMuxSelW<FmCtlSpec> {
        DaaMuxSelW::new(self, 16)
    }
    #[doc = "Bit 24 - Interface selection. Specifies the interface that is used for flash memory read operations: 0: R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. 1: C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure. Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    #[must_use]
    pub fn if_sel(&mut self) -> IfSelW<FmCtlSpec> {
        IfSelW::new(self, 24)
    }
    #[doc = "Bit 25 - 0: normal mode 1: Fm Write Enable Note: IF_SEL and WR_EN cannot be changed at the same time"]
    #[inline(always)]
    #[must_use]
    pub fn wr_en(&mut self) -> WrEnW<FmCtlSpec> {
        WrEnW::new(self, 25)
    }
}
#[doc = "Flash macro control\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmCtlSpec;
impl crate::RegisterSpec for FmCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_ctl::R`](R) reader structure"]
impl crate::Readable for FmCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`fm_ctl::W`](W) writer structure"]
impl crate::Writable for FmCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FM_CTL to value 0"]
impl crate::Resettable for FmCtlSpec {
    const RESET_VALUE: u32 = 0;
}
