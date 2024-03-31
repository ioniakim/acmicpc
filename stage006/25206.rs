use std::io;

struct ClassScore {
    credit: f32,
    grade: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut class_scores: Vec<ClassScore> = vec![];

    for line in io::stdin().lines().take(20) {
        let line = line?;
        let raw_data: Vec<&str> = line.split_whitespace().collect();
        let credit: f32 = raw_data[1].parse()?;
        let grade = raw_data[2];

        class_scores.push(ClassScore{credit, grade: grade.to_string()});
    }

    println!("{}", calculate_gpa(&class_scores));

    Ok(())
}

fn grade_score_map(grade: &str) -> Option<f32> {
    match grade {
        "A+" => Some(4.5),
        "A0" => Some(4.0),
        "B+" => Some(3.5),
        "B0" => Some(3.0),
        "C+" => Some(2.5),
        "C0" => Some(2.0),
        "D+" => Some(1.5),
        "D0" => Some(1.0),
        "F" => Some(0.0),
        _ => None
    }
}

fn calculate_gpa(scores: &Vec<ClassScore>) -> f32 {
    let (total_sum, full_sum) = scores.iter()
        .map(|s| {
            (s.credit, grade_score_map(&s.grade))
        })
        .fold((0.0, 0.0), |acc, s| {
            if let Some(v) = s.1 {
                (acc.0 + s.0 * v, acc.1 + s.0)
            } else {
                acc
            }
        });

    total_sum / full_sum
}
