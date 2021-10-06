use crate::explain::explain::*;
use crate::explain::state2::*;
use aries_planning::classical::state::*;
use aries_planning::classical::GroundProblem;
use nalgebra::base::*;
use aries_model::symbols::SymbolTable;


//Quelles sont les supports de l’étape a?
pub fn supportedby(num: usize, support: &DMatrix<i32>, plan: &Vec<Op>) -> Vec<Resume> {
    let t = support.nrows();
    let mut out = Vec::new();
    for i in 0..t {
        if support[(i, num)] == 1 {
            if !plan.get(i).is_none() {
                let u = i as i32;
                let n = newresume(*plan.get(i).unwrap(), u);
                out.push(n);
            }
        }
    }
    out
}

pub fn affichageq1(
    num: usize,
    plan: &Vec<Op>,
    sup: Vec<Resume>,
    ground: &GroundProblem,
    symbol: &World,
) {
    //let i=num as i32;
    //let n=newresume(*plan.get(num).unwrap(),i);
    //println!("L'opérateur {} de l'étape {} est supporté par ",symbol.table.format(&ground.operators.name(*plan.get(num).unwrap())),num);
    println!(
        "{}:{} supported by :",
        num,
        symbol
            .table
            .format(ground.operators.name(*plan.get(num).unwrap()))
    );
    for i in sup {
        print!(
            "    {}:{}, ",
            i.numero(),
            symbol.table.format(ground.operators.name(i.op().unwrap()))
        );
    }
    println!("");
}

//Quelles sont les actions supportés par l’étape a?
pub fn supportof(num: usize, support: &DMatrix<i32>, plan: &Vec<Op>) -> Vec<Resume> {
    let t = support.nrows();
    let mut out = Vec::new();
    for i in 0..t {
        if support[(num, i)] == 1 {
            if !plan.get(i).is_none() {
                let u = i as i32;
                let n = newresume(*plan.get(i).unwrap(), u);
                out.push(n);
            }
        }
    }
    out
}

pub fn affichageq2(
    num: usize,
    plan: &Vec<Op>,
    sup: Vec<Resume>,
    ground: &GroundProblem,
    symbol: &World,
) {
    //let i = num as i32;
    //let n=newresume(*plan.get(num).unwrap(),i);
    println!(
        "{}:{} supports ",
        num,
        symbol
            .table
            .format(ground.operators.name(*plan.get(num).unwrap()))
    );
    for i in sup {
        println!(
            "  {}:{}, ",
            i.numero(),
            symbol.table.format(ground.operators.name(i.op().unwrap()))
        );
    }
    println!("");
}

//Est-ce que l’execution de a avant b peux gêner l’execution de b?
pub fn menacefromto(a: usize, b: usize, menace: &DMatrix<i32>) -> bool {
    let mut bo = true;
    if menace[(a, b)] == 0 {
        bo = false;
    }
    bo
}

pub fn affichageq3(
    a: usize,
    b: usize,
    m: bool,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    symbol: &World,
) {
    if m {
        println!(
            "{}:{} threatens {}:{} ",
            a,
            symbol
                .table
                .format(ground.operators.name(*plan.get(a).unwrap())),
            b,
            symbol
                .table
                .format(ground.operators.name(*plan.get(b).unwrap()))
        );
    } else {
        println!(
            " {}:{} doesn't threaten {}:{} ",
            a,
            symbol
                .table
                .format(ground.operators.name(*plan.get(a).unwrap())),
            b,
            symbol
                .table
                .format(ground.operators.name(*plan.get(b).unwrap()))
        );
    }
}

//Est-ce que cette étape est nécessaire? Participe-t-elle à l’accomplissement d’un but?
pub fn isnecessary(
    num: usize,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
) -> bool {
    let allnec = dijkstra2(support, plan.clone(), ground);
    let i = num as i32;
    let r = newresume(*plan.get(num).unwrap(), i);
    let mut nec = newnecess(r);
    for n in allnec {
        if n.opnec().numero() == i {
            nec = n;
        }
    }
    nec.nec()
}

