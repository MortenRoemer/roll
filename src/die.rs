use rand::prelude::*;

pub struct Expression {
    ops: Vec<Operation>,
}

impl Expression {
    pub fn parse(source: &str) -> Self {
        let mut ops = Vec::new();
        Operation::parse(source, &mut ops);
        Self { ops }
    }

    pub fn evaluate(&self) -> i64 {
        let mut stack = Vec::with_capacity(self.ops.len());
        let mut rng = rand::thread_rng();

        for op in self.ops.iter() {
            op.evaluate(&mut stack, &mut rng);
        }

        stack.pop().unwrap()
    }
}

enum Operation {
    Push(Value),
    Add,
    Subtract,
    Multiply,
}

impl Operation {
    fn parse(source: &str, ops: &mut Vec<Operation>) {
        let source = source.trim();
        let parts: Vec<&str> = source.splitn(2, '*').collect();

        if parts.len() > 1 {
            Operation::parse(parts[0], ops);
            Operation::parse(parts[1], ops);
            ops.push(Operation::Multiply);
            return;
        }

        let parts: Vec<&str> = source.splitn(2, |c| c == '+' || c == '-').collect();

        if parts.len() > 1 {
            Operation::parse(parts[0], ops);
            Operation::parse(parts[1], ops);

            match source.chars().nth(parts[0].len()) {
                Some('+') => ops.push(Operation::Add),
                Some('-') => ops.push(Operation::Subtract),
                _ => panic!("unexpected operator"),
            }

            return;
        }

        ops.push(Operation::Push(Value::parse(source)));
    }

    fn evaluate(&self, stack: &mut Vec<i64>, rng: &mut ThreadRng) {
        use Operation::*;
        match self {
            Push(value) => {
                stack.push(value.evaluate(rng));
            }
            Add => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left.saturating_add(right));
            }
            Subtract => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left.saturating_sub(right));
            }
            Multiply => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left.saturating_mul(right));
            }
        }
    }
}

enum Value {
    Constant(i64),
    Dice(i64, i64),
}

impl Value {
    fn parse(source: &str) -> Self {
        let source = source.trim();
        let parts: Vec<&str> = source.splitn(2, 'd').collect();

        if parts.len() > 1 {
            return Value::Dice(parts[0].parse().unwrap(), parts[1].parse().unwrap());
        }

        Value::Constant(source.parse().unwrap())
    }

    fn evaluate(&self, rng: &mut ThreadRng) -> i64 {
        use Value::*;
        match self {
            Constant(value) => *value,
            Dice(count, range) => {
                let mut result = 0;

                for _i in 0..*count {
                    result += rng.gen_range(1..=*range);
                }

                result
            }
        }
    }
}
