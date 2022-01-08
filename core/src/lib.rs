use serde::Serialize;
use std::f64::consts::PI;
use svg::node::element::{Circle, LinearGradient, Polygon, Stop};
use svg::Document;

const CANVAS_SIZE: i32 = 200;
const CENTER: f64 = 100.0;
const BACKGROUND_COLOR: [&str; 5] = ["#F9F5F1", "#F5EFE8", "#F0E8DE", "#EAE0D3", "#E3D7C6"];

#[derive(Serialize, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

fn reverse(n: f64) -> f64 {
    (n - f64::from(CANVAS_SIZE)) * f64::from(-1)
}

pub fn calc(score: &[i32], progress: f64) -> Vec<Point> {
    let n = score.len() as i32;
    let mut point_list: Vec<Point> = Vec::new();

    for i in 0..n {
        let calc_score = f64::from(score[i as usize]) * progress;
        let n = f64::from(2) * PI * f64::from(i) / f64::from(n);

        let point = Point {
            x: CENTER + calc_score * n.sin(),
            y: reverse(CENTER + calc_score * n.cos()),
        };

        point_list.push(point);
    }

    point_list
}

fn convert_attr(point_list: &Vec<Point>) -> String {
    let formatted_list: Vec<String> = point_list
        .iter()
        .map(|point| format!("{x},{y}", x = point.x, y = point.y))
        .collect();
    formatted_list.join(" ")
}

pub fn radar(score: &[i32], progress: f64) -> String {
    let point_list = calc(&score, progress);

    let polygon = Polygon::new()
        .set("points", convert_attr(&point_list))
        .set("fill", "url(#fillGradient)");

    let gradient1 = Stop::new().set("offset", 0).set("stop-color", "#EA8253");
    let gradient2 = Stop::new()
        .set("offset", 1)
        .set("stop-color", "rgba(234, 130, 83, 0.3)");

    let linear_gradient = LinearGradient::new()
        .set("id", "fillGradient")
        .set("gradientTransform", "rotate(90)")
        .add(gradient1)
        .add(gradient2);

    let mut max_score: Vec<i32> = Vec::new();
    for _i in score {
        max_score.push(100);
    }

    // CANVAS_SIZEと外接円が同じ大きさの図形が出来るので下の余白を詰める
    let viewbox_height: f64 = calc(&max_score, 1.0).iter().fold(0.0 as f64, |acc, point| {
        let max = if point.y > acc { point.y } else { acc };
        max
    });

    let mut document = Document::new()
        .set("viewBox", (0, 0, CANVAS_SIZE, viewbox_height))
        .add(linear_gradient);

    for (i, color) in BACKGROUND_COLOR.iter().enumerate() {
        let bg_point_list = calc(&max_score, f64::from(1) - f64::from(i as i32) * 0.2);

        let background = Polygon::new()
            .set("points", convert_attr(&bg_point_list))
            .set("fill", *color);

        document = document.add(background);
    }

    document = document.add(polygon);

    for point in point_list {
        let circle = Circle::new()
            .set("cx", point.x)
            .set("cy", point.y)
            .set("r", 1.5);
        document = document.add(circle);
    }

    String::from(&document.to_string())
}
