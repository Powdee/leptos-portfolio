use leptos::*;

#[component]
pub fn SplashLogo() -> impl IntoView {
    view! {
        <svg
            class="lg:w-[446px] w-[200px]"
            width="446"
            height="48"
            viewBox="0 0 446 48"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <g clip-path="url(#clip0_636_151)">
                <path
                    d="M30.7363 7.91369C30.9353 6.9029 31.0596 5.86747 31.0596 4.80737H24.0967C21.8337 4.80737 19.6951 5.81816 18.2777 7.5932L15.5422 10.9707L12.8068 7.5932C11.5634 6.06469 9.77296 5.07856 7.83329 4.85668C7.68408 4.83203 7.55974 4.83203 7.41054 4.83203C7.26133 4.83203 7.13699 4.83203 6.98779 4.83203H0C0 6.16331 0.174073 7.44528 0.497352 8.67794C0.621689 9.17101 0.770895 9.66407 0.944968 10.1325C1.04444 10.379 1.11904 10.6009 1.21851 10.8228C3.58093 16.3205 9.10153 20.2157 15.5422 20.2157C23.0274 20.2157 29.294 14.9399 30.7363 7.91369Z"
                    fill="#dad6ca"
                ></path>
                <path
                    d="M97.0582 17.0847C96.7349 15.9999 96.3122 14.9645 95.8148 13.9537C95.7402 13.7811 95.6407 13.6332 95.5661 13.4607C95.3921 13.1402 95.218 12.795 95.0439 12.4745C94.7704 11.9815 94.472 11.5377 94.1487 11.0693C89.9958 4.95526 82.9583 0.961426 74.9509 0.961426C67.9134 0.961426 62.0695 2.90904 57.5685 8.45603C56.7478 9.49147 55.9521 10.6255 55.2061 11.9075C55.0817 12.1047 54.9823 12.302 54.8828 12.4992C54.6838 12.8936 54.4849 13.3127 54.286 13.7318C54.1865 13.9537 54.087 14.1509 54.0124 14.3728C51.5257 20.4622 50.7299 28.3759 44.5379 32.1479C42.1257 33.7503 39.0421 34.8351 35.6104 35.057C35.3617 35.0816 35.1131 35.0816 34.8644 35.1063C34.6157 35.1063 34.367 35.1063 34.1183 35.1063C29.866 35.1063 26.0115 33.8489 23.2264 31.8027C22.356 31.1617 21.5851 30.4468 20.9385 29.6825C20.4163 29.0662 19.9936 28.4006 19.6454 27.7103C19.5459 27.5377 19.4713 27.3651 19.3967 27.1926C18.9491 26.1325 18.7005 25.0231 18.7005 23.8397H12.3344C12.3344 25.812 12.583 27.7349 13.0555 29.5839C13.155 29.9537 13.2545 30.3235 13.3539 30.6687C13.7021 31.7534 14.0999 32.7889 14.5973 33.7996C14.6719 33.9722 14.7714 34.1448 14.846 34.2927C15.02 34.6132 15.1941 34.9583 15.3682 35.2788C15.4677 35.4268 15.5671 35.5993 15.6417 35.7473C15.8407 36.0677 16.0396 36.3636 16.2386 36.6841C16.8603 37.5963 17.5565 38.4591 18.3026 39.2727C18.5512 39.5439 18.7999 39.8151 19.0735 40.0616C19.5957 40.5793 20.1428 41.0724 20.7147 41.5408C22.7041 43.1679 24.9919 44.4745 27.4787 45.3867C27.8269 45.51 28.1999 45.6332 28.548 45.7565C30.1893 46.2496 31.8803 46.5947 33.6459 46.718C34.2427 46.7673 34.8395 46.7919 35.4363 46.7919H74.9012C82.8837 46.7919 89.9461 42.7735 94.099 36.6841C94.3974 36.2157 94.6958 35.7473 94.9942 35.2788C95.1931 34.9583 95.3672 34.6379 95.5413 34.2927C95.6159 34.1201 95.7153 33.9722 95.7899 33.7996C96.2873 32.7889 96.6852 31.7534 97.0333 30.6687C97.1577 30.2989 97.2571 29.9537 97.3317 29.5839C97.8042 27.7596 98.0529 25.8366 98.0529 23.8397C98.0529 21.8674 97.8042 19.9445 97.3317 18.0955C97.282 17.7996 97.1577 17.4298 97.0582 17.0847ZM87.7577 14.0277C87.7328 14.1756 87.708 14.3235 87.6831 14.4714C87.5836 14.8412 87.3847 15.1864 87.0863 15.4576C86.9868 15.5562 86.8874 15.6301 86.7879 15.7041C86.7382 15.7288 86.6884 15.7781 86.6387 15.8027C86.5641 15.852 86.4895 15.8767 86.4149 15.926C86.3403 15.9753 86.2408 15.9999 86.1662 16.0246C86.0667 16.0493 85.9672 16.0739 85.8678 16.0986C85.7186 16.1232 85.5694 16.1479 85.4202 16.1479C85.2958 16.1479 85.1466 16.1232 85.0223 16.1232C84.8731 16.0986 84.7487 16.0739 84.6244 15.9999C84.5995 15.9999 84.5747 15.9753 84.5498 15.9753C84.2763 15.852 84.0027 15.6795 83.7789 15.4576C82.8588 14.5454 82.8588 13.0415 83.7789 12.1294C84.0027 11.9075 84.2763 11.7349 84.5498 11.6117C84.5747 11.6117 84.5995 11.587 84.6244 11.587C84.7487 11.5377 84.8979 11.513 85.0223 11.4884C85.1466 11.4637 85.2958 11.4391 85.4202 11.4391C85.5694 11.4391 85.7186 11.4637 85.8678 11.4884C85.9672 11.513 86.0667 11.5377 86.1662 11.5624C86.2657 11.587 86.3403 11.6117 86.4149 11.661C86.4895 11.6856 86.5641 11.7349 86.6387 11.7842C86.6884 11.8089 86.7382 11.8582 86.7879 11.8828C86.8874 11.9568 86.9868 12.0308 87.0863 12.1294C87.3847 12.4252 87.5836 12.7704 87.6831 13.1155C87.7329 13.2634 87.7577 13.4113 87.7577 13.5593C87.7577 13.7318 87.7577 13.8798 87.7577 14.0277Z"
                    fill="#dad6ca"
                ></path>
                <path
                    d="M122.199 21.3251C117.599 20.7334 115.41 20.4622 115.41 18.6625C115.41 17.4052 116.679 16.3205 120.11 16.3205C123.766 16.3205 125.357 17.5038 125.581 19.2542H133.191C132.917 13.4607 128.217 9.71338 120.657 9.71338C112.725 9.71338 107.751 12.869 107.751 19.1063C107.751 24.3574 111.257 26.9707 118.419 27.8829C124.388 28.6964 126.128 29.2388 126.128 30.9645C126.128 32.6903 124.388 33.3559 121.03 33.3559C116.828 33.3559 115.187 32.0986 114.963 30.3729H107.353C107.577 36.1664 112.327 39.963 120.483 39.963C128.64 39.963 133.788 36.6101 133.788 30.5701C133.788 24.9984 129.908 22.2866 122.199 21.3251Z"
                    fill="#dad6ca"
                ></path>
                <path
                    d="M151.294 9.71338C142.64 9.71338 136.896 15.359 136.424 24.3081C136.324 25.9353 136.25 27.3405 136.25 29.51V48H144.133V36.1664C145.873 38.4345 148.833 39.7411 152.563 39.7411C160.545 39.7411 166.687 33.5038 166.687 25.2696C166.663 16.3205 160.968 9.71338 151.294 9.71338ZM151.444 32.493C147.017 32.493 144.157 29.51 144.157 24.8012C144.157 20.0924 147.117 17.0354 151.444 17.0354C155.771 17.0354 158.78 20.2897 158.78 24.8012C158.78 29.2881 155.82 32.493 151.444 32.493Z"
                    fill="#dad6ca"
                ></path>
                <path d="M177.604 0H169.672V39.322H177.604V0Z" fill="#dad6ca"></path>
                <path
                    d="M211.026 23.9137C210.479 15.3343 204.511 9.71338 196.081 9.71338C186.731 9.71338 180.588 15.9014 180.588 25.3436C180.588 34.1202 186.333 39.9876 194.912 39.9876C199.289 39.9876 202.621 38.5331 204.436 35.6487L204.536 39.3467H211.747L211.374 31.2604C211.25 27.4884 211.151 25.9846 211.026 23.9137ZM195.932 32.8135C191.381 32.8135 188.546 29.7812 188.546 24.9491C188.546 20.1171 191.555 17.0354 195.932 17.0354C200.308 17.0354 203.143 20.2403 203.143 24.9491C203.168 30.0524 200.259 32.8135 195.932 32.8135Z"
                    fill="#dad6ca"
                ></path>
                <path
                    d="M228.533 21.3251C223.932 20.7334 221.744 20.4622 221.744 18.6625C221.744 17.4052 223.012 16.3205 226.444 16.3205C230.1 16.3205 231.691 17.5038 231.915 19.2542H239.524C239.251 13.4607 234.551 9.71338 226.991 9.71338C219.083 9.71338 214.11 12.8443 214.11 19.0816C214.11 24.3328 217.616 26.946 224.778 27.8582C230.746 28.6718 232.487 29.2141 232.487 30.9399C232.487 32.6656 230.746 33.3313 227.389 33.3313C223.186 33.3313 221.545 32.0739 221.321 30.3482H213.712C213.936 36.1417 218.685 39.9383 226.842 39.9383C234.999 39.9383 240.146 36.5855 240.146 30.5454C240.146 24.9984 236.267 22.2866 228.533 21.3251Z"
                    fill="#dad6ca"
                ></path>
                <path
                    d="M259.17 9.71341C255.39 9.71341 252.704 10.7982 250.914 12.7458L251.013 0H243.081V39.322H251.013V23.963C251.013 19.4515 253.028 16.8136 257.479 16.8136C261.905 16.8136 263.397 19.0817 263.397 23.4206V39.322H271.33V22.0647C271.33 14.7427 266.357 9.71341 259.17 9.71341Z"
                    fill="#dad6ca"
                ></path>
                <path
                    d="M301.196 21.3251C296.596 20.7334 294.407 20.4622 294.407 18.6625C294.407 17.4052 295.675 16.3205 299.107 16.3205C302.763 16.3205 304.354 17.5038 304.578 19.2542H312.188C311.914 13.4607 307.214 9.71338 299.654 9.71338C291.721 9.71338 286.748 12.869 286.748 19.1063C286.748 24.3574 290.254 26.9707 297.416 27.8829C303.384 28.6964 305.125 29.2388 305.125 30.9645C305.125 32.6903 303.384 33.3559 300.027 33.3559C295.825 33.3559 294.183 32.0986 293.96 30.3729H286.35C286.574 36.1664 291.324 39.963 299.48 39.963C307.637 39.963 312.784 36.6101 312.784 30.5701C312.784 24.9984 308.905 22.2866 301.196 21.3251Z"
                    fill="#dad6ca"
                ></path>
                <path
                    d="M330.291 9.71338C321.637 9.71338 315.893 15.359 315.42 24.3081C315.321 25.9353 315.246 27.3405 315.246 29.51V48H323.129V36.1664C324.87 38.4345 327.829 39.7411 331.559 39.7411C339.542 39.7411 345.684 33.5038 345.684 25.2696C345.684 16.3205 339.989 9.71338 330.291 9.71338ZM330.465 32.493C326.039 32.493 323.179 29.51 323.179 24.8012C323.179 20.0924 326.138 17.0354 330.465 17.0354C334.792 17.0354 337.801 20.2897 337.801 24.8012C337.801 29.2881 334.842 32.493 330.465 32.493Z"
                    fill="#dad6ca"
                ></path>
                <path
                    d="M363.465 9.71338C354.487 9.71338 348.146 15.9507 348.146 24.8505C348.146 33.7504 354.487 39.9876 363.465 39.9876C372.442 39.9876 378.783 33.7504 378.783 24.8505C378.783 15.9507 372.417 9.71338 363.465 9.71338ZM363.39 32.641C359.063 32.641 356.054 29.436 356.054 24.8259C356.054 20.2157 359.113 16.9615 363.44 16.9615C367.767 16.9615 370.825 20.2157 370.825 24.8259C370.825 29.436 367.767 32.641 363.39 32.641Z"
                    fill="#dad6ca"
                ></path>
                <path
                    d="M381.518 24.0862V39.3219H389.451V24.0862C389.451 19.5253 390.918 17.8982 395.568 17.8982H397.384V10.5762H394.922C386.044 10.5762 381.518 15.137 381.518 24.0862Z"
                    fill="#dad6ca"
                ></path>
                <path
                    d="M407.48 25.4915V17.6271H417.228V10.5762H407.48V3.35278H399.547V24.456C399.547 35.0323 404.421 39.322 412.354 39.322H416.557L417.427 31.9999C417.154 31.9999 416.656 32.0492 413.971 32.0492C409.843 32.0492 407.48 30.9645 407.48 25.4915Z"
                    fill="#dad6ca"
                ></path>
                <path
                    d="M434.362 21.3251C429.762 20.7334 427.573 20.4622 427.573 18.6625C427.573 17.4052 428.841 16.3205 432.273 16.3205C435.929 16.3205 437.52 17.5038 437.744 19.2542H445.354C445.08 13.4607 440.38 9.71338 432.82 9.71338C424.888 9.71338 419.914 12.869 419.914 19.1063C419.914 24.3574 423.42 26.9707 430.582 27.8829C436.55 28.6964 438.291 29.2388 438.291 30.9645C438.291 32.6903 436.55 33.3559 433.193 33.3559C428.991 33.3559 427.349 32.0986 427.126 30.3729H419.516C419.74 36.1664 424.49 39.963 432.646 39.963C440.803 39.963 445.95 36.6101 445.95 30.5701C445.95 24.9984 442.071 22.2866 434.362 21.3251Z"
                    fill="#dad6ca"
                ></path>
            </g>
            <defs>
                <clipPath id="clip0_636_151">
                    <rect width="446" height="48" fill="#dad6ca"></rect>
                </clipPath>
            </defs>
        </svg>
    }
}