pub fn isnecessarydetail(
    num: usize,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
) -> Option<Vec<Resume>> {
    let allnec = dijkstra2(support, plan.clone(), ground);
    let i = num as i32;
    let r = newresume(*plan.get(num).unwrap(), i);
    let mut nec = newnecess(r);
    for n in allnec {
        if n.opnec().numero() == i {
            nec = n;
        }
    }
    nec.chemin()
}

pub fn affichageq4(
    num: usize,
    b: bool,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    symbol: &World,
) {
    if b {
        println!(
            " {}:{} is necessary",
            num,
            symbol
                .table
                .format(ground.operators.name(*plan.get(num).unwrap()))
        );
    } else {
        println!(
            " {}:{} isn't necessary",
            num,
            symbol
                .table
                .format(ground.operators.name(*plan.get(num).unwrap()))
        );
    }
}
//a refaire sans nec et avec option Vec
/*pub fn affichageqd4 (n:Necessaire, ground: &GroundProblem ,symbol:&World    print!("L'operateur {} de ",symbol.table.format(ground.operators.name(n.opnec().op().unwrap())));
    n.affiche();
    println!("");
}*/

pub fn affichageqd4(
    num: usize,
    chemin: Option<Vec<Resume>>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    symbol: &World,
) {
    print!(
        "{}:{} ",
        num,
        symbol
            .table
            .format(ground.operators.name(*plan.get(num).unwrap()))
    );
    if chemin.is_none() {
        println!("isn't necessary");
    } else {
        //println!("est necessaire pour notamment le chemin accomplissant un but composé par :");
        println!("is necessary to the path for  :");
        for op in chemin.unwrap() {
            println!(
                " {}:{}",
                op.numero(),
                symbol
                    .table
                    .format(ground.operators.name(op.op().unwrap()))
            );
        }
    }
    println!("");
}

//Existe-t-il un chemin entre a et b?
pub fn waybetweenbool(a: usize, b: usize, support: &DMatrix<i32>, plan: &Vec<Op>) -> bool {
    /*let step1= a as i32;
    let step2 = b as i32;
    let mut nec;
    if step1 > step2 {
        nec=explicationsupport(plan, support, step1, step2);
    }else{
        nec=explicationsupport(plan, support,  step2, step1);
    };
    nec.nec()*/
    waybetween(a, b, support, plan).is_some()
}

pub fn waybetween(
    a: usize,
    b: usize,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
) -> Option<Vec<Resume>> {
    let step1 = a as i32;
    let step2 = b as i32;
    let nec;
    if step1 > step2 {
        nec = explicationsupport(plan, support, step1, step2);
    } else {
        nec = explicationsupport(plan, support, step2, step1);
    }
    nec.chemin()
}

pub fn affichageq5(
    a: usize,
    b: usize,
    bo: bool,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    symbol: &World,
) {
    if bo {
        println!(
            " {}:{} and {}:{} are linked in support graph",
            a,
            symbol
                .table
                .format(ground.operators.name(*plan.get(a).unwrap())),
            b,
            symbol
                .table
                .format(ground.operators.name(*plan.get(b).unwrap()))
        );
    } else {
        println!(
            " {}:{} and {}:{} aren't linked in support graph",
            a,
            symbol
                .table
                .format(ground.operators.name(*plan.get(a).unwrap())),
            b,
            symbol
                .table
                .format(ground.operators.name(*plan.get(b).unwrap()))
        );
    }
}

//a refaire sans nec et avec option Vec
/*pub fn affichageqd5  (n:Necessaire, ground:&GroundProblem,symbol:&World    print!("L'operateur {} de ",symbol.table.format(ground.operators.name(n.opnec().op().unwrap())));
    n.affiche();
    println!("");
}*/
pub fn affichageqd5(
    n: &Option<Vec<Resume>>,
    ground: &GroundProblem,
    symbol: &World,
) {
    let a = n.clone();
    println!("The path contains :");
    if a.is_some() {
        for i in a.unwrap() {
            println!(
                "  {}:{} ,",
                symbol.table.format(ground.operators.name(i.op().unwrap())),
                i.numero()
            );
        }
    } else {
        println!("Nothing, it doesn't exist");
    }
}

