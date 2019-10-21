use super::{
    AdminPageDetails, NewTeamMemberDetails, Project, RegistrationDetails, SignInDetails,
    UpdateProfileDetails, UpdateProjectDetails, UserDetails, UserId,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use csv::Writer;
use postgres::Connection;
use printpdf::*;
use rand::{FromEntropy, Rng};
use rand_hc::Hc128Rng;
use rocket::{
    http::{ContentType, Cookie, Cookies},
    request::Form,
    response::{content::Content, Redirect},
};
use std::fs::File;
use std::io::BufWriter;

// Function to return the number of users
pub fn get_number_of_users(conn: &Connection) -> usize {
    conn.query("SELECT * FROM users", &[]).unwrap().len()
}

// Function to check if the name is available
pub fn name_available(conn: &Connection, name: &str) -> bool {
    if name.len() == 0 {
        // The name has length 0
        return false;
    }
    // Return whether there are no rows with the given name
    conn.query(
        "SELECT * FROM users WHERE name = $1",
        &[&name],
    )
    .unwrap()
    .is_empty()
}

// Function to check if the email address is available
pub fn email_available(conn: &Connection, email: &str) -> bool {
    println!("Email: {}", email);
    if email.len() == 0 {
        // The email address has length 0
        return false;
    }
    // Return whether there are no rows with the given email address
    conn.query(
        &format!("SELECT * FROM users WHERE email = '{}'", email),
        &[],
    )
    .unwrap()
    .is_empty()
}

/*
CREATE TABLE users_table (
0  id serial PRIMARY KEY,
1  name VARCHAR (100) UNIQUE NOT NULL,
2  email VARCHAR (100) UNIQUE,
3  phone VARCHAR (20),
4  password VARCHAR (62) NOT NULL,
5  salt BIGINT NOT NULL,
6  grade VARCHAR (3) NOT NULL,
7  section VARCHAR (10) NOT NULL,
8  is_leader boolean NOT NULL,
9  division VARCHAR (20) NOT NULL,
10 category VARCHAR (40),
11 subcategory VARCHAR (40),
12 secondary_category VARCHAR (40),
13 secondary_subcategory VARCHAR (40),
14 project_title VARCHAR (70) NOT NULL,
15 project_abstract VARCHAR,
16 project_id VARCHAR (7) NOT NULL
)
*/

// Function to register a user with the given details if they're valid
pub fn register(
    conn: &Connection,
    registration_details: Form<RegistrationDetails>,
    mut cookies: Cookies,
) -> String {
    // Check if the data is valid
    if registration_details.name.len() == 0 {
        return String::from("Error: No name");
    }
    if registration_details.password.len() < 8 {
        return String::from("Error: Password too short");
    }
    if registration_details.grade.len() == 0 {
        return String::from("Error: No class");
    }
    if registration_details.section.len() == 0 {
        return String::from("Error: No section");
    }
    if registration_details.division.len() == 0 {
        return String::from("Error: No division");
    }
    if &registration_details.division == "science" && registration_details.category == None {
        return String::from("Error: No category");
    }
    if &registration_details.division == "science" && registration_details.subcategory == None {
        return String::from("Error: No subcategory");
    }
    if registration_details.project_title.len() == 0 {
        return String::from("Error: No project title");
    }
    if &registration_details.division == "science"
        && registration_details.project_abstract.split(" ").count() < 20 {
        return String::from("Error: Project abstract is lesser than 20 words");
    }
    if name_available(&conn, &registration_details.name) == false {
        return String::from("Name already exists");
    }
    if let Some(email) = &registration_details.email {
        if !email_available(&conn, &email) {
            return String::from("Email already used");
        }
    }

    // Generate salt using a CSPRNG
    let salt = Hc128Rng::from_entropy().gen::<u32>();
    let password_hash = hash(
        &format!("{}{}", salt, registration_details.password),
        DEFAULT_COST,
    )
    .unwrap();
    // Form the project ID
    let part_one = match registration_details.division.as_ref() {
        "arts" => "ARTS",
        "business" => "BUSN",
        "mathematics" => "MATH",
        _ => match registration_details.category.as_ref().unwrap().as_ref() {
            "Tamil" => "TAM",
            "English" => "ENG",
            "Hindi" => "HIN",
            "French" => "FRE",
            "History" => "HIS",
            "Geography" => "GEO",
            "Democratic Politics" => "DEM",
            "Economics" => "ECO",
            "Biological Sciences" => "BIO",
            "Chemistry" => "CHEM",
            "Computer Science" => "CS",
            "Earth and Environmental Sciences" => "EAEV",
            "Engineering Mechanics" => "EM",
            "Medicine and Health Sciences" => "MED",
            _ => "PHY",
        },
    };
    let entries = if [
        "science".to_string(),
        "socialstudies".to_string(),
        "languages".to_string(),
    ]
    .contains(&registration_details.division) {
        conn.query(
            "SELECT DISTINCT project_id FROM users WHERE category = $1 ORDER BY project_id",
            &[&registration_details.category.as_ref().unwrap()],
        )
    } else {
        conn.query(
            "SELECT DISTINCT project_id FROM users WHERE division = $1 ORDER BY project_id",
            &[&registration_details.division],
        )
    }
    .unwrap();
    let part_two: u32 = if entries.is_empty() {
        0
    } else {
        let project_id: String = entries.get(entries.len() - 1).get(0);
        let start = project_id.find(|c:char| c.is_digit(10)).unwrap();
        project_id[start..].parse().unwrap()
    } + 1;
    // Create a new entry in the table to register the user
    if let Err(e) = conn.query(
        &format!(
            "INSERT INTO users VALUES (
                DEFAULT, $1, $2, $3, $4, {}, $5, $6, TRUE, $7, $8, $9, $10, $11, $12, $13, '{}{:03}'
            )",
            salt, part_one, part_two
        ),
        &[
            &registration_details.name,
            &registration_details.email,
            &registration_details.phone,
            &password_hash,
            &registration_details.grade,
            &registration_details.section,
            &registration_details.division,
            &registration_details.category,
            &registration_details.subcategory,
            &registration_details.secondary_category,
            &registration_details.secondary_subcategory,
            &registration_details.project_title,
            &registration_details.project_abstract,
        ],
    ) {
        println!("Error: {}", e.to_string());
        return String::from(e.to_string());
    };
    cookies.add_private(Cookie::new("name", (&registration_details.name).clone()));
    cookies.add_private(Cookie::new("hash", String::from(password_hash.trim())));
    String::from("success")
}

