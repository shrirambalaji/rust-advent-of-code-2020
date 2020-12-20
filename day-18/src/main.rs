use std::{env, fs};

fn evaluate(expression: &str, add_has_precedence: bool) -> (u64, usize) {
    let mut result = 0;
    let mut apply_operation = |operand, operator: Option<u8>| match operator {
        Some(b'+') => result += operand,
        Some(b'*') => result *= operand,
        None => result = operand,
        _ => {}
    };

    let mut operator = None;
    let mut idx = 0;

    // idx is incremented when we encounter an empty space, +, *
    while idx < expression.len() {
        let value = expression.as_bytes()[idx];
        match value {
            b' ' => idx += 1,
            b'+' => {
                operator = Some(value);
                idx += 1;
            }
            b'*' => {
                operator = Some(value);
                idx += 1;

                // When add has precedence, we need to evaluate all the other sub expressions with '+' from the current index
                // This works because there are only two operators, however would fail when there would be more than 2.
                if add_has_precedence {
                    let inner_expression_evaluation =
                        evaluate(&expression[idx..], add_has_precedence);

                    let (inner_result, inner_idx) = inner_expression_evaluation;
                    idx += inner_idx;
                    apply_operation(inner_result, operator);
                }
            }
            // when we encounter an open parantheses, we evaluate the subexpression, after the current index.
            b'(' => {
                let inner_expression_evaluation =
                    evaluate(&expression[idx + 1..], add_has_precedence);

                let (inner_result, inner_idx) = inner_expression_evaluation;
                // also skip the index for ( and ) along with the indexes for the operators / operand in the inner expression.
                idx += inner_idx + 2;
                apply_operation(inner_result, operator);
            }
            b')' => return (result, idx),
            _ => {
                if let Some(operand) = (value as char).to_digit(10) {
                    idx += 1;
                    apply_operation(operand as u64, operator);
                } else {
                    panic!("Invalid character in expression")
                }
            }
        }
    }

    (result, idx)
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filepath = args.get(1).expect("Input filepath cannot be empty!");
    let input =
        fs::read_to_string(filepath).expect("Something went wrong while reading the input!");

    // -- Part 01 --
    let sum = input
        .lines()
        .map(|expr| evaluate(expr, false))
        .fold(0, |acc, (result, _)| acc + result);

    println!("Part 01 Sum: {}", sum);

    // -- Part 02 --
    let sum = input
        .lines()
        .map(|expr| evaluate(expr, true))
        .fold(0, |acc, (result, _)| acc + result);

    println!("Part 02 Sum: {}", sum);
}
#[cfg(test)]
mod tests {
    use super::*;

    fn eval_expression(expression: &str, expected: u64) -> bool {
        let (actual, _) = evaluate(expression, false);
        actual == expected
    }

    fn eval_expression_with_add_precedence(expression: &str, expected: u64) -> bool {
        let (actual, _) = evaluate(expression, true);
        actual == expected
    }

    #[test]
    fn should_evaluate_expression() {
        assert!(eval_expression("1 + 2 * 3 + 4 * 5 + 6", 71));
        assert!(eval_expression("2 * 3 + (4 * 5)", 26));
        assert!(eval_expression("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437));
        assert!(eval_expression(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            12240
        ));

        assert!(eval_expression(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
            13632
        ));
    }

    #[test]
    fn should_evaluate_expression_with_add_precedence() {
        assert!(eval_expression_with_add_precedence(
            "1 + 2 * 3 + 4 * 5 + 6",
            231
        ));
        assert!(eval_expression_with_add_precedence("2 * 3 + (4 * 5)", 46));
        assert!(eval_expression_with_add_precedence(
            "5 + (8 * 3 + 9 + 3 * 4 * 3)",
            1445
        ));
        assert!(eval_expression_with_add_precedence(
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            669060
        ));

        assert!(eval_expression_with_add_precedence(
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
            23340
        ));
    }
}
