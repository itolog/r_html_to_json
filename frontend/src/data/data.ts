export const htmlStr  = `<!DOCTYPE html><html lang="en">
  <head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1">  
    <meta name="description" content="">
    <meta name="author" content="">
    <link rel="icon" href="mysource_files/favicon.ico">
    <title>Page Name - Squiz Matrix HTML Example</title>
    <link href="https://maxcdn.bootstrapcdn.com/bootstrap/3.3.6/css/bootstrap.min.css" rel="stylesheet">
    <link href="mysource_files/style.css" rel="stylesheet">
  </head>

  <body>

    <div class="container">

      <header class="header clearfix" style="background-color: #ffffff">

        <nav>
          <ul class="nav nav-pills pull-right">
            <li class="active"><a href="#">Home</a></li>
            <li><a href="#">About</a></li>
            <li><a href="#">Contact</a></li>
          </ul>
        </nav>
        <h1 class="h3 text-muted">Site Name</h1>
        <ol class="breadcrumb">
          <li><a href="#">Home</a></li>
          <li><a href="#">Level 1</a></li>
          <li class="active">Level 2</li>
        </ol>

      </header>

      <div class="page-heading">
        <h1>Page Heading</h1>

      </div>

      <div class="row">

        <div class="col-sm-3">
          <ul class="nav nav-pills nav-stacked">
            <li><a href="#">Level 2</a></li>
            <li class="active"><a href="#">Level 2</a>
              <ul>
                <li><a href="#">Level 3</a></li>
                <li><a href="#">Level 3</a></li>
                <li><a href="#">Level 3</a></li>
              </ul>
            </li>
            <li><a href="#">Level 2</a></li>
          </ul>

        </div>

        <div class="col-sm-6">

          <div class="page-contents">
            <h2>Sub Heading</h2>
            <p>Donec id elit non mi porta gravida at eget metus. Maecenas faucibus mollis interdum.</p>
            <h4>Sub Heading</h4>
            <p>Morbi leo risus, porta ac consectetur ac, vestibulum at eros. Cras mattis consectetur purus sit amet fermentum.</p>
            <h4>Sub Heading</h4>
            <p>Maecenas sed diam eget risus varius blandit sit amet non magna.</p>

          </div>

        </div>

        <div class="col-sm-3">
          <h2>Login</h2>          
          <h2>Search</h2>

        </div>

      </div>

      <footer class="footer">
        <p class="pull-right">
          Last Updated: Wednesday, January 6, 2016
        </p>
        <p>&copy; 2016 Company, Inc.</p>
      </footer>
    </div>
  </body>
</html>`

export const htmlStr2 = `<!doctype html>
    <html lang="en">
        <head>
            <meta charset="utf-8">
            <title>Html parser</title>
        </head>
        <body>
            <h1 data-data="data" id="a" class="b c">Hello world</h1>
        </body>
    </html>`

export const htmlStr3 =` <div class="container">
      <ul data-data="data" >
          <li>Hello <strong>World</strong></li>
      </ul>
    </div>`