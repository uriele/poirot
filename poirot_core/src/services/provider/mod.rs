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
        arxiv_category_to_string(self)
    }
}
impl ToString for ComputerScience {
    fn to_string(&self) -> String {
        arxiv_text::computer_science_to_string(self)
    }
}
impl ToString for Economics {
    fn to_string(&self) -> String {
        arxiv_text::economics_to_string(self)
    }
}
impl ToString for ElectricalEngineeringAndSystemsScience {
    fn to_string(&self) -> String {
        arxiv_text::eess_to_string(self)
    }
}
impl ToString for Mathematics {
    fn to_string(&self) -> String {
        arxiv_text::mathematics_to_string(self)
    }
}
impl ToString for PhysicsCategory {
    fn to_string(&self) -> String {
        arxiv_text::physics_category_to_string(self)
    }
}
impl ToString for QuantitativeBiology {
    fn to_string(&self) -> String {
        arxiv_text::quantitative_biology_to_string(self)
    }
}
impl ToString for QuantitativeFinance {
    fn to_string(&self) -> String {
        arxiv_text::quantitative_finance_to_string(self)
    }
}
impl ToString for Statistics {
    fn to_string(&self) -> String {
        arxiv_text::statistics_to_string(self)
    }
}
impl ToString for Astrophysics {
    fn to_string(&self) -> String {
        arxiv_text::astrophysics_to_string(self)
    }
}
impl ToString for CondensedMatter {
    fn to_string(&self) -> String {
        arxiv_text::condensed_matter_to_string(self)
    }
}
impl ToString for  Physics {
    fn to_string(&self) -> String {
        arxiv_text::physics_to_string(self)
    }
}
impl ToString for HighEnergyPhysics {
    fn to_string(&self) -> String {
        arxiv_text::high_energy_physics_to_string(self)
    }
}
impl ToString for NonLinearSciences {
    fn to_string(&self) -> String {
        arxiv_text::non_linear_sciences_to_string(self)
    }
}
impl ToString for Nuclear {
    fn to_string(&self) -> String {
        arxiv_text::nuclear_to_string(self)
    }
}


