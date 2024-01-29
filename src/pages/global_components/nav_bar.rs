use leptos::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav
            aria-label="Main"
            data-orientation="horizontal"
            dir="ltr"
            class="relative z-10 flex max-w-max flex-1 items-center justify-center"
        >
            <div style="position: relative;">
                <ul
                    data-orientation="horizontal"
                    class="group flex flex-1 list-none items-center justify-center space-x-1"
                    dir="ltr"
                    style="list-style-type: none;"
                >
                    <li>
                        <a
                            href="#"
                            class="group inline-flex h-9 w-max items-center justify-center rounded-md bg-white px-4 py-2 text-sm font-medium transition-colors hover:bg-gray-100 hover:text-gray-900 focus:bg-gray-100 focus:text-gray-900 focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[active]:bg-gray-100/50 data-[state=open]:bg-gray-100/50 dark:bg-gray-950 dark:hover:bg-gray-800 dark:hover:text-gray-50 dark:focus:bg-gray-800 dark:focus:text-gray-50 dark:data-[active]:bg-gray-800/50 dark:data-[state=open]:bg-gray-800/50"
                            data-radix-collection-item=""
                        >
                            Home
                        </a>
                    </li>
                    <li>
                        <button
                            id="radix-:rd:-trigger-radix-:rf:"
                            data-state="closed"
                            aria-expanded="false"
                            aria-controls="radix-:rd:-content-radix-:rf:"
                            class="group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground focus:outline-none disabled:pointer-events-none disabled:opacity-50 data-[active]:bg-accent/50 data-[state=open]:bg-accent/50 group"
                            data-radix-collection-item=""
                        >
                            AWS
                            <svg
                                width="15"
                                height="15"
                                viewBox="0 0 15 15"
                                fill="none"
                                xmlns="http://www.w3.org/2000/svg"
                                class="relative top-[1px] ml-1 h-3 w-3 transition duration-300 group-data-[state=open]:rotate-180"
                                aria-hidden="true"
                            >
                                <path
                                    d="M3.13523 6.15803C3.3241 5.95657 3.64052 5.94637 3.84197 6.13523L7.5 9.56464L11.158 6.13523C11.3595 5.94637 11.6759 5.95657 11.8648 6.15803C12.0536 6.35949 12.0434 6.67591 11.842 6.86477L7.84197 10.6148C7.64964 10.7951 7.35036 10.7951 7.15803 10.6148L3.15803 6.86477C2.95657 6.67591 2.94637 6.35949 3.13523 6.15803Z"
                                    fill="currentColor"
                                    fill-rule="evenodd"
                                    clip-rule="evenodd"
                                >
                                    <ul class="hidden absolute left-0 mt-2 space-y-2 bg-white border rounded-md shadow-md">
                                        <li>
                                            <a href="/s3">S3 Bucket</a>
                                        </li>
                                        <li>
                                            <a href="/lt">Launch Template</a>
                                        </li>
                                        <li>
                                            <a href="/lb">Load Balancer</a>
                                        </li>
                                        <li>
                                            <a href="/asg">ASG</a>
                                        </li>
                                        <li>
                                            <a href="/cert">Certificates</a>
                                        </li>
                                        <li>
                                            <a href="/ec2">Launch EC2</a>
                                        </li>
                                    </ul>
                                </path>
                            </svg>
                        </button>
                    </li>
                </ul>
            </div>
            <div class="absolute left-0 top-full flex justify-center"></div>
        </nav>
    }
}
