use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
      <div class="bg-white">
        <header class="absolute inset-x-0 top-0 z-50">
            <nav class="mx-auto flex max-w-7xl items-center justify-between p-6 lg:px-8" aria-label="Global">
               <h4 class="text-black text-md sm:text-xl dark:text-white flex lg:flex-1">
                  "Hey, I’m Erik Kurjak 👋"
               </h4>
               <div class="items-center justify-end gap-1 flex lg:flex-1">
                  <h4 hx-get="link.html" hx-trigger="click" hx-target="#see_more" class="cursor-pointer text-gray text-sm sm:text-md dark:text-gray">
                     This website was build using Rust and HTMX
                  </h4>
                  <div id="see_more" class="items-center justify-end" hx-trigger="once" hx-get="link.html" />
               </div>
            </nav>
        </header>
        <main>
        <div class="relative isolate">
              <div class="overflow-hidden">
                 <div class="mx-auto max-w-7xl px-6 pb-32 pt-36 sm:pt-60 lg:px-8 lg:pt-72">
                    <div class="mx-auto max-w-2xl gap-x-14 lg:mx-0 lg:flex lg:max-w-none lg:items-center">
                       <div class="w-full">
                          <h1 class="text-6xl tracking-tight text-gray-900 sm:text-10xl leading-tighter">
                             I <span class="font-light">develop</span>
                             <br /> digital products
                             <br /> from scratch
                          </h1>
                       </div>
                    </div>
                 </div>
              </div>
           </div>

           <div class="relative isolate">
           <div class="overflow-hidden">
              <div class="mx-auto max-w-7xl px-6 pb-32 pt-36 sm:pt-60 lg:px-8 lg:pt-60">
                  <div class="mx-auto max-w-2xl gap-x-14 lg:mx-0 lg:flex lg:max-w-none lg:items-center">
                     <div class="mx-auto max-w-2xl gap-x-14 lg:mx-0 lg:flex lg:max-w-none lg:items-center">
                        <div class="w-full max-w-xl lg:shrink-0 xl:max-w-2xl">
                           <img class="w-96 h-96 bg-cover bg-no-repeat bg-center rounded-full" style="background-image: url(assets/me.jpg)" title="me" alt="erik kurjak" />
                           Open to new challenges
                        </div>
                        <div class="mt-14 flex flex-col justify-end gap-8 sm:-mt-44 sm:justify-start sm:pl-20 lg:mt-0 lg:pl-0">
                           <p class="relative mt-0 mb-6 text-2xl leading-relaxish text-black sm:max-w-md lg:max-w-none">
                              A team player with a passion for building modern digital products. With a strong affinity for functional programming and a natural problem-solving ability.
                           </p>
                           <button type="button" class="rounded-full font-normal text-2xl bg-white px-4 py-16 text-black shadow-sm ring-1 ring-inset ring-gray hover:bg-black hover:text-white">"Let's connect"</button>
                        </div>
                     </div>
                 </div>
              </div>
           </div>
        </div>
        </main>
     </div>
    }
}
