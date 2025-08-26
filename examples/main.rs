use typed_endpoint::{Action, Endpoint, EndpointExt, Resource, Root};

fn main() {
    let path = Me::path().action().build();
    println!("path: {}", path);
    let path = User::path().list().build();
    println!("path: {}", path);
    let path = User::path().single("grant".to_owned()).build();
    println!("path: {}", path);
    let path = UserPost::path().bind("grant".to_owned()).list().build();
    println!("path: {}", path);
    let path = UserPost::path()
        .bind("grant".to_owned())
        .single(1023)
        .build();
    println!("path: {}", path);
    let path = FollowUser::path().bind("grant".to_owned()).action().build();
    println!("path: {}", path);
}

// Example resources
pub struct Me;

impl Endpoint for Me {
    const URL_PATH_SEGMENT: &'static str = "me";
    type Parent = Root;
}
impl Action for Me {}

pub struct User;
impl Endpoint for User {
    const URL_PATH_SEGMENT: &'static str = "users";
    type Parent = Root;
}
impl Resource for User {
    type Id = String;
}

pub struct UserPost;
impl Endpoint for UserPost {
    const URL_PATH_SEGMENT: &'static str = "posts";
    type Parent = User;
}
impl Resource for UserPost {
    type Id = u32;
}

pub struct FollowUser;
impl Endpoint for FollowUser {
    const URL_PATH_SEGMENT: &'static str = "follow";
    type Parent = User;
}
impl Action for FollowUser {}

pub struct UserPostComment;
impl Endpoint for UserPostComment {
    const URL_PATH_SEGMENT: &'static str = "comments";
    type Parent = UserPost;
}
impl Resource for UserPostComment {
    type Id = u32;
}
