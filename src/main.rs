use clap::{Parser, Subcommand};
use leptos::*;
use leptos_2048::*;
use nn::activation::ActivationFunction;
use population::Population;
use ui::RenderGame;

/// Wordle solver
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Web {},

    Cli {},
}

pub const BRAIN_MUTATION_RATE: f64 = 0.1;
pub const BRAIN_MUTATION_VARIATION: f64 = 0.1;
pub const AGENTS_KEEP_PROPORTION: f64 = 0.05;

fn main() {
    let args = Arguments::parse();

    match args.command {
        Some(Commands::Web {}) | None => mount_to_body(|| {
            view! {
                <RenderGame />
            }
        }),
        Some(Commands::Cli {}) => {
            let layers = &[16, 12, 8, 4];
            let act_funs = &[ActivationFunction::ReLU; 3];

            let rounds = 100;
            let max_steps = 10000;
            let evolution_steps = 10000;
            let n_agents = 1000;

            let mut population = Population::new(n_agents, layers, act_funs);

            for _ in 0..evolution_steps {
                for _ in 0..rounds {
                    population.play(max_steps);
                    population.resert_agents();
                }

                let best = population
                    .get_best_agent()
                    .expect("Could not get best agent");
                if population.evolution_step % 10 == 0 {
                    println!(
                        "Step {:5} - Avg Score {:7.2} - highest tile {:4}",
                        population.evolution_step,
                        best.avg_score(),
                        best.get_highest_tile().expect("Error getting best tile")
                    );
                }

                population.evolve(
                    AGENTS_KEEP_PROPORTION,
                    BRAIN_MUTATION_RATE,
                    BRAIN_MUTATION_VARIATION,
                );
            }
        }
    };
}
