#![allow(unreachable_code, unused_variables, dead_code)]
#[derive(Default, Debug)]

struct User{
    name: String,
    email: String,
    activo: bool,
    age: i32,
    user_role: UserRole,
}

#[derive(Default, Debug)]
enum UserRole{
    //#[default]
    BASIC,
    #[default]
    ADMIN,
}

fn main(){
        let user2: User = User::default();
        println!("{:?}", user2);
}
