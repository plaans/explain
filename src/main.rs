use anyhow::*;
use aries_planning::classical::search::{plan_search, Cfg};

use aries_planning::classical::{from_chronicles, grounded_problem};
use aries_planning::parsing::pddl_to_chronicles;
use explain::explain::cause::*;
use explain::explain::state2::*;
use explain::explain::explain::*;
use explain::explain::question::*;

use std::fs::File;
use std::io; /*::{Write, BufReader, BufRead, Error,stdin};*/
use std::io::Write;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use aries_utils::input::Input;

/*fn main() {
    println!("Hello, world!");
}*/

#[derive(Debug, StructOpt)]
#[structopt(name = "explain")]
struct Opt {
    /// If not set, `explain` will look for a `domain.pddl` file in the directory of the
    /// problem file or in the parent directory.
    #[structopt(long, short)]
    domain: Option<String>,
    problem: String,
    plan: String,

    ///Generate dot file for support
    #[structopt(short = "s", long = "support")]
    support: bool,

    ///Generate dot file for threat
    #[structopt(short = "m", long = "threat")]
    menace: bool,

    ///Generate dot file for temporal representation
    #[structopt(short = "t", long = "temp")]
    temp: bool,

    ///Ask question
    #[structopt(short = "q", long = "question", default_value = "0")]
    question: String,

    ///display plan
    #[structopt(short = "p", long = "plan")]
    affiche: bool,

    ///Interactive mode
    #[structopt(short = "i", long = "interact")]
    interact: bool,
}

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    let _start_time = std::time::Instant::now();

    let config = Cfg::default();
    // config.h_weight = opt.h_weight;
    //config.use_lookahead = !opt.no_lookahead;

    let problem_file = Path::new(&opt.problem);
    ensure!(
        problem_file.exists(),
        "Problem file {} does not exist",
        problem_file.display()
    );

    let problem_file = problem_file.canonicalize().unwrap();

    let plan_file = Path::new(&opt.plan);
    ensure!(
        plan_file.exists(),
        "plan file {} does not exist",
        plan_file.display()
    );

    let plan_file = plan_file.canonicalize().unwrap();

    let domain_file = match opt.domain {
        Some(name) => PathBuf::from(&name),
        None => {
            aries_planning::parsing::pddl::find_domain_of(&problem_file)?
        }
    };

    //Récupération des options
    let menace = opt.menace;
    let support = opt.support;
    let temp = opt.temp;
    let question = opt.question;
    let interact = opt.interact;
    let affiche = opt.affiche;

    //transformation de pddl
    let dom = Input::from_file(&domain_file)?;

    let prob = Input::from_file(&problem_file)?;

    let plan_string = std::fs::read_to_string(plan_file)?;

    let dom = aries_planning::parsing::pddl::parse_pddl_domain(dom)?;
    let prob = aries_planning::parsing::pddl::parse_pddl_problem(prob)?;
    let spec = pddl_to_chronicles(&dom, &prob)?;

    let lifted = from_chronicles(&spec)?;

    let grounded = grounded_problem(&lifted)?;

    let symbols = &lifted.world.table;

    println!("parse the plan");
    //parse fichier plan
    let mut plan = Vec::new();
    let lines = plan_string.lines();

    for c in lines.clone() {
        for op in grounded.operators.iter() {
            let a = symbols.format(grounded.operators.name(op));
            if a == c {
                plan.push(op);
            }
        }
    }

    println!("research support");

    //Traitement
    let mut mat = matricesupport3(&plan, &grounded);
    let mut matm = matricemenace2(&plan, &grounded);
