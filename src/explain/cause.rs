//use crate::classical::heuristics::*;
use crate::explain::explain::*;
use crate::explain::state2::*;
use aries_planning::classical::state::*;
use aries_planning::classical::GroundProblem;

//ajout pour gerer fichier
use std::fs::File;
use std::io::{Write};

//matrice facilite Dijktstra
use nalgebra::base::*;

pub fn causalite2(
    etape: i32,
    plan: &Vec<Op>,
    initial_state: &State,
    ops: &Operators,
    histo: &Vec<Resume>,
) -> (State, Vec<Resume>, Vec<Resume>) /*etat obtenu,histogramme modifié , support*/ {
    //initialisation
    let num = etape as usize;
    let opt = plan.get(num);
    let op = opt.unwrap();
    let _res = newresume(*op, etape);
    let etat = initial_state.clone();
    //liste des variables utilisé dans la précond de op
    let mut vecvar = Vec::new();
    //vecteur qui contiendra les resume ayant un lien avec l'op choisis
    let mut link = Vec::new();
    //Sélection des variable utilisé dans les préconditions
    let precond = ops.preconditions(*op);
    let mut count2 = 0;
    for var in etat.literals() {
        for pre in precond {
            if var.var() == pre.var() {
                vecvar.push(count2);
            }
        }
        count2 = count2 + 1;
    }
    //liaison opérateur grâce à histogramme et précondition opé
    for variableutilise in vecvar {
        let resume = histo.get(variableutilise).clone();
        //let resum=resume.unwrap();
        link.push(*resume.unwrap());
    }
    let h = histo.clone();
    let (e1, h2) = h_step(&etat, op, ops, etape, h);
    (e1, h2, link)
}

pub fn causalitegoals2(
    _plan: &Vec<Op>,
    initial_state: &State,
    _ops: &Operators,
    histo: &Vec<Resume>,
    goals: &Vec<Lit>,
) -> Vec<Resume> {
    //initialisation;
    let etat = initial_state.clone();
    //liste des variables utilisé dans la précond de op
    let mut vecvar = Vec::new();

    //vecteur qui contiendra les resume ayant un lien avec l'op choisis
    let mut link = Vec::new();

    //Sélection des variable utilisé dans les préconditions
    //let precond = ops.preconditions(*op);
    let mut count2 = 0;
    for var in etat.literals() {
        for pre in goals {
            if var.var() == pre.var() {
                //print!("{} estetetetete ,",count2);
                vecvar.push(count2);
            }
        }
        count2 = count2 + 1;
    }

    //liaison opérateur grâce à histogramme et précondition opé
    ///////
    //Link pas bon
    //////
    for variableutilise in vecvar {
        let resume = histo.get(variableutilise).clone();
        //let resum=resume.unwrap();
        link.push(*resume.unwrap());
    }
    link
}

//creer le fichier dot des liens causaux
pub fn fichierdot2(plan: &Vec<Op>, ground: &GroundProblem, symbol: &World) {
    //fichier de sortie
    let path = "graphique.dot";

    let mut output = File::create(path).expect("Something went wrong reading the file");

    write!(output, "digraph D {{ \n").expect("Something went wrong writing the file");

    //initialisation
    //let plan2 =plan.clone();
    //let plan3 =plan.clone();
    let mut strcause;// = String::new();

    //boucle faire lien causaux de chaque opé plan
    let mut count = 0; //pour suivre etape
    let mut e = ground.initial_state.clone();
    let mut h: Vec<Resume> = Vec::new(); //faire init h
    for _var in ground.initial_state.literals() {
        let res = defaultresume();
        h.push(res);
    }
    //let mut cause : Vec<Resume>=Vec::new();
    for _etape in plan {
        //let plan2 =plan3.clone();
        //faire cause
        let (e1, h2, cause) = causalite2(count, plan, &e, &ground.operators, &h);
        comparehisto(&h, &h2);
        h = h2;
        e = e1.clone();
        let op = plan.get(count as usize).unwrap();
        let opname = ground.operators.name(*op);

        //inscription dans fichier

        for res in cause {
            match res.op() {
                None => strcause = " i ".to_string(),
                Some(_resume) => {
                    strcause = format!(
                        "{}:{}",
                        res.numero(),
                        symbol
                            .table
                            .format(ground.operators.name(res.op().unwrap()))
                    )
                }
                //_ => (),
            }
            let stri = format!(
                "\"{}\" -> \"{}:{}\";\n",
                strcause,
                count,
                symbol.table.format(opname)
            );
            write!(output, "{}", stri).expect("Something went wrong writing the file");
        }
        count = count + 1;
    }
    let fin = causalitegoals2(
        plan,
        &ground.initial_state,
        &ground.operators,
        &h,
        &ground.goals,
    );
    for res in fin {
        match res.op() {
            None => strcause = " i ".to_string(),
            Some(_resume) => {
                strcause = format!(
                    "{}:{}",
                    res.numero(),
                    symbol
                        .table
                        .format(ground.operators.name(res.op().unwrap()))
                )
            }
        }
        let stri = format!("\"{}\" -> goals;\n", strcause);
        write!(output, "{}", stri).expect("Something went wrong writing the file");
    }
    write!(output, "}} ").expect("Something went wrong writing the file");
}