pub fn signin_user(
    conn: &Connection,
    user_details: Form<SignInDetails>,
    mut cookies: Cookies,
) -> String {
    if user_details.name.len() > 0 && user_details.password.len() > 0 {
        // Proceed if the name and password have been provided
        let rows = conn
            .query(
                "SELECT password, salt FROM users WHERE name = $1",
                &[&user_details.name],
            )
            .unwrap();
        if rows.is_empty() == false {
            // The name exists
            let password_hash: String = rows.get(0).get(0);
            let salt: i64 = rows.get(0).get(1);

            if verify(
                &format!("{}{}", salt, user_details.password),
                &password_hash.trim(),
            )
            .unwrap()
            {
                // The credentials are correct, create the cookies to sign the user in
                cookies.add_private(Cookie::new("name", (&user_details.name).clone()));
                cookies.add_private(Cookie::new("hash", String::from(password_hash.trim())));
                return String::from("success");
            }
        } else {
            // Hash something even if the name doesn't exist, so that response times are similar
            hash("foo", DEFAULT_COST).unwrap();
        }
    }
    String::from("Wrong credentials")
}

// Function to get user's details if user is signed in
pub fn get_user_details(conn: &Connection, cookies: &mut Cookies) -> UserDetails {
    if let Some(name) = cookies.get_private("name") {
        // If the name cookie exists
        if let Some(password_hash) = cookies.get_private("hash") {
            // If the password hash cookie exists as well
            let rows = conn
                .query("SELECT * FROM users WHERE name = $1", &[&name.value()])
                .unwrap();
            if rows.is_empty() {
                return UserDetails::default();
            }
            // Read the values to user_details
            let id: i32 = rows.get(0).get(0);
            let password: String = rows.get(0).get(4);
            let project_id: String = rows.get(0).get(16);
            let members = conn
                .query("SELECT * FROM users WHERE project_id = $1", &[&project_id])
                .unwrap();
            let mut team_members = Vec::new();
            // Create a vector with all the user's team members
            for member in members.iter() {
                let member_id: i32 = member.get(0);
                if member_id != id {
                    team_members.push((member.get(0), member.get(1), member.get(8)));
                }
            }
            let user_details = UserDetails {
                id,
                name: rows.get(0).get(1),
                email: rows.get(0).get(2),
                phone: rows.get(0).get(3),
                password: String::from(password.trim()),
                grade: rows.get(0).get(6),
                section: rows.get(0).get(7),
                is_leader: rows.get(0).get(8),
                division: rows.get(0).get(9),
                category: rows.get(0).get(10),
                subcategory: rows.get(0).get(11),
                secondary_category: rows.get(0).get(12),
                secondary_subcategory: rows.get(0).get(13),
                project_title: rows.get(0).get(14),
                project_abstract: rows.get(0).get(15),
                project_id,
                profile_pic: format!(
                    "https://www.gravatar.com/avatar/{:x}?d=robohash&s=256",
                    md5::compute(&rows.get(0).get::<_, Option<String>>(2).unwrap_or(String::new()).to_lowercase().as_bytes())
                ),
                team_members,
            };

            if password_hash.value() == password.trim() {
                // Return the details
                return user_details;
            }
        }
    }
    UserDetails::default()
}

