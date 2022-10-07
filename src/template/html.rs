pub const BASE: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">

    <link rel="stylesheet" href="./style.css">
    <link rel="icon" href="./favicon.ico" type="image/x-icon">
    <script src="index.js"></script>
    <title>My Website</title>

  </head>
  <body>
    <main>
      <h1>Welcome to My {{ project }}</h1>
    </main>
  </body>
</html>
"#;
