use crate::components::ui::assets::{
    ayx_logo::AyxLogo, helpie_logo::HelpieLogo, invaders_logo::InvadersLogo,
    madesense_logo::MadesenseLogo, oms_logo::OmsLogo, splash_logo::SplashLogo,
};
use crate::components::ui::{card::Card, layout::Layout};

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
                    name="alteryx"
                    class_name="md:col-span-3 md:row-span-3 min-h-card_1_row_mobile md:min-h-card_1_row"
                >
                    <div class="h-full flex justify-center items-center">
                        <AyxLogo/>
                    </div>
                // <div class="absolute bottom-0">
                // <AyxVector/>
                // </div>
                </Card>
                <Card
                    name="oms"
                    class_name="md:col-span-2 md:row-span-3 min-h-card_1_row_mobile md:min-h-card_1_row"
                >
                    <div class="h-full flex justify-center items-center">
                        <OmsLogo/>
                    </div>
                </Card>
                <Card
                    name="madesense"
                    class_name="md:col-span-2 md:row-span-4 min-h-card_2_row_mobile md:min-h-card_2_row"
                >
                    <div class="h-full flex justify-center items-center">
                        <MadesenseLogo/>
                    </div>
                </Card>
                <Card
                    name="splashsports"
                    class_name="md:col-span-3 md:row-span-2 min-h-card_2_row_mobile"
                >
                    <div class="h-full flex justify-center items-center">
                        <SplashLogo/>
                    </div>
                </Card>
                <Card
                    name="invaders"
                    class_name="md:col-span-1 md:row-span-2 bg-gray-8 min-h-card_2_row_mobile"
                >
                    <div class="h-full flex justify-center items-center">
                        <InvadersLogo/>
                    </div>
                </Card>
                <Card
                    name="helpie"
                    class_name="md:col-span-2 md:row-span-2 min-h-card_2_row_mobile"
                >
                    <div class="h-full flex justify-center items-center">
                        <HelpieLogo/>
                    </div>
                </Card>
            </div>
        </Layout>
    }
}