// Function to create a user as part of a team
pub fn add_team_member(
    conn: &Connection,
    new_team_member_details: Form<NewTeamMemberDetails>,
    mut cookies: Cookies,
) -> String {
    // Check if the data is valid
    if new_team_member_details.name.len() == 0 {
        return String::from("Error: No name");
    }
    if new_team_member_details.password.len() < 8 {
        return String::from("Error: Password too short");
    }
    if new_team_member_details.grade.len() == 0 {
        return String::from("Error: No class");
    }
    if new_team_member_details.section.len() == 0 {
        return String::from("Error: No section");
    }
    if name_available(&conn, &new_team_member_details.name) == false {
        return String::from("Name already exists");
    }
    if let Some(email) = &new_team_member_details.email {
        if !email_available(&conn, &email) {
            return String::from("Email already used");
        }
    }
    if let Some(name) = cookies.get_private("name") {
        // If the name cookie exists
        if let Some(password_hash) = cookies.get_private("hash") {
            // If the password hash cookie exists as well
            let rows = conn
                .query("SELECT password, division, category, subcategory, secondary_category, secondary_subcategory, project_title, project_abstract, project_id FROM users WHERE name = $1", &[&name.value()])
                .unwrap();
            if rows.is_empty() {
                return String::from("Error: Not signed in");
            }
            let password: String = rows.get(0).get(0);
            // Generate salt using a CSPRNG
            let salt = Hc128Rng::from_entropy().gen::<u32>();
            let new_password_hash = hash(
                &format!("{}{}", salt, new_team_member_details.password),
                DEFAULT_COST,
            )
            .unwrap();
            if password_hash.value() == password.trim() {
                // The user is signed in, proceed to create the new user
                if let Err(e) = conn.query(
                    &format!(
                        "INSERT INTO users VALUES (
                            DEFAULT, $1, $2, NULL, $3, {}, $4, $5, FALSE, $6, $7, $8, $9, $10, $11, $12, $13
                        )",
                        salt
                    ),
                    &[
                        &new_team_member_details.name,
                        &new_team_member_details.email,
                        &new_password_hash,
                        &new_team_member_details.grade,
                        &new_team_member_details.section,
                        &rows.get(0).get::<_, String>(1),
                        &rows.get(0).get::<_, Option<String>>(2),
                        &rows.get(0).get::<_, Option<String>>(3),
                        &rows.get(0).get::<_, Option<String>>(4),
                        &rows.get(0).get::<_, Option<String>>(5),
                        &rows.get(0).get::<_, String>(6),
                        &rows.get(0).get::<_, String>(7),
                        &rows.get(0).get::<_, String>(8),
                    ],
                ) {
                    println!("Error: {}", e.to_string());
                    return String::from(e.to_string());
                };
                return String::from("success");
            }
        }
    }
    String::from("Error: Not signed in")
}