//Est-ce que les étapes a et b sont parallélisable? privilege support
pub fn parallelisablebool(
    a: usize,
    b: usize,
    support: &DMatrix<i32>,
    menace: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
) -> Parallelisable {
    /*let mut p: Parallelisable = Parallelisable::Oui;
    let ai = a as i32;
    let bi = b as i32;
    if a > b {
        let nec=explicationsupport(plan, support, ai, bi);
        if nec.nec(){p= Parallelisable::NonSupport{origine:a,vers:b};}
    }else{
        let nec=explicationsupport(plan, support, bi, ai);
        if nec.nec(){p= Parallelisable::NonSupport{origine:b,vers:a};}
    }
    if p == Parallelisable::Oui{
        let m =explicationmenacequestion(plan, menace, support, ai, bi);
        if m{
          p=Parallelisable::NonMenace{origine:a,vers:b};
        }
        let m=explicationmenacequestion(plan, menace, support,  bi, ai);
        if m{
            p=Parallelisable::NonMenace{origine:b,vers:a};
        }
    }
    p*/
    let qd = parallelisable(a, b, support, menace, plan, ground);
    /*if qd == Parallelisabledetail::Oui{ return Parallelisable::Oui}
    else if qd == Parallelisabledetail::SupportDirect{origine : a,vers:b} || qd==Parallelisabledetail::SupportIndirect{origine:a,vers:b,chemin}{
        return Parallelisable::NonSupport{origine:a,vers:b}
    }
    else if qd == Parallelisabledetail::SupportDirect{origine : b,vers:a} || qd==Parallelisabledetail::SupportIndirect{origine:b,vers:a,chemin}{
        return Parallelisable::NonSupport{origine:b,vers:a}
    }
    else{return Parallelisable::NonMenace{origine:a,vers:b}}*/
    match qd {
        Parallelisabledetail::MenaceAvant {
            origine,
            vers,
            supportconcern: _,
        } => return Parallelisable::NonMenace { origine, vers },
        Parallelisabledetail::MenaceApres { origine, vers } => {
            return Parallelisable::NonMenace { origine, vers }
        }
        Parallelisabledetail::SupportDirect { origine, vers } => {
            return Parallelisable::NonSupport { origine, vers }
        }
        Parallelisabledetail::SupportIndirect {
            origine,
            vers,
            chemin: _,
        } => return Parallelisable::NonSupport { origine, vers },
        Parallelisabledetail::Oui => return Parallelisable::Oui,
    }
}

pub fn parallelisable(
    a: usize,
    b: usize,
    support: &DMatrix<i32>,
    menace: &DMatrix<i32>,
    plan: &Vec<Op>,
    _ground: &GroundProblem,
) -> Parallelisabledetail {
    let mut p = Parallelisabledetail::Oui;
    let ai = a as i32;
    let bi = b as i32;
    if a > b {
        if support[(b, a)] == 1 {
            p = Parallelisabledetail::SupportDirect {
                origine: b,
                vers: a,
            };
        } else {
            let nec = explicationsupport(plan, support, ai, bi);
            if nec.nec() {
                p = Parallelisabledetail::SupportIndirect {
                    origine: a,
                    vers: b,
                    chemin: nec.chemin(),
                };
            }
        }
    } else {
        if support[(a, b)] == 1 {
            p = Parallelisabledetail::SupportDirect {
                origine: a,
                vers: b,
            };
        } else {
            let nec = explicationsupport(plan, support, bi, ai);
            if nec.nec() {
                p = Parallelisabledetail::SupportIndirect {
                    origine: a,
                    vers: b,
                    chemin: nec.chemin(),
                };
            }
        }
    }
    if p == Parallelisabledetail::Oui {
        let opt = explicationmenacequestiondetail(plan, menace, support, ai, bi);
        if opt.is_some() {
            let (s1, s2, i) = opt.unwrap();
            if i.is_some() {
                p = Parallelisabledetail::MenaceAvant {
                    origine: s1,
                    vers: s2,
                    supportconcern: i,
                }
            } else {
                p = Parallelisabledetail::MenaceApres {
                    origine: s1,
                    vers: s2,
                };
            }
        }

        let opt = explicationmenacequestiondetail(plan, menace, support, bi, ai);
        if opt.is_some() {
            let (s1, s2, i) = opt.unwrap();
            if i.is_some() {
                p = Parallelisabledetail::MenaceAvant {
                    origine: s1,
                    vers: s2,
                    supportconcern: i,
                }
            } else {
                p = Parallelisabledetail::MenaceApres {
                    origine: s1,
                    vers: s2,
                };
            }
        }
    }
    p
}

