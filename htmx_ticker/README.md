# Stocks ticker app with HTMX
I built this app to learn how easy it is to adopt HTMX and golang for creating
interactive web applications with a minimal technology stack.
Especially, I am intrigued about learning how to get rid of the inflationary
use of JavaScript frameworks for building web applications.

## Starting off
I first started building the app based on a YouTube video dubbed
[A very simple tech stack](https://youtu.be/huMTT5Pb8b8?si=Fwh0pJTLalPVXZ3w).
Since the example code in the video was not complete, I wrote the missing
pieces myself and also changed some details to have a running solution -
which was a good learning exercise.
Then I refactored the app for further experimentation and to better fit my
personal taste.

## Refactoring
### No more Fiber
I removed the [Fiber](https://gofiber.io/) dependencies, since I wanted a more
minimal solution, as close to using only the go standard library as reasonable.
[This video](https://youtu.be/F9H6vYelYyU?si=KIElXSjBUC2mnIOc)
was very helpful in understanding how to use the Go standard library package
`net/http` for hosting a simple server.

However, I adopted [gorilla mux](https://github.com/gorilla/mux) for routing,
since I wanted to deferr implementing my own logic for extracting path
parameters to a later time.

### Remove CDN dependencies
I decided that I don't want to load external dependencies via a content
delivery network (CDN).
Therefore I changed the app to serve it's own local copy of `htmx.min.js`
instead of loading the script from a remote CDN server.

### Adopt Bootstrap
The first YouTube video recommended [UnoCSS](https://unocss.dev/) for styling
the web application.
I ditched that in favor of a local copy of
[bootstrap.min.css](https://getbootstrap.com/docs/5.3/getting-started/download/).
I find [Bootstrap](https://getbootstrap.com/) to be a decent library for
getting things done.
In particular, I find it appealing for what I consider to mainly be
"backend developers" who do not want to spend significant time customizing the
style of their app, but are happy with some off-the-shelf defaults designed by
professional UI designers.
In the end, I built this for fun and am currently not that interested in
diving deeply into CSS or a popular library like *Tailwind CSS* with frontend
developers as the target group.
