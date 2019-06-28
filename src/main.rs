#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use crate::db_operations::*;
use postgres::{Connection, TlsMode};
use rocket::{
    http::{Cookie, Cookies},
    request::Form,
    response::{content::Content, Redirect},
    Config, State,
};
use rocket_contrib::{serve::StaticFiles, templates::Template};
use std::{env, sync::Mutex};

mod db_operations;

#[derive(Serialize)]
pub struct UserDetails {
    id: i32,
    name: String,
    email: String,
    phone: Option<String>,
    profile_pic: String,
    password: String,
    grade: String,
    is_leader: bool,
    section: String,
    division: String,
    category: Option<String>,
    subcategory: Option<String>,
    secondary_category: Option<String>,
    secondary_subcategory: Option<String>,
    project_title: String,
    project_abstract: String,
    project_id: String,
    team_members: Vec<(i32, String, bool)>,
}

#[derive(Serialize)]
struct HomePageDetails {
    id: i32,
    profile_pic: String,
    users: usize,
}

#[derive(Serialize)]
pub struct Project {
    project_id: String,
    project_title: String,
    project_abstract: String,
    team_members: Vec<(i32, String, String, String)>,
    division: String,
    category: Option<String>,
    subcategory: Option<String>,
    secondary_category: Option<String>,
    secondary_subcategory: Option<String>,
}

#[derive(Serialize)]
pub struct AdminPageDetails {
    id: i8,
    profile_pic: String,
    scope: String,
    total_registrations: usize,
    scope_registrations_ix: usize,
    scope_registrations_x: usize,
    scope_registrations_xi: usize,
    scope_registrations_xii: usize,
    scope_submissions: Vec<Project>,
}

#[derive(FromForm)]
pub struct Email {
    email: String,
}

#[derive(FromForm)]
pub struct RegistrationDetails {
    name: String,
    email: String,
    phone: Option<String>,
    password: String,
    grade: String,
    section: String,
    division: String,
    category: Option<String>,
    subcategory: Option<String>,
    secondary_category: Option<String>,
    secondary_subcategory: Option<String>,
    project_title: String,
    project_abstract: String,
}

#[derive(FromForm)]
pub struct NewTeamMemberDetails {
    name: String,
    email: String,
    password: String,
    grade: String,
    section: String,
}

#[derive(FromForm)]
pub struct SignInDetails {
    email: String,
    password: String,
}

#[derive(FromForm)]
pub struct UserId {
    id: i32,
}

#[derive(FromForm)]
pub struct UpdateProfileDetails {
    name: String,
    phone: Option<String>,
    new_password: Option<String>,
}

#[derive(FromForm)]
pub struct UpdateProjectDetails {
    project_title: String,
    project_abstract: String,
}

#[derive(FromForm)]
struct AdminSignInDetails {
    scope: String,
    password: String,
}

impl Default for UserDetails {
    fn default() -> Self {
        UserDetails {
            id: -1,
            name: String::from("x"),
            email: String::from("x"),
            phone: None,
            profile_pic: String::from("x"),
            password: String::from("x"),
            grade: String::from("x"),
            section: String::from("x"),
            is_leader: false,
            division: String::from("x"),
            category: None,
            subcategory: None,
            secondary_category: None,
            secondary_subcategory: None,
            project_title: String::from("x"),
            project_abstract: String::from("x"),
            project_id: String::from("x"),
            team_members: Vec::new(),
        }
    }
}

#[get("/")]
fn index_route(conn: State<Mutex<Connection>>, mut cookies: Cookies) -> Template {
    let user_details = get_user_details(&conn.lock().unwrap(), &mut cookies);
    Template::render(
        "index",
        HomePageDetails {
            id: user_details.id.clone(),
            profile_pic: user_details.profile_pic.clone(),
            users: get_number_of_users(&conn.lock().unwrap()),
        },
    )
}

#[get("/categories")]
fn categories_route(conn: State<Mutex<Connection>>, mut cookies: Cookies) -> Template {
    let context = get_user_details(&conn.lock().unwrap(), &mut cookies);
    Template::render("categories", context)
}

#[get("/guidelines")]
fn guidelines_route(conn: State<Mutex<Connection>>, mut cookies: Cookies) -> Template {
    let context = get_user_details(&conn.lock().unwrap(), &mut cookies);
    Template::render("guidelines", context)
}

#[get("/rules")]
fn rules_route(conn: State<Mutex<Connection>>, mut cookies: Cookies) -> Template {
    let context = get_user_details(&conn.lock().unwrap(), &mut cookies);
    Template::render("rules", context)
}

#[get("/project")]
fn project_route(
    conn: State<Mutex<Connection>>,
    mut cookies: Cookies,
) -> Result<Template, Redirect> {
    let context = get_user_details(&conn.lock().unwrap(), &mut cookies);
    if context.id == -1 {
        return Err(Redirect::to("/register"));
    }
    Ok(Template::render("project", context))
}

#[get("/register")]
fn register_route(conn: State<Mutex<Connection>>, mut cookies: Cookies) -> Template {
    let context = get_user_details(&conn.lock().unwrap(), &mut cookies);
    Template::render("register", context)
}

#[get("/signin")]
fn signin_route(conn: State<Mutex<Connection>>, mut cookies: Cookies) -> Template {
    let context = get_user_details(&conn.lock().unwrap(), &mut cookies);
    Template::render("signin", context)
}

// Admin page

