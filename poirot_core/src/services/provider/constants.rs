
        

pub mod arxiv_text{

    use super::arxiv_categories;
    use std::fmt::Display;
    use crate::services::provider::utils::arxiv_category_to_text;
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


    impl Display for Categories{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",arxiv_category_to_text(self))
        }
    }
    
   impl Display for ComputerScience{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",computer_science(self))
        }
    }

    impl Display for Economics{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",economics(self))
        }
    }

    impl Display for ElectricalEngineeringAndSystemsScience{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",eess(self))
        }

    }

    
    impl Display for Mathematics{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",mathematics(self))
        }
    }

    impl Display for PhysicsCategory{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",physics_category(self))
        }
    }

    impl Display for QuantitativeBiology {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",quantitative_biology(self))
        }
    }

    impl Display for QuantitativeFinance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",quantitative_finance(self))
        }
    }

    impl Display for Statistics {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",statistics(self))
        }
    }

    impl Display for Astrophysics {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",astrophysics(self))
        }
    }

    impl Display for CondensedMatter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",condensed_matter(self))
        }
    }

    impl Display for  Physics {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",physics(self))
        }
    }

    impl Display for HighEnergyPhysics {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",high_energy_physics(self))
        }
    }

    impl Display for NonLinearSciences {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",non_linear_sciences(self))
        }
    }

    impl Display for Nuclear {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"{}",nuclear(self))
        }
    }




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
            ComputerScience::AI => arxiv_categories::CS[0],
            ComputerScience::CL => arxiv_categories::CS[1],
            ComputerScience::CV => arxiv_categories::CS[2],
            ComputerScience::CY => arxiv_categories::CS[3],
            ComputerScience::DB => arxiv_categories::CS[4],
            ComputerScience::DL => arxiv_categories::CS[5],
            ComputerScience::DM => arxiv_categories::CS[6],
            ComputerScience::DC => arxiv_categories::CS[7],
            ComputerScience::GL => arxiv_categories::CS[8],
            ComputerScience::GR => arxiv_categories::CS[9],
            ComputerScience::HC => arxiv_categories::CS[10],
            ComputerScience::IR => arxiv_categories::CS[11],
            ComputerScience::IT => arxiv_categories::CS[12],
            ComputerScience::LG => arxiv_categories::CS[13],
            ComputerScience::LO => arxiv_categories::CS[14],
            ComputerScience::MS => arxiv_categories::CS[15],
            ComputerScience::NE => arxiv_categories::CS[16],
            ComputerScience::NI => arxiv_categories::CS[17],
            ComputerScience::OS => arxiv_categories::CS[18],
            ComputerScience::PE => arxiv_categories::CS[19],
            ComputerScience::PL => arxiv_categories::CS[20],
            ComputerScience::RO => arxiv_categories::CS[21],
            ComputerScience::SE => arxiv_categories::CS[22],
            ComputerScience::SY => arxiv_categories::CS[23],
        }
    }

    pub enum Economics{
        EM,        
        GN,
        TH,
    }

    pub fn economics(cat: &Economics) -> &'static str{
        match cat{
            Economics::EM => arxiv_categories::ECON[0],
            Economics::GN => arxiv_categories::ECON[1],
            Economics::TH => arxiv_categories::ECON[2],
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
            ElectricalEngineeringAndSystemsScience::AS => arxiv_categories::EESS[0],
            ElectricalEngineeringAndSystemsScience::IV => arxiv_categories::EESS[1],
            ElectricalEngineeringAndSystemsScience::SP => arxiv_categories::EESS[2],
            ElectricalEngineeringAndSystemsScience::SY => arxiv_categories::EESS[3],
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
            Mathematics::AC => arxiv_categories::MATH[0],
            Mathematics::AG => arxiv_categories::MATH[1],
            Mathematics::AP => arxiv_categories::MATH[2],
            Mathematics::AT => arxiv_categories::MATH[3],
            Mathematics::CA => arxiv_categories::MATH[4],
            Mathematics::CO => arxiv_categories::MATH[5],
            Mathematics::CT => arxiv_categories::MATH[6],
            Mathematics::CV => arxiv_categories::MATH[7],
            Mathematics::DG => arxiv_categories::MATH[8],
            Mathematics::DS => arxiv_categories::MATH[9],
            Mathematics::FA => arxiv_categories::MATH[10],
            Mathematics::GM => arxiv_categories::MATH[11],
            Mathematics::GN => arxiv_categories::MATH[12],
            Mathematics::GR => arxiv_categories::MATH[13],
            Mathematics::GT => arxiv_categories::MATH[14],
            Mathematics::HO => arxiv_categories::MATH[15],
            Mathematics::IT => arxiv_categories::MATH[16],
            Mathematics::KT => arxiv_categories::MATH[17],
            Mathematics::LO => arxiv_categories::MATH[18],
            Mathematics::MG => arxiv_categories::MATH[19],
            Mathematics::MP => arxiv_categories::MATH[20],
            Mathematics::NA => arxiv_categories::MATH[21],
            Mathematics::OA => arxiv_categories::MATH[22],
            Mathematics::OC => arxiv_categories::MATH[23],
            Mathematics::PR => arxiv_categories::MATH[24],
            Mathematics::QA => arxiv_categories::MATH[25],
            Mathematics::RT => arxiv_categories::MATH[26],
            Mathematics::SG => arxiv_categories::MATH[27],
            Mathematics::SP => arxiv_categories::MATH[28],
            Mathematics::ST => arxiv_categories::MATH[29],
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
            Astrophysics::CO => arxiv_categories::ASTRO_PH[0],
            Astrophysics::EP => arxiv_categories::ASTRO_PH[1],
            Astrophysics::GA => arxiv_categories::ASTRO_PH[2],
            Astrophysics::HE => arxiv_categories::ASTRO_PH[3],
            Astrophysics::IM => arxiv_categories::ASTRO_PH[4],
            Astrophysics::SR => arxiv_categories::ASTRO_PH[5],
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
            CondensedMatter::DisNn    => arxiv_categories::COND_MAT[0],
            CondensedMatter::MesHall  => arxiv_categories::COND_MAT[1],
            CondensedMatter::MtrlSci  => arxiv_categories::COND_MAT[2],
            CondensedMatter::Other    => arxiv_categories::COND_MAT[3],
            CondensedMatter::QuantGas => arxiv_categories::COND_MAT[4],
            CondensedMatter::Soft     => arxiv_categories::COND_MAT[5],
            CondensedMatter::StatMech => arxiv_categories::COND_MAT[6],
            CondensedMatter::StrEl    => arxiv_categories::COND_MAT[7],
            CondensedMatter::SuprCon  => arxiv_categories::COND_MAT[8],
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
            HighEnergyPhysics::EX  => arxiv_categories::HEP[0],
            HighEnergyPhysics::LAT => arxiv_categories::HEP[1],
            HighEnergyPhysics::PH  => arxiv_categories::HEP[2],
            HighEnergyPhysics::TH  => arxiv_categories::HEP[3],
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
            NonLinearSciences::AO => arxiv_categories::NLIN[0],
            NonLinearSciences::CD => arxiv_categories::NLIN[1],
            NonLinearSciences::CG => arxiv_categories::NLIN[2],
            NonLinearSciences::PS => arxiv_categories::NLIN[3],
            NonLinearSciences::SI => arxiv_categories::NLIN[4],
        }
    }

    pub enum Nuclear{
        EX,
        TH,
    }

    pub fn nuclear(cat: &Nuclear) -> &'static str{
        match cat{
            Nuclear::EX => arxiv_categories::NUCL[0],
            Nuclear::TH => arxiv_categories::NUCL[1],
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
            Physics::AccPh => arxiv_categories::PHYSICS[0],
            Physics::AoPh => arxiv_categories::PHYSICS[1],
            Physics::AppPh => arxiv_categories::PHYSICS[2],
            Physics::AtmClus => arxiv_categories::PHYSICS[3],
            Physics::AtomPh => arxiv_categories::PHYSICS[4],
            Physics::BioPh => arxiv_categories::PHYSICS[5],
            Physics::ChemPh => arxiv_categories::PHYSICS[6],
            Physics::ClassPh => arxiv_categories::PHYSICS[7],
            Physics::CompPh => arxiv_categories::PHYSICS[8],
            Physics::DataAn => arxiv_categories::PHYSICS[9],
            Physics::EdPh => arxiv_categories::PHYSICS[10],
            Physics::FluDyn => arxiv_categories::PHYSICS[11],
            Physics::GenPh => arxiv_categories::PHYSICS[12],
            Physics::GeoPh => arxiv_categories::PHYSICS[13],
            Physics::HistTh => arxiv_categories::PHYSICS[14],
            Physics::InsDet => arxiv_categories::PHYSICS[15],
            Physics::MedPh => arxiv_categories::PHYSICS[16],
            Physics::Optics => arxiv_categories::PHYSICS[17],
            Physics::PlasmaPh => arxiv_categories::PHYSICS[18],
            Physics::PopPh => arxiv_categories::PHYSICS[19],
            Physics::SocPh => arxiv_categories::PHYSICS[20],
            Physics::SpacePh => arxiv_categories::PHYSICS[21],
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
            QuantitativeBiology::BM => arxiv_categories::Q_BIO[0],
            QuantitativeBiology::CB => arxiv_categories::Q_BIO[1],
            QuantitativeBiology::GN => arxiv_categories::Q_BIO[2],
            QuantitativeBiology::MN => arxiv_categories::Q_BIO[3],
            QuantitativeBiology::NC => arxiv_categories::Q_BIO[4],
            QuantitativeBiology::OT => arxiv_categories::Q_BIO[5],
            QuantitativeBiology::PE => arxiv_categories::Q_BIO[6],
            QuantitativeBiology::QM => arxiv_categories::Q_BIO[7],
            QuantitativeBiology::SC => arxiv_categories::Q_BIO[8],
            QuantitativeBiology::TO => arxiv_categories::Q_BIO[9],
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
            QuantitativeFinance::CP => arxiv_categories::Q_FIN[0],
            QuantitativeFinance::EC => arxiv_categories::Q_FIN[1],
            QuantitativeFinance::GN => arxiv_categories::Q_FIN[2],
            QuantitativeFinance::MF => arxiv_categories::Q_FIN[3],
            QuantitativeFinance::PM => arxiv_categories::Q_FIN[4],
            QuantitativeFinance::PR => arxiv_categories::Q_FIN[5],
            QuantitativeFinance::RM => arxiv_categories::Q_FIN[6],
            QuantitativeFinance::ST => arxiv_categories::Q_FIN[7],
            QuantitativeFinance::TR => arxiv_categories::Q_FIN[8],
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
            Statistics::AP => arxiv_categories::STAT[0],
            Statistics::CO => arxiv_categories::STAT[1],
            Statistics::ME => arxiv_categories::STAT[2],
            Statistics::ML => arxiv_categories::STAT[3],
            Statistics::OT => arxiv_categories::STAT[4],
            Statistics::TH => arxiv_categories::STAT[5],
        }
    }
}