pub fn affichageq6(p: Parallelisable) {
    match p {
        Parallelisable::Oui => println!("are parallelizable "),
        Parallelisable::NonMenace { origine: _, vers: _ } => {
            println!(" aren't parallelizable because of the existence of a threat ")
        }
        Parallelisable::NonSupport { origine: _, vers: _ } => {
            println!("aren't parallelizable because of a support relation ")
        }
    }
}

pub fn affichageqd6(p: Parallelisabledetail) {
    match p {
        Parallelisabledetail::Oui => println!("are parallelizable"),
        Parallelisabledetail::SupportDirect { origine: _, vers: _ } => {
            println!("aren't parallelizable because of a direct support relation")
        }
        Parallelisabledetail::SupportIndirect {
            origine: _,
            vers: _,
            chemin: _,
        } => println!("aren't parallelizable because of an indirect support relation "),
        Parallelisabledetail::MenaceApres { origine: _, vers: _ } => println!("aren't parallelizable  because the most recent step threathen the older step"),
        Parallelisabledetail::MenaceAvant {
            origine: _,
            vers: _,
            supportconcern: _,
        } => {
            println!("aren't parallelizable because the older step threaten the most recent step ")
        }
    }
}

//L’action accomplit-elle directement un goal?
pub fn achievegoal(num: usize, support: &DMatrix<i32>) -> bool {
    let t = support.nrows();
    let mut g = false;
    if support[(num, t - 2)] == 1 {
        g = true;
    }
    g
}

pub fn affichageq7(
    num: usize,
    b: bool,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    symbol: &World,
) {
    if b {
        println!(
            "{}:{} performs a goal ",
            num,
            symbol
                .table
                .format(ground.operators.name(*plan.get(num).unwrap()))
        );
    } else {
        println!(
            "{}:{} doesn't perform any goal",
            num,
            symbol
                .table
                .format(ground.operators.name(*plan.get(num).unwrap()))
        );
    }
}

pub fn researchsynchro(
    parametre: &Vec<String>,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    symbol: &SymbolTable,
) -> Vec<Resume> {
    let hash = coordination(parametre, plan, ground, symbol);
    let out = synchronisation(&hash, support, plan);
    out
}

pub fn affichageq8s(
    listesynchro: &Vec<Resume>,
    ground: &GroundProblem,
    symbol: &World,
) {
    for step in listesynchro {
        println!(
            " {}:{} is a synchronization point between 2 groups of plan actions ",
            step.numero(),
            symbol
                .table
                .format(ground.operators.name(step.op().unwrap()))
        );
    }
}

pub fn affichageq8b(
    listgoulot: Vec<(Resume, f32)>,
    ground: &GroundProblem,
    symbol: &World,
) {
    for step in listgoulot {
        //println!("L'opérateur {} de l'étape {} est un point de passage important du plan de score {} ",symbol.table.format(ground.operators.name(step.0.op().unwrap())),step.0.numero(),step.1);
        println!(
            "{}:{} is an important step in plan, his score is {} ",
            step.0.numero(),
            symbol
                .table
                .format(ground.operators.name(step.0.op().unwrap())),
            step.1
        );
    }
}