#[get("/admin")]
fn admin_route(conn: State<Mutex<Connection>>, mut cookies: Cookies) -> Template {
    if let Some(password) = cookies.get_private("admin_password") {
        if password.value() == "<password>" {
            if let Some(scope) = cookies.get_private("scope") {
                let context = get_admin_info(&conn.lock().unwrap(), String::from(scope.value()));
                return Template::render(
                    String::from(if scope.value() == "all" {
                        "all"
                    } else {
                        "specific"
                    }) + "_admin",
                    context,
                );
            }
        }
    }
    Template::render("admin_signin", UserDetails::default())
    //String::from("Page under maintanence. Please check back later.")
}

#[get("/admin/project.pdf?<project_id>")]
fn admin_project_pdf_download_route(
    conn: State<Mutex<Connection>>,
    project_id: String,
    mut cookies: Cookies,
) -> Result<Content<Vec<u8>>, Redirect> {
    if let Some(password) = cookies.get_private("admin_password") {
        if password.value() == "<password>" {
            return generate_pdf(&conn.lock().unwrap(), project_id);
        }
    }
    Err(Redirect::to("/admin"))
}

#[get("/admin/spse_admin_data.csv")]
fn admin_download_route(
    conn: State<Mutex<Connection>>,
    mut cookies: Cookies,
) -> Result<Content<String>, Redirect> {
    if let Some(password) = cookies.get_private("admin_password") {
        if password.value() == "<password>" {
            if let Some(scope) = cookies.get_private("scope") {
                return generate_csv(&conn.lock().unwrap(), scope.value().to_string());
            }
        }
    }
    Err(Redirect::to("/admin"))
}

// POST requests

#[post("/email_available", data = "<email>")]
fn email_available_route(conn: State<Mutex<Connection>>, email: Form<Email>) -> String {
    email_available(&conn.lock().unwrap(), &email.email).to_string()
}

#[post("/register", data = "<registration_details>")]
fn registration_route(
    conn: State<Mutex<Connection>>,
    registration_details: Form<RegistrationDetails>,
    cookies: Cookies,
) -> String {
    register(&conn.lock().unwrap(), registration_details, cookies)
}

#[post("/signin_user", data = "<sign_in_details>")]
fn signin_user_route(
    conn: State<Mutex<Connection>>,
    sign_in_details: Form<SignInDetails>,
    cookies: Cookies,
) -> String {
    signin_user(&conn.lock().unwrap(), sign_in_details, cookies)
}

#[post("/update_profile", data = "<update_profile_details>")]
fn update_profile_route(
    conn: State<Mutex<Connection>>,
    update_profile_details: Form<UpdateProfileDetails>,
    cookies: Cookies,
) -> String {
    update_profile(&conn.lock().unwrap(), update_profile_details, cookies)
}

#[post("/update_project", data = "<update_project_details>")]
fn update_project_route(
    conn: State<Mutex<Connection>>,
    update_project_details: Form<UpdateProjectDetails>,
    cookies: Cookies,
) -> String {
    update_project(&conn.lock().unwrap(), update_project_details, cookies)
}

#[post("/add_team_member", data = "<new_team_member_details>")]
fn add_team_member_route(
    conn: State<Mutex<Connection>>,
    new_team_member_details: Form<NewTeamMemberDetails>,
    cookies: Cookies,
) -> String {
    add_team_member(&conn.lock().unwrap(), new_team_member_details, cookies)
}

#[post("/make_leader", data = "<user_id>")]
fn make_leader_route(
    conn: State<Mutex<Connection>>,
    user_id: Form<UserId>,
    cookies: Cookies,
) -> String {
    make_leader(&conn.lock().unwrap(), user_id, cookies)
}

#[post("/signout")]
fn signout_route(mut cookies: Cookies) -> String {
    signout_user(&mut cookies)
}

#[post("/admin_signin", data = "<sign_in_details>")]
fn admin_signin_route(sign_in_details: Form<AdminSignInDetails>, mut cookies: Cookies) -> String {
    if &sign_in_details.password != "<password>" {
        return String::from("Incorrect password");
    }
    cookies.add_private(Cookie::new("scope", String::from(&sign_in_details.scope)));
    cookies.add_private(Cookie::new(
        "admin_password",
        String::from(&sign_in_details.password),
    ));
    String::from("success")
}

#[post("/admin_signout")]
fn admin_signout_route(mut cookies: Cookies) -> String {
    // Delete the scope cookie
    cookies.remove_private(Cookie::named("scope"));
    // Delete the admin password cookie
    cookies.remove_private(Cookie::named("admin_password"));
    String::from("success")
}

fn configure() -> Config {
    // Configure Rocket to serve on the port requested by Heroku.
    let mut config = Config::active().expect("could not load configuration");
    config
        .set_secret_key("<secret key>")
        .unwrap();
    if let Ok(port_str) = env::var("PORT") {
        let port = port_str.parse().expect("could not parse PORT");
        config.set_port(port);
    }
    config
}

fn rocket() -> rocket::Rocket {
    rocket::custom(configure())
        .mount(
            "/",
            routes![
                index_route,
                categories_route,
                guidelines_route,
                rules_route,
                project_route,
                register_route,
                signin_route,
                admin_route,
                admin_project_pdf_download_route,
                admin_download_route,
                email_available_route,
                registration_route,
                signin_user_route,
                update_profile_route,
                update_project_route,
                add_team_member_route,
                make_leader_route,
                signout_route,
                admin_signin_route,
                admin_signout_route,
            ],
        )
        .mount("/styles", StaticFiles::from("static/styles"))
        .mount("/scripts", StaticFiles::from("static/scripts"))
        .mount("/fonts", StaticFiles::from("static/fonts"))
        .mount("/images", StaticFiles::from("static/images"))
        .attach(Template::fairing())
}

fn main() {
    let client = Connection::connect("<database URL>", TlsMode::None).unwrap();
    rocket().manage(Mutex::new(client)).launch();
}