// Function to create a user as part of a team
pub fn make_leader(conn: &Connection, user_id: Form<UserId>, mut cookies: Cookies) -> String {
    if let Some(name) = cookies.get_private("name") {
        // If the name cookie exists
        if let Some(password_hash) = cookies.get_private("hash") {
            // If the password hash cookie exists as well
            let rows = conn
                .query(
                    "SELECT password, is_leader, id FROM users WHERE name = $1",
                    &[&name.value()],
                )
                .unwrap();
            if rows.is_empty() {
                return String::from("Error: Not signed in");
            }
            let password: String = rows.get(0).get(0);
            if password_hash.value() == password.trim() {
                // The user is signed in
                if rows.get(0).get::<_, bool>(1) == false {
                    // The signed in user is not the team leader
                    return String::from("Error: Only team leader can change the leader");
                }
                // The signed in user is the leader, proceed to change the leader
                if let Err(e) = conn.query(
                    "UPDATE users SET is_leader = TRUE WHERE id = $1",
                    &[&user_id.id],
                ) {
                    println!("Error: {}", e.to_string());
                    return String::from(e.to_string());
                };
                if let Err(e) = conn.query(
                    "UPDATE users SET is_leader = FALSE WHERE id = $1",
                    &[&rows.get(0).get::<_, i32>(2)],
                ) {
                    println!("Error: {}", e.to_string());
                    return String::from(e.to_string());
                };
                return String::from("success");
            }
        }
    }
    String::from("Error: Not signed in")
}

// Function to update the user's profile
pub fn update_profile(
    conn: &Connection,
    update_profile_details: Form<UpdateProfileDetails>,
    mut cookies: Cookies,
) -> String {
    if let Some(name) = cookies.get_private("name") {
        // If the name cookie exists
        if let Some(password_hash) = cookies.get_private("hash") {
            // If the password hash cookie exists as well
            let user = conn
                .query(
                    "SELECT password, salt FROM users WHERE name = $1",
                    &[&name.value()],
                )
                .unwrap();
            let password: String = user.get(0).get(0);
            let salt: i64 = user.get(0).get(1);
            if &password_hash.value() != &password {
                return String::from("Error: Not signed in");
            }
            if let Some(new_password) = &update_profile_details.new_password {
                let new_hash = hash(&format!("{}{}", salt, new_password), DEFAULT_COST).unwrap();
                cookies.remove_private(Cookie::named("hash"));
                cookies.add_private(Cookie::new("hash", new_hash.clone()));
                if let Err(e) = conn.query(
                    "UPDATE users SET email = $1,
                        phone = $2,
                        password = $3
                    WHERE name = $4",
                    &[
                        &update_profile_details.email,
                        &update_profile_details.phone,
                        &new_hash,
                        &name.value(),
                    ],
                ) {
                    println!("Error: {}", e.to_string());
                    return String::from(e.to_string());
                }
            } else if let Err(e) = conn.query(
                "UPDATE users SET email = $1,
                        phone = $2
                    WHERE name = $3",
                &[
                    &update_profile_details.email,
                    &update_profile_details.phone,
                    &name.value(),
                ],
            ) {
                println!("Error: {}", e.to_string());
                return String::from(e.to_string());
            }
            return String::from("success");
        }
    }
    String::from("Error: Not signed in")
}

// Function to update the user's project
pub fn update_project(
    conn: &Connection,
    update_project_details: Form<UpdateProjectDetails>,
    mut cookies: Cookies,
) -> String {
    if let Some(name) = cookies.get_private("name") {
        // If the name cookie exists
        if let Some(password_hash) = cookies.get_private("hash") {
            // If the password hash cookie exists as well
            let user = conn
                .query("SELECT * FROM users WHERE name = $1", &[&name.value()])
                .unwrap();
            let password: String = user.get(0).get(4);
            let project_id: String = user.get(0).get(16);
            if &password_hash.value() != &password {
                return String::from("Error: Not signed in");
            }
            if let Err(e) = conn.query(
                "UPDATE users SET project_title = $1,
                        project_abstract = $2
                    WHERE project_id = $3",
                &[
                    &update_project_details.project_title,
                    &update_project_details.project_abstract,
                    &project_id,
                ],
            ) {
                println!("Error: {}", e.to_string());
                return String::from(e.to_string());
            }
            return String::from("success");
        }
    }
    String::from("Error: Not signed in")
}

