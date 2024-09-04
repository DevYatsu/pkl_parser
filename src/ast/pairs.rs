use super::{
    annotation::Annotation,
    doc_comment::DocComment,
    expr::Expr,
    stmt::{amends::Amends, import::Import, module::Module, typealias::TypeAlias},
    types::Type,
};
use crate::{
    ast::stmt::{extends::Extends, ModifiersList},
    PklResult, Rule, Span,
};
use pest::iterators::Pair;

trait AsRng {
    fn as_rng(&self) -> Span;
}
impl<'a> AsRng for pest::Span<'a> {
    fn as_rng(&self) -> Span {
        self.start()..self.end()
    }
}

fn pkl_str_content(s: &str) -> &str {
    &s[1..s.len() - 1]
}

fn idents(element: Pair<Rule>) -> PklResult<Vec<(&str, Span)>> {
    let mut idents = Vec::new();
    for x in element.into_inner() {
        match x.as_rule() {
            Rule::ident => idents.push((x.as_str(), x.as_span().as_rng())),
            _ => unreachable!("idents"),
        }
    }

    Ok(idents)
}

fn ident(element: Pair<Rule>) -> PklResult<(&str, Span)> {
    for x in element.into_inner() {
        match x.as_rule() {
            Rule::ident => return Ok((x.as_str(), x.as_span().as_rng())),
            _ => unreachable!("ident"),
        }
    }

    unreachable!("ident")
}

fn type_attribute(element: Pair<Rule>) -> PklResult<Vec<(Type, Span)>> {
    let mut types = Vec::new();

    for t in element.into_inner() {
        let span = t.as_span().as_rng();
        match t.as_rule() {
            Rule::TYPE => types.push((pkl_type(t)?, span)),
            _ => unreachable!(),
        }
    }

    Ok(types)
}

fn pkl_type(element: Pair<Rule>) -> PklResult<Type> {
    let mut _type = Type::default();

    for part in element.into_inner() {
        match part.as_rule() {
            Rule::default_prefix => _type.set_default(),

            Rule::primary_type => {
                for t in part.into_inner() {
                    match t.as_rule() {
                        Rule::ident_with_opt_dots => {
                            _type.set_name((t.as_str().to_owned(), t.as_span().as_rng()))
                        }
                        Rule::type_attribute => _type.set_params(type_attribute(t)?),
                        Rule::expr => {}
                        _ => unreachable!(),
                    }
                }
            }
            Rule::string => {
                _type = Type::String {
                    value: (
                        pkl_str_content(part.as_str()).to_owned(),
                        part.as_span().as_rng(),
                    ),
                    is_nullable: false,
                    is_default: _type.is_default(),
                }
            }
            Rule::function_type => {
                todo!()
            }
            Rule::paren_type => {
                for t in part.into_inner() {
                    match t.as_rule() {
                        Rule::TYPE => {
                            let paren_type = pkl_type(t)?;
                            match _type {
                                Type::Union(_, _) => {
                                    let is_default = _type.is_default();
                                    _type = Type::Union(
                                        Box::new(_type),
                                        Box::new(Type::Parenthesized {
                                            value: Box::new(paren_type),
                                            is_nullable: false,
                                            is_default,
                                        }),
                                    );
                                }
                                _ => {
                                    _type = Type::Parenthesized {
                                        value: Box::new(paren_type),
                                        is_nullable: false,
                                        is_default: _type.is_default(),
                                    };
                                }
                            };
                        }
                        _ => unreachable!(),
                    }
                }
            }

            Rule::nullable => _type.set_nullable(),
            Rule::union => _type = Type::Union(Box::new(_type), Box::new(Type::default())),

            _ => unreachable!("type"),
        }
    }

    Ok(_type)
}

fn expr(element: Pair<Rule>) -> PklResult<Expr> {
    todo!()
}

