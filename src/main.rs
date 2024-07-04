use clap::{Parser, Subcommand};
use leptos::*;
use leptos_2048::*;
use nn::{activation::ActivationFunction, NeuralNetwork};
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

    Train {
        /// Save the model every 10 steps
        #[arg(short, long)]
        save: Option<String>,

        /// Load the model from file
        #[arg(short, long)]
        load: Option<String>,
    },
}

pub const BRAIN_MUTATION_RATE: f64 = 0.1;
pub const BRAIN_MUTATION_VARIATION: f64 = 0.1;
pub const AGENTS_KEEP_PROPORTION: f64 = 0.02;

fn main() {
    let args = Arguments::parse();

    match args.command {
        Some(Commands::Web {}) | None => mount_to_body(|| {
            view! {
                <RenderGame />
            }
        }),
        Some(Commands::Train { save, load }) => {
            let rounds = 100;
            let max_steps = 10000;
            let evolution_steps = 10000;
            let n_agents = 1000;

            let mut population = match load {
                Some(file) => {
                    let nn = NeuralNetwork::load(&file).expect("Failed to load NN");
                    Population::from_nn(n_agents, nn)
                }
                None => {
                    let layers = &[16, 128, 64, 4];
                    let act_funs = &[
                        ActivationFunction::ReLU,
                        ActivationFunction::ReLU,
                        ActivationFunction::None,
                    ];
                    Population::new(n_agents, layers, act_funs)
                }
            };

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
                    if let Some(file) = save.clone() {
                        best.nn.save(&file).expect("Failed to save model");
                    }
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
