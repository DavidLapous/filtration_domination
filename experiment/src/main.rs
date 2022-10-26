mod experiments;
mod table;

use crate::experiments::orders::{compare_orders, OrderCli};
use crate::table::{display, display_duration, Row, Table};

use filtration_domination::datasets;

use clap::Parser;

use std::fmt::Formatter;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::time::Duration;

const TABLE_OUTPUT_DIRECTORY: &str = "charts";

pub(crate) const TIMEOUT_SECONDS: u64 = 60 * 60 * 2;

const TIMEOUT_DURATION: Duration = Duration::from_secs(TIMEOUT_SECONDS);

/// Run experiments for edge collapse
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
enum ExperimentCli {
    Order(OrderCli),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, clap::ArgEnum)]
enum CliDataset {
    Senate,
    Eleg,
    Netwsc,
    Hiv,
    Dragon,
    Sphere,
    Uniform,
    Circle,
    Torus,
    SwissRoll,
}

const ALL_DATASETS: [CliDataset; 10] = [
    CliDataset::Senate,
    CliDataset::Eleg,
    CliDataset::Netwsc,
    CliDataset::Hiv,
    CliDataset::Dragon,
    CliDataset::Sphere,
    CliDataset::Uniform,
    CliDataset::Circle,
    CliDataset::Torus,
    CliDataset::SwissRoll,
];

impl CliDataset {
    fn to_internal_dataset(self, n_points: Option<usize>) -> datasets::Dataset {
        match self {
            CliDataset::Senate => datasets::Dataset::Senate,
            CliDataset::Eleg => datasets::Dataset::Eleg,
            CliDataset::Netwsc => datasets::Dataset::Netwsc,
            CliDataset::Hiv => datasets::Dataset::Hiv,
            CliDataset::Dragon => datasets::Dataset::Dragon,
            CliDataset::Uniform => datasets::Dataset::Uniform {
                n_points: n_points.unwrap_or(400),
            },
            CliDataset::Sphere => datasets::Dataset::Sphere {
                n_points: n_points.unwrap_or(100),
            },
            CliDataset::Circle => datasets::Dataset::Circle {
                n_points: n_points.unwrap_or(100),
            },
            CliDataset::Torus => datasets::Dataset::Torus {
                n_points: n_points.unwrap_or(200),
            },
            CliDataset::SwissRoll => datasets::Dataset::SwissRoll {
                n_points: n_points.unwrap_or(200),
            },
        }
    }

    fn to_static_str(self) -> &'static str {
        match self {
            CliDataset::Senate => "senate",
            CliDataset::Eleg => "eleg",
            CliDataset::Netwsc => "netwsc",
            CliDataset::Hiv => "hiv",
            CliDataset::Dragon => "dragon",
            CliDataset::Uniform => "uniform",
            CliDataset::Sphere => "sphere",
            CliDataset::Circle => "circle",
            CliDataset::Torus => "torus",
            CliDataset::SwissRoll => "swiss roll",
        }
    }
}

impl std::fmt::Display for CliDataset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub(crate) enum Algorithm {
    FiltrationDomination,
    StrongFiltrationDomination,
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithm::FiltrationDomination => {
                write!(f, "Filtration-domination")
            }
            Algorithm::StrongFiltrationDomination => {
                write!(f, "Strong filtration-domination")
            }
        }
    }
}

fn save_table<R: Row>(table: Table<R>, name: &str) -> anyhow::Result<()> {
    let out_dir = Path::new(TABLE_OUTPUT_DIRECTORY);
    let out_base_file = out_dir.join(name);

    let csv_file = File::create(&out_base_file.with_extension("csv"))?;
    let mut writer = BufWriter::new(csv_file);
    table.display_as_csv(&mut writer)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let opt: ExperimentCli = ExperimentCli::parse();

    match opt {
        ExperimentCli::Order(opts) => {
            compare_orders(opts)?;
        }
    }

    Ok(())
}