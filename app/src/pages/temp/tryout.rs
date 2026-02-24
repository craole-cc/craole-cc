use crate::prelude::*;

#[component]
pub fn TryOut() -> impl IntoView {
  view! {
    <div class="bg-white">
      <div class="pt-6">
        <nav aria-label="Breadcrumb">
          <ol
            role="list"
            class="flex items-center px-4 mx-auto space-x-2 max-w-2xl sm:px-6 lg:px-8 lg:max-w-7xl"
          >
            <li>
              <div class="flex items-center">
                <a href="#" class="mr-2 text-sm font-medium text-gray-900">
                  Men
                </a>
                <svg
                  viewBox="0 0 16 20"
                  width="16"
                  height="20"
                  fill="currentColor"
                  aria-hidden="true"
                  class="w-4 h-5 text-gray-300"
                >
                  <path d="M5.697 4.34L8.98 16.532h1.327L7.025 4.341H5.697z" />
                </svg>
              </div>
            </li>
            <li>
              <div class="flex items-center">
                <a href="#" class="mr-2 text-sm font-medium text-gray-900">
                  Clothing
                </a>
                <svg
                  viewBox="0 0 16 20"
                  width="16"
                  height="20"
                  fill="currentColor"
                  aria-hidden="true"
                  class="w-4 h-5 text-gray-300"
                >
                  <path d="M5.697 4.34L8.98 16.532h1.327L7.025 4.341H5.697z" />
                </svg>
              </div>
            </li>

            <li class="text-sm">
              <a href="#" aria-current="page" class="font-medium text-gray-500 hover:text-gray-600">
                Basic Tee 6-Pack
              </a>
            </li>
          </ol>
        </nav>

        <div class="mx-auto mt-6 max-w-2xl sm:px-6 lg:grid lg:grid-cols-3 lg:gap-8 lg:px-8 lg:max-w-7xl">
          <img
            src="https://tailwindcss.com/plus-assets/img/ecommerce-images/product-page-02-secondary-product-shot.jpg"
            alt="Two each of gray, white, and black shirts laying flat."
            class="object-cover row-span-2 rounded-lg aspect-3/4 size-full max-lg:hidden"
          />
          <img
            src="https://tailwindcss.com/plus-assets/img/ecommerce-images/product-page-02-tertiary-product-shot-01.jpg"
            alt="Model wearing plain black basic tee."
            class="object-cover col-start-2 rounded-lg aspect-3/2 size-full max-lg:hidden"
          />
          <img
            src="https://tailwindcss.com/plus-assets/img/ecommerce-images/product-page-02-tertiary-product-shot-02.jpg"
            alt="Model wearing plain gray basic tee."
            class="object-cover col-start-2 row-start-2 rounded-lg aspect-3/2 size-full max-lg:hidden"
          />
          <img
            src="https://tailwindcss.com/plus-assets/img/ecommerce-images/product-page-02-featured-product-shot.jpg"
            alt="Model wearing plain white basic tee."
            class="object-cover row-span-2 sm:rounded-lg aspect-4/5 size-full lg:aspect-3/4"
          />
        </div>

        <div class="px-4 pt-10 pb-16 mx-auto max-w-2xl sm:px-6 lg:grid lg:grid-cols-3 lg:gap-x-8 lg:px-8 lg:pt-16 lg:pb-24 lg:max-w-7xl lg:grid-rows-[auto_auto_1fr]">
          <div class="lg:col-span-2 lg:pr-8 lg:border-r lg:border-gray-200">
            <h1 class="text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl">
              Basic Tee 6-Pack
            </h1>
          </div>

          <div class="mt-4 lg:row-span-3 lg:mt-0">
            <h2 class="sr-only">Product information</h2>
            <p class="text-3xl tracking-tight text-gray-900">$192</p>

            <div class="mt-6">
              <h3 class="sr-only">Reviews</h3>
              <div class="flex items-center">
                <div class="flex items-center">

                  <svg
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    data-slot="icon"
                    aria-hidden="true"
                    class="text-gray-900 size-5 shrink-0"
                  >
                    <path
                      d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401Z"
                      clip-rule="evenodd"
                      fill-rule="evenodd"
                    />
                  </svg>
                  <svg
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    data-slot="icon"
                    aria-hidden="true"
                    class="text-gray-900 size-5 shrink-0"
                  >
                    <path
                      d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401Z"
                      clip-rule="evenodd"
                      fill-rule="evenodd"
                    />
                  </svg>
                  <svg
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    data-slot="icon"
                    aria-hidden="true"
                    class="text-gray-900 size-5 shrink-0"
                  >
                    <path
                      d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401Z"
                      clip-rule="evenodd"
                      fill-rule="evenodd"
                    />
                  </svg>
                  <svg
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    data-slot="icon"
                    aria-hidden="true"
                    class="text-gray-900 size-5 shrink-0"
                  >
                    <path
                      d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401Z"
                      clip-rule="evenodd"
                      fill-rule="evenodd"
                    />
                  </svg>
                  <svg
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    data-slot="icon"
                    aria-hidden="true"
                    class="text-gray-200 size-5 shrink-0"
                  >
                    <path
                      d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401Z"
                      clip-rule="evenodd"
                      fill-rule="evenodd"
                    />
                  </svg>
                </div>
                <p class="sr-only">4 out of 5 stars</p>
                <a href="#" class="ml-3 text-sm font-medium text-indigo-600 hover:text-indigo-500">
                  117 reviews
                </a>
              </div>
            </div>

            <form class="mt-10">
              <div>
                <h3 class="text-sm font-medium text-gray-900">Color</h3>

                <fieldset aria-label="Choose a color" class="mt-4">
                  <div class="flex gap-x-3 items-center">
                    <div class="flex rounded-full outline -outline-offset-1 outline-black/10">
                      <input
                        type="radio"
                        name="color"
                        value="white"
                        checked
                        aria-label="White"
                        class="bg-white rounded-full appearance-none size-8 forced-color-adjust-none checked:outline-2 checked:outline-offset-2 checked:outline-gray-400 focus-visible:outline-3 focus-visible:outline-offset-3"
                      />
                    </div>
                    <div class="flex rounded-full outline -outline-offset-1 outline-black/10">
                      <input
                        type="radio"
                        name="color"
                        value="gray"
                        aria-label="Gray"
                        class="bg-gray-200 rounded-full appearance-none size-8 forced-color-adjust-none checked:outline-2 checked:outline-offset-2 checked:outline-gray-400 focus-visible:outline-3 focus-visible:outline-offset-3"
                      />
                    </div>
                    <div class="flex rounded-full outline -outline-offset-1 outline-black/10">
                      <input
                        type="radio"
                        name="color"
                        value="black"
                        aria-label="Black"
                        class="bg-gray-900 rounded-full appearance-none size-8 forced-color-adjust-none checked:outline-2 checked:outline-offset-2 checked:outline-gray-900 focus-visible:outline-3 focus-visible:outline-offset-3"
                      />
                    </div>
                  </div>
                </fieldset>
              </div>

              <div class="mt-10">
                <div class="flex justify-between items-center">
                  <h3 class="text-sm font-medium text-gray-900">Size</h3>
                  <a href="#" class="text-sm font-medium text-indigo-600 hover:text-indigo-500">
                    Size guide
                  </a>
                </div>

                <fieldset aria-label="Choose a size" class="mt-4">
                  <div class="grid grid-cols-4 gap-3">
                    <label
                      aria-label="XXS"
                      class="flex relative justify-center items-center p-3 bg-white rounded-md border border-gray-300 group has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25"
                    >
                      <input
                        type="radio"
                        name="size"
                        disabled
                        class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed"
                      />
                      <span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white">
                        XXS
                      </span>
                    </label>
                    <label
                      aria-label="XS"
                      class="flex relative justify-center items-center p-3 bg-white rounded-md border border-gray-300 group has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25"
                    >
                      <input
                        type="radio"
                        name="size"
                        class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed"
                      />
                      <span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white">
                        XS
                      </span>
                    </label>
                    <label
                      aria-label="S"
                      class="flex relative justify-center items-center p-3 bg-white rounded-md border border-gray-300 group has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25"
                    >
                      <input
                        type="radio"
                        name="size"
                        checked
                        class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed"
                      />
                      <span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white">
                        S
                      </span>
                    </label>
                    <label
                      aria-label="M"
                      class="flex relative justify-center items-center p-3 bg-white rounded-md border border-gray-300 group has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25"
                    >
                      <input
                        type="radio"
                        name="size"
                        class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed"
                      />
                      <span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white">
                        M
                      </span>
                    </label>
                    <label
                      aria-label="L"
                      class="flex relative justify-center items-center p-3 bg-white rounded-md border border-gray-300 group has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25"
                    >
                      <input
                        type="radio"
                        name="size"
                        class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed"
                      />
                      <span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white">
                        L
                      </span>
                    </label>
                    <label
                      aria-label="XL"
                      class="flex relative justify-center items-center p-3 bg-white rounded-md border border-gray-300 group has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25"
                    >
                      <input
                        type="radio"
                        name="size"
                        class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed"
                      />
                      <span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white">
                        XL
                      </span>
                    </label>
                    <label
                      aria-label="2XL"
                      class="flex relative justify-center items-center p-3 bg-white rounded-md border border-gray-300 group has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25"
                    >
                      <input
                        type="radio"
                        name="size"
                        class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed"
                      />
                      <span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white">
                        2XL
                      </span>
                    </label>
                    <label
                      aria-label="3XL"
                      class="flex relative justify-center items-center p-3 bg-white rounded-md border border-gray-300 group has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25"
                    >
                      <input
                        type="radio"
                        name="size"
                        class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed"
                      />
                      <span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white">
                        3XL
                      </span>
                    </label>
                  </div>
                </fieldset>
              </div>

              <button
                type="submit"
                class="flex justify-center items-center py-3 px-8 mt-10 w-full text-base font-medium text-white bg-indigo-600 rounded-md border border-transparent hover:bg-indigo-700 focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:outline-hidden"
              >
                Add to bag
              </button>
            </form>
          </div>

          <div class="py-10 lg:col-span-2 lg:col-start-1 lg:pt-6 lg:pr-8 lg:pb-16 lg:border-r lg:border-gray-200">
            <div>
              <h3 class="sr-only">Description</h3>

              <div class="space-y-6">
                <p class="text-base text-gray-900">
                  The Basic Tee 6-Pack allows you to fully express your vibrant personality with three grayscale options. Feeling adventurous? Put on a heather gray tee. Want to be a trendsetter? Try our exclusive colorway: &quot;Black&quot;. Need to add an extra pop of color to your outfit? Our white tee has you covered.
                </p>
              </div>
            </div>

            <div class="mt-10">
              <h3 class="text-sm font-medium text-gray-900">Highlights</h3>

              <div class="mt-4">
                <ul role="list" class="pl-4 space-y-2 text-sm list-disc">
                  <li class="text-gray-400">
                    <span class="text-gray-600">Hand cut and sewn locally</span>
                  </li>
                  <li class="text-gray-400">
                    <span class="text-gray-600">Dyed with our proprietary colors</span>
                  </li>
                  <li class="text-gray-400">
                    <span class="text-gray-600">Pre-washed &amp; pre-shrunk</span>
                  </li>
                  <li class="text-gray-400">
                    <span class="text-gray-600">Ultra-soft 100% cotton</span>
                  </li>
                </ul>
              </div>
            </div>

            <div class="mt-10">
              <h2 class="text-sm font-medium text-gray-900">Details</h2>

              <div class="mt-4 space-y-6">
                <p class="text-sm text-gray-600">
                  The 6-Pack includes two black, two white, and two heather gray Basic Tees. Sign up for our subscription service and be the first to get new, exciting colors, like our upcoming &quot;Charcoal Gray&quot; limited release.
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  }
}
