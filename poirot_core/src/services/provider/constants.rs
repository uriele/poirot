
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
    init_astro_ph_categories().into().copied()
        .chain(init_cond_mat_categories().into().copied())
        .chain(init_gr_qc_categories().into().copied())
        .chain(init_hep_categories().into().copied())
        .chain(init_math_ph_categories().into().copied())
        .chain(init_nlin_categories().into().copied())
        .chain(init_nucl_categories().into().copied())
        .chain(init_physics_categories().into().copied())
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