pub fn fichierdotmat(
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    symbol: &World,
) {
    //fichier de sortie
    let path = "graphique.dot";

    let mut output = File::create(path).expect("Something went wrong reading the file");

    write!(output, "digraph D {{ \n").expect("Something went wrong writing the file");

    //initialisation
    //let plan2 =plan.clone();
    //let plan3 =plan.clone();
    let mut strcause ;//= String::new();

    //boucle faire lien causaux de chaque opé plan
    //let mut count = 0;//pour suivre etape
    //let e,h,cause;
    let t = plan.len();
    let row = support.nrows();
    let col = support.ncols();

    for r in 0..row {
        for c in 0..col {
            if support[(r, c)] == 1 {
                if r == t {
                    strcause = " Goal ".to_string();
                    if c == t {
                        let _stri = format!("\"{}\" -> \" Goal \";\n", strcause);
                    } else if c == t + 1 {
                        let _stri = format!("\"{}\" -> \" i \"\n", strcause);
                    } else {
                        let stri = format!(
                            "\"{}\" -> \"{}:{}\";\n",
                            strcause,
                            c,
                            symbol
                                .table
                                .format(ground.operators.name(*plan.get(c).unwrap()))
                        );
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    }
                } else if r == t + 1 {
                    strcause = " i ".to_string();
                    if c == t {
                        let stri = format!("\"{}\" -> \" Goal \";\n", strcause);
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    } else if c == t + 1 {
                        let stri = format!("\"{}\" -> \" i \"\n", strcause);
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    } else {
                        let stri = format!(
                            "\"{}\" -> \"{}:{}\";\n",
                            strcause,
                            c,
                            symbol
                                .table
                                .format(ground.operators.name(*plan.get(c).unwrap()))
                        );
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    }
                } else {
                    strcause = format!(
                        "{}:{}",
                        r,
                        symbol
                            .table
                            .format(ground.operators.name(*plan.get(r).unwrap()))
                    );
                    if c == t {
                        let stri = format!("\"{}\" -> \" Goal \";\n", strcause);
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    } else if c == t + 1 {
                        let stri = format!("\"{}\" -> \" i \"\n", strcause);
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    } else {
                        let stri = format!(
                            "\"{}\" -> \"{}:{}\";\n",
                            strcause,
                            c,
                            symbol
                                .table
                                .format(ground.operators.name(*plan.get(c).unwrap()))
                        );
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    }
                }
            }
        }
    }
    write!(output, "}} ").expect("Something went wrong writing the file");
}

pub fn matricesupport2(plan: &Vec<Op>, ground: &GroundProblem) -> DMatrix<i32> {
    let _init = &ground.initial_state;
    let _ops = &ground.operators;
    let _goals = &ground.goals;
    let length = plan.len();
    let _l2 = length as u32;

    let mut matrice = DMatrix::from_diagonal_element(length + 2, length + 2, 0);

    let mut count = 0;
    let mut e = ground.initial_state.clone();
    let mut h: Vec<Resume> = Vec::new();
    for _var in ground.initial_state.literals() {
        let res = defaultresume();
        h.push(res);
    }
    //let mut cause : Vec<Resume>=Vec::new();
    for _i in plan {
        let (e1, h2, cause) =
            causalite2(count, /*&*/ plan /*2*/, &e, &ground.operators, &h);
        //println!("c {}, {},",cause.is_empty(),cause.len());
        for r in &cause {
            //println!("cause {}",r.numero());
            if r.numero() >= 0 {
                // println!("r.num");
                let r = r.numero() as usize;
                let c = count as usize;
                matrice[(r, c)] = 1;
            }
            //prise en compte de l'état initial, pour le calcul de centralité notamment
            else if r.numero() == (-1) {
                let row = length + 1 as usize;
                let c = count as usize;
                matrice[(row, c)] = 1;
            }
        }
        count = count + 1;
        h = h2;
        e = e1;
    }
    let cause = causalitegoals2(plan, &e, &ground.operators, &h, &ground.goals);
    for r in &cause {
        if r.numero() >= 0 {
            let r = r.numero() as usize;
            let c = count as usize;
            matrice[(r, c)] = 1;
        }
        //prise en compte de l'état initial, pour le calcul de centralité notamment
        else if r.numero() == (-1) {
            let row = length + 1 as usize;
            let c = count as usize;
            matrice[(row, c)] = 1;
        }
    }

    matrice
}

