use crate::datasets::points::{
    sample_noisy_sphere, sample_random_points, sample_swiss_roll, sample_torus,
};
use crate::datasets::{Dataset, DATASET_DIRECTORY};
use crate::distance_matrix::input::read_lower_triangular_distance_matrix;
use crate::distance_matrix::output::write_lower_triangular_distance_matrix;
use crate::distance_matrix::DistanceMatrix;
use crate::points::PointCloud;
use ordered_float::OrderedFloat;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::{fs, io};

fn read_distance_matrix_from_file<P: AsRef<Path>>(
    filepath: P,
) -> io::Result<DistanceMatrix<OrderedFloat<f64>>> {
    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(&file);
    let distance_matrix = read_lower_triangular_distance_matrix(reader)?;

    Ok(distance_matrix)
}

fn sample_distance_matrix<F: Fn(usize) -> PointCloud<f64, N>, const N: usize>(
    n_points: usize,
    f: F,
) -> DistanceMatrix<OrderedFloat<f64>> {
    let points: PointCloud<OrderedFloat<f64>, N> = f(n_points).into();
    points.distance_matrix()
}

fn read_or_save_distance_matrix<
    P: AsRef<Path>,
    F: FnOnce() -> DistanceMatrix<OrderedFloat<f64>>,
>(
    dst_filename: P,
    distance_matrix_builder: F,
    use_cache: bool,
) -> io::Result<DistanceMatrix<OrderedFloat<f64>>> {
    if dst_filename.as_ref().is_file() && use_cache {
        read_distance_matrix_from_file(dst_filename)
    } else {
        let distance_matrix = distance_matrix_builder();

        if use_cache {
            let dst_file = fs::File::create(dst_filename)?;
            let mut dst_writer = BufWriter::new(dst_file);
            write_lower_triangular_distance_matrix(&distance_matrix, &mut dst_writer)?;
        }

        Ok(distance_matrix)
    }
}

/// Returns the distance matrix of the given dataset.
pub fn get_dataset_distance_matrix(
    dataset: Dataset,
    use_cache: bool,
) -> io::Result<DistanceMatrix<OrderedFloat<f64>>> {
    let dataset_directory: &Path = Path::new(DATASET_DIRECTORY);
    match dataset {
        Dataset::Senate => read_distance_matrix_from_file(
            dataset_directory.join("senate104_edge_list.txt_0.68902_distmat.txt"),
        ),
        Dataset::Eleg => read_distance_matrix_from_file(dataset_directory.join(
            "celegans_weighted_undirected_reindexed_for_matlab_maxdist_2.6429_SP_distmat.txt",
        )),
        Dataset::Netwsc => read_distance_matrix_from_file(
            dataset_directory.join("network379_edge_list.txt_38.3873_distmat.txt"),
        ),
        Dataset::Hiv => read_distance_matrix_from_file(
            dataset_directory.join("HIV1_2011.all.nt.concat.fa_hdm.txt"),
        ),
        Dataset::Dragon => read_distance_matrix_from_file(
            dataset_directory.join("dragon_vrip.ply.txt_2000_.txt_distmat.txt"),
        ),
        Dataset::Sphere { n_points } => {
            let dst_filename = dataset_directory.join(format!("sphere_{n_points}_distmat.txt"));
            read_or_save_distance_matrix(
                dst_filename,
                || {
                    sample_distance_matrix(n_points, |n| {
                        sample_noisy_sphere::<f64, 3>(n, 0.9, 0.75, 0.3)
                    })
                },
                use_cache,
            )
        }
        Dataset::Torus { n_points } => {
            let dst_filename = dataset_directory.join(format!("torus_{n_points}_distmat.txt"));
            read_or_save_distance_matrix(
                dst_filename,
                || sample_distance_matrix(n_points, sample_torus),
                use_cache,
            )
        }
        Dataset::SwissRoll { n_points } => {
            let dst_filename = dataset_directory.join(format!("swiss_roll_{n_points}_distmat.txt"));
            read_or_save_distance_matrix(
                dst_filename,
                || sample_distance_matrix(n_points, sample_swiss_roll),
                use_cache,
            )
        }
        Dataset::Circle { n_points } => {
            let dst_filename = dataset_directory.join(format!("circle_{n_points}_distmat.txt"));
            read_or_save_distance_matrix(
                dst_filename,
                || {
                    sample_distance_matrix(n_points, |n| {
                        sample_noisy_sphere::<f64, 2>(n, 1., 0., 0.)
                    })
                },
                use_cache,
            )
        }
        Dataset::Uniform { n_points } => {
            let dst_filename = dataset_directory.join(format!("uniform_{n_points}_distmat.txt"));
            read_or_save_distance_matrix(
                dst_filename,
                || sample_distance_matrix(n_points, sample_random_points::<f64, 2>),
                use_cache,
            )
        }
    }
}