/*
    println!("Essai groupe");
    let groupe= abstractionop(&mat,&plan,&grounded);
    if groupe.len()>=1{
        for i in &groupe{
            println!("groupe:");
            for &op in i{
                print!("{:?}, {} ",op, symbols.format(grounded.operators.name(op)))
            }
            println!("");
        }
        let g2 =abstractionregroupop(&groupe);
        if g2.len()>=1{
            for i in g2{
                println!("regroupe:");
                for op in i{
                    print!("{:?}, {} ",op, symbols.format(grounded.operators.name(op)))
                }
                println!("");
            }
        }
    }
*/
    

    //Non interactif
    if affiche {
        println!("Got plan: {} actions", plan.len());
        println!("=============");
        let mut count = 0;
        for &op in &plan {
            println!("{}:{}", count, symbols.format(grounded.operators.name(op)));
            count = count + 1;
        }
        println!("");
    }
    if menace {
        println!("file graphique.dot created for support relations");
        fichierdotmenacemat(&matm, &plan, &grounded, &lifted.world);
    }
    if support {
        println!("file graphiquemenace2.dot created for threat relations");
        fichierdotmat(&mat, &plan, &grounded, &lifted.world);
    }
    if temp {
        println!("file graphiquetemp.dot created");
        fichierdottempmat2(&mat, &matm, &plan, &grounded, &lifted.world);
    }

    let mut decompoquestion = Vec::new();

    if question != "0" {
        for i in question.rsplit(' ') {
            decompoquestion.insert(0, i);
        }
        choixquestionsmultiple(
            &decompoquestion,
            &mat,
            &matm,
            &plan,
            &grounded,
            &lifted.world,
            &symbols,
        );
    }

    let mut rien = false;

    if !support & !menace & !temp & !affiche {
        rien = true;
    }
    //Interactif
    if interact | rien {
        let mut bool = true;
        help();
        while bool {
            println!("What do you want to do?");
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let mut decompo = Vec::new();
            println!("-----Response------ \n");
            for index in guess.split_whitespace() {
                decompo.push(index);
            }

            let cmd = decompo[0];

            match cmd {
                "s" | "support-graph" => {
                    fichierdotmat(&mat, &plan, &grounded, &lifted.world);
                    println!("File graphique.dot rewrited for support relations  ");
                    affichagematrice(&mat);
                }
                "m" | "threat-graph" => {
                    fichierdotmenacemat(&matm, &plan, &grounded, &lifted.world);
                    println!("file graphiquemenace2.dot rewrited for threat relations");
                    affichagematrice(&matm);
                }
                "question" => {
                    //let q=decompo[1];
                    //if decompo.is_empty()
                    decompo.remove(0);
                    //choixquestions(&decompo, &mat, &matm, &plan, &grounded, &lifted.world, &symbols);
                    choixquestionsmultiple(
                        &decompo,
                        &mat,
                        &matm,
                        &plan,
                        &grounded,
                        &lifted.world,
                        &symbols,
                    );
                }
                "gg" => {
                    let search_result = plan_search(
                        &grounded.initial_state,
                        &grounded.operators,
                        &grounded.goals,
                        &config,
                    );
                    let _result = match search_result {
                        Some(plan2) => {
                            println!("Got plan: {} actions", plan2.len());
                            println!("=============");

                            let path = "./plan";
                            let mut output =
                                File::create(path).expect("Something went wrong reading the file");

                            for &op in &plan2 {
                                write!(output, "{}\n", symbols.format(grounded.operators.name(op)))
                                    .expect("Something went wrong writing the file");
                                println!("{}", symbols.format(grounded.operators.name(op)));
                            }
                            mat = matricesupport2(&plan2, &grounded);
                            matm = matricemenace2(&plan2, &grounded);
                            plan = plan2;
                        }
                        None => {
                            println!("Got plan");
                        }
                    };
                }
                "p" | "plan" => {
                    println!("Got plan: {} actions", plan.len());
                    println!("=============");
                    let mut count = 0;
                    for &op in &plan {
                        println!("{}:{}", count, symbols.format(grounded.operators.name(op)));
                        count = count + 1;
                    }
                    println!("");
                }
                "h" | "help" => {
                    help();
                }
                "q" | "e" | "exit" => bool = false,
                _ =>{
                    let sq=selectionquestion(cmd);
                    if sq == Question::Qundefined{
                        println!("Not an available entry {}", cmd)
                    }
                    else{
                        choixquestionsmultiple(
                        &decompo,
                        &mat,
                        &matm,
                        &plan,
                        &grounded,
                        &lifted.world,
                        &symbols,
                        );
                    }  
                },
            }
            println!("\n=====End of the interaction=======");
        }
        println!("");
    }
    println!("End of the command");
    Ok(())
}