pub mod arxiv_categories{
use lazy_static::lazy_static;
lazy_static!{
    //Computer Science
    pub static ref CS: &'static[&'static str] = init_cs_categories();
    pub static ref ECON: &'static[&'static str] = init_econ_categories();
    pub static ref EESS: &'static[&'static str] = init_eess_categories();
    pub static ref MATH: &'static[&'static str] = init_math_categories();
    pub static ref ASTRO_PH: &'static[&'static str] = init_astro_ph_categories();
    pub static ref COND_MAT: &'static[&'static str] = init_cond_mat_categories();
    pub static ref GR_QC: &'static[&'static str] = init_gr_qc_categories();
    pub static ref HEP: &'static[&'static str] = init_hep_categories();
    pub static ref MATH_PH: &'static[&'static str] = init_math_ph_categories();
    pub static ref NLIN: &'static[&'static str] = init_nlin_categories();
    pub static ref NUCL: &'static[&'static str] = init_nucl_categories();
    pub static ref PHYSICS: &'static[&'static str] = init_physics_categories();
    pub static ref PHYSICS_EXT: &'static[&'static str] = init_physics_ext_categories();
    pub static ref QUANT_PH: &'static[&'static str] = init_quant_ph_categories();
    pub static ref Q_BIO: &'static[&'static str] = init_q_bio_categories();
    pub static ref Q_FIN: &'static[&'static str] = init_q_fin_categories();
    pub static ref STAT: &'static[&'static str] = init_stat_categories();

    pub static ref ALL_CATEGORIES: &'static[&'static str] = init_all_categories();
}


