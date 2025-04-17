pub const INIT_HTML: &'static str = r#"
                <html>
                    <head><title>Ana Sayfa</title></head>
                    <body>
                        <h1>Merhaba, dünya!</h1>
                        <p>Bu, Rust ile yapılmış basit bir web sunucusudur.</p>
                    </body>
                </html>
            "#;

pub const ABOUT_US: &'static str = r#"
             <html>
                <head><title>Hakkında</title></head>
                <body>
                    <h1>Bu Proje Hakkında</h1>
                    <p>Rust ve Hyper kullanılarak geliştirilmiştir.</p>
                </body>
            </html>
        "#;

pub const NOT_FOUNT_404: &'static str = r#"
                <html>
                    <head><title>404</title></head>
                    <body>
                        <h1>404 - Sayfa Bulunamadı</h1>
                        <p>Aradığınız sayfa mevcut değil.</p>
                    </body>
                </html>
            "#;

pub const HTML_FORM: &'static str = r#"
    <html>
        <head><title>Form</title></head>
        <body>
            <form action="/form" method="post">
                <label>Adınız: <input name="name" /></label>
                <button type="submit">Gönder</button>
            </form>
        </body>
    </html>
"#;