pub fn arxiv_category_to_string(category: &arxiv_text::Categories) -> String{
    match category{
        arxiv_text::Categories::ComputerScience(cs_cat) => arxiv_text::computer_science_to_string(cs_cat),
        arxiv_text::Categories::Economics(econ_cat) => arxiv_text::economics_to_string(econ_cat),
        arxiv_text::Categories::ElectricalEngineeringAndSystemsScience(eess_cat) => arxiv_text::eess_to_string(eess_cat),
        arxiv_text::Categories::Mathematics(math_cat) => arxiv_text::mathematics_to_string(math_cat),
        arxiv_text::Categories::Physics(phys_cat) => arxiv_text::physics_category_to_string(phys_cat),
        arxiv_text::Categories::QuantitativeBiology(qbio_cat) => arxiv_text::quantitative_biology_to_string(qbio_cat),
        arxiv_text::Categories::QuantitativeFinance(qfin_cat) => arxiv_text::quantitative_finance_to_string(qfin_cat),
        arxiv_text::Categories::Statistics(stat_cat) => arxiv_text::statistics_to_string(stat_cat),
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

    pub fn computer_science_to_string(cat: &ComputerScience) -> String{
        match cat{
            ComputerScience::AI => "cs.AI".to_string(),
            ComputerScience::CL => "cs.CL".to_string(),
            ComputerScience::CV => "cs.CV".to_string(),
            ComputerScience::CY => "cs.CY".to_string(),
            ComputerScience::DB => "cs.DB".to_string(),
            ComputerScience::DL => "cs.DL".to_string(),
            ComputerScience::DM => "cs.DM".to_string(),
            ComputerScience::DC => "cs.DC".to_string(),
            ComputerScience::GL => "cs.GL".to_string(),
            ComputerScience::GR => "cs.GR".to_string(),
            ComputerScience::HC => "cs.HC".to_string(),
            ComputerScience::IR => "cs.IR".to_string(),
            ComputerScience::IT => "cs.IT".to_string(),
            ComputerScience::LG => "cs.LG".to_string(),
            ComputerScience::LO => "cs.LO".to_string(),
            ComputerScience::MS => "cs.MS".to_string(),
            ComputerScience::NE => "cs.NE".to_string(),
            ComputerScience::NI => "cs.NI".to_string(),
            ComputerScience::OS => "cs.OS".to_string(),
            ComputerScience::PE => "cs.PE".to_string(),
            ComputerScience::PL => "cs.PL".to_string(),
            ComputerScience::RO => "cs.RO".to_string(),
            ComputerScience::SE => "cs.SE".to_string(),
            ComputerScience::SY => "cs.SY".to_string(),
        }
    }

    pub enum Economics{
        EM,        
        GN,
        TH,
    }

    pub fn economics_to_string(cat: &Economics) -> String{
        match cat{
            Economics::EM => "econ.EM".to_string(),
            Economics::GN => "econ.GN".to_string(),
            Economics::TH => "econ.TH".to_string(),
        }
    }

    pub enum ElectricalEngineeringAndSystemsScience{
        AS ,
        IV ,
        SP ,
        SY ,
        }

    pub fn eess_to_string(cat: &ElectricalEngineeringAndSystemsScience) -> String{
        match cat{
            ElectricalEngineeringAndSystemsScience::AS => "eess.AS".to_string(),
            ElectricalEngineeringAndSystemsScience::IV => "eess.IV".to_string(),
            ElectricalEngineeringAndSystemsScience::SP => "eess.SP".to_string(),
            ElectricalEngineeringAndSystemsScience::SY => "eess.SY".to_string(),
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

    pub fn mathematics_to_string(cat: &Mathematics) -> String{
        match cat{
            Mathematics::AC => "math.AC".to_string(),
            Mathematics::AG => "math.AG".to_string(),
            Mathematics::AP => "math.AP".to_string(),
            Mathematics::AT => "math.AT".to_string(),
            Mathematics::CA => "math.CA".to_string(),
            Mathematics::CO => "math.CO".to_string(),
            Mathematics::CT => "math.CT".to_string(),
            Mathematics::CV => "math.CV".to_string(),
            Mathematics::DG => "math.DG".to_string(),
            Mathematics::DS => "math.DS".to_string(),
            Mathematics::FA => "math.FA".to_string(),
            Mathematics::GM => "math.GM".to_string(),
            Mathematics::GN => "math.GN".to_string(),
            Mathematics::GR => "math.GR".to_string(),
            Mathematics::GT => "math.GT".to_string(),
            Mathematics::HO => "math.HO".to_string(),
            Mathematics::IT => "math.IT".to_string(),
            Mathematics::KT => "math.KT".to_string(),
            Mathematics::LO => "math.LO".to_string(),
            Mathematics::MG => "math.MG".to_string(),
            Mathematics::MP => "math.MP".to_string(),
            Mathematics::NA => "math.NA".to_string(),
            Mathematics::OA => "math.OA".to_string(),
            Mathematics::OC => "math.OC".to_string(),
            Mathematics::PR => "math.PR".to_string(),
            Mathematics::QA => "math.QA".to_string(),
            Mathematics::RT => "math.RT".to_string(),
            Mathematics::SG => "math.SG".to_string(),
            Mathematics::SP => "math.SP".to_string(),
            Mathematics::ST => "math.ST".to_string(),
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

    pub fn physics_category_to_string(cat: &PhysicsCategory) -> String{
        match cat{
            PhysicsCategory::Astrophysics(astro_cat) => astrophysics_to_string(astro_cat),
            PhysicsCategory::CondensedMatter(cm_cat) => condensed_matter_to_string(cm_cat),
            PhysicsCategory::GeneralRelativityAndQuantumCosmology => "gr-qc".to_string(),
            PhysicsCategory::HighEnergyPhysics(hep_cat) => high_energy_physics_to_string(hep_cat),
            PhysicsCategory::MathematicalPhysics => "math-ph".to_string(),
            PhysicsCategory::NonLinearSciences(nlin_cat) => non_linear_sciences_to_string(nlin_cat),
            PhysicsCategory::Nuclear(nucl_cat) => nuclear_to_string(nucl_cat),
            PhysicsCategory::Physics(phys_cat) => physics_to_string(phys_cat),
            PhysicsCategory::QuantumPhysics => "quant-ph".to_string(),
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

    pub fn astrophysics_to_string(cat: &Astrophysics) -> String{
        match cat{
            Astrophysics::CO => "astro-ph.CO".to_string(),
            Astrophysics::EP => "astro-ph.EP".to_string(),
            Astrophysics::GA => "astro-ph.GA".to_string(),
            Astrophysics::HE => "astro-ph.HE".to_string(),
            Astrophysics::IM => "astro-ph.IM".to_string(),
            Astrophysics::SR => "astro-ph.SR".to_string(),
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

    pub fn condensed_matter_to_string(cat: &CondensedMatter) -> String{
        match cat{
            CondensedMatter::DisNn => "cond-mat.dis-nn".to_string(),
            CondensedMatter::MesHall => "cond-mat.mes-hall".to_string(),
            CondensedMatter::MtrlSci => "cond-mat.mtrl-sci".to_string(),
            CondensedMatter::Other => "cond-mat.other".to_string(),
            CondensedMatter::QuantGas => "cond-mat.quant-gas".to_string(),
            CondensedMatter::Soft => "cond-mat.soft".to_string(),
            CondensedMatter::StatMech => "cond-mat.stat-mech".to_string(),
            CondensedMatter::StrEl => "cond-mat.str-el".to_string(),
            CondensedMatter::SuprCon => "cond-mat.supr-con".to_string(),
        }
    }
    
    pub enum HighEnergyPhysics{
        EX,
        LAT,
        PH,
        TH,
    }

    pub fn high_energy_physics_to_string(cat: &HighEnergyPhysics) -> String{
        match cat{
            HighEnergyPhysics::EX => "hep-ex".to_string(),
            HighEnergyPhysics::LAT => "hep-lat".to_string(),
            HighEnergyPhysics::PH => "hep-ph".to_string(),
            HighEnergyPhysics::TH => "hep-th".to_string(),
        }
    }

    pub enum NonLinearSciences{
        AO,
        CD,
        CG,
        PS,
        SI,
    }

    pub fn non_linear_sciences_to_string(cat: &NonLinearSciences) -> String{
        match cat{
            NonLinearSciences::AO => "nlin.AO".to_string(),
            NonLinearSciences::CD => "nlin.CD".to_string(),
            NonLinearSciences::CG => "nlin.CG".to_string(),
            NonLinearSciences::PS => "nlin.PS".to_string(),
            NonLinearSciences::SI => "nlin.SI".to_string(),
        }
    }

    pub enum Nuclear{
        EX,
        TH,
    }

    pub fn nuclear_to_string(cat: &Nuclear) -> String{
        match cat{
            Nuclear::EX => "nucl-ex".to_string(),
            Nuclear::TH => "nucl-th".to_string(),
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
     
    pub fn physics_to_string(cat: &Physics) -> String{
        match cat{
            Physics::AccPh => "physics.acc-ph".to_string(),
            Physics::AoPh => "physics.ao-ph".to_string(),
            Physics::AppPh => "physics.app-ph".to_string(),
            Physics::AtmClus => "physics.atm-clus".to_string(),
            Physics::AtomPh => "physics.atom-ph".to_string(),
            Physics::BioPh => "physics.bio-ph".to_string(),
            Physics::ChemPh => "physics.chem-ph".to_string(),
            Physics::ClassPh => "physics.class-ph".to_string(),
            Physics::CompPh => "physics.comp-ph".to_string(),
            Physics::DataAn => "physics.data-an".to_string(),
            Physics::EdPh => "physics.ed-ph".to_string(),
            Physics::FluDyn => "physics.flu-dyn".to_string(),
            Physics::GenPh => "physics.gen-ph".to_string(),
            Physics::GeoPh => "physics.geo-ph".to_string(),
            Physics::HistTh => "physics.hist-ph".to_string(),
            Physics::InsDet => "physics.ins-det".to_string(),
            Physics::MedPh => "physics.med-ph".to_string(),
            Physics::Optics => "physics.optics".to_string(),
            Physics::PlasmaPh => "physics.plasm-ph".to_string(),
            Physics::PopPh => "physics.pop-ph".to_string(),
            Physics::SocPh => "physics.soc-ph".to_string(),
            Physics::SpacePh => "physics.space-ph".to_string(),
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

    pub fn quantitative_biology_to_string(cat: &QuantitativeBiology) -> String{
        match cat{
            QuantitativeBiology::BM => "q-bio.BM".to_string(),
            QuantitativeBiology::CB => "q-bio.CB".to_string(),
            QuantitativeBiology::GN => "q-bio.GN".to_string(),
            QuantitativeBiology::MN => "q-bio.MN".to_string(),
            QuantitativeBiology::NC => "q-bio.NC".to_string(),
            QuantitativeBiology::OT => "q-bio.OT".to_string(),
            QuantitativeBiology::PE => "q-bio.PE".to_string(),
            QuantitativeBiology::QM => "q-bio.QM".to_string(),
            QuantitativeBiology::SC => "q-bio.SC".to_string(),
            QuantitativeBiology::TO => "q-bio.TO".to_string(),
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

    pub fn quantitative_finance_to_string(cat: &QuantitativeFinance) -> String{
        match cat{
            QuantitativeFinance::CP => "q-fin.CP".to_string(),
            QuantitativeFinance::EC => "q-fin.EC".to_string(),
            QuantitativeFinance::GN => "q-fin.GN".to_string(),
            QuantitativeFinance::MF => "q-fin.MF".to_string(),
            QuantitativeFinance::PM => "q-fin.PM".to_string(),
            QuantitativeFinance::PR => "q-fin.PR".to_string(),
            QuantitativeFinance::RM => "q-fin.RM".to_string(),
            QuantitativeFinance::ST => "q-fin.ST".to_string(),
            QuantitativeFinance::TR => "q-fin.TR".to_string(),
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

    pub fn statistics_to_string(cat: &Statistics) -> String{
        match cat{
            Statistics::AP => "stat.AP".to_string(),
            Statistics::CO => "stat.CO".to_string(),
            Statistics::ME => "stat.ME".to_string(),
            Statistics::ML => "stat.ML".to_string(),
            Statistics::OT => "stat.OT".to_string(),
            Statistics::TH => "stat.TH".to_string(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_arxiv_category_to_string() {
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
            assert_eq!(arxiv_category_to_string(category), *expected);
        }
        
        
    }
}