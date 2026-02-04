use actix_web::{web, HttpResponse, HttpRequest};
use actix_web::http::header;
use serde_json::json;
use validator::Validate;

use crate::{AppState, models::*, auth, utils};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/posts", web::get().to(get_posts))
            .route("/posts", web::post().to(create_post))
            .route("/posts/{slug}", web::get().to(get_post))
            .route("/posts/{id}", web::put().to(update_post))
            .route("/posts/{id}", web::delete().to(delete_post))
            .route("/posts/{slug}/comments", web::get().to(get_comments))
            .route("/posts/{slug}/comments", web::post().to(create_comment))
            .route("/comments/{id}/approve", web::post().to(approve_comment))
            .route("/comments/{id}", web::delete().to(delete_comment))
    )
    .service(
        web::scope("")
            .route("/", web::get().to(index))
            .route("/post/{slug}", web::get().to(view_post))
            .route("/admin", web::get().to(admin_panel))
    );
}

async fn register(
    state: web::Data<AppState>,
    req: web::Json<RegisterRequest>,
) -> HttpResponse {
    if let Err(e) = req.validate() {
        return HttpResponse::BadRequest().json(json!({"error": e.to_string()}));
    }

    if state.db.get_user_by_username(&req.username).await.unwrap().is_some() {
        return HttpResponse::BadRequest().json(json!({"error": "Username already exists"}));
    }

    if state.db.get_user_by_email(&req.email).await.unwrap().is_some() {
        return HttpResponse::BadRequest().json(json!({"error": "Email already exists"}));
    }

    let password_hash = match bcrypt::hash(&req.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Failed to hash password"})),
    };

    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: req.username.clone(),
        email: req.email.clone(),
        password_hash,
        is_admin: false,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    match state.db.create_user(&user).await {
        Ok(_) => HttpResponse::Created().json(json!({"message": "User created successfully"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Failed to create user"})),
    }
}

async fn login(
    state: web::Data<AppState>,
    req: web::Json<LoginRequest>,
) -> HttpResponse {
    let user = match state.db.get_user_by_username(&req.username).await {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::Unauthorized().json(json!({"error": "Invalid credentials"})),
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Database error"})),
    };

    if !bcrypt::verify(&req.password, &user.password_hash).unwrap_or(false) {
        return HttpResponse::Unauthorized().json(json!({"error": "Invalid credentials"}));
    }

    let token = match auth::create_token(&user.id, &user.username, user.is_admin, &state.jwt_secret) {
        Ok(token) => token,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Failed to create token"})),
    };

    HttpResponse::Ok().json(json!({"token": token, "username": user.username, "is_admin": user.is_admin}))
}

async fn get_posts(state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let is_admin = auth::verify_admin(&req, &state.jwt_secret);
    let posts = match state.db.get_all_posts(!is_admin).await {
        Ok(posts) => posts,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch posts"})),
    };

    HttpResponse::Ok().json(posts)
}

async fn create_post(
    state: web::Data<AppState>,
    req: HttpRequest,
    post_req: web::Json<CreatePostRequest>,
) -> HttpResponse {
    if !auth::verify_admin(&req, &state.jwt_secret) {
        return HttpResponse::Unauthorized().json(json!({"error": "Admin access required"}));
    }

    if let Err(e) = post_req.validate() {
        return HttpResponse::BadRequest().json(json!({"error": e.to_string()}));
    }

    let claims = match auth::extract_claims(&req, &state.jwt_secret) {
        Some(claims) => claims,
        None => return HttpResponse::Unauthorized().json(json!({"error": "Invalid token"})),
    };

    let slug = utils::slugify(&post_req.title);
    
    if state.db.get_post_by_slug(&slug).await.unwrap().is_some() {
        return HttpResponse::BadRequest().json(json!({"error": "Post with this title already exists"}));
    }

    let post = Post {
        id: uuid::Uuid::new_v4().to_string(),
        title: post_req.title.clone(),
        slug,
        content: post_req.content.clone(),
        summary: post_req.summary.clone(),
        author_id: claims.sub,
        published: post_req.published,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    match state.db.create_post(&post).await {
        Ok(_) => HttpResponse::Created().json(post),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Failed to create post"})),
    }
}

async fn get_post(state: web::Data<AppState>, slug: web::Path<String>) -> HttpResponse {
    match state.db.get_post_by_slug(&slug).await {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "Post not found"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Database error"})),
    }
}

async fn update_post(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<String>,
    post_req: web::Json<CreatePostRequest>,
) -> HttpResponse {
    if !auth::verify_admin(&req, &state.jwt_secret) {
        return HttpResponse::Unauthorized().json(json!({"error": "Admin access required"}));
    }

    if let Err(e) = post_req.validate() {
        return HttpResponse::BadRequest().json(json!({"error": e.to_string()}));
    }

    let mut post = match state.db.get_post_by_slug(&id).await {
        Ok(Some(post)) => post,
        Ok(None) => return HttpResponse::NotFound().json(json!({"error": "Post not found"})),
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Database error"})),
    };

    post.title = post_req.title.clone();
    post.content = post_req.content.clone();
    post.summary = post_req.summary.clone();
    post.published = post_req.published;
    post.updated_at = chrono::Utc::now().to_rfc3339();

    match state.db.update_post(&post).await {
        Ok(_) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Failed to update post"})),
    }
}

async fn delete_post(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<String>,
) -> HttpResponse {
    if !auth::verify_admin(&req, &state.jwt_secret) {
        return HttpResponse::Unauthorized().json(json!({"error": "Admin access required"}));
    }

    match state.db.delete_post(&id).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Post deleted"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Failed to delete post"})),
    }
}

