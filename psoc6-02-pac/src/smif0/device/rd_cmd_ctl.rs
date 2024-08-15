#[doc = "Register `RD_CMD_CTL` reader"]
pub type R = crate::R<RdCmdCtlSpec>;
#[doc = "Register `RD_CMD_CTL` writer"]
pub type W = crate::W<RdCmdCtlSpec>;
#[doc = "Field `CODE` reader - Command byte code."]
pub type CodeR = crate::FieldReader;
#[doc = "Field `CODE` writer - Command byte code."]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIDTH` reader - Width of data transfer: '0': 1 bit/cycle (single data transfer). '1': 2 bits/cycle (dual data transfer). '2': 4 bits/cycle (quad data transfer). '3': 8 bits/cycle (octal data transfer)."]
pub type WidthR = crate::FieldReader;
#[doc = "Field `WIDTH` writer - Width of data transfer: '0': 1 bit/cycle (single data transfer). '1': 2 bits/cycle (dual data transfer). '2': 4 bits/cycle (quad data transfer). '3': 8 bits/cycle (octal data transfer)."]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRESENT` reader - Presence of command field: '0': not present '1': present"]
pub type PresentR = crate::BitReader;
#[doc = "Field `PRESENT` writer - Presence of command field: '0': not present '1': present"]
pub type PresentW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command byte code."]
    #[inline(always)]
    pub fn code(&self) -> CodeR {
        CodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Width of data transfer: '0': 1 bit/cycle (single data transfer). '1': 2 bits/cycle (dual data transfer). '2': 4 bits/cycle (quad data transfer). '3': 8 bits/cycle (octal data transfer)."]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - Presence of command field: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&self) -> PresentR {
        PresentR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command byte code."]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CodeW<RdCmdCtlSpec> {
        CodeW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Width of data transfer: '0': 1 bit/cycle (single data transfer). '1': 2 bits/cycle (dual data transfer). '2': 4 bits/cycle (quad data transfer). '3': 8 bits/cycle (octal data transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WidthW<RdCmdCtlSpec> {
        WidthW::new(self, 16)
    }
    #[doc = "Bit 31 - Presence of command field: '0': not present '1': present"]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PresentW<RdCmdCtlSpec> {
        PresentW::new(self, 31)
    }
}
#[doc = "Read command control\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_cmd_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_cmd_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdCmdCtlSpec;
impl crate::RegisterSpec for RdCmdCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_cmd_ctl::R`](R) reader structure"]
impl crate::Readable for RdCmdCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`rd_cmd_ctl::W`](W) writer structure"]
impl crate::Writable for RdCmdCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RD_CMD_CTL to value 0"]
impl crate::Resettable for RdCmdCtlSpec {
    const RESET_VALUE: u32 = 0;
}