// Function to sign the user out
pub fn signout_user(cookies: &mut Cookies) -> String {
    // Delete the name cookie
    cookies.remove_private(Cookie::named("name"));
    // Delete the hash cookie
    cookies.remove_private(Cookie::named("hash"));
    String::from("success")
}

// Function to get admin info
pub fn get_admin_info(conn: &Connection, scope: String) -> AdminPageDetails {
    // Set the profile picture based on the scope
    let profile_pic = match scope.as_ref() {
        "all" => "https://png.pngtree.com/svg/20170418/287473299e.png".to_string(),
        "arts" => "data:image/png;base64,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".to_string(),
        "business" => "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQN0kmsPEhYhPe1mhJCA8JXlnHBbWzMDO-3z_dkg06V750CBDPr".to_string(),
        "mathematics" => "https://cdn1.iconfinder.com/data/icons/banking-and-finance-2-4/128/89-512.png".to_string(),
        "languages" => "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSaViIYX4lJUuzx3YKtuYEbvtMPs6504rtticwM7cbdGAbgNsEJ".to_string(),
        "socialstudies" => "https://cdn.iconscout.com/icon/free/png-256/earth-global-globe-international-map-planet-world-4-12846.png".to_string(),
        _ => "https://cdn2.iconfinder.com/data/icons/science-flat-style-1/64/science-atom-education-nuclear-physics-atomic-Electron-512.png".to_string()
    };
    // Retrieve all the rows from the database
    let rows = conn.query(
            "SELECT id, name, email, grade, section, division, project_id, project_title, project_abstract, category, subcategory, secondary_category, secondary_subcategory, is_leader FROM users ORDER BY grade, section, project_id",
            &[]
        )
        .unwrap();
    // Initialize variables to store info about the registrations
    let mut total_registrations = 0;
    let mut scope_registrations_ix = 0;
    let mut scope_registrations_x = 0;
    let mut scope_registrations_xi = 0;
    let mut scope_registrations_xii = 0;
    let mut scope_submissions = Vec::new();
    if rows.is_empty() {
        // The database is empty; return now
        return AdminPageDetails {
            id: -2,
            profile_pic,
            scope: match scope.as_ref() {
                "arts" => "Arts".to_string(),
                "business" => "Business".to_string(),
                "languages" => "Languages".to_string(),
                "socialstudies" => "Social Studies".to_string(),
                "science" => "Science".to_string(),
                _ => "All".to_string(),
            },
            total_registrations,
            scope_registrations_ix,
            scope_registrations_x,
            scope_registrations_xi,
            scope_registrations_xii,
            scope_submissions,
        };
    }
    for row in &rows {
        if row.get::<_, bool>(13) {
            // This is the leader of a project, increase the count of the total number of projects
            total_registrations += 1;
        }
        if row.get::<_, bool>(13) && (&scope == "all" || scope == row.get::<_, String>(5)) {
            // This is the leader of a project under the signed in user's scope
            // Increase the count of the number of projects in the scope
            match row.get::<_, String>(3).as_ref() {
                "IX" => scope_registrations_ix += 1,
                "X" => scope_registrations_x += 1,
                "XI" => scope_registrations_xi += 1,
                _ => scope_registrations_xii += 1,
            };
            // Add the project to the submissions list
            scope_submissions.push(Project {
                project_id: row.get(6),
                project_title: row.get(7),
                project_abstract: row.get(8),
                team_members: vec![(
                    row.get(0),
                    row.get(1),
                    row.get::<_, Option<String>>(2).unwrap_or(String::new()),
                    row.get::<_, String>(3) + " " + &row.get::<_, String>(4),
                )],
                division: row.get(5),
                category: row.get(9),
                subcategory: row.get(10),
                secondary_category: row.get(11),
                secondary_subcategory: row.get(12),
            });
        }
    }
    for row in &rows {
        if !row.get::<_, bool>(13) && (&scope == "all" || scope == row.get::<_, String>(5)) {
            // This user is not a leader
            for submission in &mut scope_submissions {
                if submission.project_id == row.get::<_, String>(6) {
                    submission.team_members.push((
                        row.get(0),
                        row.get(1),
                        row.get::<_, Option<String>>(2).unwrap_or(String::new()),
                        row.get::<_, String>(3) + " " + &row.get::<_, String>(4),
                    ));
                }
            }
        }
    }
    AdminPageDetails {
        id: -2,
        profile_pic,
        scope: match scope.as_ref() {
            "arts" => "Arts".to_string(),
            "business" => "Business".to_string(),
            "languages" => "Languages".to_string(),
            "socialstudies" => "Social Studies".to_string(),
            "science" => "Science".to_string(),
            _ => "All".to_string(),
        },
        total_registrations,
        scope_registrations_ix,
        scope_registrations_x,
        scope_registrations_xi,
        scope_registrations_xii,
        scope_submissions,
    }
}