async fn get_comments(
    state: web::Data<AppState>,
    req: HttpRequest,
    slug: web::Path<String>,
) -> HttpResponse {
    let is_admin = auth::verify_admin(&req, &state.jwt_secret);
    
    let post = match state.db.get_post_by_slug(&slug).await {
        Ok(Some(post)) => post,
        Ok(None) => return HttpResponse::NotFound().json(json!({"error": "Post not found"})),
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Database error"})),
    };

    let comments = match state.db.get_comments_by_post(&post.id, !is_admin).await {
        Ok(comments) => comments,
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Failed to fetch comments"})),
    };

    HttpResponse::Ok().json(comments)
}

async fn create_comment(
    state: web::Data<AppState>,
    slug: web::Path<String>,
    comment_req: web::Json<CreateCommentRequest>,
) -> HttpResponse {
    if let Err(e) = comment_req.validate() {
        return HttpResponse::BadRequest().json(json!({"error": e.to_string()}));
    }

    let post = match state.db.get_post_by_slug(&slug).await {
        Ok(Some(post)) => post,
        Ok(None) => return HttpResponse::NotFound().json(json!({"error": "Post not found"})),
        Err(_) => return HttpResponse::InternalServerError().json(json!({"error": "Database error"})),
    };

    let comment = Comment {
        id: uuid::Uuid::new_v4().to_string(),
        post_id: post.id,
        author_name: comment_req.author_name.clone(),
        author_email: comment_req.author_email.clone(),
        content: comment_req.content.clone(),
        approved: false,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    match state.db.create_comment(&comment).await {
        Ok(_) => HttpResponse::Created().json(json!({"message": "Comment submitted for approval"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Failed to create comment"})),
    }
}

async fn approve_comment(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<String>,
) -> HttpResponse {
    if !auth::verify_admin(&req, &state.jwt_secret) {
        return HttpResponse::Unauthorized().json(json!({"error": "Admin access required"}));
    }

    match state.db.approve_comment(&id).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Comment approved"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Failed to approve comment"})),
    }
}

async fn delete_comment(
    state: web::Data<AppState>,
    req: HttpRequest,
    id: web::Path<String>,
) -> HttpResponse {
    if !auth::verify_admin(&req, &state.jwt_secret) {
        return HttpResponse::Unauthorized().json(json!({"error": "Admin access required"}));
    }

    match state.db.delete_comment(&id).await {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Comment deleted"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Failed to delete comment"})),
    }
}

async fn index(
    state: web::Data<AppState>,
    tmpl: web::Data<tera::Tera>,
) -> HttpResponse {
    let posts = match state.db.get_all_posts(true).await {
        Ok(posts) => posts,
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    let mut ctx = tera::Context::new();
    ctx.insert("posts", &posts);
    ctx.insert("title", "Blog Home");

    match tmpl.render("blog/index.html", &ctx) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().body("Template error"),
    }
}

async fn view_post(
    state: web::Data<AppState>,
    tmpl: web::Data<tera::Tera>,
    slug: web::Path<String>,
) -> HttpResponse {
    let post = match state.db.get_post_by_slug(&slug).await {
        Ok(Some(post)) => post,
        Ok(None) => return HttpResponse::NotFound().body("Post not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    let comments = match state.db.get_comments_by_post(&post.id, true).await {
        Ok(comments) => comments,
        Err(_) => vec![],
    };

    let html_content = utils::markdown_to_html(&post.content);

    let mut ctx = tera::Context::new();
    ctx.insert("post", &post);
    ctx.insert("comments", &comments);
    ctx.insert("html_content", &html_content);
    ctx.insert("title", &post.title);

    match tmpl.render("blog/post.html", &ctx) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().body("Template error"),
    }
}

async fn admin_panel(
    state: web::Data<AppState>,
    tmpl: web::Data<tera::Tera>,
    req: HttpRequest,
) -> HttpResponse {
    if !auth::verify_admin(&req, &state.jwt_secret) {
        return HttpResponse::Unauthorized()
            .insert_header((header::LOCATION, "/"))
            .finish();
    }

    let posts = match state.db.get_all_posts(false).await {
        Ok(posts) => posts,
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    let mut ctx = tera::Context::new();
    ctx.insert("posts", &posts);
    ctx.insert("title", "Admin Panel");

    match tmpl.render("admin/dashboard.html", &ctx) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(_) => HttpResponse::InternalServerError().body("Template error"),
    }
}
