#[doc = "Register `CLK_PATH_SELECT[%s]` reader"]
pub type R = crate::R<ClkPathSelectSpec>;
#[doc = "Register `CLK_PATH_SELECT[%s]` writer"]
pub type W = crate::W<ClkPathSelectSpec>;
#[doc = "Selects a source for clock PATH&lt;i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PathMux {
    #[doc = "0: IMO - Internal R/C Oscillator"]
    Imo = 0,
    #[doc = "1: EXTCLK - External Clock Pin"]
    Extclk = 1,
    #[doc = "2: ECO - External-Crystal Oscillator"]
    Eco = 2,
    #[doc = "3: ALTHF - Alternate High-Frequency clock input (product-specific clock)"]
    Althf = 3,
    #[doc = "4: DSI_MUX - Output of DSI mux for this path. Using a DSI source directly as root of HFCLK will result in undefined behavior."]
    DsiMux = 4,
}
impl From<PathMux> for u8 {
    #[inline(always)]
    fn from(variant: PathMux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PathMux {
    type Ux = u8;
}
impl crate::IsEnum for PathMux {}
#[doc = "Field `PATH_MUX` reader - Selects a source for clock PATH&lt;i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
pub type PathMuxR = crate::FieldReader<PathMux>;
impl PathMuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PathMux> {
        match self.bits {
            0 => Some(PathMux::Imo),
            1 => Some(PathMux::Extclk),
            2 => Some(PathMux::Eco),
            3 => Some(PathMux::Althf),
            4 => Some(PathMux::DsiMux),
            _ => None,
        }
    }
    #[doc = "IMO - Internal R/C Oscillator"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == PathMux::Imo
    }
    #[doc = "EXTCLK - External Clock Pin"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == PathMux::Extclk
    }
    #[doc = "ECO - External-Crystal Oscillator"]
    #[inline(always)]
    pub fn is_eco(&self) -> bool {
        *self == PathMux::Eco
    }
    #[doc = "ALTHF - Alternate High-Frequency clock input (product-specific clock)"]
    #[inline(always)]
    pub fn is_althf(&self) -> bool {
        *self == PathMux::Althf
    }
    #[doc = "DSI_MUX - Output of DSI mux for this path. Using a DSI source directly as root of HFCLK will result in undefined behavior."]
    #[inline(always)]
    pub fn is_dsi_mux(&self) -> bool {
        *self == PathMux::DsiMux
    }
}
#[doc = "Field `PATH_MUX` writer - Selects a source for clock PATH&lt;i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
pub type PathMuxW<'a, REG> = crate::FieldWriter<'a, REG, 3, PathMux>;
impl<'a, REG> PathMuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IMO - Internal R/C Oscillator"]
    #[inline(always)]
    pub fn imo(self) -> &'a mut crate::W<REG> {
        self.variant(PathMux::Imo)
    }
    #[doc = "EXTCLK - External Clock Pin"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(PathMux::Extclk)
    }
    #[doc = "ECO - External-Crystal Oscillator"]
    #[inline(always)]
    pub fn eco(self) -> &'a mut crate::W<REG> {
        self.variant(PathMux::Eco)
    }
    #[doc = "ALTHF - Alternate High-Frequency clock input (product-specific clock)"]
    #[inline(always)]
    pub fn althf(self) -> &'a mut crate::W<REG> {
        self.variant(PathMux::Althf)
    }
    #[doc = "DSI_MUX - Output of DSI mux for this path. Using a DSI source directly as root of HFCLK will result in undefined behavior."]
    #[inline(always)]
    pub fn dsi_mux(self) -> &'a mut crate::W<REG> {
        self.variant(PathMux::DsiMux)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects a source for clock PATH&lt;i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    pub fn path_mux(&self) -> PathMuxR {
        PathMuxR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects a source for clock PATH&lt;i>. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn path_mux(&mut self) -> PathMuxW<ClkPathSelectSpec> {
        PathMuxW::new(self, 0)
    }
}
#[doc = "Clock Path Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_path_select::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_path_select::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkPathSelectSpec;
impl crate::RegisterSpec for ClkPathSelectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_path_select::R`](R) reader structure"]
impl crate::Readable for ClkPathSelectSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_path_select::W`](W) writer structure"]
impl crate::Writable for ClkPathSelectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_PATH_SELECT[%s]
to value 0"]
impl crate::Resettable for ClkPathSelectSpec {
    const RESET_VALUE: u32 = 0;
}