pub fn doc_comment(element: Pair<Rule>) -> PklResult<DocComment> {
    let mut comment_text = element.as_str().to_string();
    comment_text = comment_text
        .strip_prefix("///")
        .unwrap()
        .split("\n///")
        .map(|s| if s.trim().len() == 0 { "\n" } else { s.trim() })
        .collect::<Vec<_>>()
        .join(" ");

    Ok(DocComment(comment_text))
}

pub fn annotation(element: Pair<Rule>) -> PklResult<Annotation> {
    let mut name = None;

    for annotation in element.into_inner() {
        match annotation.as_rule() {
            Rule::ident => name = Some(annotation.as_str()),
            Rule::obj_body => {
                return Ok(Annotation(
                    name.unwrap().to_owned(),
                    Some(expr(annotation)?),
                ))
            }
            _ => unreachable!(),
        }
    }

    Ok(Annotation(name.unwrap().to_owned(), None))
}

pub fn amends(element: Pair<Rule>) -> PklResult<Amends> {
    let span = element.as_span();
    for inner_elements in element.into_inner() {
        match inner_elements.as_rule() {
            Rule::src => {
                let span = span.start()..span.end();
                return Ok(Amends {
                    name: pkl_str_content(inner_elements.as_str()),
                    span,
                });
            }
            _ => unreachable!(),
        }
    }

    unreachable!("amends src should be already parsed")
}

pub fn import(element: Pair<Rule>) -> PklResult<Import> {
    let span = element.as_span();
    let span = span.start()..span.end();
    let mut name = ("", 0..0);
    let mut is_globbed = false;
    let mut local_name = None;

    for inner_element in element.into_inner() {
        match inner_element.as_rule() {
            Rule::src => {
                name = (
                    pkl_str_content(inner_element.as_str()),
                    inner_element.as_span().as_rng(),
                )
            }
            Rule::import_as => local_name = Some(ident(inner_element)?),
            Rule::globbed_import => is_globbed = true,

            // x => {
            //     println!("unreachable rule {x:?}")
            // }
            _ => unreachable!(),
        }
    }

    Ok(Import {
        name,
        local_name,
        span,
        is_globbed,
    })
}

pub fn module(element: Pair<Rule>) -> PklResult<Module> {
    let mut modifiers = ModifiersList::new();
    for inner_elements in element.into_inner() {
        match inner_elements.as_rule() {
            Rule::PREFIX_KEYWORD => {
                modifiers.push(inner_elements.as_str());
            }
            Rule::ident_with_opt_dots => {
                return Ok(Module {
                    modifiers,
                    full_name: (inner_elements.as_str(), inner_elements.as_span().as_rng()),
                });
            }
            _ => unreachable!(),
        }
    }

    unreachable!("module src should be already parsed")
}

pub fn extends(element: Pair<Rule>) -> PklResult<Extends> {
    let span = element.as_span();
    for inner_elements in element.into_inner() {
        match inner_elements.as_rule() {
            Rule::src => {
                let span = span.start()..span.end();
                return Ok(Extends {
                    name: pkl_str_content(inner_elements.as_str()),
                    span,
                });
            }
            _ => unreachable!(),
        }
    }

    unreachable!("extends src should be already parsed")
}

pub fn typealias(element: Pair<Rule>) -> PklResult<TypeAlias> {
    let mut modifiers = ModifiersList::new();
    let mut name = ("", 0..0);
    let mut attributes = Vec::new();
    let mut refering_type = None;

    for inner_elements in element.into_inner() {
        println!("{:?}", inner_elements);
        match inner_elements.as_rule() {
            Rule::PREFIX_KEYWORD => {
                modifiers.push(inner_elements.as_str());
            }
            Rule::ident => {
                name = (inner_elements.as_str(), inner_elements.as_span().as_rng());
            }
            Rule::typealias_params => {
                attributes = idents(inner_elements)?;
            }
            Rule::TYPE => refering_type = Some(pkl_type(inner_elements)?),
            _ => unreachable!(),
        }
    }

    Ok(TypeAlias {
        modifiers,
        name,
        attributes,
        refering_type: refering_type.unwrap(),
    })
}
