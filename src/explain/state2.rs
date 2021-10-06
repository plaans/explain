//use aries_planning::classical::state::*;
use aries_planning::classical::state::Op;

//Pour lier Op et une etape
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Resume {
    opkey: Option<Op>,
    etape: i32,
}

impl Resume {
    pub fn op(&self) -> Option<Op> {
        self.opkey
    }

    pub fn numero(&self) -> i32 {
        self.etape
    }
}
pub fn newresume(ope: Op, num: i32) -> Resume {
    Resume {
        opkey: Some(ope),
        etape: num,
    }
}

pub fn defaultresume() -> Resume {
    Resume {
        opkey: None,
        etape: -1,
    }
}

pub fn goalresume(num: i32) -> Resume {
    Resume {
        opkey: None,
        etape: num,
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Necessaire {
    operateur: Resume,
    nec: bool,
    chemin: Option<Vec<Resume>>,
    longueur: u32,
}

impl Necessaire {
    pub fn opnec(&self) -> Resume {
        self.operateur
    }
    pub fn nec(&self) -> bool {
        self.nec
    }
    pub fn chemin(&self) -> Option<Vec<Resume>> {
        self.chemin.clone()
    }
    pub fn long(&self) -> u32 {
        self.longueur
    }
    pub fn presence(&self, res: Resume) -> bool {
        self.operateur == res
    }

    pub fn affiche(&self) {
        println!(
            " l'étape {} est nécessaire {} dans le chemin de longueur {} composé par :", /*,self.opnec().op()*/
            self.opnec().numero(),
            self.nec,
            self.long()
        );
        if self.chemin().is_none() {
            println!("pas de chemin");
        } else {
            for res in self.chemin().unwrap() {
                println!(" l'étape {}", res.numero());
            }
        }
    }
}
pub fn newnec(op: Resume, b: bool, way: Vec<Resume>, l: u32) -> Necessaire {
    Necessaire {
        operateur: op,
        chemin: Some(way),
        nec: b,
        longueur: l,
    }
}

pub fn newnecgoal(op: Resume) -> Necessaire {
    Necessaire {
        operateur: op,
        nec: true,
        chemin: None,
        longueur: 0,
    }
}

pub fn newnecess(op: Resume) -> Necessaire {
    Necessaire {
        operateur: op,
        nec: false,
        chemin: None,
        longueur: 0,
    }
}

pub fn initnec(op: Resume, inf: u32) -> Necessaire {
    Necessaire {
        operateur: op,
        nec: false,
        chemin: None,
        longueur: inf,
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Unique {
    operateur: Op,
    unicite: bool,
}

impl Unique {
    pub fn operateur(&self) -> Op {
        self.operateur
    }
    pub fn unicite(&self) -> bool {
        self.unicite
    }
    pub fn duplicite(&mut self) {
        self.unicite = false;
    }
}

pub fn newunique(ope: Op) -> Unique {
    Unique {
        operateur: ope,
        unicite: true,
    }
}

pub struct Obligationtemp {
    ope1: Op,
    etape1: i32,
    ope2: Op,
    etape2: i32,
}

impl Obligationtemp {
    pub fn operateur(&self) -> (Op, Op) {
        (self.ope1, self.ope2)
    }
    pub fn etape(&self) -> (i32, i32) {
        (self.etape1, self.etape2)
    }
    pub fn premiereetape(&self) -> (Op, i32) {
        if self.etape2 > self.etape1 {
            (self.ope1, self.etape1)
        } else {
            (self.ope2, self.etape2)
        }
    }
    pub fn deuxiemeetape(&self) -> (Op, i32) {
        if self.etape1 > self.etape2 {
            (self.ope1, self.etape1)
        } else {
            (self.ope2, self.etape2)
        }
    }
    pub fn affichage(&self) {
        println!(
            " l'étape {} et l'étape {} ne sont pas inversible",
            self.etape1, self.etape2
        );
    }
}

pub fn newot(ope: Op, step: i32, oper: Op, next: i32) -> Obligationtemp {
    Obligationtemp {
        ope1: ope,
        etape1: step,
        ope2: oper,
        etape2: next,
    }
}

#[derive(PartialEq)]
pub enum Parallelisable {
    Oui,
    NonMenace { origine: usize, vers: usize },
    NonSupport { origine: usize, vers: usize },
}

pub fn originenonp(p: Parallelisable) -> usize {
    match p {
        Parallelisable::NonMenace { origine, vers: _ } => origine,
        Parallelisable::NonSupport { origine, vers: _ } => origine,
        _ => {
            println!("Les 2 étapes sont parallelisable");
            0
        }
    }
}

pub fn ciblenonp(p: Parallelisable) -> usize {
    match p {
        Parallelisable::NonMenace { origine: _, vers } => vers,
        Parallelisable::NonSupport { origine: _, vers } => vers,
        _ => {
            println!("Les 2 étapes sont parallelisable");
            0
        }
    }
}

#[derive(PartialEq)]
pub enum Parallelisabledetail {
    Oui,
    MenaceApres {
        origine: usize,
        vers: usize,
    },
    MenaceAvant {
        origine: usize,
        vers: usize,
        supportconcern: Option<usize>,
    },
    SupportDirect {
        origine: usize,
        vers: usize,
    },
    SupportIndirect {
        origine: usize,
        vers: usize,
        chemin: Option<Vec<Resume>>,
    },
}

pub fn originenonpad(p: Parallelisabledetail) -> usize {
    match p {
        Parallelisabledetail::MenaceApres { origine, vers: _ } => origine,
        Parallelisabledetail::MenaceAvant {
            origine,
            vers: _,
            supportconcern: _,
        } => origine,
        Parallelisabledetail::SupportDirect { origine, vers: _ } => origine,
        Parallelisabledetail::SupportIndirect {
            origine,
            vers: _,
            chemin: _,
        } => origine,
        _ => {
            println!("Les 2 étapes sont parallelisable");
            0
        }
    }
}

pub fn ciblenonpad(p: Parallelisabledetail) -> usize {
    match p {
        Parallelisabledetail::MenaceApres { origine: _, vers } => vers,
        Parallelisabledetail::MenaceAvant {
            origine: _,
            vers,
            supportconcern: _,
        } => vers,
        Parallelisabledetail::SupportDirect { origine: _, vers } => vers,
        Parallelisabledetail::SupportIndirect {
            origine: _,
            vers,
            chemin: _,
        } => vers,
        _ => {
            println!("Les 2 étapes sont parallelisable");
            0
        }
    }
}
//match à refaire pour avoir sortie cohérente refaire menace avant en vec.
pub fn pad_detail(p: Parallelisabledetail) -> Vec<Option<usize>> {
    match p {
        Parallelisabledetail::MenaceAvant {
            origine: _,
            vers: _,
            supportconcern,
        } => {
            let mut n = Vec::new();
            n.push(supportconcern);
            n
        }
        Parallelisabledetail::SupportIndirect {
            origine: _,
            vers: _,
            chemin,
        } => {
            let mut v = Vec::new();
            for n in chemin {
                for step in n {
                    let i = step.numero();
                    let u = i as usize;
                    v.push(Some(u));
                }
            }
            v
        }
        _ => {
            println!(" Pas de détails supplémentaire");
            let mut v = Vec::new();
            v.push(None);
            v
        }
    }
}

#[derive(PartialEq)]
pub enum Question {
    NoQuestion,
    SupportBy,
    SupportOf,
    Menace,
    Necessary,
    Necessarybool,
    Waybetweenbool,
    Waybetween,
    Parallelisablebool,
    Parallelisable,
    AchieveGoal,
    Synchronisation,
    Weigthway,
    Qundefined,
}

pub fn selectionquestion(s: &str) -> Question {
    match s {
        "0" => Question::NoQuestion,
        "1" | "Supporté" | "supporté" | "supported" | "supportépar" | "Supportépar"
        | "Supportby" | "supportby" | "SupportBy" | "supportBy" | "Supportedby" | "supportedby" => {
            Question::SupportBy
        }
        "2" | "Support" | "support" | "supportde" | "Supportde" | "supportDe" | "SupportDe"
        | "Supportof" | "supportof" | "SupportOf" | "supportOf" => Question::SupportOf,
        "3" | "Menace" | "menace" | "menaceentre" | "Menaceentre" | "menaceEntre"
        | "MenaceEntre" | "threat" | "threatbetween" => Question::Menace,
        "4" | "nécessaire" | "Nécessaire" | "necessaire" | "Necessaire" | "necessary"
        | "Necessary" => Question::Necessarybool,
        "4d"
        | "nécessaire-D"
        | "Nécessaire-D"
        | "necessaire-D"
        | "Necessaire-D"
        | "necessary-D"
        | "Necessary-D"
        | "nécessaire-d"
        | "Nécessaire-d"
        | "necessaire-d"
        | "Necessaire-d"
        | "necessary-d"
        | "Necessary-d"
        | "nécessaire-Détail"
        | "Nécessaire-Détail"
        | "necessaire-Détail"
        | "Necessaire-Détail"
        | "necessary-Detail"
        | "Necessary-Detail"
        | "necessary-detail" => Question::Necessary,
        "5" | "cheminentre" | "Chemin" | "Cheminentre" | "chemin" | "CheminEntre"
        | "cheminEntre" | "Waybetween" | "waybetween" | "WayBetween" | "wayBetween" | "path" => {
            Question::Waybetweenbool
        }
        "5d" | "cheminentre-d" | "Chemin-d" | "Cheminentre-d" | "chemin-d" | "CheminEntre-d"
        | "cheminEntre-d" | "Chemin-détail" | "Chemin-detail" | "waybetween-d"
        | "waybetween-detail" | "path-d" | "path-detail" => Question::Waybetween,
        "6" | "parallélisable" | "parallelisable" | "parallelizable" => {
            Question::Parallelisablebool
        }
        "6d"
        | "parallélisable-d"
        | "parallelisable-d"
        | "parallelizable-d"
        | "parallélisable-detail"
        | "parallelisable-detail"
        | "parallelizable-detail" => Question::Parallelisable,
        "7" | "goal" | "but" => Question::AchieveGoal,
        "8s" | "Synchro" | "synchronisation" | "synchro" => Question::Synchronisation,
        "9" | "poids" | "weight" | "weighted" | "weightedpath" => Question::Weigthway,
        _ => Question::Qundefined,
    }
}
