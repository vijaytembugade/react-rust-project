use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate_expression(expr: &str) -> Result<f64, String> {
    // Simple recursive descent parser
    let mut chars = expr.chars().collect::<Vec<_>>();
    let mut pos = 0;
    
    // Parse expression (add/subtract terms)
    fn parse_expr(chars: &[char], pos: &mut usize) -> Result<f64, String> {
        let mut result = parse_term(chars, pos)?;
        
        while *pos < chars.len() {
            match chars[*pos] {
                '+' => {
                    *pos += 1;
                    result += parse_term(chars, pos)?;
                },
                '-' => {
                    *pos += 1;
                    result -= parse_term(chars, pos)?;
                },
                _ => break,
            }
        }
        
        Ok(result)
    }
    
    // Parse term (multiply/divide factors)
    fn parse_term(chars: &[char], pos: &mut usize) -> Result<f64, String> {
        let mut result = parse_factor(chars, pos)?;
        
        while *pos < chars.len() {
            match chars[*pos] {
                '*' => {
                    *pos += 1;
                    result *= parse_factor(chars, pos)?;
                },
                '/' => {
                    *pos += 1;
                    let divisor = parse_factor(chars, pos)?;
                    if divisor == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    result /= divisor;
                },
                _ => break,
            }
        }
        
        Ok(result)
    }
    
    // Parse factor (number or parenthesized expression)
    fn parse_factor(chars: &[char], pos: &mut usize) -> Result<f64, String> {
        // Skip whitespace
        while *pos < chars.len() && chars[*pos].is_whitespace() {
            *pos += 1;
        }
        
        if *pos >= chars.len() {
            return Err("Unexpected end of expression".to_string());
        }
        
        match chars[*pos] {
            '(' => {
                *pos += 1;
                let result = parse_expr(chars, pos)?;
                
                // Expect closing parenthesis
                if *pos >= chars.len() || chars[*pos] != ')' {
                    return Err("Missing closing parenthesis".to_string());
                }
                *pos += 1;
                
                Ok(result)
            },
            '0'..='9' => {
                let mut num_str = String::new();
                
                while *pos < chars.len() && (chars[*pos].is_digit(10) || chars[*pos] == '.') {
                    num_str.push(chars[*pos]);
                    *pos += 1;
                }
                
                match num_str.parse::<f64>() {
                    Ok(num) => Ok(num),
                    Err(_) => Err(format!("Invalid number: {}", num_str)),
                }
            },
            _ => Err(format!("Unexpected character: {}", chars[*pos])),
        }
    }
    
    let result = parse_expr(&chars, &mut pos)?;
    
    // Check if we've consumed the entire expression
    if pos < chars.len() {
        return Err(format!("Unexpected character: {}", chars[pos]));
    }
    
    Ok(result)
}