use axum::{extract::Path, Json};

use crate::models::post::Post;

pub async fn list_posts() -> Json<Vec<Post>> {
    let posts = vec![
        Post {
            id: 1,
            title: "İlk Yazı".to_string(),
            content: "# Merhaba\nBu bir markdown yazısı.".to_string(),
        },
        Post {
            id: 2,
            title: "İkinci Yazı".to_string(),
            content: "## Alt Başlık\nDaha fazla içerik.".to_string(),
        },
    ];

    Json(posts)
}

pub async fn get_post_by_id(Path(id): Path<i32>) -> Json<Option<Post>> {
    let posts = vec![
        Post {
            id: 1,
            title: "İlk Yazı".to_string(),
            content: "# Merhaba\nBu bir markdown yazısı.".to_string(),
        },
        Post {
            id: 2,
            title: "İkinci Yazı".to_string(),
            content: "## Alt Başlık\nDaha fazla içerik.".to_string(),
        },
    ];

    let post = posts.into_iter().find(|p| p.id == id);
    Json(post)
}
