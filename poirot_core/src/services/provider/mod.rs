pub mod arxiv;
pub mod atomfeed;
pub mod utils;
pub mod arxivquery_ast;
pub use arxiv_text::{
    Categories,
    ComputerScience,
    Economics,
    ElectricalEngineeringAndSystemsScience,
    Mathematics,
    PhysicsCategory,
    QuantitativeBiology,
    QuantitativeFinance,
    Statistics,
    Astrophysics,
    CondensedMatter,
    Physics,
    HighEnergyPhysics,
    NonLinearSciences,
    Nuclear,
};
pub mod constants;

impl ToString for Categories {
    fn to_string(&self) -> String {
        arxiv_category(self)
    }
}
impl ToString for ComputerScience {
    fn to_string(&self) -> String {
        arxiv_text::computer_science(self)
    }
}
impl ToString for Economics {
    fn to_string(&self) -> String {
        arxiv_text::economics(self)
    }
}
impl ToString for ElectricalEngineeringAndSystemsScience {
    fn to_string(&self) -> String {
        arxiv_text::eess(self)
    }
}
impl ToString for Mathematics {
    fn to_string(&self) -> String {
        arxiv_text::mathematics(self)
    }
}
impl ToString for PhysicsCategory {
    fn to_string(&self) -> String {
        arxiv_text::physics_category(self)
    }
}
impl ToString for QuantitativeBiology {
    fn to_string(&self) -> String {
        arxiv_text::quantitative_biology(self)
    }
}
impl ToString for QuantitativeFinance {
    fn to_string(&self) -> String {
        arxiv_text::quantitative_finance(self)
    }
}
impl ToString for Statistics {
    fn to_string(&self) -> String {
        arxiv_text::statistics(self)
    }
}
impl ToString for Astrophysics {
    fn to_string(&self) -> String {
        arxiv_text::astrophysics(self)
    }
}
impl ToString for CondensedMatter {
    fn to_string(&self) -> String {
        arxiv_text::condensed_matter(self)
    }
}
impl ToString for  Physics {
    fn to_string(&self) -> String {
        arxiv_text::physics(self)
    }
}
impl ToString for HighEnergyPhysics {
    fn to_string(&self) -> String {
        arxiv_text::high_energy_physics(self)
    }
}
impl ToString for NonLinearSciences {
    fn to_string(&self) -> String {
        arxiv_text::non_linear_sciences(self)
    }
}
impl ToString for Nuclear {
    fn to_string(&self) -> String {
        arxiv_text::nuclear(self)
    }
}


pub fn arxiv_category(category: &arxiv_text::Categories) -> &'static str{
    match category{
        arxiv_text::Categories::ComputerScience(cs_cat) => arxiv_text::computer_science(cs_cat),
        arxiv_text::Categories::Economics(econ_cat) => arxiv_text::economics(econ_cat),
        arxiv_text::Categories::ElectricalEngineeringAndSystemsScience(eess_cat) => arxiv_text::eess(eess_cat),
        arxiv_text::Categories::Mathematics(math_cat) => arxiv_text::mathematics(math_cat),
        arxiv_text::Categories::Physics(phys_cat) => arxiv_text::physics_category(phys_cat),
        arxiv_text::Categories::QuantitativeBiology(qbio_cat) => arxiv_text::quantitative_biology(qbio_cat),
        arxiv_text::Categories::QuantitativeFinance(qfin_cat) => arxiv_text::quantitative_finance(qfin_cat),
        arxiv_text::Categories::Statistics(stat_cat) => arxiv_text::statistics(stat_cat),
    }
}
        

pub mod arxiv_text{
    pub const ARXIV_ABS_HTTP: &str = "http://arxiv.org/abs/";   
    pub const ARXIV_ABS_HTTPS: &str = "https://arxiv.org/abs/";
    pub const BASEURL: &str = "http://export.arxiv.org/api/query";   
    pub const SEARCH_QUERY: &str = "search_query=";
    pub const SORT_BY :&str = "sortBy=";
    pub const SORT_ORDER: &str = "sortOrder=";

    pub const START: &str = "start=";
    pub const MAX_RESULTS: &str = "max_results=";


    pub const TO: &str = "+TO+";

    pub const ID_LIST: &str = "id_list=";
    
    pub const TITLE: &str = "ti:";
    pub const AUTHOR: &str ="au:";
    pub const ABSTRACT: &str="abs:";
    pub const COMMENT: &str="co:";
    pub const JOURNAL_REF: &str="jr:";
    pub const REPORT_NUMBER: &str="rn:";
        
    pub const ALL: &str="all:";
    pub const AND: &str="+AND+";
    pub const OR: &str="+OR+";
    pub const ANDNOT: &str="+ANDNOT+";