fn init_cs_categories() -> &'static[&'static str]{
    &[
        "cs.AI","cs.AR","cs.CC","cs.CE",
        "cs.CG","cs.CL","cs.CR","cs.CV",
        "cs.CY","cs.DB","cs.DC","cs.DL",
        "cs.DM","cs.DS","cs.ET","cs.FL",
        "cs.GL","cs.GR","cs.GT","cs.HC",
        "cs.IR","cs.IT","cs.LG","cs.LO",
        "cs.MA","cs.MM","cs.MS","cs.NA",
        "cs.NE","cs.NI","cs.OH","cs.OS",
        "cs.PF","cs.PL","cs.RO","cs.SC",
        "cs.SD","cs.SE","cs.SI","cs.SY",
    ]
}
fn init_econ_categories() -> &'static[&'static str] {
   &[
        "econ.EM","econ.GN","econ.TH",
    ]
}
//Electrical Engineering and Systems Science
fn init_eess_categories() -> &'static[&'static str] {
   &[
        "eess.AS","eess.IV","eess.SP","eess.SY",
    ]
}

fn init_math_categories() -> &'static[&'static str] {
   &[
        "math.AC","math.AG","math.AP","math.AT",
        "math.CA","math.CO","math.CT","math.CV",
        "math.DG","math.DS","math.FA","math.GM",
        "math.GN","math.GR","math.GT","math.HO",
        "math.IT","math.KT","math.LO","math.MG",
        "math.MP","math.NA","math.NT","math.OA",
        "math.OC","math.PR","math.QA","math.RA",
        "math.RT","math.SG","math.SP","math.ST",
    ]
}
//Physic
//Astrophysics
fn init_astro_ph_categories() -> &'static[&'static str] {
   &[
        "astro-ph.CO",
        "astro-ph.EP",
        "astro-ph.GA",
        "astro-ph.HE",
        "astro-ph.IM",
        "astro-ph.SR",
    ]
}
//Condensed Matte
fn init_cond_mat_categories() -> &'static[&'static str] {
   &[
        "cond-mat.dis-nn",
        "cond-mat.mes-hall",
        "cond-mat.mtrl-dis",
        "cond-mat.mtrl-opt",
        "cond-mat.mtrl-sci",
        "cond-mat.other",
        "cond-mat.quant-gas",
        "cond-mat.soft",
        "cond-mat.stat-mech",
        "cond-mat.str-el",
        "cond-mat.supr-con",
    ]
}


