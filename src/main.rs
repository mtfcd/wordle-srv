#[macro_use] extern crate rocket;
use db::Problem;
use rocket::serde::json::Json;
use std::collections::HashSet;

mod db;


#[post("/create?<word>")]
fn create(word: &str) -> String {
    let res = db::insert_problem(word);
    if let Ok(id) = res {
        return format!("/problem?id={}", id)
    } else {
        return "".to_string()
    }
}

#[get("/getProblem?<id>")]
fn get_problem(id: i64) -> Json<usize> {
    let res = db::get_problem_by_id(id);
    if let Ok(problem) = res {
        return Json(problem.word.chars().count())
    } else {
        return Json(0)
    }
}

#[get("/check?<id>&<guess>")]
fn check(id: i64, guess: &str) -> Vec<u8> {
    let res = db::get_problem_by_id(id);
    match res {
        Ok(problem) if problem.word.chars().count() != guess.chars().count() => return vec![0],
        Ok(problem) => {
            let mut holder = problem.word.chars().collect::<Vec<char>>();
            guess.chars().enumerate().map(|(idx, c)| {
                if holder[idx] == c {
                    holder[idx] = '0';
                    return 2
                } else if let Some(p) = holder.iter().position(|&r| r == c) {
                    holder[p] = '0';
                    return 1
                } else {
                    return 0
                }
            }).collect()
        }
        Err(_)  => return vec![0]
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![create, get_problem, check])
}