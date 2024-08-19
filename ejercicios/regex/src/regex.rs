use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct RegexStep {
    rep: RegexRep,
    val: RegexVal,
}

#[derive(Debug, Clone)]
enum RegexVal {
    Literal(char),
    Wildcard,
    Clase(RegexClase),
}

#[derive(Debug, Clone)]
enum RegexRep {
    Any,
    Exact(usize),
    Range {
        min: Option<usize>,
        max: Option<usize>,
    },
}

#[derive(Debug, Clone)]
enum RegexClase {
    Alnum,
    Alpha,
    Digit,
    //etc...
}

#[derive(Debug)]
pub struct EvaluatedStep {
    step: RegexStep,
    match_size: usize,
    backtrackable: bool,
}

#[derive(Debug)]
pub struct Regex {
    steps: Vec<RegexStep>,
}

impl RegexVal {
    pub fn matches(&self, value: &str) -> usize {
        match self {
            Self::Literal(l) => {
                if Some(*l) == value.chars().next() {
                    println!("matcheo literal {} size: {}", l, l.len_utf8());
                    l.len_utf8() // es 1 porque valide que el input sea ascii
                } else {
                    println!("no match for {}", l);
                    0
                }
            }
            Self::Clase(_) => 0,
            Self::Wildcard => {
                if let Some(c) = value.chars().next() {
                    println!("matcheo wildcard size: {}", c.len_utf8());
                    c.len_utf8() // es 1 porque valide que el input sea ascii
                } else {
                    println!("no match for wildcard");
                    0
                }
            }
        }
    }
}

impl TryFrom<&str> for Regex {
    type Error = &'static str;

    fn try_from(expression: &str) -> Result<Self, Self::Error> {
        let mut steps: Vec<RegexStep> = Vec::new();

        //expression.chars().for_each(f) no me sirve porque a veces debo moverme mas de una posicion

        let mut chars_iter = expression.chars();
        while let Some(c) = chars_iter.next() {
            let step: Option<RegexStep> = match c {
                '.' => Some(RegexStep {
                    val: RegexVal::Wildcard,
                    rep: RegexRep::Exact(1),
                }),
                'a'..='z' => Some(RegexStep {
                    val: RegexVal::Literal(c),
                    rep: RegexRep::Exact(1),
                }),
                '*' => {
                    if let Some(last) = steps.last_mut() {
                        last.rep = RegexRep::Any;
                    } else {
                        return Err("se encontro un caracter '*' inesperado");
                    }
                    None
                }
                '?' => {
                    if let Some(last) = steps.last_mut() {
                        last.rep = RegexRep::Range {
                            min: Some(0),
                            max: Some(1),
                        };
                    } else {
                        return Err("se encontro un caracter '?' inesperado");
                    }
                    None
                }
                '{' => {
                    let mut range_str = String::new();
                    while let Some(c) = chars_iter.next() {
                        if c == '}' {
                            break;
                        }
                        range_str.push(c);
                    }

                    // TODO: solo soporta {n}, implementar el range {n,m}
                    let range: usize = range_str
                        .parse()
                        .map_err(|_| "el rango especificado no esta soportado")?;

                    if let Some(last) = steps.last_mut() {
                        last.rep = RegexRep::Exact(range);
                    } else {
                        return Err("se encontro un caracter '{' inesperado");
                    }
                    None
                }
                '\\' => match chars_iter.next() {
                    Some(literal) => Some(RegexStep {
                        val: RegexVal::Literal(literal),
                        rep: RegexRep::Exact(1),
                    }),
                    None => return Err("no se encontro un caracter despues de '\'"),
                },
                _ => return Err("se encontro un caracter inesperado"),
            };

            // solo agrego un step si step tiene un valor
            if let Some(p) = step {
                steps.push(p);
            }
        }

        Ok(Regex { steps })
    }
}

impl Regex {
    pub fn new(expression: &str) -> Result<Self, &str> {
        Regex::try_from(expression)
    }

    pub fn test(self, value: &str) -> Result<bool, &str> {
        // if !value.is_ascii() {
        //     return Err("el input no es ascii");
        // }

        let mut queue = VecDeque::from(self.steps);
        let mut stack: Vec<EvaluatedStep> = Vec::new();
        let mut index = 0;
        while let Some(step) = queue.pop_front() {
            match step.rep {
                RegexRep::Exact(n) => {
                    let match_size = match_exact(n, &step.val, &value[index..]);
                    if match_size != 0 {
                        index += match_size;
                        stack.push(EvaluatedStep {
                            step,
                            match_size,
                            backtrackable: false,
                        });
                    } else {
                        // intento hacer el backtrack
                        match backtrack(step, &mut stack, &mut queue) {
                            Some(size) => {
                                index -= size;
                            }
                            None => return Ok(false),
                        }
                    }
                }
                RegexRep::Any => {
                    let matches = match_any(&step.val, &value[index..]);
                    for size in matches {
                        // por cada match debo mover el indice
                        index += size;

                        // cada match es bracktrackable individualmente
                        stack.push(EvaluatedStep {
                            // estoy clonando este step porque realmente necesito duplicarlo
                            step: step.clone(),
                            match_size: size,
                            backtrackable: true,
                        });
                    }
                }
                _ => return Ok(false),
            }
        }

        Ok(true)
    }
}

fn match_exact(count_to_match: usize, val_to_match: &RegexVal, input: &str) -> usize {
    let mut matched_size = 0;

    for _ in 0..count_to_match {
        let s = val_to_match.matches(&input[matched_size..]);
        if s != 0 {
            matched_size += s;
        } else {
            return 0;
        }
    }

    matched_size
}

fn match_any(val_to_match: &RegexVal, input: &str) -> Vec<usize> {
    let mut total_matched_size = 0;
    let mut matched_sizes = Vec::new();

    loop {
        let match_size = val_to_match.matches(&input[total_matched_size..]);
        if match_size != 0 {
            // acumulo cuantos matches tengo
            total_matched_size += match_size;
            matched_sizes.push(match_size);
        } else {
            // no puedo seguir matcheando
            break;
        }
    }

    matched_sizes
}

fn backtrack(
    current: RegexStep,
    evaluated: &mut Vec<EvaluatedStep>,
    next: &mut VecDeque<RegexStep>,
) -> Option<usize> {
    let mut back_size = 0;
    next.push_front(current);
    while let Some(e) = evaluated.pop() {
        back_size += e.match_size;
        if e.backtrackable {
            println!("backtrack {:?}", back_size);
            return Some(back_size);
        } else {
            next.push_front(e.step);
        }
    }
    None
}
