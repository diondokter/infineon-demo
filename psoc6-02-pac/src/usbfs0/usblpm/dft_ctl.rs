#[doc = "Register `DFT_CTL` reader"]
pub type R = crate::R<DftCtlSpec>;
#[doc = "Register `DFT_CTL` writer"]
pub type W = crate::W<DftCtlSpec>;
#[doc = "DDFT output select signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DdftOutSel {
    #[doc = "0: Nothing connected, output 0"]
    Off = 0,
    #[doc = "1: Single Ended output of DP"]
    DpSe = 1,
    #[doc = "2: Single Ended output of DM"]
    DmSe = 2,
    #[doc = "3: Output Enable"]
    Txoe = 3,
    #[doc = "4: Differential Receiver output"]
    RcvDf = 4,
    #[doc = "5: GPIO output of DP"]
    GpioDpOut = 5,
    #[doc = "6: GPIO output of DM"]
    GpioDmOut = 6,
}
impl From<DdftOutSel> for u8 {
    #[inline(always)]
    fn from(variant: DdftOutSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DdftOutSel {
    type Ux = u8;
}
impl crate::IsEnum for DdftOutSel {}
#[doc = "Field `DDFT_OUT_SEL` reader - DDFT output select signal"]
pub type DdftOutSelR = crate::FieldReader<DdftOutSel>;
impl DdftOutSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DdftOutSel> {
        match self.bits {
            0 => Some(DdftOutSel::Off),
            1 => Some(DdftOutSel::DpSe),
            2 => Some(DdftOutSel::DmSe),
            3 => Some(DdftOutSel::Txoe),
            4 => Some(DdftOutSel::RcvDf),
            5 => Some(DdftOutSel::GpioDpOut),
            6 => Some(DdftOutSel::GpioDmOut),
            _ => None,
        }
    }
    #[doc = "Nothing connected, output 0"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DdftOutSel::Off
    }
    #[doc = "Single Ended output of DP"]
    #[inline(always)]
    pub fn is_dp_se(&self) -> bool {
        *self == DdftOutSel::DpSe
    }
    #[doc = "Single Ended output of DM"]
    #[inline(always)]
    pub fn is_dm_se(&self) -> bool {
        *self == DdftOutSel::DmSe
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub fn is_txoe(&self) -> bool {
        *self == DdftOutSel::Txoe
    }
    #[doc = "Differential Receiver output"]
    #[inline(always)]
    pub fn is_rcv_df(&self) -> bool {
        *self == DdftOutSel::RcvDf
    }
    #[doc = "GPIO output of DP"]
    #[inline(always)]
    pub fn is_gpio_dp_out(&self) -> bool {
        *self == DdftOutSel::GpioDpOut
    }
    #[doc = "GPIO output of DM"]
    #[inline(always)]
    pub fn is_gpio_dm_out(&self) -> bool {
        *self == DdftOutSel::GpioDmOut
    }
}
#[doc = "Field `DDFT_OUT_SEL` writer - DDFT output select signal"]
pub type DdftOutSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, DdftOutSel>;
impl<'a, REG> DdftOutSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Nothing connected, output 0"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(DdftOutSel::Off)
    }
    #[doc = "Single Ended output of DP"]
    #[inline(always)]
    pub fn dp_se(self) -> &'a mut crate::W<REG> {
        self.variant(DdftOutSel::DpSe)
    }
    #[doc = "Single Ended output of DM"]
    #[inline(always)]
    pub fn dm_se(self) -> &'a mut crate::W<REG> {
        self.variant(DdftOutSel::DmSe)
    }
    #[doc = "Output Enable"]
    #[inline(always)]
    pub fn txoe(self) -> &'a mut crate::W<REG> {
        self.variant(DdftOutSel::Txoe)
    }
    #[doc = "Differential Receiver output"]
    #[inline(always)]
    pub fn rcv_df(self) -> &'a mut crate::W<REG> {
        self.variant(DdftOutSel::RcvDf)
    }
    #[doc = "GPIO output of DP"]
    #[inline(always)]
    pub fn gpio_dp_out(self) -> &'a mut crate::W<REG> {
        self.variant(DdftOutSel::GpioDpOut)
    }
    #[doc = "GPIO output of DM"]
    #[inline(always)]
    pub fn gpio_dm_out(self) -> &'a mut crate::W<REG> {
        self.variant(DdftOutSel::GpioDmOut)
    }
}
#[doc = "DDFT input select signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DdftInSel {
    #[doc = "0: Nothing connected, output 0"]
    Off = 0,
    #[doc = "1: GPIO input of DP"]
    GpioDpIn = 1,
    #[doc = "2: GPIO input of DM"]
    GpioDmIn = 2,
}
impl From<DdftInSel> for u8 {
    #[inline(always)]
    fn from(variant: DdftInSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DdftInSel {
    type Ux = u8;
}
impl crate::IsEnum for DdftInSel {}
#[doc = "Field `DDFT_IN_SEL` reader - DDFT input select signal"]
pub type DdftInSelR = crate::FieldReader<DdftInSel>;
impl DdftInSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DdftInSel> {
        match self.bits {
            0 => Some(DdftInSel::Off),
            1 => Some(DdftInSel::GpioDpIn),
            2 => Some(DdftInSel::GpioDmIn),
            _ => None,
        }
    }
    #[doc = "Nothing connected, output 0"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DdftInSel::Off
    }
    #[doc = "GPIO input of DP"]
    #[inline(always)]
    pub fn is_gpio_dp_in(&self) -> bool {
        *self == DdftInSel::GpioDpIn
    }
    #[doc = "GPIO input of DM"]
    #[inline(always)]
    pub fn is_gpio_dm_in(&self) -> bool {
        *self == DdftInSel::GpioDmIn
    }
}
#[doc = "Field `DDFT_IN_SEL` writer - DDFT input select signal"]
pub type DdftInSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, DdftInSel>;
impl<'a, REG> DdftInSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Nothing connected, output 0"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(DdftInSel::Off)
    }
    #[doc = "GPIO input of DP"]
    #[inline(always)]
    pub fn gpio_dp_in(self) -> &'a mut crate::W<REG> {
        self.variant(DdftInSel::GpioDpIn)
    }
    #[doc = "GPIO input of DM"]
    #[inline(always)]
    pub fn gpio_dm_in(self) -> &'a mut crate::W<REG> {
        self.variant(DdftInSel::GpioDmIn)
    }
}
impl R {
    #[doc = "Bits 0:2 - DDFT output select signal"]
    #[inline(always)]
    pub fn ddft_out_sel(&self) -> DdftOutSelR {
        DdftOutSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - DDFT input select signal"]
    #[inline(always)]
    pub fn ddft_in_sel(&self) -> DdftInSelR {
        DdftInSelR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DDFT output select signal"]
    #[inline(always)]
    #[must_use]
    pub fn ddft_out_sel(&mut self) -> DdftOutSelW<DftCtlSpec> {
        DdftOutSelW::new(self, 0)
    }
    #[doc = "Bits 3:4 - DDFT input select signal"]
    #[inline(always)]
    #[must_use]
    pub fn ddft_in_sel(&mut self) -> DdftInSelW<DftCtlSpec> {
        DdftInSelW::new(self, 3)
    }
}
#[doc = "DFT control\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftCtlSpec;
impl crate::RegisterSpec for DftCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dft_ctl::R`](R) reader structure"]
impl crate::Readable for DftCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dft_ctl::W`](W) writer structure"]
impl crate::Writable for DftCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFT_CTL to value 0"]
impl crate::Resettable for DftCtlSpec {
    const RESET_VALUE: u32 = 0;
}
