#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

mod db;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Serialize)]
struct Resp<T> {
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
fn create(word: &str) -> Json<Resp<String>> {
    let res = db::insert_problem(word);
    if let Ok(id) = res {
        return Json(Resp::success(format!("/problem?id={}", id)))
    } else {
        return Json(Resp::err("invalid word"))
    }
}

#[get("/getProblem?<id>")]
fn get_problem(id: i64) -> Json<Resp<usize>> {
    let res = db::get_problem_by_id(id);
    if let Ok(problem) = res {
        return Json(Resp::success(problem.word.chars().count()))
    } else {
        return Json(Resp::err("invalid problem"))
    }
}

#[get("/check?<id>&<guess>")]
fn check(id: i64, guess: &str) -> Json<Resp<Vec<u8>>> {
    let res = db::get_problem_by_id(id);
    match res {
        Ok(problem) if problem.word.chars().count() != guess.chars().count() => Json(Resp::err("bad input")),
        Ok(problem) => {
            let mut holder = problem.word.chars().collect::<Vec<char>>();
            let res = guess.chars().enumerate().map(|(idx, c)| {
                if holder[idx] == c {
                    holder[idx] = '0';
                    return 2
                } else if let Some(p) = holder.iter().position(|&r| r == c) {
                    holder[p] = '0';
                    return 1
                } else {
                    return 0
                }
            }).collect();
            Json(Resp::success(res))
        }
        Err(_)  => Json(Resp::err("not a word"))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![create, get_problem, check])
}