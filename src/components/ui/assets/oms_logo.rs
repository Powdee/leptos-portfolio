use leptos::*;

#[component]
pub fn OmsLogo() -> impl IntoView {
    view! {
        <svg
            width="167"
            height="145"
            viewBox="0 0 167 145"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <g clip-path="url(#clip0_638_178)">
                <path
                    d="M167.435 1.3064C167.435 49.3092 167.435 97.3119 167.435 145.436C139.882 145.436 112.33 145.436 84.619 145.288C92.1008 131.821 99.7409 118.501 107.381 105.181C107.262 105.045 107.143 104.909 107.025 104.773C99.7338 108.477 92.0369 110.459 83.8824 110.61C75.6574 110.763 68.0329 108.45 60.2143 104.659C62.3115 108.305 64.0988 111.43 65.9038 114.544C69.5379 120.815 73.2121 127.063 76.8104 133.355C79.0955 137.35 81.2738 141.407 83.4999 145.436C55.8348 145.436 28.1696 145.436 0.434814 145.436C0.434814 97.5285 0.434814 49.6215 0.531869 1.52018C0.628925 1.32583 0.824003 1.32581 0.846205 1.48375C6.00059 10.6521 11.0992 19.6818 16.2736 28.6678C22.4778 39.4423 28.7281 50.1904 34.994 60.9291C39.1094 67.9821 43.2405 75.0268 47.4516 82.0227C51.5965 88.9088 57.226 94.2893 64.3853 97.9623C72.3396 102.043 80.7198 103.605 89.667 102.374C98.5719 101.149 106.248 97.4524 112.802 91.4434C118.504 86.2156 122.262 79.5167 126.069 72.8937C130.108 65.8679 134.093 58.8109 138.129 51.7832C142.235 44.6329 146.367 37.4972 150.502 30.3637C153.996 24.3351 157.542 18.3358 161.007 12.2908C163.091 8.65591 165.065 4.95816 167.175 1.30742C167.26 1.32616 167.435 1.3064 167.435 1.3064Z"
                    fill="#212529"
                ></path>
                <path
                    d="M83.6449 145.435C81.2738 141.407 79.0955 137.35 76.8105 133.355C73.2121 127.063 69.5379 120.815 65.9038 114.544C64.0988 111.43 62.3116 108.305 60.2144 104.659C68.033 108.45 75.6575 110.763 83.8825 110.61C92.0369 110.459 99.7338 108.477 107.025 104.773C107.143 104.909 107.262 105.045 107.381 105.181C99.7409 118.501 92.1008 131.821 84.4152 145.288C84.1765 145.435 83.9832 145.435 83.6449 145.435Z"
                    fill="#F8F9FA"
                ></path>
                <path
                    d="M167.435 1.08867C167.435 1.30639 167.261 1.32615 167.042 1.17159C166.178 0.968163 165.532 0.876591 164.887 0.876512C110.941 0.869923 56.9944 0.86861 3.04822 0.889335C2.30679 0.88962 1.56548 1.17401 0.824116 1.32581C0.824116 1.32581 0.629038 1.32582 0.531982 1.3161C0.579893 1.0161 0.673262 0.529719 0.880244 0.475121C1.35469 0.349971 1.88458 0.435516 2.39196 0.435516C56.7539 0.435516 111.116 0.433994 165.478 0.45357C166.13 0.453805 166.783 0.725809 167.435 1.08867Z"
                    fill="#F8F9FA"
                ></path>
                <path
                    d="M0.846191 1.4836C1.56535 1.17388 2.30667 0.889486 3.04809 0.889201C56.9943 0.868476 110.94 0.869789 164.887 0.876378C165.532 0.876457 166.178 0.968029 166.956 1.15271C165.065 4.95801 163.091 8.65576 161.007 12.2906C157.542 18.3357 153.996 24.335 150.502 30.3635C146.367 37.4971 142.235 44.6328 138.129 51.783C134.093 58.8108 130.108 65.8678 126.069 72.8936C122.261 79.5165 118.504 86.2154 112.802 91.4433C106.248 97.4522 98.5719 101.149 89.667 102.374C80.7198 103.605 72.3396 102.043 64.3853 97.9622C57.226 94.2892 51.5965 88.9086 47.4515 82.0226C43.2405 75.0267 39.1094 67.9819 34.994 60.9289C28.7281 50.1903 22.4778 39.4422 16.2736 28.6677C11.0992 19.6817 6.00058 10.6519 0.846191 1.4836ZM102.231 49.3966C97.4861 44.3308 91.5747 41.6175 84.7149 41.4502C69.7527 41.0854 58.4698 53.4331 59.0244 67.4847C59.6036 82.1569 73.0849 93.703 87.5589 91.3189C97.2588 89.7212 104.187 84.2627 107.493 74.9212C110.786 65.6172 108.921 57.0955 102.231 49.3966ZM117.954 53.1191C121.783 51.6996 122.721 50.027 121.273 46.255C120.14 43.3006 118.584 40.5086 117.115 37.4332C119.203 35.3772 121.483 33.1523 123.74 30.9033C125.22 29.4271 125.358 26.4805 124.043 25.1575C122.402 23.5056 119.865 23.528 118.089 25.2571C116.324 26.976 114.514 28.6606 112.885 30.5026C111.777 31.7547 110.985 31.8077 109.477 30.9721C106.902 29.5443 104.137 28.3845 101.337 27.459C98.4009 26.4885 96.6384 27.4948 95.8689 30.0734C95.1318 32.5431 96.2545 34.2662 99.113 35.3055C100.833 35.9306 102.583 36.5717 104.152 37.4927C108.977 40.3257 112.132 44.4998 113.643 49.9152C114.194 51.8927 115.387 53.1622 117.954 53.1191ZM60.0405 29.3703C59.057 29.9341 57.9339 30.3469 57.1236 31.0979C55.9797 32.1581 55.2988 31.7043 54.4406 30.8027C52.7943 29.0732 51.0783 27.41 49.3882 25.7223C47.2425 23.5797 44.783 23.3585 42.9745 25.1412C41.4345 26.6593 41.6831 29.7734 43.5221 31.5896C45.683 33.7238 47.8693 35.8323 49.659 37.5761C48.1539 40.9552 46.6884 43.936 45.4955 47.0224C44.562 49.4374 45.2982 51.2987 46.7577 52.3671C49.1356 54.1077 52.4861 52.655 53.5437 48.7895C54.6602 44.7088 56.9346 41.1489 60.5029 38.8156C63.0115 37.1752 65.9162 36.1402 68.6505 34.8484C70.7647 33.8494 71.7941 31.8665 71.1126 30.0584C70.2277 27.7102 68.283 26.6341 65.9487 27.3214C64.0554 27.8788 62.2015 28.57 60.0405 29.3703Z"
                    fill="#F8F9FA"
                ></path>
                <path
                    d="M102.34 49.5071C108.921 57.0955 110.786 65.6173 107.493 74.9212C104.187 84.2628 97.2589 89.7212 87.5589 91.319C73.085 93.7031 59.6037 82.157 59.0245 67.4848C58.4698 53.4332 69.7527 41.0855 84.715 41.4503C91.5747 41.6175 97.4861 44.3308 102.34 49.5071Z"
                    fill="#6C757D"
                ></path>
                <path
                    d="M117.778 53.1207C115.387 53.1622 114.194 51.8926 113.643 49.9152C112.132 44.4997 108.977 40.3256 104.152 37.4926C102.583 36.5717 100.833 35.9306 99.1132 35.3054C96.2546 34.2662 95.1319 32.5431 95.869 30.0733C96.6385 27.4948 98.401 26.4885 101.337 27.459C104.137 28.3845 106.902 29.5442 109.478 30.972C110.985 31.8077 111.777 31.7546 112.885 30.5025C114.514 28.6605 116.324 26.9759 118.089 25.2571C119.865 23.528 122.402 23.5055 124.044 25.1575C125.358 26.4804 125.221 29.427 123.74 30.9033C121.483 33.1523 119.203 35.3771 117.115 37.4332C118.584 40.5085 120.14 43.3005 121.273 46.2549C122.721 50.027 121.783 51.6995 117.778 53.1207Z"
                    fill="#212529"
                ></path>
                <path
                    d="M60.1855 29.286C62.2017 28.5699 64.0556 27.8788 65.9489 27.3213C68.2832 26.6341 70.2279 27.7102 71.1128 30.0583C71.7943 31.8664 70.7649 33.8494 68.6507 34.8483C65.9164 36.1402 63.0117 37.1752 60.5031 38.8155C56.9348 41.1488 54.6604 44.7087 53.5439 48.7895C52.4863 52.6549 49.1358 54.1076 46.7579 52.367C45.2985 51.2986 44.5622 49.4374 45.4957 47.0224C46.6886 43.936 48.1541 40.9551 49.6592 37.576C47.8695 35.8322 45.6832 33.7238 43.5223 31.5895C41.6833 29.7733 41.4347 26.6592 42.9747 25.1411C44.7832 23.3584 47.2427 23.5797 49.3884 25.7222C51.0785 27.4099 52.7945 29.0731 54.4408 30.8027C55.299 31.7042 55.9799 32.1581 57.1238 31.0979C57.9342 30.3468 59.0572 29.934 60.1855 29.286Z"
                    fill="#212529"
                ></path>
            </g>
            <defs>
                <clipPath id="clip0_638_178">
                    <rect width="167" height="145" rx="64" fill="white"></rect>
                </clipPath>
            </defs>
        </svg>
    }
}

