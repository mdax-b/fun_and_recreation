# Stocks ticker app with HTMX (Rust version)
I partially rebuilt the Stock Ticker HTMX app with Rust, to get a better feeling
how the experience building web applications in Rust and Go differ.

## Tech Stack
- Askama (for html templating)
- Axum (routing and http server)
- Bootstrap (make the app at least somewhat nice to look at)
- HTMX (no custom JS in the Frontend)

## First impressions
- Compile-time safety of HTML templates is awesome: once the app compiled, I
  experienced no errors.
  - This could be alleviated in Go with proper unit tests.
- More code for the same functionality

## TODO
For now, this is only a mock.
I might implement the functionality to query the real stocks ticker API later.
