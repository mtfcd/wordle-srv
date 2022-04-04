use rocket::serde::{Serialize, json::Json};

use crate::db;


#[derive(Serialize)]
pub struct Resp<T> {
    code: i64,
    msg: &'static str,
    data: Option<T>
}

impl<T> Resp<T> {
    fn success(data: T) -> Self {
        Self {
            code: 0,
            msg: "ok",
            data: Some(data)
        }
    }

    fn err(msg: &'static str) -> Self {
        Self {
            code: 1,
            msg,
            data: None
        }
    }

}

#[post("/create?<word>")]
pub fn create(word: &str) -> Json<Resp<String>> {
    let word1 = word.to_ascii_lowercase();
    if db::find_word(&word1).is_err() {
        return Json(Resp::err("not a word"))
    }
    let res = db::insert_problem(&word1);
    if let Ok(id) = res {
        return Json(Resp::success(format!("/problem?id={}", id)))
    } else {
        return Json(Resp::err("invalid word"))
    }
}

#[get("/getProblem?<id>")]
pub fn get_problem(id: i64) -> Json<Resp<usize>> {
    let res = db::get_problem_by_id(id);
    if let Ok(problem) = res {
        return Json(Resp::success(problem.word.chars().count()))
    } else {
        return Json(Resp::err("invalid problem"))
    }
}

fn compute_word(problem: String, guess: &str) -> Vec<u8> {
    let mut holder = problem.chars().collect::<Vec<char>>();
    guess.chars().enumerate().map(|(idx, c)| {
        if Some(c) == problem.chars().nth(idx) {
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

#[get("/check?<id>&<guess>&<line>")]
pub fn check(id: i64, guess: String, line: usize) -> Json<Resp<Vec<u8>>> {
    let guess1 = guess.to_ascii_lowercase();
    let res = db::get_problem_by_id(id);
    match res {
        Ok(problem) if problem.word.chars().count() != guess1.chars().count() => Json(Resp::err("bad input")),
        Ok(problem) => {
            if db::find_word(&guess1).is_err() {
                return Json(Resp::err("not a word"))
            }
            if let Err(e) = db::insert_guess(id, &guess1, line) {
                dbg!(e);
                return Json(Resp::err("db err"))
            }
            let res = compute_word(problem.word, &guess1);
            Json(Resp::success(res))
        }
        Err(_)  => Json(Resp::err("not a word"))
    }
}

#[derive(Serialize)]
pub struct Guesses {
    guesses: Vec<String>,
    res: Vec<Vec<u8>>,
}

#[get("/getGuesses?<id>")]
pub fn get_guesses(id: i64) -> Json<Resp<Guesses>> {
    let res = db::get_guesses(id);
    let problem_res = db::get_problem_by_id(id);
    if problem_res.is_err() {
        return Json(Resp::err("invalid problem"))
    }
    let problem = problem_res.unwrap().word;
    if let Ok(guesses) = res {
        let res = guesses.iter().map(|g| compute_word(problem.clone(), g)).collect();
        let data = Guesses{
            guesses: guesses.iter().map(|s| s.to_ascii_uppercase()).collect(),
            res
        };
        return Json(Resp::success(data))
    } else {
        return Json(Resp::err("invalid problem"))
    }
}