//GeneralRelativityAndQuantumCosmolog
fn init_gr_qc_categories() -> &'static[&'static str] {
    &["gr-qc"]
}
//High Energy Physics
fn init_hep_categories() -> &'static[&'static str] {
    &["hep-ex","hep-lat","hep-ph","hep-th"]
}   

//Mathematical Physics
fn init_math_ph_categories() -> &'static[&'static str] {
    &["math-ph"]
}
//Nonlinear Sciences
fn init_nlin_categories() -> &'static[&'static str] {
    &["nlin.AO","nlin.CD","nlin.CG","nlin.PS",
    "nlin.SI"]
}
//Nuclear  Physics
fn init_nucl_categories() -> &'static[&'static str] {
    &["nucl-ex","nucl-th"]
}
//Physics
fn init_physics_categories() -> &'static[&'static str] {
   &[
        "physics.acc-ph","physics.ao-ph","physics.app-ph",
        "physics.atm-clus","physics.atom-ph","physics.bio-ph",
        "physics.chem-ph","physics.class-ph","physics.comp-ph",
        "physics.data-an","physics.ed-ph","physics.flu-dyn",
        "physics.gen-ph","physics.geo-ph","physics.hist-ph",
        "physics.ins-det","physics.med-ph","physics.optics",
        "physics.plasm-ph","physics.pop-ph","physics.soc-ph",
        "physics.space-ph",
    ]
}

//Extended Physics Categories from other categories
//Includes all physics related categories from other main categories

fn init_physics_ext_categories() -> &'static[&'static str] {
    init_astro_ph_categories().iter().copied()
        .chain(init_cond_mat_categories().iter().copied())
        .chain(init_gr_qc_categories().iter().copied())
        .chain(init_hep_categories().iter().copied())
        .chain(init_math_ph_categories().iter().copied())
        .chain(init_nlin_categories().iter().copied())
        .chain(init_nucl_categories().iter().copied())
        .chain(init_physics_categories().iter().copied())
        .collect::<Vec<&'static str>>()
        .leak()
}
 

//Quantum Physics
fn init_quant_ph_categories() -> &'static[&'static str] {
    &[
        "quant-ph"
    ]
}
//Quantitative Biology
fn init_q_bio_categories() -> &'static[&'static str] {
    &[
        "q-bio.BM","q-bio.CB","q-bio.GN","q-bio.MN",
        "q-bio.NC","q-bio.OT","q-bio.PE","q-bio.QM",
        "q-bio.SC","q-bio.TO"
    ]
}
//Quantitative Finance
fn init_q_fin_categories() -> &'static[&'static str] {
    &[
        "q-fin.CP","q-fin.EC","q-fin.GN","q-fin.MF",
        "q-fin.PM","q-fin.PR","q-fin.RM","q-fin.ST",
        "q-fin.TR"
    ]
}
//Statistics
fn init_stat_categories() -> &'static[&'static str] {
    &[
        "stat.AP","stat.CO","stat.ME","stat.ML",
        "stat.OT","stat.TH"
    ]
}


fn init_all_categories() -> &'static[&'static str] {
     init_cs_categories().iter().copied()
    .chain(init_econ_categories().iter().copied())
    .chain(init_eess_categories().iter().copied())
    .chain(init_math_categories().iter().copied())
    .chain(init_physics_ext_categories().iter().copied())
    .chain(init_q_bio_categories().iter().copied())
    .chain(init_q_fin_categories().iter().copied())
    .chain(init_stat_categories().iter().copied())
    .collect::<Vec<&'static str>>() //need to specify in what I want to collect
    .leak() //leak the data from Vector to create the static memory reference
}
}



