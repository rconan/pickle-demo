use serde::Deserialize;
use serde_pickle as pickle;
use std::{fs::File, io::BufReader};

#[derive(Deserialize)]
struct QPData {
    #[serde(rename = "D")]
    dmat: Vec<f64>,
    #[serde(rename = "W2")]
    w2: Vec<f64>,
    #[serde(rename = "W3")]
    w3: Vec<f64>,
    #[serde(rename = "K")]
    k: f64,
    #[serde(rename = "wfsMask")]
    wfs_mask: Vec<Vec<bool>>,
    umin: Vec<f64>,
    umax: Vec<f64>,
    rm_mean_slopes: bool,
    #[serde(rename = "_Tu")]
    tu: Vec<f64>,
    rho_3: f64,
    end2end_ordering: bool,
}
#[derive(Deserialize)]
struct QP {
    #[serde(rename = "SHAcO_qp")]
    data: QPData,
}

fn main() {
    let file = File::open("SHAcO_qp_rhoP1e-3_kIp5.rs.pkl").unwrap();
    let mut rdr = BufReader::with_capacity(10_000, file);
    let qp: QP = pickle::from_reader(rdr).unwrap();
}