fn overflow_text(text: String, length_break: usize) -> Vec<String> {
    let words = text.split(" ");
    let mut overflowed_text = Vec::new();
    let mut current_line = String::new();
    for word in words {
        if word.len() + current_line.len() < length_break {
            if current_line.len() > 0 {
                current_line += " ";
            }
            current_line += word;
        } else {
            overflowed_text.push(current_line);
            current_line = String::from(word);
        }
    }
    overflowed_text.push(current_line);
    overflowed_text
}

// Function to generate PDF file with the data of the requested project ID
pub fn generate_pdf(conn: &Connection, project_id: String) -> Result<Content<Vec<u8>>, Redirect> {
    let mut team_members: Vec<(String, String)> = Vec::new();
    let rows = conn
        .query(
            "SELECT * FROM users WHERE project_id = $1 ORDER BY is_leader",
            &[&project_id],
        )
        .unwrap();

    let (doc, page1, layer1) = PdfDocument::new(
        "SPSE Project ".to_string() + &project_id,
        Mm(210.0),
        Mm(297.0),
        "Layer 1".to_string(),
    );
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let main_heading = "                          Suguna PIPS Expo 2019";
    let project_title: String = rows.get(0).get(14);
    let project_abstract: String = rows.get(0).get(15);
    let division: String = rows.get(0).get(9);
    let category: Option<String> = rows.get(0).get(10);
    let subcategory: Option<String> = rows.get(0).get(11);
    let secondary_category: Option<String> = rows.get(0).get(12);
    let secondary_subcategory: Option<String> = rows.get(0).get(13);
    for user in &rows {
        team_members.push((
            user.get(1),
            user.get::<_, String>(6) + " " + &user.get::<_, String>(7),
        ));
    }

    let roboto_medium_font = doc
        .add_external_font(File::open("static/fonts/Roboto/Roboto-Medium.ttf").unwrap())
        .unwrap();
    let dosis_font = doc
        .add_external_font(File::open("static/fonts/Dosis/Dosis-Medium.ttf").unwrap())
        .unwrap();
    let iosevka_font = doc
        .add_external_font(File::open("static/fonts/Iosevka/iosevka-regular.ttf").unwrap())
        .unwrap();
    let source_sans_pro_font = doc
        .add_external_font(
            File::open("static/fonts/Source_Sans_Pro/SourceSansPro-Regular.ttf").unwrap(),
        )
        .unwrap();

    let points1 = vec![
        (Point::new(Mm(5.0), Mm(277.0)), false),
        (Point::new(Mm(205.0), Mm(277.0)), false),
    ];

    // Is the shape stroked? Is the shape closed? Is the shape filled?
    let line1 = Line {
        points: points1,
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };
    current_layer.add_shape(line1);

    current_layer.begin_text_section();

    current_layer.set_text_cursor(Mm(10.0), Mm(282.0));
    current_layer.set_line_height(28);

    current_layer.set_font(&dosis_font, 28);
    current_layer.write_text(main_heading, &dosis_font);
    current_layer.add_line_break();
    current_layer.add_line_break();

    current_layer.set_font(&roboto_medium_font, 28);
    for (i, line) in overflow_text(project_title, 33).iter().enumerate() {
        if i > 0 {
            current_layer.add_line_break();
        }
        current_layer.write_text(line, &roboto_medium_font);
    }
    current_layer.set_font(&roboto_medium_font, 18);
    current_layer.set_line_height(18);
    current_layer.write_text("    ", &roboto_medium_font);
    current_layer.write_text(&project_id, &roboto_medium_font);
    current_layer.add_line_break();
    current_layer.add_line_break();

    current_layer.set_font(&iosevka_font, 14);
    current_layer.set_line_height(16);
    for (i, line) in overflow_text(project_abstract, 77).iter().enumerate() {
        if i > 0 {
            current_layer.add_line_break();
        }
        current_layer.write_text(line, &iosevka_font);
    }
    current_layer.add_line_break();
    current_layer.add_line_break();

    current_layer.set_font(&source_sans_pro_font, 16);
    current_layer.set_line_height(18);
    current_layer.write_text(
        "Registered under ".to_string()
            + match division.as_ref() {
                "arts" => "Arts",
                "business" => "Business",
                "languages" => "Languages",
                "socialstudies" => "Social Studies",
                _ => "Science",
            },
        &source_sans_pro_font,
    );
    current_layer.add_line_break();
    current_layer.set_line_height(8);
    current_layer.add_line_break();

    current_layer.set_font(&source_sans_pro_font, 14);
    current_layer.set_line_height(16);
    if [
        "science".to_string(),
        "socialstudies".to_string(),
        "languages".to_string(),
        "all".to_string(),
    ]
    .contains(&division)
    {
        current_layer.write_text(
            "Category: ".to_string() + &category.unwrap(),
            &source_sans_pro_font,
        );
        current_layer.add_line_break();

        if ["science".to_string(), "all".to_string()].contains(&division) {
            current_layer.write_text(
                "Subcategory: ".to_string() + &subcategory.unwrap(),
                &source_sans_pro_font,
            );
            current_layer.add_line_break();
            current_layer.write_text(
                "Secondary Category: ".to_string()
                    + &match secondary_category {
                        Some(sec_cat) => sec_cat,
                        None => "None".to_string(),
                    },
                &source_sans_pro_font,
            );
            current_layer.add_line_break();
            current_layer.write_text(
                "Secondary Subcategory: ".to_string()
                    + &match secondary_subcategory {
                        Some(sec_subcat) => sec_subcat,
                        None => "None".to_string(),
                    },
                &source_sans_pro_font,
            );
            current_layer.add_line_break();
        }
    }

    for (i, member) in team_members.iter().enumerate() {
        current_layer.add_line_break();
        current_layer.write_text(
            member.0.clone()
                + " from "
                + &member.1
                + if i == 0 { "  LEADER" } else { "" },
            &source_sans_pro_font,
        );
    }

    current_layer.end_text_section();
    let mut generated_pdf = BufWriter::new(Vec::new());
    doc.save(&mut generated_pdf).unwrap();
    Ok(Content(
        ContentType::PDF,
        generated_pdf.into_inner().unwrap(),
    ))
}

