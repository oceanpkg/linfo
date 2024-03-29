use super::Map;

macro_rules! spdx_license {
    ($($l:ident = $id:literal, $name:literal, $libre:expr, $osi:expr;)+) => {
        /// A commonly found license listed [here](https://spdx.org/licenses).
        ///
        /// This list is based on version 3.7 (2019-10-22). Please submit a pull
        /// request or issue if you see that this list is out-of-date.
        ///
        /// **SemVer Compatibility:** this license is intended to have the
        /// semantics of `#[non_exhaustive]`. This library reserves the right to
        /// add, reorganize, or otherwise adjust variants. These changes are
        /// allowed between otherwise API-compatible versions.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        // TODO: Add `#[non_exhaustive]` when stable
        pub enum SpdxLicense {
            $(
                #[doc = $name]
                #[doc = "— `"]
                #[doc = $id]
                #[doc = "`."]
                $l,
            )+
        }

        impl SpdxLicense {
            // Defined here to make use of macro repetition; the public API is
            // `COUNT` which is declared with the other public items.
            pub(crate) const _COUNT: usize = count!($($l)+);

            // TODO: URLs for where each license corpus can be found
            pub(crate) const ID:    Map<&'static str> = [$($id,)+];
            pub(crate) const NAME:  Map<&'static str> = [$($name,)+];
            pub(crate) const LIBRE: Map<bool> = [$($libre,)+];
            pub(crate) const OSI:   Map<bool> = [$($osi,)+];

            // Creates static a hash map if `phf` is enabled, else resorts to a
            // good ol' `match` statement :D
            pub(crate) fn _from_id(id: &str) -> Option<Self> {
                #[cfg(feature = "phf")]
                {
                    type Map = phf::Map<&'static str, SpdxLicense>;

                    static ID_TO_LICENSE: Map = phf::phf_map! {
                        $($id => SpdxLicense::$l,)+
                    };

                    ID_TO_LICENSE.get(id).map(|&l| l)
                }

                #[cfg(not(feature = "phf"))]
                match id {
                    $($id => Some(Self::$l),)+
                    _ => None,
                }
            }
        }
    };
}