pub fn weightwaygoal(
    num: usize,
    exclusion: usize,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    poids: i32,
) -> bool {
    let exclu = choixpredaction2(exclusion, plan, ground);
    let necs = dijkstrapoids(plan, ground, support, &exclu, poids);
    let mut b = false;
    let n = num as i32;
    for i in necs {
        if i.opnec().numero() == n {
            b = true;
        }
    }
    b
}

pub fn weightwaygoal2(
    num: usize,
    action: String,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    wo: &SymbolTable,
    poids: i32,
) -> Option<Vec<Resume>> {
    let exclu = choixpredaction3(action, plan, ground, wo);
    let necs = dijkstrapoids(plan, ground, support, &exclu, poids);
    let mut out;
    let n = num as i32;
    let r = newresume(*plan.get(num).unwrap(), n);
    let nec = newnecess(r);
    out = nec.chemin();
    for i in necs {
        //        i.affiche();
        if i.opnec().numero() == n {
            out = i.chemin();
        }
    }
    out
}

pub fn weightway(
    step1: usize,
    step2: usize,
    action: String,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    wo: &SymbolTable,
    poids: i32,
) -> bool {
    let out = weightwaydetail(step1, step2, action, support, plan, ground, wo, poids);
    out.is_some()
}

pub fn weightwaydetail(
    step1: usize,
    step2: usize,
    action: String,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    wo: &SymbolTable,
    poids: i32,
) -> Option<Vec<Resume>> {
    let s1 = step1 as i32;
    let s2 = step2 as i32;
    let exclu = choixpredaction3(action, plan, ground, wo);
    let necs = supportindirectpoid(s1, s2, plan, ground, support, &exclu, poids);
    necs.chemin()
}

pub fn inverseweightway(
    step1: usize,
    step2: usize,
    action: String,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    wo: &SymbolTable,
    poids: i32,
) -> bool {
    let out = inverseweightwaydetail(step1, step2, action, support, plan, ground, wo, poids);
    out.is_some()
}

pub fn inverseweightwaydetail(
    step1: usize,
    step2: usize,
    action: String,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    wo: &SymbolTable,
    poids: i32,
) -> Option<Vec<Resume>> {
    let s1 = step1 as i32;
    let s2 = step2 as i32;
    let exclu = choixpredaction3(action, plan, ground, wo);
    let necs = supportindirectavantagepoid(s1, s2, plan, ground, support, &exclu, poids);
    necs.chemin()
}
//en utilisanst le num d'étapes
pub fn weightwayetape(
    step1: usize,
    step2: usize,
    step: usize,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    poids: i32,
) -> bool {
    let out = weightwaydetailetape(step1, step2, step, support, plan, ground, poids);
    out.is_some()
}

pub fn weightwaydetailetape(
    step1: usize,
    step2: usize,
    step: usize,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    poids: i32,
) -> Option<Vec<Resume>> {
    let s1 = step1 as i32;
    let s2 = step2 as i32;
    let exclu = choixpredaction2(step, plan, ground);
    let necs = supportindirectpoid(s1, s2, plan, ground, support, &exclu, poids);
    necs.chemin()
}

pub fn inverseweightwayetape(
    step1: usize,
    step2: usize,
    step: usize,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    poids: i32,
) -> bool {
    let out = inverseweightwaydetailetape(step1, step2, step, support, plan, ground, poids);
    out.is_some()
}

pub fn inverseweightwaydetailetape(
    step1: usize,
    step2: usize,
    step: usize,
    support: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    poids: i32,
) -> Option<Vec<Resume>> {
    let s1 = step1 as i32;
    let s2 = step2 as i32;
    let exclu = choixpredaction2(step, plan, ground);
    let necs = supportindirectavantagepoid(s1, s2, plan, ground, support, &exclu, poids);
    necs.chemin()
}