// Function to generate CSV file of the data for the requested scope
pub fn generate_csv(conn: &Connection, scope: String) -> Result<Content<String>, Redirect> {
    let mut wtr = Writer::from_writer(vec![]);
    let rows = match scope.as_ref() {
        "all" => conn.query(
            "SELECT * FROM users ORDER BY grade, section, project_id",
            &[],
        ),
        _ => conn.query(
            "SELECT * FROM users WHERE division = $1 ORDER BY grade, section, project_id",
            &[&scope],
        ),
    }
    .unwrap();
    if ["all".to_string(), "science".to_string()].contains(&scope) {
        // All four category fields must be present
        wtr.write_record(&[
            "Name",
            "Class",
            "Category",
            "Subcategory",
            "Sec. Cat.",
            "Sec. Subcat.",
            "Project Title",
            "Is Leader",
            "Project ID",
        ])
        .unwrap();
        for row in &rows {
            wtr.write_record(&[
                row.get::<_, String>(1),
                row.get::<_, String>(6) + " " + row.get::<_, String>(7).as_ref(),
                match row.get::<_, String>(10).as_ref() {
                    "Tamil" => "TAM",
                    "English" => "ENG",
                    "Hindi" => "HIN",
                    "French" => "FRE",
                    "History" => "HIS",
                    "Geography" => "GEO",
                    "Democratic Politics" => "DEM",
                    "Economics" => "ECO",
                    "Biological Sciences" => "BIO",
                    "Chemistry" => "CHEM",
                    "Computer Science" => "CS",
                    "Earth and Environmental Sciences" => "EAEV",
                    "Engineering Mechanics" => "EM",
                    "Medicine and Health Sciences" => "MED",
                    _ => "PHY",
                }
                .to_string(),
                row.get::<_, String>(11),
                match row.get::<_, Option<String>>(12) {
                    Some(secondary_category) => match secondary_category.as_ref() {
                        "Tamil" => "TAM",
                        "English" => "ENG",
                        "Hindi" => "HIN",
                        "French" => "FRE",
                        "History" => "HIS",
                        "Geography" => "GEO",
                        "Democratic Politics" => "DEM",
                        "Economics" => "ECO",
                        "Biological Sciences" => "BIO",
                        "Chemistry" => "CHEM",
                        "Computer Science" => "CS",
                        "Earth and Environmental Sciences" => "EAEV",
                        "Engineering Mechanics" => "EM",
                        "Medicine and Health Sciences" => "MED",
                        _ => "PHY",
                    }
                    .to_string(),
                    None => String::from("None"),
                },
                match row.get::<_, Option<String>>(13) {
                    Some(secondary_subcategory) => secondary_subcategory,
                    None => String::from("None"),
                },
                row.get::<_, String>(14),
                row.get::<_, bool>(8).to_string(),
                row.get::<_, String>(16),
            ])
            .unwrap();
        }
        return Ok(Content(
            ContentType::CSV,
            String::from_utf8(wtr.into_inner().unwrap()).unwrap(),
        ));
    }
    if ["socialstudies".to_string(), "languages".to_string()].contains(&scope) {
        // The primary category field should be present
        wtr.write_record(&[
            "Name",
            "Class",
            "Email",
            "Category",
            "Project Title",
            "Is Leader",
            "Project ID",
        ])
        .unwrap();
        for row in &rows {
            wtr.write_record(&[
                row.get::<_, String>(1),
                row.get::<_, String>(6) + " " + row.get::<_, String>(7).as_ref(),
                match row.get::<_, String>(10).as_ref() {
                    "Tamil" => "TAM",
                    "English" => "ENG",
                    "Hindi" => "HIN",
                    "French" => "FRE",
                    "History" => "HIS",
                    "Geography" => "GEO",
                    "Democratic Politics" => "DEM",
                    "Economics" => "ECO",
                    "Biological Sciences" => "BIO",
                    "Chemistry" => "CHEM",
                    "Computer Science" => "CS",
                    "Earth and Environmental Sciences" => "EAEV",
                    "Engineering Mechanics" => "EM",
                    "Medicine and Health Sciences" => "MED",
                    _ => "PHY",
                }
                .to_string(),
                row.get::<_, String>(14),
                row.get::<_, bool>(8).to_string(),
                row.get::<_, String>(16),
            ])
            .unwrap();
        }
        return Ok(Content(
            ContentType::CSV,
            String::from_utf8(wtr.into_inner().unwrap()).unwrap(),
        ));
    }
    // None of the category fields should be present
    wtr.write_record(&[
        "Name",
        "Class",
        "Email",
        "Project Title",
        "Is Leader",
        "Project ID",
    ])
    .unwrap();
    for row in &rows {
        wtr.write_record(&[
            row.get::<_, String>(1),
            row.get::<_, String>(6) + " " + row.get::<_, String>(7).as_ref(),
            row.get::<_, String>(14),
            row.get::<_, bool>(8).to_string(),
            row.get::<_, String>(16),
        ])
        .unwrap();
    }
    Ok(Content(
        ContentType::CSV,
        String::from_utf8(wtr.into_inner().unwrap()).unwrap(),
    ))
}