pub fn matricemenace2(plan: &Vec<Op>, ground: &GroundProblem) -> DMatrix<i32> {
    let ops = &ground.operators;
    let length = plan.len();
    //let l2=length as u32;
    let plan1 = plan.clone();

    let plan3 = plan.clone();
    let mut matrice = DMatrix::from_diagonal_element(length + 1, length + 1, 0);
    //matrice arc lien causaux goal
    let mut e = ground.initial_state.clone();
    let mut h: Vec<Resume> = Vec::new();
    for _var in ground.initial_state.literals() {
        let res = defaultresume();
        h.push(res);
    }
    let mut cause: Vec<Vec<Resume>> = Vec::new();
    let mut step = 0 as i32;
    for _i in plan1 {
        //let plan2=plan.clone();
        //let e = causalite(step,plan2,&ground.initial_state,ops);
        let (e1, h2, c) = causalite2(step, plan, &e, &ground.operators, &h);
        cause.push(c);
        e = e1;
        h = h2;
        step = step + 1;
    }
    let c = causalitegoals2(plan, &e, &ground.operators, &h, &ground.goals);
    cause.push(c);

    let plan2 = plan.clone();
    let mut count = 0;
    for i in plan2 {
        let mut count2 = 0;
        for j in &plan3 {
            if count != count2 {
                //Vec de resume
                let support = cause.get(count);
                let support2 = cause.get(count2);
                let mut supportbool = true;
                //println!("support {}",support.len());
                for su in support {
                    //println!("su len :{}",su.len());
                    for s in su {
                        //println!("s resume {}",s.numero());
                        if s.op().is_none() == false {
                            if *j == s.op().unwrap() {
                                supportbool = false;
                            }
                        }
                    }
                }
                for su in support2 {
                    for s in su {
                        if s.op().is_none() == false {
                            if i == s.op().unwrap() {
                                supportbool = false;
                            }
                        }
                    }
                }
                if supportbool {
                    let precon = ops.preconditions(i);
                    let effet = ops.effects(*j);
                    for pre in precon {
                        for eff in effet {
                            if pre.var() == eff.var() {
                                if count2 > count {
                                    matrice[(count2, count)] = 1; //1 on place i après j
                                } else {
                                    for su in support {
                                        for s in su {
                                            if !s.op().is_none() {
                                                let effs = ops.effects(s.op().unwrap());
                                                for f in effs {
                                                    if eff.var() == f.var() {
                                                        let ot = s.numero() as usize;
                                                        //changt aberration 14 11 block
                                                        if count2 < ot {
                                                            matrice[(count2, ot)] = -1; //-1 on place i avant j
                                                            matrice[(count2, count)] = -2;
                                                        } else {
                                                            matrice[(count2, count)] = -1;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            count2 = count2 + 1;
        }
        count = count + 1;
    }
    matrice
}

pub fn fichierdottempmat(
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    symbol: &World,
) {
    //fichier de sortie
    let path = "graphiquetemp.dot";
    let mut output = File::create(path).expect("Something went wrong reading the file");

    write!(output, "digraph D {{ \n").expect("Something went wrong writing the file");

    //initialisation
    let plan3 = plan.clone();
    let mut strcause ;//= String::new();

    //boucle faire lien causaux de chaque opé plan
    let t = plan.len();
    let row = support.nrows();
    let col = support.ncols();

    for r in 0..row {
        for c in 0..col {
            if support[(r, c)] == 1 {
                if r == t {
                    strcause = " Goal ".to_string();
                    if c == t {
                        let _stri = format!("\"{}\" -> \" Goal \";\n", strcause);
                    } else if c == t + 1 {
                        let _stri = format!("\"{}\" -> \" i \"\n", strcause);
                    } else {
                        let stri = format!(
                            "\"{}\" -> \"{}:{}\";\n",
                            strcause,
                            c,
                            symbol
                                .table
                                .format(ground.operators.name(*plan.get(c).unwrap()))
                        );
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    }
                } else if r == t + 1 {
                    strcause = " i ".to_string();
                    if c == t {
                        let stri = format!("\"{}\" -> \" Goal \";\n", strcause);
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    } else if c == t + 1 {
                        let stri = format!("\"{}\" -> \" i \"\n", strcause);
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    } else {
                        let stri = format!(
                            "\"{}\" -> \"{}:{}\";\n",
                            strcause,
                            c,
                            symbol
                                .table
                                .format(ground.operators.name(*plan.get(c).unwrap()))
                        );
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    }
                } else {
                    strcause = format!(
                        "{}:{}",
                        r,
                        symbol
                            .table
                            .format(ground.operators.name(*plan.get(r).unwrap()))
                    );
                    if c == t {
                        let stri = format!("\"{}\" -> \" Goal \";\n", strcause);
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    } else if c == t + 1 {
                        let stri = format!("\"{}\" -> \" i \"\n", strcause);
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    } else {
                        let stri = format!(
                            "\"{}\" -> \"{}:{}\";\n",
                            strcause,
                            c,
                            symbol
                                .table
                                .format(ground.operators.name(*plan.get(c).unwrap()))
                        );
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    }
                }
            }
        }
    }
    //pour les goals
    let plan2 = plan3.clone();

    write!(output, "edge [color=red];\n").expect("Something went wrong writing the file");

    let temp = inversibilite(plan2, ground);
    for t in temp {
        let (op1, op2) = t.operateur();
        let (num1, num2) = t.etape();
        let opname1 = ground.operators.name(op1);
        let opname = ground.operators.name(op2);
        let stri = format!(
            "\"{}:{}\" -> \"{}:{}\";\n",
            num1,
            symbol.table.format(opname1),
            num2,
            symbol.table.format(opname)
        );
        write!(output, "{}", stri).expect("Something went wrong writing the file");
    }

    write!(output, "}} ").expect("Something went wrong writing the file");
}

pub fn fichierdotmenacemat(
    mat: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    symbol: &World,
) {
    //fichier de sortie
    let path = "graphiquemenace2.dot";

    let mut output = File::create(path).expect("Something went wrong reading the file");

    write!(output, "digraph D {{ \n").expect("Something went wrong writing the file");

    //initialisation

    let t = plan.len();
    let row = mat.nrows();
    let col = mat.ncols();

    for r in 0..row {
        for c in 0..col {
            if mat[(r, c)] == 1 {
                if c == t {
                    println!("on ne menace pas les buts: erreur taille de la matrice");
                } else if c == t + 1 {
                    println!("erreur taille de la matrice en {}{}", r, c);
                } else {
                    let namer = ground.operators.name(*plan.get(r).unwrap());
                    let namec = ground.operators.name(*plan.get(c).unwrap());
                    write!(output, "edge [color=blue];\n")
                        .expect("Something went wrong writing the file");
                    let stri = format!(
                        "\"{}:{}\" -> \"{}:{}\";\n",
                        r,
                        symbol.table.format(namer),
                        c,
                        symbol.table.format(namec)
                    );
                    write!(output, "{}", stri).expect("Something went wrong writing the file");
                }
            } else if mat[(r, c)] == -1 {
                if c == t {
                    println!("on ne menace pas les buts: erreur taille de la matrice");
                } else if c == t + 1 {
                    println!("erreur taille de la matrice en {}{}", r, c);
                } else {
                    let namer = ground.operators.name(*plan.get(r).unwrap());
                    let namec = ground.operators.name(*plan.get(c).unwrap());
                    write!(output, "edge [color=red];\n")
                        .expect("Something went wrong writing the file");
                    let stri = format!(
                        "\"{}:{}\" -> \"{}:{}\";\n",
                        r,
                        symbol.table.format(namer),
                        c,
                        symbol.table.format(namec)
                    );
                    write!(output, "{}", stri).expect("Something went wrong writing the file");
                }
            } else if mat[(r, c)] == -2 {
                if c == t {
                    println!("on ne menace pas les buts");
                } else if c == t + 1 {
                    println!("erreur taille de la matrice en {}{}", r, c);
                } else {
                    let namer = ground.operators.name(*plan.get(r).unwrap());
                    let namec = ground.operators.name(*plan.get(c).unwrap());
                    write!(output, "edge [color=yellow];\n")
                        .expect("Something went wrong writing the file");
                    let stri = format!(
                        "\"{}:{}\" -> \"{}:{}\";\n",
                        r,
                        symbol.table.format(namer),
                        c,
                        symbol.table.format(namec)
                    );
                    write!(output, "{}", stri).expect("Something went wrong writing the file");
                }
            } else if mat[(r, c)] != 0 {
                println!("erreur dans la matrice en {}{}", r, c);
            }
        }
    }

    write!(output, "}} ").expect("Something went wrong writing the file");
}

pub fn fichierdottempmat2(
    support: &DMatrix<i32>,
    menace: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    symbol: &World,
) {
    //fichier de sortie
    let path = "graphiquetemp.dot";
    let mut output = File::create(path).expect("Something went wrong reading the file");

    write!(output, "digraph D {{ \n").expect("Something went wrong writing the file");

    //initialisation
    //let plan3 =plan.clone();
    let mut strcause ;//= String::new();

    //boucle faire lien causaux de chaque opé plan
    let t = plan.len();
    let row = support.nrows();
    let col = support.ncols();

    for r in 0..row {
        for c in 0..col {
            if support[(r, c)] == 1 {
                if r == t {
                    strcause = " Goal ".to_string();
                    if c == t {
                        let _stri = format!("\"{}\" -> \" Goal \";\n", strcause);
                    } else if c == t + 1 {
                        let _stri = format!("\"{}\" -> \" i \"\n", strcause);
                    } else {
                        let stri = format!(
                            "\"{}\" -> \"{}:{}\";\n",
                            strcause,
                            c,
                            symbol
                                .table
                                .format(ground.operators.name(*plan.get(c).unwrap()))
                        );
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    }
                } else if r == t + 1 {
                    strcause = " i ".to_string();
                    if c == t {
                        let stri = format!("\"{}\" -> \" Goal \";\n", strcause);
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    } else if c == t + 1 {
                        let stri = format!("\"{}\" -> \" i \"\n", strcause);
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    } else {
                        let stri = format!(
                            "\"{}\" -> \"{}:{}\";\n",
                            strcause,
                            c,
                            symbol
                                .table
                                .format(ground.operators.name(*plan.get(c).unwrap()))
                        );
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    }
                } else {
                    strcause = format!(
                        "{}:{}",
                        r,
                        symbol
                            .table
                            .format(ground.operators.name(*plan.get(r).unwrap()))
                    );
                    if c == t {
                        let stri = format!("\"{}\" -> \" Goal \";\n", strcause);
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    } else if c == t + 1 {
                        let stri = format!("\"{}\" -> \" i \"\n", strcause);
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    } else {
                        let stri = format!(
                            "\"{}\" -> \"{}:{}\";\n",
                            strcause,
                            c,
                            symbol
                                .table
                                .format(ground.operators.name(*plan.get(c).unwrap()))
                        );
                        write!(output, "{}", stri).expect("Something went wrong writing the file");
                    }
                }
            }
        }
    }

    write!(output, "edge [color=green];\n").expect("Something went wrong writing the file");

    for inv in 0..t - 1 {
        if menace[(inv + 1, inv)] != 0 {
            //print!("{}, ",inv);
            let opname1 = ground.operators.name(*plan.get(inv).unwrap());
            let opname = ground.operators.name(*plan.get(inv + 1).unwrap());
            let stri = format!(
                "\"{}:{}\" -> \"{}:{}\";\n",
                inv,
                symbol.table.format(opname1),
                inv + 1,
                symbol.table.format(opname)
            );
            write!(output, "{}", stri).expect("Something went wrong writing the file");
        }
    }
    /*let stri=format!("\" i \" -> \" Goal \";\n" );
    write!(output,"{}" ,stri)
        .expect("Something went wrong writing the file");*/

    write!(output, "}} ").expect("Something went wrong writing the file");
}

pub fn matricesupport3(plan: &[Op], ground: &GroundProblem) -> DMatrix<i32> {
    let ops = &ground.operators;
    let length = plan.len();

    let mut matrice = DMatrix::from_diagonal_element(length + 2, length + 2, 0);

    let goal_state_id = plan.len();
    let init_state_id = plan.len() + 1;

    // for each state variable, the step in which it was changed
    let mut changed = vec![init_state_id; ground.initial_state.num_variables()];

    for (step, &op) in plan.iter().enumerate() {
        for cond in ops.preconditions(op) {
            let var_id: usize = cond.var().into();
            matrice[(changed[var_id], step)] = 1;
        }
        for eff in ops.effects(op) {
            let var_id: usize = eff.var().into();
            // record that the var was change at this step
            changed[var_id] = step;
        }
    }

    for goal in &ground.goals {
        let var_id: usize = goal.var().into();
        matrice[(changed[var_id], goal_state_id)] = 1;
    }

    matrice
}
