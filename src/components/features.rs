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
                    has_illustration=true
                    name="alteryx"
                    class_name="md:col-span-3 md:row-span-3 min-h-card_1_row_mobile md:min-h-card_1_row"
                />
                <Card
                    name="splashsports"
                    class_name="md:col-span-2 md:row-span-3 min-h-card_1_row_mobile md:min-h-card_1_row"
                />
                <Card
                    name="oms"
                    class_name="md:col-span-2 md:row-span-4 min-h-card_2_row_mobile md:min-h-card_2_row"
                    has_illustration=true
                />
                <Card
                    name="helpie"
                    class_name="md:col-span-3 md:row-span-2 min-h-card_2_row_mobile"
                />
                <Card
                    name="invaders"
                    class_name="md:col-span-1 md:row-span-2 bg-gray-8 min-h-card_2_row_mobile"
                />
                <Card
                    name="madesense"
                    class_name="md:col-span-2 md:row-span-2 min-h-card_2_row_mobile"
                />
            </div>
        </Layout>
    }
}

