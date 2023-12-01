use crate::components::ui::card::Card;
use crate::components::ui::layout::Layout;

use leptos::*;

#[component]
pub fn Features() -> impl IntoView {
    view! {
        <Layout aria_label="Features" class_name="flex-col">
            <h1 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl tracking-tight text-gray-9 leading-tighter">
                Featured <br/> <span class="font-light">work</span> experience <br/> and
                <span class="font-light">projects</span>
            </h1>

            <div class="grid gap-4 md:grid-cols-5 md:grid-rows-7 mt-20 md:mt-40">
                <Card
                    style="height:400px"
                    has_illustration=true
                    name="alteryx"
                    class_name="md:col-span-3 md:row-span-3"
                >
                    <span></span>
                </Card>
                <Card
                    style="height:400px"
                    name="splashsports"
                    class_name="md:col-span-2 md:row-span-3"
                >
                    <span></span>
                </Card>
                <Card
                    name="onemanshowfoundation"
                    class_name="md:col-span-2 md:row-span-4"
                    style="height:620px"
                    has_illustration=true
                >
                    <span></span>
                </Card>
            // <Card name="helpie" class_name="md:col-span-3 md:row-span-2">
            // <span></span>
            // </Card>
            // <Card name="invaders" class_name="md:col-span-1 md:row-span-2 bg-gray-8">
            // <span></span>
            // </Card>
            // <Card name="madesense" class_name="md:col-span-2 md:row-span-2">
            // <span></span>
            // </Card>
            </div>
        </Layout>
    }
}

