use crate::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
  view! {
    <div class="bg-gray-900">
      <header class="absolute inset-x-0 top-0 z-50">
        <nav aria-label="Global" class="flex justify-between items-center p-6 lg:px-8">
          <div class="flex lg:flex-1">
            <a href="#" class="p-1.5 -m-1.5">
              <span class="sr-only">Your Company</span>
              <img
                src="https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=500"
                alt=""
                class="w-auto h-8"
              />
            </a>
          </div>
          <div class="flex lg:hidden">
            <button
              type="button"
              command="show-modal"
              commandfor="mobile-menu"
              class="inline-flex justify-center items-center p-2.5 -m-2.5 text-gray-200 rounded-md"
            >
              <span class="sr-only">Open main menu</span>
              <svg
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.5"
                data-slot="icon"
                aria-hidden="true"
                class="size-6"
              >
                <path
                  d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                />
              </svg>
            </button>
          </div>
          <div class="hidden lg:flex lg:gap-x-12">
            <a href="#" class="font-semibold text-white text-sm/6">
              Product
            </a>
            <a href="#" class="font-semibold text-white text-sm/6">
              Features
            </a>
            <a href="#" class="font-semibold text-white text-sm/6">
              Marketplace
            </a>
            <a href="#" class="font-semibold text-white text-sm/6">
              Company
            </a>
          </div>
          <div class="hidden lg:flex lg:flex-1 lg:justify-end">
            <a href="#" class="font-semibold text-white text-sm/6">
              Log in
              <span aria-hidden="true">&rarr;</span>
            </a>
          </div>
        </nav>
        <el-dialog>
          <dialog id="mobile-menu" class="lg:hidden backdrop:bg-transparent">
            <div tabindex="0" class="fixed inset-0 focus:outline-none">
              <el-dialog-panel class="overflow-y-auto fixed inset-y-0 right-0 z-50 p-6 w-full bg-gray-900 sm:max-w-sm sm:ring-1 sm:ring-gray-100/10">
                <div class="flex justify-between items-center">
                  <a href="#" class="p-1.5 -m-1.5">
                    <span class="sr-only">Your Company</span>
                    <img
                      src="https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=500"
                      alt=""
                      class="w-auto h-8"
                    />
                  </a>
                  <button
                    type="button"
                    command="close"
                    commandfor="mobile-menu"
                    class="p-2.5 -m-2.5 text-gray-200 rounded-md"
                  >
                    <span class="sr-only">Close menu</span>
                    <svg
                      viewBox="0 0 24 24"
                      fill="none"
                      stroke="currentColor"
                      stroke-width="1.5"
                      data-slot="icon"
                      aria-hidden="true"
                      class="size-6"
                    >
                      <path
                        d="M6 18 18 6M6 6l12 12"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                      />
                    </svg>
                  </button>
                </div>
                <div class="flow-root mt-6">
                  <div class="-my-6 divide-y divide-white/10">
                    <div class="py-6 space-y-2">
                      <a
                        href="#"
                        class="block py-2 px-3 -mx-3 font-semibold text-white rounded-lg text-base/7 hover:bg-white/5"
                      >
                        Product
                      </a>
                      <a
                        href="#"
                        class="block py-2 px-3 -mx-3 font-semibold text-white rounded-lg text-base/7 hover:bg-white/5"
                      >
                        Features
                      </a>
                      <a
                        href="#"
                        class="block py-2 px-3 -mx-3 font-semibold text-white rounded-lg text-base/7 hover:bg-white/5"
                      >
                        Marketplace
                      </a>
                      <a
                        href="#"
                        class="block py-2 px-3 -mx-3 font-semibold text-white rounded-lg text-base/7 hover:bg-white/5"
                      >
                        Company
                      </a>
                    </div>
                    <div class="py-6">
                      <a
                        href="#"
                        class="block py-2.5 px-3 -mx-3 font-semibold text-white rounded-lg text-base/7 hover:bg-white/5"
                      >
                        Log in
                      </a>
                    </div>
                  </div>
                </div>
              </el-dialog-panel>
            </div>
          </dialog>
        </el-dialog>
      </header>

      <div class="relative px-6 pt-14 lg:px-8 isolate">
        <div
          aria-hidden="true"
          class="overflow-hidden absolute inset-x-0 -top-40 transform-gpu sm:-top-80 -z-10 blur-3xl"
        >
          <div
            style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"
            class="relative opacity-30 -translate-x-1/2 left-[calc(50%-11rem)] aspect-1155/678 w-144.5 rotate-30 bg-linear-to-tr from-[#ff80b5] to-[#9089fc] sm:left-[calc(50%-30rem)] sm:w-288.75"
          ></div>
        </div>
        <div class="py-32 mx-auto max-w-2xl sm:py-48 lg:py-56">
          <div class="hidden sm:flex sm:justify-center sm:mb-8">
            <div class="relative py-1 px-3 text-gray-400 rounded-full ring-1 text-sm/6 ring-white/10 hover:ring-white/20">
              Announcing our next round of funding.
              <a href="#" class="font-semibold text-indigo-400">
                <span aria-hidden="true" class="absolute inset-0"></span>
                Read more
                <span aria-hidden="true">&rarr;</span>
              </a>
            </div>
          </div>
          <div class="text-center">
            <h1 class="text-5xl font-semibold tracking-tight text-white sm:text-7xl text-balance">
              Data to enrich your online business
            </h1>
            <p class="mt-8 text-lg font-medium text-gray-400 text-pretty sm:text-xl/8">
              Anim aute id magna aliqua ad ad non deserunt sunt. Qui irure qui lorem cupidatat commodo. Elit sunt amet fugiat veniam occaecat.
            </p>
            <div class="flex gap-x-6 justify-center items-center mt-10">
              <a
                href="#"
                class="py-2.5 px-3.5 text-sm font-semibold text-white bg-indigo-500 rounded-md hover:bg-indigo-400 shadow-xs focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
              >
                Get started
              </a>
            // <a href="#" class="text-sm/6 font-semibold text-white">Learn more <span aria-hidden="true">→</span></a>
            </div>
          </div>
        </div>
        <div
          aria-hidden="true"
          class="overflow-hidden absolute inset-x-0 transform-gpu top-[calc(100%-13rem)] -z-10 blur-3xl sm:top-[calc(100%-30rem)]"
        >
          <div
            style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"
            class="relative opacity-30 -translate-x-1/2 left-[calc(50%+3rem)] aspect-1155/678 w-144.5 bg-linear-to-tr from-[#ff80b5] to-[#9089fc] sm:left-[calc(50%+36rem)] sm:w-288.75"
          ></div>
        </div>
      </div>
    </div>
  }
}
