use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{User, NewUser};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Ссылка на базу данных не найдена");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Недействительная ссылка на базу"))
}

pub mod models;
pub mod schema;

pub fn get_user(limit: i32) -> Vec<User> {
    use crate::schema::user::dsl::*;

    let connect = &mut establish_connection();
    let result = user.limit(limit.into()).load::<User>(connect);

    match result {
        Ok(users) => users,
        Err(_) => vec![],
    }
}

pub fn get_user_id(id_user: i32) -> User {
    use crate::schema::user::dsl::*;

    let connect = &mut establish_connection();
    let result = user
        .filter(id.eq(id_user))
        .first::<User>(connect);
        // .expect("Пользователь с таким id не найден");
    match result {
        Ok(u) => u,
        Err(_) => User::new(),
    }
}

pub fn create_user( login: &str, email: &str, password: &str ) -> User {
    use crate::schema::user;

    let connect = &mut establish_connection();

    let new_user = NewUser{ login, email, password };

    diesel::insert_into(user::table)
        .values(&new_user)
        .get_result(connect)
        .expect("Ошибка добавления")

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = get_user(5);
        // result.iter().for_each(|user| println!("{:?}", user));

        let user_by_id = get_user_id(1);
        println!("{:?}", user_by_id);
    }
}