pub fn affichageq9d(
    chemin: &Option<Vec<Resume>>,
    ground: &GroundProblem,
    wo: &SymbolTable,
) {
    if chemin.is_some() {
        let n = chemin.clone();
        println!("Le chemin entre les 2 étapes est composé par :");
        for i in n {
            for step in i {
                println!(
                    "l'opérateur {} de l'étapes {} ",
                    wo.format(ground.operators.name(step.op().unwrap())),
                    step.numero()
                );
            }
        }
    } else {
        println!("les étapes aren't liés par une relation de support!");
    }
}

//Q9 paramètres
pub fn q9param(
    poids: i32,
    step1: i32,
    step2: i32,
    parametre: &Vec<String>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    symbol: &SymbolTable,
    support: &DMatrix<i32>
)->Option<Vec<Resume>>{
    let hash=coordination(parametre,plan,ground,symbol);
    let newsupport=poidsparametredesavantage(poids,support,&hash,plan,ground);
    affichagematrice(&newsupport);
    let nec=explicationsupport(plan,&newsupport,step1,step2);
    nec.chemin()
}


//
pub fn choixquestions(
    decompoquestion: &Vec<&str>,
    support: &DMatrix<i32>,
    menace: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    lifted: &World,
    symbol: &SymbolTable,
) {
    let q = decompoquestion[0];

    match q {
        "0" => println!(""),
        "1" => {
            let mystring = decompoquestion[1].to_string();
            let num = mystring.parse::<usize>().unwrap();
            let v = supportedby(num, support, plan);
            affichageq1(num, plan, v, ground, lifted);
            println!("");
        }
        "2" => {
            let mystring = decompoquestion[1].to_string();
            let num = mystring.parse::<usize>().unwrap();
            let v = supportof(num, support, plan);
            affichageq2(num, plan, v, ground, lifted);
            println!("");
        }
        "3" => {
            let mystring1 = decompoquestion[1].to_string();
            let num1 = mystring1.parse::<usize>().unwrap();
            let mystring2 = decompoquestion[2].to_string();
            let num2 = mystring2.parse::<usize>().unwrap();
            let v = menacefromto(num1, num2, menace);
            affichageq3(num1, num2, v, plan, ground, lifted);
            println!("");
        }
        "4" => {
            let mystring = decompoquestion[1].to_string();
            let num = mystring.parse::<usize>().unwrap();
            let v = isnecessary(num, support, plan, ground);
            affichageq4(num, v, plan, ground, lifted);
            println!("");
        }
        "4d" => {
            let mystring = decompoquestion[1].to_string();
            let num = mystring.parse::<usize>().unwrap();
            let v = isnecessarydetail(num, support, plan, ground);
            affichageqd4(num, v, plan, ground, lifted);
            println!("");
        }
        "5" => {
            let mystring1 = decompoquestion[1].to_string();
            let num1 = mystring1.parse::<usize>().unwrap();
            let mystring2 = decompoquestion[2].to_string();
            let num2 = mystring2.parse::<usize>().unwrap();
            let v = waybetweenbool(num1, num2, support, plan);
            affichageq5(num1, num2, v, plan, ground, lifted);
            println!("");
        }
        "5d" => {
            let mystring1 = decompoquestion[1].to_string();
            let num1 = mystring1.parse::<usize>().unwrap();
            let mystring2 = decompoquestion[2].to_string();
            let num2 = mystring2.parse::<usize>().unwrap();
            let v = waybetween(num1, num2, support, plan);
            affichageqd5(&v, ground, lifted);
            println!("");
        }
        "6" => {
            let mystring1 = decompoquestion[1].to_string();
            let num1 = mystring1.parse::<usize>().unwrap();
            let mystring2 = decompoquestion[2].to_string();
            let num2 = mystring2.parse::<usize>().unwrap();
            let v = parallelisablebool(num1, num2, support, menace, plan, ground);
            affichageq6(v);
            println!("");
        }
        "6d" => {
            let mystring1 = decompoquestion[1].to_string();
            let num1 = mystring1.parse::<usize>().unwrap();
            let mystring2 = decompoquestion[2].to_string();
            let num2 = mystring2.parse::<usize>().unwrap();
            let v = parallelisable(num1, num2, support, menace, plan, ground);
            affichageqd6(v);
            println!("");
        }
        "7" => unimplemented!(),
        "8s" | "Synchro" | "synchronisation" | "synchro" => {
            let t = decompoquestion.len();
            let mut listparam = Vec::new();
            for i in 1..t {
                listparam.push(decompoquestion[i].to_string());
            }
            let listesynchro = researchsynchro(&listparam, support, plan, ground, symbol);
            affichageq8s(&listesynchro, ground, lifted);
            println!("");
        }
        "9" => unimplemented!(),
        _ => println!("Not a question available"),
    }
}

