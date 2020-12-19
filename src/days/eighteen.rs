use crate::aoc_error::AocError;

pub const NAME: &str = "Operation Order";

fn tokenize(expression: &str) -> Vec<String> {
    expression
        // This is pretty janky, but in my defense I'm stealing bad ideas from
        // the best https://norvig.com/lispy.html
        .replace("(", "( ")
        .replace(")", " )")
        .split(" ")
        .map(|s| s.to_string())
        .collect()
}

fn matched_paren(tokens: &[String], start_paren: usize) -> Result<usize, AocError> {
    if tokens[start_paren] != "(" {
        return Err(AocError::Misc("Can't find matched paren without start paren".to_string()));
    }

    let mut depth = 1;
    let mut index = start_paren;
    while depth > 0 {
        index += 1;
        if tokens[index] == "(" {
            depth += 1;
        } else if tokens[index] == ")" {
            depth -= 1;
        }
    }

    Ok(index)
}

// I'm sure it's possible to combine the part one and part two solutions more
// by having two strategies for building an AST and then a single eval function
// the evals the tree, but it's pretty late and I don't feel like it.

fn eval_term_p1(tokens: &[String]) -> Result<(isize, &[String]), AocError> {
    if tokens[0] == "(" {
        let end = matched_paren(tokens, 0)?;
        let val = eval_expression_p1(&tokens[1..end])?;
        Ok((val, &tokens[(end + 1)..]))
    } else {
        let constant = tokens[0].parse()?;
        Ok((constant, &tokens[1..]))
    }
}

fn eval_expression_p1(tokens: &[String]) -> Result<isize, AocError> {
    let (mut value, mut tokens) = eval_term_p1(tokens)?;

    while tokens.len() > 0 {
        let op = &tokens[0];
        let (right, remaining) = eval_term_p1(&tokens[1..])?;
        tokens = remaining;

        match op.as_str() {
            "+" => value += right,
            "*" => value *= right,
            _ => return Err(AocError::Misc("Invalid operator".to_string()))
        }
    }

    Ok(value)
}

#[derive(Debug, Copy, Clone)]
enum Term {
    Constant(isize),
    Add,
    Mul
}

fn eval_term_p2(tokens: &[String]) -> Result<(Term, &[String]), AocError> {
    if tokens[0] == "(" {
        let end = matched_paren(tokens, 0)?;
        let val = eval_expression_p2(&tokens[1..end])?;
        Ok((Term::Constant(val), &tokens[(end + 1)..]))
    } else if tokens[0] == "+" {
        Ok((Term::Add, &tokens[1..]))
    } else if tokens[0] == "*" {
        Ok((Term::Mul, &tokens[1..]))
    } else {
        let val = tokens[0].parse()?;
        Ok((Term::Constant(val), &tokens[1..]))
    }
}

fn eval_expression_p2(tokens: &[String]) -> Result<isize, AocError> {
    let mut terms = Vec::new();
    let mut tokens = tokens;
    while tokens.len() > 0 {
        let (term, remaining) = eval_term_p2(tokens)?;
        terms.push(term);
        tokens = remaining;
    }

    // This is a pretty elaborate way to avoid making an AST, but it's pretty
    // late at night.

    let mut added_terms = Vec::new();
    let mut i = 0;
    while i < terms.len() {
        let term = &terms[i];
        match term {
            Term::Constant(x) => added_terms.push(*x),
            Term::Mul => { },
            Term::Add => {
                let left = added_terms
                    .pop()
                    .ok_or_else(|| AocError::Misc("Bad expression".to_string()))?;
                let right = &terms[i + 1];
                match right {
                    Term::Constant(right) => {
                        added_terms.push(left + right);
                    },
                    _ => return Err(AocError::Misc("Bad expression".to_string()))
                }
                i += 1;
            }
        }

        i += 1;
    }

    let value = added_terms.iter().product();
    Ok(value)
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let mut answer = 0;
    for line in input.lines() {
        answer += eval_expression_p1(&tokenize(line))?;
    }

    Ok(answer.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut answer = 0;
    for line in input.lines() {
        answer += eval_expression_p2(&tokenize(line))?;
    }

    Ok(answer.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_op_add_p1() {
        let s = "1 + 2";
        let value = eval_expression_p1(&tokenize(s)).unwrap();
        assert_eq!(value, 3);
    }

    #[test]
    fn root_parens_p1() {
        let s = "(2 * 3)";
        let value = eval_expression_p1(&tokenize(s)).unwrap();
        assert_eq!(value, 6);
    }

    #[test]
    fn interior_parens_p1() {
        let s = "1 + (2 + 3) * 4";
        let value = eval_expression_p1(&tokenize(s)).unwrap();
        assert_eq!(value, 24);
    }

    #[test]
    fn nested_parens_p1() {
        let s = "((1 * 2) + (3 + 4)) + 5";
        let value = eval_expression_p1(&tokenize(s)).unwrap();
        assert_eq!(value, 14);
    }

    #[test]
    fn one_op_add_p2() {
        let s = "1 + 2";
        let value = eval_expression_p2(&tokenize(s)).unwrap();
        assert_eq!(value, 3);
    }

    #[test]
    fn add_precedence_p2() {
        let s = "1 + 2 * 3 + 4 * 5 + 6";
        let value = eval_expression_p2(&tokenize(s)).unwrap();
        assert_eq!(value, 231);
    }

    #[test]
    fn examples_p2() {
        let s = "1 + (2 * 3) + (4 * (5 + 6))";
        let value = eval_expression_p2(&tokenize(s)).unwrap();
        assert_eq!(value, 51);

        let s = "2 * 3 + (4 * 5)";
        let value = eval_expression_p2(&tokenize(s)).unwrap();
        assert_eq!(value, 46);

        let s = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let value = eval_expression_p2(&tokenize(s)).unwrap();
        assert_eq!(value, 1445);

        let s = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let value = eval_expression_p2(&tokenize(s)).unwrap();
        assert_eq!(value, 669060);

        let s = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let value = eval_expression_p2(&tokenize(s)).unwrap();
        assert_eq!(value, 23340);
    }
}
