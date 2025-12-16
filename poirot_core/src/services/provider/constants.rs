
pub mod arxiv_categories{
use lazy_static::lazy_static;
lazy_static!{
    //Computer Science
    pub static ref CS: Vec<String> = init_cs_categories();
    pub static ref ECON: Vec<String> = init_econ_categories();
    pub static ref EESS: Vec<String> = init_eess_categories();
    pub static ref MATH: Vec<String> = init_math_categories();
    pub static ref ASTRO_PH: Vec<String> = init_astro_ph_categories();
    pub static ref COND_MAT: Vec<String> = init_cond_mat_categories();
    pub static ref GR_QC: Vec<String> = init_gr_qc_categories();
    pub static ref HEP: Vec<String> = init_hep_categories();
    pub static ref MATH_PH: Vec<String> = init_math_ph_categories();
    pub static ref NLIN: Vec<String> = init_nlin_categories();
    pub static ref NUCL: Vec<String> = init_nucl_categories();
    pub static ref PHYSICS: Vec<String> = init_physics_categories();
    pub static ref PHYSICS_EXT: Vec<String> = init_physics_ext_categories();
    pub static ref QUANT_PH: Vec<String> = init_quant_ph_categories();
    pub static ref Q_BIO: Vec<String> = init_q_bio_categories();
    pub static ref Q_FIN: Vec<String> = init_q_fin_categories();
    pub static ref STAT: Vec<String> = init_stat_categories();

    pub static ref ALL_CATEGORIES: Vec<String> = init_all_categories();
}


fn init_cs_categories() ->Vec<String>{
    vec![
        "cs.AI".to_string(),"cs.AR".to_string(),"cs.CC".to_string(),"cs.CE".to_string(),
        "cs.CG".to_string(),"cs.CL".to_string(),"cs.CR".to_string(),"cs.CV".to_string(),
        "cs.CY".to_string(),"cs.DB".to_string(),"cs.DC".to_string(),"cs.DL".to_string(),
        "cs.DM".to_string(),"cs.DS".to_string(),"cs.ET".to_string(),"cs.FL".to_string(),
        "cs.GL".to_string(),"cs.GR".to_string(),"cs.GT".to_string(),"cs.HC".to_string(),
        "cs.IR".to_string(),"cs.IT".to_string(),"cs.LG".to_string(),"cs.LO".to_string(),
        "cs.MA".to_string(),"cs.MM".to_string(),"cs.MS".to_string(),"cs.NA".to_string(),
        "cs.NE".to_string(),"cs.NI".to_string(),"cs.OH".to_string(),"cs.OS".to_string(),
        "cs.PF".to_string(),"cs.PL".to_string(),"cs.RO".to_string(),"cs.SC".to_string(),
        "cs.SD".to_string(),"cs.SE".to_string(),"cs.SI".to_string(),"cs.SY".to_string(),
    ]
}
fn init_econ_categories() -> Vec<String> {
    vec![
        "econ.EM".to_string(),"econ.GN".to_string(),"econ.TH".to_string(),
    ]
}
//Electrical Engineering and Systems Science
fn init_eess_categories() -> Vec<String> {
    vec![
        "eess.AS".to_string(),"eess.IV".to_string(),"eess.SP".to_string(),"eess.SY".to_string(),
    ]
}

fn init_math_categories() -> Vec<String> {
    vec![
        "math.AC".to_string(),"math.AG".to_string(),"math.AP".to_string(),"math.AT".to_string(),
        "math.CA".to_string(),"math.CO".to_string(),"math.CT".to_string(),"math.CV".to_string(),
        "math.DG".to_string(),"math.DS".to_string(),"math.FA".to_string(),"math.GM".to_string(),
        "math.GN".to_string(),"math.GR".to_string(),"math.GT".to_string(),"math.HO".to_string(),
        "math.IT".to_string(),"math.KT".to_string(),"math.LO".to_string(),"math.MG".to_string(),
        "math.MP".to_string(),"math.NA".to_string(),"math.NT".to_string(),"math.OA".to_string(),
        "math.OC".to_string(),"math.PR".to_string(),"math.QA".to_string(),"math.RA".to_string(),
        "math.RT".to_string(),"math.SG".to_string(),"math.SP".to_string(),"math.ST".to_string(),
    ]
}
//Physic
//Astrophysics
fn init_astro_ph_categories() -> Vec<String> {
    vec![
        "astro-ph.CO".to_string(),
        "astro-ph.EP".to_string(),
        "astro-ph.GA".to_string(),
        "astro-ph.HE".to_string(),
        "astro-ph.IM".to_string(),
        "astro-ph.SR".to_string(),
    ]
}
//Condensed Matte
fn init_cond_mat_categories() -> Vec<String> {
    vec![
        "cond-mat.dis-nn".to_string(),
        "cond-mat.mes-hall".to_string(),
        "cond-mat.mtrl-dis".to_string(),
        "cond-mat.mtrl-opt".to_string(),
        "cond-mat.mtrl-sci".to_string(),
        "cond-mat.other".to_string(),
        "cond-mat.quant-gas".to_string(),
        "cond-mat.soft".to_string(),
        "cond-mat.stat-mech".to_string(),
        "cond-mat.str-el".to_string(),
        "cond-mat.supr-con".to_string(),
    ]
}


//GeneralRelativityAndQuantumCosmolog
fn init_gr_qc_categories() -> Vec<String> {
    vec!["gr-qc".to_string()]
}
//High Energy Physics
fn init_hep_categories() -> Vec<String> {
    vec!["hep-ex".to_string(),"hep-lat".to_string(),"hep-ph".to_string(),"hep-th".to_string()]
}   

//Mathematical Physics
fn init_math_ph_categories() -> Vec<String> {
    vec!["math-ph".to_string()]
}
//Nonlinear Sciences
fn init_nlin_categories() -> Vec<String> {
    vec!["nlin.AO".to_string(),"nlin.CD".to_string(),"nlin.CG".to_string(),"nlin.PS".to_string(),
    "nlin.SI".to_string()]
}
//Nuclear  Physics
fn init_nucl_categories() -> Vec<String> {
    vec!["nucl-ex".to_string(),"nucl-th".to_string()]
}
//Physics
fn init_physics_categories() -> Vec<String> {
    vec![
        "physics.acc-ph".to_string(),"physics.ao-ph".to_string(),"physics.app-ph".to_string(),
        "physics.atm-clus".to_string(),"physics.atom-ph".to_string(),"physics.bio-ph".to_string(),
        "physics.chem-ph".to_string(),"physics.class-ph".to_string(),"physics.comp-ph".to_string(),
        "physics.data-an".to_string(),"physics.ed-ph".to_string(),"physics.flu-dyn".to_string(),
        "physics.gen-ph".to_string(),"physics.geo-ph".to_string(),"physics.hist-ph".to_string(),
        "physics.ins-det".to_string(),"physics.med-ph".to_string(),"physics.optics".to_string(),
        "physics.plasm-ph".to_string(),"physics.pop-ph".to_string(),"physics.soc-ph".to_string(),
        "physics.space-ph".to_string(),
    ]
}

//Extended Physics Categories from other categories
//Includes all physics related categories from other main categories

fn init_physics_ext_categories() -> Vec<String> {
    let mut categories = init_astro_ph_categories();
    categories.extend(init_cond_mat_categories());
    categories.extend(init_gr_qc_categories());
    categories.extend(init_hep_categories());
    categories.extend(init_math_ph_categories());
    categories.extend(init_nlin_categories());
    categories.extend(init_nucl_categories());
    categories.extend(init_physics_categories());
    categories
}
 

//Quantum Physics
fn init_quant_ph_categories() -> Vec<String> {
    vec!["quant-ph".to_string()]
}
//Quantitative Biology
fn init_q_bio_categories() -> Vec<String> {
    vec!["q-bio.BM".to_string(),"q-bio.CB".to_string(),"q-bio.GN".to_string(),"q-bio.MN".to_string(),
    "q-bio.NC".to_string(),"q-bio.OT".to_string(),"q-bio.PE".to_string(),"q-bio.QM".to_string(),
    "q-bio.SC".to_string(),"q-bio.TO".to_string()]
}
//Quantitative Finance
fn init_q_fin_categories() -> Vec<String> {
    vec!["q-fin.CP".to_string(),"q-fin.EC".to_string(),"q-fin.GN".to_string(),"q-fin.MF".to_string(),
    "q-fin.PM".to_string(),"q-fin.PR".to_string(),"q-fin.RM".to_string(),"q-fin.ST".to_string(),"q-fin.TR".to_string()]
}
//Statistics
fn init_stat_categories() -> Vec<String> {
    vec!["stat.AP".to_string(),"stat.CO".to_string(),"stat.ME".to_string(),"stat.ML".to_string(),
    "stat.OT".to_string(),"stat.TH".to_string()]
}


fn init_all_categories() -> Vec<String> {
    let mut categories = Vec::new();
    categories.extend(init_cs_categories());
    categories.extend(init_econ_categories());
    categories.extend(init_eess_categories());
    categories.extend(init_math_categories());
    categories.extend(init_physics_ext_categories());
    categories.extend(init_q_bio_categories());
    categories.extend(init_q_fin_categories());
    categories.extend(init_stat_categories());
    categories
}
}