pub fn choixquestionsmultiple(
    decompoquestion: &Vec<&str>,
    support: &DMatrix<i32>,
    menace: &DMatrix<i32>,
    plan: &Vec<Op>,
    ground: &GroundProblem,
    lifted: &World,
    symbol: &SymbolTable,
) {
    let q = decompoquestion[0];
    let sq = selectionquestion(q);

    match sq {
        Question::NoQuestion => println!(""),
        Question::SupportBy => {
            if decompoquestion.len()<=1{
                println!("Missing parameters, verify your question");
            }
            else{
                let mystring = decompoquestion[1].to_string();
                let num = mystring.parse::<usize>().unwrap();
                let v = supportedby(num, support, plan);
                affichageq1(num, plan, v, ground, lifted);
            }     
        }
        Question::SupportOf => {
            if decompoquestion.len()<=1{
                println!("Missing parameters, verify your question");
            }
            else{
                let mystring = decompoquestion[1].to_string();
                let num = mystring.parse::<usize>().unwrap();
                let v = supportof(num, support, plan);
                affichageq2(num, plan, v, ground, lifted);
            }
        }
        Question::Menace => {
            if decompoquestion.len()<=2{
                println!("Missing parameters, verify your question");
            }
            else{
                let mystring1 = decompoquestion[1].to_string();
                let num1 = mystring1.parse::<usize>().unwrap();
                let mystring2 = decompoquestion[2].to_string();
                let num2 = mystring2.parse::<usize>().unwrap();
                let v = menacefromto(num1, num2, menace);
                affichageq3(num1, num2, v, plan, ground, lifted);
            }
        }
        Question::Necessarybool => {
            if decompoquestion.len()<=1{
                println!("Missing parameters, verify your question");
            }
            else{
                let mystring = decompoquestion[1].to_string();
                let num = mystring.parse::<usize>().unwrap();
                let v = isnecessary(num, support, plan, ground);
                affichageq4(num, v, plan, ground, lifted);
            }
        }
        Question::Necessary => {
            if decompoquestion.len()<=1{
                println!("Missing parameters, verify your question");
            }
            else{
                let mystring = decompoquestion[1].to_string();
                let num = mystring.parse::<usize>().unwrap();
                let v = isnecessarydetail(num, support, plan, ground);
                affichageqd4(num, v, plan, ground, lifted);
            }
        }
        Question::Waybetweenbool => {
            if decompoquestion.len()<=2{
                println!("Missing parameters, verify your question");
            }
            else{
                let mystring1 = decompoquestion[1].to_string();
                let num1 = mystring1.parse::<usize>().unwrap();
                let mystring2 = decompoquestion[2].to_string();
                let num2 = mystring2.parse::<usize>().unwrap();
                let v = waybetweenbool(num1, num2, support, plan);
                affichageq5(num1, num2, v, plan, ground, lifted);
            }
        }
        Question::Waybetween => {
            if decompoquestion.len()<=2{
                println!("Missing parameters, verify your question");
            }
            else{
                let mystring1 = decompoquestion[1].to_string();
                let num1 = mystring1.parse::<usize>().unwrap();
                let mystring2 = decompoquestion[2].to_string();
                let num2 = mystring2.parse::<usize>().unwrap();
                let v = waybetween(num1, num2, support, plan);
                affichageqd5(&v, ground, lifted);
            }
        }
        Question::Parallelisablebool => {
            if decompoquestion.len()<=2{
                println!("Missing parameters, verify your question");
            }
            else{
                let mystring1 = decompoquestion[1].to_string();
                let num1 = mystring1.parse::<usize>().unwrap();
                let mystring2 = decompoquestion[2].to_string();
                let num2 = mystring2.parse::<usize>().unwrap();
                let v = parallelisablebool(num1, num2, support, menace, plan, ground);
                affichageq6(v);
            }
        }
        Question::Parallelisable => {
            if decompoquestion.len()<=2{
                println!("Missing parameters, verify your question");
            }
            else{
                let mystring1 = decompoquestion[1].to_string();
                let num1 = mystring1.parse::<usize>().unwrap();
                let mystring2 = decompoquestion[2].to_string();
                let num2 = mystring2.parse::<usize>().unwrap();
                let v = parallelisable(num1, num2, support, menace, plan, ground);
                affichageqd6(v);
            }
        }
        Question::AchieveGoal => {
            if decompoquestion.len()<=1{
                println!("Missing parameters, verify your question");
            }
            else{
                let mystring = decompoquestion[1].to_string();
                let num = mystring.parse::<usize>().unwrap();
                let v = achievegoal(num, support);
                affichageq7(num, v, plan, ground, lifted);
            }   
        },
        Question::Synchronisation => {
            if decompoquestion.len()<=1{
                println!("Missing parameters, verify your question");
            }
            else{
                let t = decompoquestion.len();
                let mut listparam = Vec::new();
                for i in 1..t {
                    listparam.push(decompoquestion[i].to_string());
                }
                let listesynchro = researchsynchro(&listparam, support, plan, ground, symbol);
                affichageq8s(&listesynchro, ground, lifted);
            }
        }
        Question::Weigthway => if decompoquestion.len()<=3{
            println!("Missing parameters, verify your question");
        }
        else{
            let t = decompoquestion.len();
            let mut listparam = Vec::new();
            let mystring1 = decompoquestion[1].to_string();
            let num1 = mystring1.parse::<i32>().unwrap();
            let mystring2 = decompoquestion[2].to_string();
            let num2 = mystring2.parse::<i32>().unwrap();
            let mystring1 = decompoquestion[3].to_string();
            let poids = mystring1.parse::<i32>().unwrap();
            let chemin;
            for i in 4..t {
                listparam.push(decompoquestion[i].to_string());
            }
            if num1>num2{
                 chemin = q9param(poids,num1,num2,&listparam,plan,ground,symbol,support);
            }
            else{
                 chemin = q9param(poids,num2,num1,&listparam,plan,ground,symbol,support);
            }
            affichageq9d(&chemin, ground, symbol);
        }
        ,
        Question::Qundefined => println!("Not a question available"),
        //_ => println!("Reach Unreachable"),
    }
}

pub fn help(){
    println!("
s or support-graph   Generate dot support and display matrixsupport
m or threat-graph   Generate dot threat and display matrix menace
question 
gg  Make plan with aries planificator if you have suspicion about your plan
p   Display plan
h   Help
e   exit

Questions available:
-support <step>
#Display others steps support by step 

-supported <step>
#Display others steps support of step

-goal <step>
#Display true if step accomplish a goal

-necessary <step>
#Display if step participates to the accomplishment of a goal,
#       necessary-d to have the shortest path

-path <source-step> <target-step>
#Display path between two steps, path-d to have the path.

-threat <source-step> <target-step>
#Display if source-step threats target-step if it put right before.

-betweeness <n-score>
#Display all step with a betweeness upper than the n-th score.

-synchro <parameters>
#Display step that make link between group based on parameters

-parallelizable <step> <step>
#Display a boolean to know if the two steps are parallelizable,
#       parallelizable-d to have more detail

-weightedpath <step> <step> <weight> <parameters>
#Display a path between two steps
with the least possible parameters of the list
(weight is a number wich penalize arch of operator
with  excluded parameters)
");
}