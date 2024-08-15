#[doc = "Register `IPTAT_TRIM0` reader"]
pub type R = crate::R<IptatTrim0Spec>;
#[doc = "Register `IPTAT_TRIM0` writer"]
pub type W = crate::W<IptatTrim0Spec>;
#[doc = "Field `IPTAT_CORE_TRIM` reader - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
pub type IptatCoreTrimR = crate::FieldReader;
#[doc = "Field `IPTAT_CORE_TRIM` writer - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
pub type IptatCoreTrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IPTAT_CTBM_TRIM` reader - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
pub type IptatCtbmTrimR = crate::FieldReader;
#[doc = "Field `IPTAT_CTBM_TRIM` writer - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
pub type IptatCtbmTrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn iptat_core_trim(&self) -> IptatCoreTrimR {
        IptatCoreTrimR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
    #[inline(always)]
    pub fn iptat_ctbm_trim(&self) -> IptatCtbmTrimR {
        IptatCtbmTrimR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
    #[inline(always)]
    #[must_use]
    pub fn iptat_core_trim(&mut self) -> IptatCoreTrimW<IptatTrim0Spec> {
        IptatCoreTrimW::new(self, 0)
    }
    #[doc = "Bits 4:7 - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
    #[inline(always)]
    #[must_use]
    pub fn iptat_ctbm_trim(&mut self) -> IptatCtbmTrimW<IptatTrim0Spec> {
        IptatCtbmTrimW::new(self, 4)
    }
}
#[doc = "IPTAT Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`iptat_trim0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iptat_trim0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IptatTrim0Spec;
impl crate::RegisterSpec for IptatTrim0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iptat_trim0::R`](R) reader structure"]
impl crate::Readable for IptatTrim0Spec {}
#[doc = "`write(|w| ..)` method takes [`iptat_trim0::W`](W) writer structure"]
impl crate::Writable for IptatTrim0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPTAT_TRIM0 to value 0"]
impl crate::Resettable for IptatTrim0Spec {
    const RESET_VALUE: u32 = 0;
}
