pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Sunface");
        let target = "孙飞";
        assert!(
            result.contains(target),
            "你的问候中并没有包含目标姓名 {} ，你的问候是 `{}`",
            target,
            result
        );
    }
}
