use axum::{response::Html, routing::get, Router};
use leptos::view;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

  // build an axum application with a route
  let app = Router::new().route("/", get(home));

  // run app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  println!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn home() -> Html<String> {
  Html(leptos::ssr::render_to_string(|cx| {
    view! { cx,
      <html>
        <head>
          <script src="https://cdn.tailwindcss.com"></script>
        </head>
        <body class="text-slate-500 min-h-screen">
          <header class="pb-20 sm:pb-24 lg:pb-32 bg-gradient-to-tr from-indigo-500/[.06] from-10% via-sky-500/[.06] via-30% to-emerald-500/[.06] to-90%">
            <div class="px-4 sm:px-6 md:px-8">
              <div class="pt-6 lg:pt-8 flex items-center justify-between text-slate-700 font-semibold text-sm leading-6">
                <a href="/" class="text-lg font-semibold">{"htmx-axum-leptos"}</a>
                <a target="_blank" rel="noopener" href="https://github.com/ayrock-dev/htmx-axum-leptos" class="text-slate-400 hover:text-slate-500">
                  <span class="sr-only">{"htmx-axum-leptos on GitHub"}</span>
                  <svg viewBox="0 0 16 16" class="w-5 h-5" fill="currentColor" aria-hidden="true"><path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
                </a>
              </div>
            </div>
            <div class="relative max-w-5xl mx-auto pt-20 sm:pt-24 lg:pt-32">
              <h1 class="text-slate-900 font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight text-center">
                {"htmx-axum-leptos"}
              </h1>
            </div>
            <div class="mt-6 sm:mt-10 flex justify-center space-x-6 text-sm">
              <p class="mt-6 text-lg font-semibold text-slate-900 text-center max-w-3xl mx-auto">
                {"A Rust frontend? Who me? 🦀"}
              </p>
            </div>
          </header>
          <section class="text-center px-8 my-20 sm:my-32 md:my-40">
            <h2 class="text-slate-900 text-4xl tracking-tight font-extrabold sm:text-5xl">
              {"“Rewrite it in OCaml”"}
            </h2>
            <figure>
              <blockquote>
                <p class="mt-6 max-w-3xl mx-auto text-lg">
                  {"A thing inspired by "}<a href="https://twitter.com/ThePrimeagen">{"ThePrimeagen"}</a>.
                </p>
              </blockquote>
              <figcaption class="mt-6 flex items-center justify-center space-x-4 text-left">
                <img src="https://pbs.twimg.com/profile_images/1614986714795180033/yOQly3os_200x200.jpg" alt="ThePrimeagen" class="w-14 h-14 rounded-full" loading="lazy" decoding="async" />
                <div>
                  <div class="text-slate-900 font-semibold">{"ThePrimeagen"}</div><div class="mt-0.5 text-sm leading-6">
                    {"Filthy Milk Drinker"}
                  </div>
                </div>
              </figcaption>
            </figure>
          </section>
          <section tabindex="-1" class="relative max-w-7xl mx-auto px-4 focus:outline-none sm:px-3 md:px-5">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 md:px-8">
              <p class="mt-4 text-3xl sm:text-4xl text-slate-900 font-extrabold tracking-tight">{"The stack built for 🦀 Rustaceans. "}</p>
            </div>
            <div class="mt-4 grid grid-cols-1 gap-6 lg:gap-8 sm:grid-cols-2 lg:grid-cols-3 max-h-[33rem] overflow-hidden text-sm leading-6">
                  <div class="bg-slate-50 rounded-lg p-6"><p><span class="text-base text-slate-900 font-semibold">{"htmx"}</span><br />{"client markup + interactivity"}</p></div>

                  <div class="bg-slate-50 rounded-lg p-6"><p><span class="text-base text-slate-900 font-semibold">{"axum"}</span><br />{"web server"}</p></div>

                  <div class="bg-slate-50 rounded-lg p-6"><p><span class="text-base text-slate-900 font-semibold">{"leptos"}</span><br />{"ssr"}</p></div>

                  <div class="bg-slate-50 rounded-lg p-6"><p><span class="text-base text-slate-900 font-semibold">{"TailwindCSS"}</span><br />{"because tech twitter said so"}</p></div>

            </div>
          </section>
          <footer class="pb-16 text-sm leading-6">
            <div class="max-w-7xl mx-auto divide-y divide-slate-200 px-4 sm:px-6 md:px-8">
              <div class="mt-16 pt-10">
                <a href="/" class="text-lg font-semibold">{"htmx-axum-leptos"}</a>
              </div>
            </div>
          </footer>
          <script src="https://unpkg.com/htmx.org/dist/htmx.min.js"></script>
        </body>
      </html>
    }
  }))
}