    pub const RELEVANCE: &str= "relevance";
    pub const LAST_UPDATED_DATE: &str= "lastUpdatedDate";
    pub const SUBMITTED_DATE: &str= "submittedDate";
    pub const ASCENDING: &str= "ascending";
    pub const DESCENDING: &str= "descending";

    pub enum Categories{
        ComputerScience(ComputerScience),
        Economics(Economics),
        ElectricalEngineeringAndSystemsScience(ElectricalEngineeringAndSystemsScience),
        Mathematics(Mathematics),
        Physics(PhysicsCategory),
        QuantitativeBiology(QuantitativeBiology),
        QuantitativeFinance(QuantitativeFinance),
        Statistics(Statistics),
    }


    pub enum ComputerScience{
        AI ,
        CL ,
        CV ,
        CY ,
        DB ,
        DL ,
        DM ,
        DC ,
        GL ,
        GR ,
        HC ,
        IR ,
        IT ,
        LG ,
        LO ,
        MS ,
        NE ,
        NI ,
        OS ,
        PE ,
        PL ,
        RO ,
        SE ,
        SY ,
    }

    pub fn computer_science(cat: &ComputerScience) -> &'static str{
        match cat{
            ComputerScience::AI => "cs.AI",
            ComputerScience::CL => "cs.CL",
            ComputerScience::CV => "cs.CV",
            ComputerScience::CY => "cs.CY",
            ComputerScience::DB => "cs.DB",
            ComputerScience::DL => "cs.DL",
            ComputerScience::DM => "cs.DM",
            ComputerScience::DC => "cs.DC",
            ComputerScience::GL => "cs.GL",
            ComputerScience::GR => "cs.GR",
            ComputerScience::HC => "cs.HC",
            ComputerScience::IR => "cs.IR",
            ComputerScience::IT => "cs.IT",
            ComputerScience::LG => "cs.LG",
            ComputerScience::LO => "cs.LO",
            ComputerScience::MS => "cs.MS",
            ComputerScience::NE => "cs.NE",
            ComputerScience::NI => "cs.NI",
            ComputerScience::OS => "cs.OS",
            ComputerScience::PE => "cs.PE",
            ComputerScience::PL => "cs.PL",
            ComputerScience::RO => "cs.RO",
            ComputerScience::SE => "cs.SE",
            ComputerScience::SY => "cs.SY",
        }
    }

    pub enum Economics{
        EM,        
        GN,
        TH,
    }

    pub fn economics(cat: &Economics) -> &'static str{
        match cat{
            Economics::EM => "econ.EM",
            Economics::GN => "econ.GN",
            Economics::TH => "econ.TH",
        }
    }

    pub enum ElectricalEngineeringAndSystemsScience{
        AS ,
        IV ,
        SP ,
        SY ,
        }

    pub fn eess(cat: &ElectricalEngineeringAndSystemsScience) -> &'static str{
        match cat{
            ElectricalEngineeringAndSystemsScience::AS => "eess.AS",
            ElectricalEngineeringAndSystemsScience::IV => "eess.IV",
            ElectricalEngineeringAndSystemsScience::SP => "eess.SP",
            ElectricalEngineeringAndSystemsScience::SY => "eess.SY",
        }
    }

    pub enum Mathematics{
        AC ,
        AG ,
        AP ,
        AT ,
        CA ,
        CO ,
        CT ,
        CV ,
        DG ,
        DS ,
        FA ,
        GM ,
        GN ,
        GR ,
        GT ,
        HO ,
        IT ,
        KT ,
        LO ,
        MG ,
        MP ,
        NA ,
        OA ,
        OC ,
        PR ,
        QA ,
        RT ,
        SG ,
        SP ,
        ST ,
    }

    pub fn mathematics(cat: &Mathematics) -> &'static str{
        match cat{
            Mathematics::AC => "math.AC",
            Mathematics::AG => "math.AG",
            Mathematics::AP => "math.AP",
            Mathematics::AT => "math.AT",
            Mathematics::CA => "math.CA",
            Mathematics::CO => "math.CO",
            Mathematics::CT => "math.CT",
            Mathematics::CV => "math.CV",
            Mathematics::DG => "math.DG",
            Mathematics::DS => "math.DS",
            Mathematics::FA => "math.FA",
            Mathematics::GM => "math.GM",
            Mathematics::GN => "math.GN",
            Mathematics::GR => "math.GR",
            Mathematics::GT => "math.GT",
            Mathematics::HO => "math.HO",
            Mathematics::IT => "math.IT",
            Mathematics::KT => "math.KT",
            Mathematics::LO => "math.LO",
            Mathematics::MG => "math.MG",
            Mathematics::MP => "math.MP",
            Mathematics::NA => "math.NA",
            Mathematics::OA => "math.OA",
            Mathematics::OC => "math.OC",
            Mathematics::PR => "math.PR",
            Mathematics::QA => "math.QA",
            Mathematics::RT => "math.RT",
            Mathematics::SG => "math.SG",
            Mathematics::SP => "math.SP",
            Mathematics::ST => "math.ST",
        }   
    }



    pub enum PhysicsCategory{
        Astrophysics(Astrophysics),
        CondensedMatter(CondensedMatter),
        GeneralRelativityAndQuantumCosmology,
        HighEnergyPhysics(HighEnergyPhysics),
        MathematicalPhysics,
        NonLinearSciences(NonLinearSciences),
        Nuclear(Nuclear),
        Physics(Physics),
        QuantumPhysics,
    }

    pub fn physics_category(cat: &PhysicsCategory) -> &'static str{
        match cat{
            PhysicsCategory::Astrophysics(astro_cat) => astrophysics(astro_cat),
            PhysicsCategory::CondensedMatter(cm_cat) => condensed_matter(cm_cat),
            PhysicsCategory::GeneralRelativityAndQuantumCosmology => "gr-qc",
            PhysicsCategory::HighEnergyPhysics(hep_cat) => high_energy_physics(hep_cat),
            PhysicsCategory::MathematicalPhysics => "math-ph",
            PhysicsCategory::NonLinearSciences(nlin_cat) => non_linear_sciences(nlin_cat),
            PhysicsCategory::Nuclear(nucl_cat) => nuclear(nucl_cat),
            PhysicsCategory::Physics(phys_cat) => physics(phys_cat),
            PhysicsCategory::QuantumPhysics => "quant-ph",
        }
    }

    pub enum Astrophysics{
        CO,
        EP,
        GA,
        HE,
        IM,
        SR,
    }

    pub fn astrophysics(cat: &Astrophysics) -> &'static str{
        match cat{
            Astrophysics::CO => "astro-ph.CO",
            Astrophysics::EP => "astro-ph.EP",
            Astrophysics::GA => "astro-ph.GA",
            Astrophysics::HE => "astro-ph.HE",
            Astrophysics::IM => "astro-ph.IM",
            Astrophysics::SR => "astro-ph.SR",
        }
    }
    pub enum CondensedMatter{
        DisNn,
        MesHall,
        MtrlSci,
        Other,
        QuantGas,
        Soft,
        StatMech,
        StrEl,
        SuprCon,
    }

    pub fn condensed_matter(cat: &CondensedMatter) -> &'static str{
        match cat{
            CondensedMatter::DisNn => "cond-mat.dis-nn",
            CondensedMatter::MesHall => "cond-mat.mes-hall",
            CondensedMatter::MtrlSci => "cond-mat.mtrl-sci",
            CondensedMatter::Other => "cond-mat.other",
            CondensedMatter::QuantGas => "cond-mat.quant-gas",
            CondensedMatter::Soft => "cond-mat.soft",
            CondensedMatter::StatMech => "cond-mat.stat-mech",
            CondensedMatter::StrEl => "cond-mat.str-el",
            CondensedMatter::SuprCon => "cond-mat.supr-con",
        }
    }
    
    pub enum HighEnergyPhysics{
        EX,
        LAT,
        PH,
        TH,
    }

    pub fn high_energy_physics(cat: &HighEnergyPhysics) -> &'static str{
        match cat{
            HighEnergyPhysics::EX => "hep-ex",
            HighEnergyPhysics::LAT => "hep-lat",
            HighEnergyPhysics::PH => "hep-ph",
            HighEnergyPhysics::TH => "hep-th",
        }
    }

    pub enum NonLinearSciences{
        AO,
        CD,
        CG,
        PS,
        SI,
    }

    pub fn non_linear_sciences(cat: &NonLinearSciences) -> &'static str{
        match cat{
            NonLinearSciences::AO => "nlin.AO",
            NonLinearSciences::CD => "nlin.CD",
            NonLinearSciences::CG => "nlin.CG",
            NonLinearSciences::PS => "nlin.PS",
            NonLinearSciences::SI => "nlin.SI",
        }
    }

    pub enum Nuclear{
        EX,
        TH,
    }

    pub fn nuclear(cat: &Nuclear) -> &'static str{
        match cat{
            Nuclear::EX => "nucl-ex",
            Nuclear::TH => "nucl-th",
        }
    }
    pub enum Physics{
        AccPh,
        AoPh,
        AppPh,
        AtmClus,
        AtomPh,
        BioPh,
        ChemPh,
        ClassPh,
        CompPh,
        DataAn,
        EdPh,
        FluDyn,
        GenPh,
        GeoPh,
        HistTh,
        InsDet,
        MedPh,
        Optics,
        PlasmaPh,
        PopPh,
        SocPh,
        SpacePh,
    }
     
    pub fn physics(cat: &Physics) -> &'static str{
        match cat{
            Physics::AccPh => "physics.acc-ph",
            Physics::AoPh => "physics.ao-ph",
            Physics::AppPh => "physics.app-ph",
            Physics::AtmClus => "physics.atm-clus",
            Physics::AtomPh => "physics.atom-ph",
            Physics::BioPh => "physics.bio-ph",
            Physics::ChemPh => "physics.chem-ph",
            Physics::ClassPh => "physics.class-ph",
            Physics::CompPh => "physics.comp-ph",
            Physics::DataAn => "physics.data-an",
            Physics::EdPh => "physics.ed-ph",
            Physics::FluDyn => "physics.flu-dyn",
            Physics::GenPh => "physics.gen-ph",
            Physics::GeoPh => "physics.geo-ph",
            Physics::HistTh => "physics.hist-ph",
            Physics::InsDet => "physics.ins-det",
            Physics::MedPh => "physics.med-ph",
            Physics::Optics => "physics.optics",
            Physics::PlasmaPh => "physics.plasm-ph",
            Physics::PopPh => "physics.pop-ph",
            Physics::SocPh => "physics.soc-ph",
            Physics::SpacePh => "physics.space-ph",
        }
    }


    pub enum QuantitativeBiology{
        BM,
        CB,
        GN,
        MN,
        NC,
        OT,
        PE,
        QM,
        SC,
        TO,
    }

    pub fn quantitative_biology(cat: &QuantitativeBiology) -> &'static str{
        match cat{
            QuantitativeBiology::BM => "q-bio.BM",
            QuantitativeBiology::CB => "q-bio.CB",
            QuantitativeBiology::GN => "q-bio.GN",
            QuantitativeBiology::MN => "q-bio.MN",
            QuantitativeBiology::NC => "q-bio.NC",
            QuantitativeBiology::OT => "q-bio.OT",
            QuantitativeBiology::PE => "q-bio.PE",
            QuantitativeBiology::QM => "q-bio.QM",
            QuantitativeBiology::SC => "q-bio.SC",
            QuantitativeBiology::TO => "q-bio.TO",
        }
    }
    pub enum QuantitativeFinance{
        CP,
        EC,
        GN,
        MF,
        PM,
        PR,
        RM,
        ST,
        TR,
    }

    pub fn quantitative_finance(cat: &QuantitativeFinance) -> &'static str{
        match cat{
            QuantitativeFinance::CP => "q-fin.CP",
            QuantitativeFinance::EC => "q-fin.EC",
            QuantitativeFinance::GN => "q-fin.GN",
            QuantitativeFinance::MF => "q-fin.MF",
            QuantitativeFinance::PM => "q-fin.PM",
            QuantitativeFinance::PR => "q-fin.PR",
            QuantitativeFinance::RM => "q-fin.RM",
            QuantitativeFinance::ST => "q-fin.ST",
            QuantitativeFinance::TR => "q-fin.TR",
        }
    }

    pub enum Statistics{ 
        AP,
        CO,
        ME,
        ML,
        OT,
        TH,
    }

    pub fn statistics(cat: &Statistics) -> &'static str{
        match cat{
            Statistics::AP => "stat.AP",
            Statistics::CO => "stat.CO",
            Statistics::ME => "stat.ME",
            Statistics::ML => "stat.ML",
            Statistics::OT => "stat.OT",
            Statistics::TH => "stat.TH",
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_arxiv_category() {
        let cs_cat = ComputerScience::AI;
        let econ_cat = Economics::EM;
        let eess_cat = ElectricalEngineeringAndSystemsScience::AS;
        let math_cat = Mathematics::AC;
        let phys_cat = PhysicsCategory::Astrophysics(Astrophysics::CO);
        let qbio_cat = QuantitativeBiology::BM;
        let qfin_cat = QuantitativeFinance::CP;
        let stat_cat = Statistics::AP;

        let categories = vec![
            Categories::ComputerScience(cs_cat),
            Categories::Economics(econ_cat),
            Categories::ElectricalEngineeringAndSystemsScience(eess_cat),
            Categories::Mathematics(math_cat),
            Categories::Physics(phys_cat),
            Categories::QuantitativeBiology(qbio_cat),
            Categories::QuantitativeFinance(qfin_cat),
            Categories::Statistics(stat_cat),
        ];

        let expected_strings = vec![
            "cs.AI",
            "econ.EM",
            "eess.AS",
            "math.AC",
            "astro-ph.CO",
            "q-bio.BM",
            "q-fin.CP",
            "stat.AP",
        ];

        for (category, expected) in categories.iter().zip(expected_strings.iter()) {
            assert_eq!(arxiv_category(category), *expected);
        }
        
        
    }
}