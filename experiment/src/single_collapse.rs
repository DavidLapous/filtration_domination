use anyhow::Result;
use filtration_domination::edges::{EdgeList, FilteredEdge, write_edge_list};
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::time::Duration;
use filtration_domination::OneCriticalGrade;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SingleCollapser {
    Giotto,
    Glisse,
}

/// Runs a single-parameter edge collapser on the given edge list, and returns the number of
/// resulting edges.
/// Not thread-safe, because it writes to a fixed file.
pub fn run_single_parameter_edge_collapse(
    edges: &EdgeList<FilteredEdge<OneCriticalGrade<usize, 1>>>,
    collapser: SingleCollapser,
) -> Result<(usize, Duration)> {
    let edges_out_file = "edges.txt";
    {
        let mut out_edges_file = fs::File::create(edges_out_file)?;
        write_edge_list(edges, &mut out_edges_file)?;
        out_edges_file.sync_data()?;
    }

    let command_name = match collapser {
        SingleCollapser::Giotto => "giotto_collapser",
        SingleCollapser::Glisse => "glisse_collapser",
    };

    let mut collapser_command = Command::new(command_name);
    collapser_command.args(vec![edges_out_file]);
    let collapser_output = collapser_command.output()?;

    let mut stdout = BufReader::new(&collapser_output.stdout[..]);

    let mut buffer = String::new();
    stdout.read_line(&mut buffer)?;
    let resulting_edges: usize = buffer.trim().parse()?;
    buffer.clear();
    stdout.read_line(&mut buffer)?;
    let seconds: f64 = buffer.trim().parse()?;

    Ok((resulting_edges, Duration::from_secs_f64(seconds)))
}