use leptos::prelude::*;

#[component]
pub fn Test() -> impl IntoView {
  let (count, set_count) = signal(0);
  let on_click = move |_| set_count.update(|count| *count += 1);

  view! {
    <p>"This is a test."</p>
    <button on:click=move |_| { set_count.set(1) }>"Click me: " {move || count.get()}</button>
    <p>"Double count: " {move || count.get() * 2}</p>

    <button
      on:click=move |_| {
        *set_count.write() += 1;
      }
      // the class: syntax reactively updates a single class
      // here, we'll set the `red` class when `count` is odd
      class:red=move || count.get() % 2 == 1
    >
      "Click me"
    </button>

    <button
      on:click=on_click
      // on:click=move |_| {
      // *set_count.write() += 10;
      // }
      // set the `style` attribute
      style="position: absolute"
      // and toggle individual CSS properties with `style:`
      style:left=move || format!("{}px", count.get() + 100)
      style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
      style:max-width="400px"
      // Set a CSS variable for stylesheet use
      style=("--columns", move || count.get().to_string())
    >
      "Click to Move"
    </button>

    <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
    <p class="px-10 pb-10 text-left">
      "Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."
    </p>
    <button
      class="py-3 px-5 text-white rounded-lg bg-sky-600 hover:bg-sky-700"
      on:click=move |_| *set_count.write() += 1
    >
      {move || if count.get() == 0 { "Click me!".to_string() } else { count.get().to_string() }}
    </button>

    <div class="flex flex-col min-h-screen font-mono text-white from-blue-800 to-blue-500 bg-linear-to-tl">
      <div class="flex flex-row-reverse flex-wrap m-auto">
        <button
          on:click=on_click
          class="py-2 px-3 m-1 text-white bg-blue-700 rounded border-l-2 border-b-4 border-blue-800 shadow-lg"
        >
          "Click number "
          {count}
        </button>
      </div>
    </div>
  }
}