spdx_license! {
    Bsd0 = "BSD 0", "BSD Zero Clause License", false, true;
    Aal = "AAL", "Attribution Assurance License", false, true;
    Abstyles = "Abstyles", "Abstyles License", false, false;
    Adobe2006 = "Adobe-2006", "Adobe Systems Incorporated Source Code License Agreement", false, false;
    AdobeGlyph = "Adobe-Glyph", "Adobe Glyph List License", false, false;
    Adsl = "ADSL", "Amazon Digital Services License", false, false;
    Afl1_1 = "AFL-1.1", "Academic Free License v1.1", true, true;
    Afl1_2 = "AFL-1.2", "Academic Free License v1.2", true, true;
    Afl2 = "AFL-2.0", "Academic Free License v2.0", true, true;
    Afl2_1 = "AFL-2.1", "Academic Free License v2.1", true, true;
    Afl3 = "AFL-3.0", "Academic Free License v3.0", true, true;
    Afmparse = "Afmparse", "Afmparse License", false, false;
    // CORRECTNESS: Hello future hacker, `Agpl1Only` is considered the first
    // Affero GPL as `MIN` in `is_agpl`.
    Agpl1Only = "AGPL-1.0-only", "Affero General Public License v1.0 only", false, false;
    Agpl1OrLater = "AGPL-1.0-or-later", "Affero General Public License v1.0 or later", false, false;
    Agpl3Only = "AGPL-3.0-only", "GNU Affero General Public License v3.0 only", true, true;
    // CORRECTNESS: Hello future hacker, similarly to `Agpl1Only`,
    // `Agpl3OrLater` is considered the *last* Affero GPL as `MAX` in `is_agpl`.
    Agpl3OrLater = "AGPL-3.0-or-later", "GNU Affero General Public License v3.0 or later", true, true;
    Aladdin = "Aladdin", "Aladdin Free Public License", false, false;
    Amdplpa = "AMDPLPA", "AMD's plpa_map.c License", false, false;
    Aml = "AML", "Apple MIT License", false, false;
    Ampas = "AMPAS", "Academy of Motion Picture Arts and Sciences BSD", false, false;
    AntlrPd = "ANTLR-PD", "ANTLR Software Rights Notice", false, false;
    Apache1 = "Apache-1.0", "Apache License 1.0", true, false;
    Apache1_1 = "Apache-1.1", "Apache License 1.1", true, true;
    Apache2 = "Apache-2.0", "Apache License 2.0", true, true;
    Apafml = "APAFML", "Adobe Postscript AFM License", false, false;
    Apl1 = "APL-1.0", "Adaptive Public License 1.0", false, true;
    Apsl1 = "APSL-1.0", "Apple Public Source License 1.0", false, true;
    Apsl1_1 = "APSL-1.1", "Apple Public Source License 1.1", false, true;
    Apsl1_2 = "APSL-1.2", "Apple Public Source License 1.2", false, true;
    Apsl2 = "APSL-2.0", "Apple Public Source License 2.0", true, true;
    Artistic1 = "Artistic-1.0", "Artistic License 1.0", false, true;
    Artistic1Cl8 = "Artistic-1.0-cl8", "Artistic License 1.0 w/clause 8", false, true;
    Artistic1Perl = "Artistic-1.0-Perl", "Artistic License 1.0 (Perl)", false, true;
    Artistic2 = "Artistic-2.0", "Artistic License 2.0", true, true;
    Bahyph = "Bahyph", "Bahyph License", false, false;
    Barr = "Barr", "Barr License", false, false;
    Beerware = "Beerware", "Beerware License", false, false;
    BitTorrent1 = "BitTorrent-1.0", "BitTorrent Open Source License v1.0", false, false;
    BitTorrent1_1 = "BitTorrent-1.1", "BitTorrent Open Source License v1.1", true, false;
    Blessing = "blessing", "SQLite Blessing", false, false;
    BlueOak1 = "BlueOak-1.0.0", "Blue Oak Model License 1.0.0", false, false;
    Borceux = "Borceux", "Borceux license", false, false;
    Bsd1Clause = "BSD-1-Clause", "BSD 1-Clause License", false, false;
    Bsd2Clause = "BSD-2-Clause", "BSD 2-Clause \"Simplified\" License", false, true;
    Bsd2ClauseFreeBsd = "BSD-2-Clause-FreeBSD", "BSD 2-Clause FreeBSD License", true, false;
    Bsd2ClauseNetBsd = "BSD-2-Clause-NetBSD", "BSD 2-Clause NetBSD License", false, false;
    Bsd2ClausePatent = "BSD-2-Clause-Patent", "BSD-2-Clause Plus Patent License", false, true;
    Bsd3Clause = "BSD-3-Clause", "BSD 3-Clause \"New\" or \"Revised\" License", true, true;
    Bsd3ClauseAttribution = "BSD-3-Clause-Attribution", "BSD with attribution", false, false;
    Bsd3ClauseClear = "BSD-3-Clause-Clear", "BSD 3-Clause Clear License", true, false;
    Bsd3ClauseLbnl = "BSD-3-Clause-LBNL", "Lawrence Berkeley National Labs BSD variant license", false, true;
    Bsd3ClauseNoNuclearLicense = "BSD-3-Clause-No-Nuclear-License", "BSD 3-Clause No Nuclear License", false, false;
    Bsd3ClauseNoNuclearLicense2014 = "BSD-3-Clause-No-Nuclear-License-2014", "BSD 3-Clause No Nuclear License 2014", false, false;
    Bsd3ClauseNoNuclearWarranty = "BSD-3-Clause-No-Nuclear-Warranty", "BSD 3-Clause No Nuclear Warranty", false, false;
    Bsd3ClauseOpenMpi = "BSD-3-Clause-Open-MPI", "BSD 3-Clause Open MPI variant", false, false;
    Bsd4Clause = "BSD-4-Clause", "BSD 4-Clause \"Original\" or \"Old\" License", true, false;
    Bsd4ClauseUc = "BSD-4-Clause-UC", "BSD-4-Clause (University of California-Specific)", false, false;
    BsdProtection = "BSD-Protection", "BSD Protection License", false, false;
    BsdSourceCode = "BSD-Source-Code", "BSD Source Code Attribution", false, false;
    Bsl1 = "BSL-1.0", "Boost Software License 1.0", true, true;
    Bzip21_0_5 = "bzip2-1.0.5", "bzip2 and libbzip2 License v1.0.5", false, false;
    Bzip21_0_6 = "bzip2-1.0.6", "bzip2 and libbzip2 License v1.0.6", false, false;
    Caldera = "Caldera", "Caldera License", false, false;
    Catosl1_1 = "CATOSL-1.1", "Computer Associates Trusted Open Source License 1.1", false, true;
    // CORRECTNESS: Hello future hacker, `CcBy1` is considered the first
    // Creative Commons license as `MIN` in `is_creative_commons`.
    CcBy1 = "CC-BY-1.0", "Creative Commons Attribution 1.0 Generic", false, false;
    CcBy2 = "CC-BY-2.0", "Creative Commons Attribution 2.0 Generic", false, false;
    CcBy2_5 = "CC-BY-2.5", "Creative Commons Attribution 2.5 Generic", false, false;
    CcBy3 = "CC-BY-3.0", "Creative Commons Attribution 3.0 Unported", false, false;
    CcBy4 = "CC-BY-4.0", "Creative Commons Attribution 4.0 International", true, false;
    CcByNc1 = "CC-BY-NC-1.0", "Creative Commons Attribution Non Commercial 1.0 Generic", false, false;
    CcByNc2 = "CC-BY-NC-2.0", "Creative Commons Attribution Non Commercial 2.0 Generic", false, false;
    CcByNc2_5 = "CC-BY-NC-2.5", "Creative Commons Attribution Non Commercial 2.5 Generic", false, false;
    CcByNc3 = "CC-BY-NC-3.0", "Creative Commons Attribution Non Commercial 3.0 Unported", false, false;
    CcByNc4 = "CC-BY-NC-4.0", "Creative Commons Attribution Non Commercial 4.0 International", false, false;
    CcByNcNd1 = "CC-BY-NC-ND-1.0", "Creative Commons Attribution Non Commercial No Derivatives 1.0 Generic", false, false;
    CcByNcNd2 = "CC-BY-NC-ND-2.0", "Creative Commons Attribution Non Commercial No Derivatives 2.0 Generic", false, false;
    CcByNcNd2_5 = "CC-BY-NC-ND-2.5", "Creative Commons Attribution Non Commercial No Derivatives 2.5 Generic", false, false;
    CcByNcNd3 = "CC-BY-NC-ND-3.0", "Creative Commons Attribution Non Commercial No Derivatives 3.0 Unported", false, false;
    CcByNcNd4 = "CC-BY-NC-ND-4.0", "Creative Commons Attribution Non Commercial No Derivatives 4.0 International", false, false;
    CcByNcSa1 = "CC-BY-NC-SA-1.0", "Creative Commons Attribution Non Commercial Share Alike 1.0 Generic", false, false;
    CcByNcSa2 = "CC-BY-NC-SA-2.0", "Creative Commons Attribution Non Commercial Share Alike 2.0 Generic", false, false;
    CcByNcSa2_5 = "CC-BY-NC-SA-2.5", "Creative Commons Attribution Non Commercial Share Alike 2.5 Generic", false, false;
    CcByNcSa3 = "CC-BY-NC-SA-3.0", "Creative Commons Attribution Non Commercial Share Alike 3.0 Unported", false, false;
    CcByNcSa4 = "CC-BY-NC-SA-4.0", "Creative Commons Attribution Non Commercial Share Alike 4.0 International", false, false;
    CcByNd1 = "CC-BY-ND-1.0", "Creative Commons Attribution No Derivatives 1.0 Generic", false, false;
    CcByNd2 = "CC-BY-ND-2.0", "Creative Commons Attribution No Derivatives 2.0 Generic", false, false;
    CcByNd2_5 = "CC-BY-ND-2.5", "Creative Commons Attribution No Derivatives 2.5 Generic", false, false;
    CcByNd3 = "CC-BY-ND-3.0", "Creative Commons Attribution No Derivatives 3.0 Unported", false, false;
    CcByNd4 = "CC-BY-ND-4.0", "Creative Commons Attribution No Derivatives 4.0 International", false, false;
    CcBySa1 = "CC-BY-SA-1.0", "Creative Commons Attribution Share Alike 1.0 Generic", false, false;
    CcBySa2 = "CC-BY-SA-2.0", "Creative Commons Attribution Share Alike 2.0 Generic", false, false;
    CcBySa2_5 = "CC-BY-SA-2.5", "Creative Commons Attribution Share Alike 2.5 Generic", false, false;
    CcBySa3 = "CC-BY-SA-3.0", "Creative Commons Attribution Share Alike 3.0 Unported", false, false;
    CcBySa4 = "CC-BY-SA-4.0", "Creative Commons Attribution Share Alike 4.0 International", true, false;
    CcPddc = "CC-PDDC", "Creative Commons Public Domain Dedication and Certification", false, false;
    // CORRECTNESS: Hello future hacker, similarly to `CcBy1`, `CC01` is
    // considered the *last* Creative Commons license as `MAX` in
    // `is_creative_commons`.
    CC01 = "CC0-1.0", "Creative Commons Zero v1.0 Universal", true, false;
    Cddl1 = "CDDL-1.0", "Common Development and Distribution License 1.0", true, true;
    Cddl1_1 = "CDDL-1.1", "Common Development and Distribution License 1.1", false, false;
    CdlaPermissive1 = "CDLA-Permissive-1.0", "Community Data License Agreement Permissive 1.0", false, false;
    CdlaSharing1 = "CDLA-Sharing-1.0", "Community Data License Agreement Sharing 1.0", false, false;
    Cecill1 = "CECILL-1.0", "CeCILL Free Software License Agreement v1.0", false, false;
    Cecill1_1 = "CECILL-1.1", "CeCILL Free Software License Agreement v1.1", false, false;
    Cecill2 = "CECILL-2.0", "CeCILL Free Software License Agreement v2.0", true, false;
    Cecill2_1 = "CECILL-2.1", "CeCILL Free Software License Agreement v2.1", false, true;
    CecillB = "CECILL-B", "CeCILL-B Free Software License Agreement", true, false;
    CecillC = "CECILL-C", "CeCILL-C Free Software License Agreement", true, false;
    CernOhl1_1 = "CERN-OHL-1.1", "CERN Open Hardware Licence v1.1", false, false;
    CernOhl1_2 = "CERN-OHL-1.2", "CERN Open Hardware Licence v1.2", false, false;
    ClArtistic = "ClArtistic", "Clarified Artistic License", true, false;
    CnriJython = "CNRI-Jython", "CNRI Jython License", false, false;
    CnriPython = "CNRI-Python", "CNRI Python License", false, true;
    CnriPythonGplCompatible = "CNRI-Python-GPL-Compatible", "CNRI Python Open Source GPL Compatible License Agreement", false, false;
    Condor1_1 = "Condor-1.1", "Condor Public License v1.1", true, false;
    CopyleftNext0_3 = "copyleft-next-0.3.0", "copyleft-next 0.3.0", false, false;
    CopyleftNext0_3_1 = "copyleft-next-0.3.1", "copyleft-next 0.3.1", false, false;
    Cpal1 = "CPAL-1.0", "Common Public Attribution License 1.0", true, true;
    Cpl1 = "CPL-1.0", "Common Public License 1.0", true, true;
    Cpol1_02 = "CPOL-1.02", "Code Project Open License 1.02", false, false;
    Crossword = "Crossword", "Crossword License", false, false;
    CrystalStacker = "CrystalStacker", "CrystalStacker License", false, false;
    CuaOpl1 = "CUA-OPL-1.0", "CUA Office Public License v1.0", false, true;
    Cube = "Cube", "Cube License", false, false;
    Curl = "curl", "curl License", false, false;
    DFsl1 = "D-FSL-1.0", "Deutsche Freie Software Lizenz", false, false;
    Diffmark = "diffmark", "diffmark license", false, false;
    Doc = "DOC", "DOC License", false, false;
    Dotseqn = "Dotseqn", "Dotseqn License", false, false;
    Dsdp = "DSDP", "DSDP License", false, false;
    Dvipdfm = "dvipdfm", "dvipdfm License", false, false;
    Ecl1 = "ECL-1.0", "Educational Community License v1.0", false, true;
    Ecl2 = "ECL-2.0", "Educational Community License v2.0", true, true;
    Efl1 = "EFL-1.0", "Eiffel Forum License v1.0", false, true;
    Efl2 = "EFL-2.0", "Eiffel Forum License v2.0", true, true;
    EGenix = "eGenix", "eGenix.com Public License 1.1.0", false, false;
    Entessa = "Entessa", "Entessa Public License v1.0", false, true;
    Epl1 = "EPL-1.0", "Eclipse Public License 1.0", true, true;
    Epl2 = "EPL-2.0", "Eclipse Public License 2.0", true, true;
    ErlPl1_1 = "ErlPL-1.1", "Erlang Public License v1.1", false, false;
    Etalab2 = "etalab-2.0", "Etalab Open License 2.0", false, false;
    EUDatagrid = "EUDatagrid", "EU DataGrid Software License", true, true;
    Eupl1 = "EUPL-1.0", "European Union Public License 1.0", false, false;
    Eupl1_1 = "EUPL-1.1", "European Union Public License 1.1", true, true;
    Eupl1_2 = "EUPL-1.2", "European Union Public License 1.2", true, true;
    Eurosym = "Eurosym", "Eurosym License", false, false;
    Fair = "Fair", "Fair License", false, true;
    Frameworx1 = "Frameworx-1.0", "Frameworx Open License 1.0", false, true;
    FreeImage = "FreeImage", "FreeImage Public License v1.0", false, false;
    Fsfap = "FSFAP", "FSF All Permissive License", true, false;
    Fsful = "FSFUL", "FSF Unlimited License", false, false;
    Fsfullr = "FSFULLR", "FSF Unlimited License (with License Retention)", false, false;
    Ftl = "FTL", "Freetype Project License", true, false;
    Gfdl1_1Only = "GFDL-1.1-only", "GNU Free Documentation License v1.1 only", true, false;
    Gfdl1_1OrLater = "GFDL-1.1-or-later", "GNU Free Documentation License v1.1 or later", true, false;
    Gfdl1_2Only = "GFDL-1.2-only", "GNU Free Documentation License v1.2 only", true, false;
    Gfdl1_2OrLater = "GFDL-1.2-or-later", "GNU Free Documentation License v1.2 or later", true, false;
    Gfdl1_3Only = "GFDL-1.3-only", "GNU Free Documentation License v1.3 only", true, false;
    Gfdl1_3OrLater = "GFDL-1.3-or-later", "GNU Free Documentation License v1.3 or later", true, false;
    Giftware = "Giftware", "Giftware License", false, false;
    GL2Ps = "GL2PS", "GL2PS License", false, false;
    Glide = "Glide", "3dfx Glide License", false, false;
    Glulxe = "Glulxe", "Glulxe License", false, false;
    Gnuplot = "gnuplot", "gnuplot License", true, false;
    // CORRECTNESS: Hello future hacker, `Gpl1Only` is considered the first
    // GNU GPL license as `MIN` in `is_gpl`.
    Gpl1Only = "GPL-1.0-only", "GNU General Public License v1.0 only", false, false;
    Gpl1OrLater = "GPL-1.0-or-later", "GNU General Public License v1.0 or later", false, false;
    Gpl2Only = "GPL-2.0-only", "GNU General Public License v2.0 only", true, true;
    Gpl2OrLater = "GPL-2.0-or-later", "GNU General Public License v2.0 or later", true, true;
    Gpl3Only = "GPL-3.0-only", "GNU General Public License v3.0 only", true, true;
    // CORRECTNESS: Hello future hacker, similarly to `Gpl1Only`, `Gpl3OrLater`
    // is considered the *last* GNU GPL as `MAX` in `is_gpl`.
    Gpl3OrLater = "GPL-3.0-or-later", "GNU General Public License v3.0 or later", true, true;
    GSoap1_3b = "gSOAP-1.3b", "gSOAP Public License v1.3b", false, false;
    HaskellReport = "HaskellReport", "Haskell Language Report License", false, false;
    Hpnd = "HPND", "Historical Permission Notice and Disclaimer", true, true;
    HpndSellVariant = "HPND-sell-variant", "Historical Permission Notice and Disclaimer - sell variant", false, false;
    IbmPibs = "IBM-pibs", "IBM PowerPC Initialization and Boot Software", false, false;
    Icu = "ICU", "ICU License", false, false;
    Ijg = "IJG", "Independent JPEG Group License", true, false;
    ImageMagick = "ImageMagick", "ImageMagick License", false, false;
    IMatix = "iMatix", "iMatix Standard Function Library Agreement", true, false;
    Imlib2 = "Imlib2", "Imlib2 License", true, false;
    InfoZip = "Info-ZIP", "Info-ZIP License", false, false;
    Intel = "Intel", "Intel Open Source License", true, true;
    IntelAcpi = "Intel-ACPI", "Intel ACPI Software License Agreement", false, false;
    Interbase1 = "Interbase-1.0", "Interbase Public License v1.0", false, false;
    Ipa = "IPA", "IPA Font License", true, true;
    Ipl1 = "IPL-1.0", "IBM Public License v1.0", true, true;
    Isc = "ISC", "ISC License", true, true;
    JasPer2 = "JasPer-2.0", "JasPer License", false, false;
    Jpnic = "JPNIC", "Japan Network Information Center License", false, false;
    Json = "JSON", "JSON License", false, false;
    Lal1_2 = "LAL-1.2", "Licence Art Libre 1.2", false, false;
    Lal1_3 = "LAL-1.3", "Licence Art Libre 1.3", false, false;
    Latex2e = "Latex2e", "Latex2e License", false, false;
    Leptonica = "Leptonica", "Leptonica License", false, false;
    Lgpl2Only = "LGPL-2.0-only", "GNU Library General Public License v2 only", false, true;
    Lgpl2OrLater = "LGPL-2.0-or-later", "GNU Library General Public License v2 or later", false, true;
    Lgpl2_1Only = "LGPL-2.1-only", "GNU Lesser General Public License v2.1 only", true, true;
    Lgpl2_1OrLater = "LGPL-2.1-or-later", "GNU Lesser General Public License v2.1 or later", true, true;
    Lgpl3Only = "LGPL-3.0-only", "GNU Lesser General Public License v3.0 only", true, true;
    Lgpl3OrLater = "LGPL-3.0-or-later", "GNU Lesser General Public License v3.0 or later", true, true;
    Lgpllr = "LGPLLR", "Lesser General Public License For Linguistic Resources", false, false;
    Libpng = "Libpng", "libpng License", false, false;
    Libpng2 = "libpng-2.0", "PNG Reference Library version 2", false, false;
    Libtiff = "libtiff", "libtiff License", false, false;
    LiLiQP1_1 = "LiLiQ-P-1.1", "Licence Libre du Québec – Permissive version 1.1", false, true;
    LiLiQR1_1 = "LiLiQ-R-1.1", "Licence Libre du Québec – Réciprocité version 1.1", false, true;
    LiLiQRplus1_1 = "LiLiQ-Rplus-1.1", "Licence Libre du Québec – Réciprocité forte version 1.1", false, true;
    LinuxOpenIb = "Linux-OpenIB", "Linux Kernel Variant of OpenIB.org license", false, false;
    Lpl1 = "LPL-1.0", "Lucent Public License Version 1.0", false, true;
    Lpl1_02 = "LPL-1.02", "Lucent Public License v1.02", true, true;
    Lppl1 = "LPPL-1.0", "LaTeX Project Public License v1.0", false, false;
    Lppl1_1 = "LPPL-1.1", "LaTeX Project Public License v1.1", false, false;
    Lppl1_2 = "LPPL-1.2", "LaTeX Project Public License v1.2", true, false;
    Lppl1_3a = "LPPL-1.3a", "LaTeX Project Public License v1.3a", true, false;
    Lppl1_3c = "LPPL-1.3c", "LaTeX Project Public License v1.3c", false, true;
    MakeIndex = "MakeIndex", "MakeIndex License", false, false;
    MirOs = "MirOS", "The MirOS Licence", false, true;
    Mit = "MIT", "MIT License", true, true;
    Mit0 = "MIT-0", "MIT No Attribution", false, true;
    MitAdvertising = "MIT-advertising", "Enlightenment License (e16)", false, false;
    MitCmu = "MIT-CMU", "CMU License", false, false;
    MitEnna = "MIT-enna", "enna License", false, false;
    MitFeh = "MIT-feh", "feh License", false, false;
    Mitnfa = "MITNFA", "MIT +no-false-attribs license", false, false;
    Motosoto = "Motosoto", "Motosoto License", false, true;
    Mpich2 = "mpich2", "mpich2 License", false, false;
    Mpl1 = "MPL-1.0", "Mozilla Public License 1.0", false, true;
    Mpl1_1 = "MPL-1.1", "Mozilla Public License 1.1", true, true;
    Mpl2 = "MPL-2.0", "Mozilla Public License 2.0", true, true;
    Mpl2NoCopyleftException = "MPL-2.0-no-copyleft-exception", "Mozilla Public License 2.0 (no copyleft exception)", false, true;
    MsPl = "MS-PL", "Microsoft Public License", true, true;
    MsRl = "MS-RL", "Microsoft Reciprocal License", true, true;
    Mtll = "MTLL", "Matrix Template Library License", false, false;
    MulanPsl1 = "MulanPSL-1.0", "Mulan Permissive Software License, Version 1", false, false;
    Multics = "Multics", "Multics License", false, true;
    Mup = "Mup", "Mup License", false, false;
    Nasa1_3 = "NASA-1.3", "NASA Open Source Agreement 1.3", false, true;
    Naumen = "Naumen", "Naumen Public License", false, true;
    Nbpl1 = "NBPL-1.0", "Net Boolean Public License v1", false, false;
    Ncsa = "NCSA", "University of Illinois/NCSA Open Source License", true, true;
    NetSnmp = "Net-SNMP", "Net-SNMP License", false, false;
    NetCdf = "NetCDF", "NetCDF license", false, false;
    Newsletr = "Newsletr", "Newsletr License", false, false;
    Ngpl = "NGPL", "Nethack General Public License", false, true;
    Nlod1 = "NLOD-1.0", "Norwegian Licence for Open Government Data", false, false;
    Nlpl = "NLPL", "No Limit Public License", false, false;
    Nokia = "Nokia", "Nokia Open Source License", true, true;
    Nosl = "NOSL", "Netizen Open Source License", true, false;
    Noweb = "Noweb", "Noweb License", false, false;
    Npl1 = "NPL-1.0", "Netscape Public License v1.0", true, false;
    Npl1_1 = "NPL-1.1", "Netscape Public License v1.1", true, false;
    Nposl3 = "NPOSL-3.0", "Non-Profit Open Software License 3.0", false, true;
    Nrl = "NRL", "NRL License", false, false;
    Ntp = "NTP", "NTP License", false, true;
    OcctPl = "OCCT-PL", "Open CASCADE Technology Public License", false, false;
    Oclc2 = "OCLC-2.0", "OCLC Research Public License 2.0", false, true;
    ODbL1 = "ODbL-1.0", "ODC Open Database License v1.0", true, false;
    OdcBy1 = "ODC-By-1.0", "Open Data Commons Attribution License v1.0", false, false;
    Ofl1 = "OFL-1.0", "SIL Open Font License 1.0", true, false;
    Ofl1_1 = "OFL-1.1", "SIL Open Font License 1.1", true, true;
    OglCanada2 = "OGL-Canada-2.0", "Open Government Licence - Canada", false, false;
    OglUk1 = "OGL-UK-1.0", "Open Government Licence v1.0", false, false;
    OglUk2 = "OGL-UK-2.0", "Open Government Licence v2.0", false, false;
    OglUk3 = "OGL-UK-3.0", "Open Government Licence v3.0", false, false;
    Ogtsl = "OGTSL", "Open Group Test Suite License", false, true;
    Oldap1_1 = "OLDAP-1.1", "Open LDAP Public License v1.1", false, false;
    Oldap1_2 = "OLDAP-1.2", "Open LDAP Public License v1.2", false, false;
    Oldap1_3 = "OLDAP-1.3", "Open LDAP Public License v1.3", false, false;
    Oldap1_4 = "OLDAP-1.4", "Open LDAP Public License v1.4", false, false;
    Oldap2 = "OLDAP-2.0", "Open LDAP Public License v2.0 (or possibly 2.0A and 2.0B)", false, false;
    Oldap2_0_1 = "OLDAP-2.0.1", "Open LDAP Public License v2.0.1", false, false;
    Oldap2_1 = "OLDAP-2.1", "Open LDAP Public License v2.1", false, false;
    Oldap2_2 = "OLDAP-2.2", "Open LDAP Public License v2.2", false, false;
    Oldap2_2_1 = "OLDAP-2.2.1", "Open LDAP Public License v2.2.1", false, false;
    Oldap2_2_2 = "OLDAP-2.2.2", "Open LDAP Public License 2.2.2", false, false;
    Oldap2_3 = "OLDAP-2.3", "Open LDAP Public License v2.3", true, false;
    Oldap2_4 = "OLDAP-2.4", "Open LDAP Public License v2.4", false, false;
    Oldap2_5 = "OLDAP-2.5", "Open LDAP Public License v2.5", false, false;
    Oldap2_6 = "OLDAP-2.6", "Open LDAP Public License v2.6", false, false;
    Oldap2_7 = "OLDAP-2.7", "Open LDAP Public License v2.7", true, false;
    Oldap2_8 = "OLDAP-2.8", "Open LDAP Public License v2.8", false, false;
    Oml = "OML", "Open Market License", false, false;
    OpenSsl = "OpenSSL", "OpenSSL License", true, false;
    Opl1 = "OPL-1.0", "Open Public License v1.0", false, false;
    OsetPl2_1 = "OSET-PL-2.1", "OSET Public License version 2.1", false, true;
    Osl1 = "OSL-1.0", "Open Software License 1.0", true, true;
    Osl1_1 = "OSL-1.1", "Open Software License 1.1", true, false;
    Osl2 = "OSL-2.0", "Open Software License 2.0", true, true;
    Osl2_1 = "OSL-2.1", "Open Software License 2.1", true, true;
    Osl3 = "OSL-3.0", "Open Software License 3.0", true, true;
    Parity6 = "Parity-6.0.0", "The Parity Public License 6.0.0", false, false;
    Pddl1 = "PDDL-1.0", "ODC Public Domain Dedication & License 1.0", false, false;
    Php3 = "-PHP 3.0", "PHP License v3.0", false, true;
    Php3_01 = "-PHP 3.01", "PHP License v3.01", true, false;
    Plexus = "Plexus", "Plexus Classworlds License", false, false;
    PostgreSql = "PostgreSQL", "PostgreSQL License", false, true;
    Psfrag = "psfrag", "psfrag License", false, false;
    Psutils = "psutils", "psutils License", false, false;
    Python2 = "Python-2.0", "Python License 2.0", true, true;
    Qhull = "Qhull", "Qhull License", false, false;
    Qpl1 = "QPL-1.0", "Q Public License 1.0", true, true;
    Rdisc = "Rdisc", "Rdisc License", false, false;
    RHeCos1_1 = "RHeCos-1.1", "Red Hat eCos Public License v1.1", false, false;
    Rpl1_1 = "RPL-1.1", "Reciprocal Public License 1.1", false, true;
    Rpl1_5 = "RPL-1.5", "Reciprocal Public License 1.5", false, true;
    Rpsl1 = "RPSL-1.0", "RealNetworks Public Source License v1.0", true, true;
    RsaMd = "RSA-MD", "RSA Message-Digest License", false, false;
    Rscpl = "RSCPL", "Ricoh Source Code Public License", false, true;
    Ruby = "Ruby", "Ruby License", true, false;
    SaxPd = "SAX-PD", "Sax Public Domain Notice", false, false;
    Saxpath = "Saxpath", "Saxpath License", false, false;
    Scea = "SCEA", "SCEA Shared Source License", false, false;
    Sendmail = "Sendmail", "Sendmail License", false, false;
    Sendmail8_23 = "Sendmail-8.23", "Sendmail License 8.23", false, false;
    SgiB1 = "SGI-B-1.0", "SGI Free Software License B v1.0", false, false;
    SgiB1_1 = "SGI-B-1.1", "SGI Free Software License B v1.1", false, false;
    SgiB2 = "SGI-B-2.0", "SGI Free Software License B v2.0", true, false;
    Shl0_5 = "SHL-0.5", "Solderpad Hardware License v0.5", false, false;
    Shl0_51 = "SHL-0.51", "Solderpad Hardware License, Version 0.51", false, false;
    SimPl2 = "SimPL-2.0", "Simple Public License 2.0", false, true;
    Sissl = "SISSL", "Sun Industry Standards Source License v1.1", true, true;
    Sissl1_2 = "SISSL-1.2", "Sun Industry Standards Source License v1.2", false, false;
    Sleepycat = "Sleepycat", "Sleepycat License", true, true;
    Smlnj = "SMLNJ", "Standard ML of New Jersey License", true, false;
    Smppl = "SMPPL", "Secure Messaging Protocol Public License", false, false;
    Snia = "SNIA", "SNIA Public License 1.1", false, false;
    Spencer86 = "Spencer-86", "Spencer License 86", false, false;
    Spencer94 = "Spencer-94", "Spencer License 94", false, false;
    Spencer99 = "Spencer-99", "Spencer License 99", false, false;
    Spl1 = "SPL-1.0", "Sun Public License v1.0", true, true;
    SshOpenSsh = "SSH-OpenSSH", "SSH OpenSSH license", false, false;
    SshShort = "SSH-short", "SSH short notice", false, false;
    Sspl1 = "SSPL-1.0", "Server Side Public License, v 1", false, false;
    SugarCrm1_1_3 = "SugarCRM-1.1.3", "SugarCRM Public License v1.1.3", false, false;
    Swl = "SWL", "Scheme Widget Library (SWL) Software License Agreement", false, false;
    TaprOhl1 = "TAPR-OHL-1.0", "TAPR Open Hardware License v1.0", false, false;
    Tcl = "TCL", "TCL/TK License", false, false;
    TcpWrappers = "TCP-wrappers", "TCP Wrappers License", false, false;
    TMate = "TMate", "TMate Open Source License", false, false;
    Torque1_1 = "TORQUE-1.1", "TORQUE v2.5+ Software License v1.1", false, false;
    Tosl = "TOSL", "Trusster Open Source License", false, false;
    TuBerlin1 = "TU-Berlin-1.0", "Technische Universitaet Berlin License 1.0", false, false;
    TuBerlin2 = "TU-Berlin-2.0", "Technische Universitaet Berlin License 2.0", false, false;
    Ucl1 = "UCL-1.0", "Upstream Compatibility License v1.0", false, true;
    UnicodeDfs2015 = "Unicode-DFS-2015", "Unicode License Agreement - Data Files and Software (2015)", false, false;
    UnicodeDfs2016 = "Unicode-DFS-2016", "Unicode License Agreement - Data Files and Software (2016)", false, false;
    UnicodeTou = "Unicode-TOU", "Unicode Terms of Use", false, false;
    Unlicense = "Unlicense", "The Unlicense", true, false;
    Upl1 = "UPL-1.0", "Universal Permissive License v1.0", true, true;
    Vim = "Vim", "Vim License", true, false;
    Vostrom = "VOSTROM", "VOSTROM Public License for Open Source", false, false;
    Vsl1 = "VSL-1.0", "Vovida Software License v1.0", false, true;
    W3C = "W3C", "W3C Software Notice and License (2002-12-31)", true, true;
    W3C19980720 = "W3C-19980720", "W3C Software Notice and License (1998-07-20)", false, false;
    W3C20150513 = "W3C-20150513", "W3C Software Notice and Document License (2015-05-13)", false, false;
    Watcom1 = "Watcom-1.0", "Sybase Open Watcom Public License 1.0", false, true;
    Wsuipa = "Wsuipa", "Wsuipa License", false, false;
    Wtfpl = "WTFPL", "Do What The F*ck You Want To Public License", true, false;
    X11 = "X11", "X11 License", true, false;
    Xerox = "Xerox", "Xerox License", false, false;
    XFree861_1 = "XFree86-1.1", "XFree86 License 1.1", true, false;
    Xinetd = "xinetd", "xinetd License", true, false;
    Xnet = "Xnet", "X.Net License", false, true;
    Xpp = "xpp", "XPP License", false, false;
    XSkat = "XSkat", "XSkat License", false, false;
    Ypl1 = "YPL-1.0", "Yahoo! Public License v1.0", false, false;
    Ypl1_1 = "YPL-1.1", "Yahoo! Public License v1.1", true, false;
    Zed = "Zed", "Zed License", false, false;
    Zend2 = "Zend-2.0", "Zend License v2.0", true, false;
    Zimbra1_3 = "Zimbra-1.3", "Zimbra Public License v1.3", true, false;
    Zimbra1_4 = "Zimbra-1.4", "Zimbra Public License v1.4", false, false;
    Zlib = "Zlib", "zlib License", true, true;
    ZlibAcknowledgement = "zlib-acknowledgement", "zlib/libpng License with Acknowledgement", false, false;
    Zpl1_1 = "ZPL-1.1", "Zope Public License 1.1", false, false;
    Zpl2 = "ZPL-2.0", "Zope Public License 2.0", true, true;
    Zpl2_1 = "ZPL-2.1", "Zope Public License 2.1", true, false;
}
